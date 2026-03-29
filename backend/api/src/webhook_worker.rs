use std::time::Duration;
use tokio::sync::mpsc::Receiver;
use uuid::Uuid;

use crate::{git_service, AppState};
use entity::{webhook_job, WebhookJob};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect,
};

pub async fn run(state: AppState, mut rx: Receiver<Uuid>) {
    let mut interval = tokio::time::interval(Duration::from_secs(5));
    loop {
        tokio::select! {
            Some(job_id) = rx.recv() => {
                process_one(&state, job_id).await;
            }
            _ = interval.tick() => {
                process_pending_batch(&state).await;
            }
        }
    }
}

async fn process_pending_batch(state: &AppState) {
    let jobs = match WebhookJob::find()
        .filter(webhook_job::Column::Status.eq("pending"))
        .order_by_asc(webhook_job::Column::CreatedAt)
        .limit(10)
        .all(&state.db_conn)
        .await
    {
        Ok(j) => j,
        Err(e) => {
            tracing::error!(error = %e, "failed to query pending webhook jobs");
            return;
        }
    };

    for job in jobs {
        process_one(state, job.id).await;
    }
}

async fn process_one(state: &AppState, job_id: Uuid) {
    let job = match WebhookJob::find_by_id(job_id).one(&state.db_conn).await {
        Ok(Some(j)) => j,
        Ok(None) => return,
        Err(e) => {
            tracing::error!(error = %e, job_id = %job_id, "failed to fetch webhook job");
            return;
        }
    };

    if job.status != "pending" {
        return;
    }

    let updated = {
        let mut active: webhook_job::ActiveModel = job.clone().into();
        active.status = Set("processing".to_string());
        active.attempts = Set(job.attempts + 1);
        match active.update(&state.db_conn).await {
            Ok(u) => u,
            Err(e) => {
                tracing::error!(error = %e, job_id = %job_id, "failed to claim webhook job");
                return;
            }
        }
    };

    let aes_key = &state.aes_key;
    let result = git_service::process_webhook_mr(&state.db_conn, aes_key, &updated).await;

    let now = chrono::Utc::now().fixed_offset();
    match result {
        Ok(()) => {
            let mut active: webhook_job::ActiveModel = updated.into();
            active.status = Set("done".to_string());
            active.processed_at = Set(Some(now));
            if let Err(e) = active.update(&state.db_conn).await {
                tracing::error!(error = %e, job_id = %job_id, "failed to mark webhook job done");
            }
        }
        Err(e) => {
            tracing::error!(error = %e, job_id = %job_id, attempts = %updated.attempts, "webhook job failed");
            let new_status = if updated.attempts >= 3 {
                "failed"
            } else {
                "pending"
            };
            let mut active: webhook_job::ActiveModel = updated.into();
            active.status = Set(new_status.to_string());
            active.last_error = Set(Some(e.to_string()));
            if let Err(db_err) = active.update(&state.db_conn).await {
                tracing::error!(error = %db_err, job_id = %job_id, "failed to update webhook job error state");
            }
        }
    }
}
