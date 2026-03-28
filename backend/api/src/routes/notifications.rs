use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::sse::{Event, Sse},
    Json, Router,
};
use futures_util::stream::Stream;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;
use uuid::Uuid;

use crate::{
    auth::middleware::AuthenticatedUser,
    notification_service,
    sse_service::SseEvent,
    AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/notifications", axum::routing::get(list_notifications))
        .route(
            "/notifications/{notification_id}/read",
            axum::routing::post(mark_read),
        )
        .route(
            "/notifications/read-all",
            axum::routing::post(mark_all_read),
        )
        .route("/notifications/unread-count", axum::routing::get(unread_count))
        .route("/notifications/sse", axum::routing::get(sse_handler))
        .route(
            "/organizations/{org_id}/projects/{project_id}/issues/{issue_id}/subscribe",
            axum::routing::post(subscribe).delete(unsubscribe),
        )
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub unread_only: Option<bool>,
}

async fn list_notifications(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    axum::extract::Query(query): axum::extract::Query<ListQuery>,
) -> Result<Json<Vec<entity::notification::Model>>, StatusCode> {
    notification_service::list_for_user(
        &state.db_conn,
        auth.0.user_id,
        query.unread_only.unwrap_or(false),
    )
    .await
    .map(Json)
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn mark_read(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(notification_id): Path<Uuid>,
) -> StatusCode {
    match notification_service::mark_read(&state.db_conn, notification_id, auth.0.user_id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn mark_all_read(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
) -> StatusCode {
    match notification_service::mark_all_read(&state.db_conn, auth.0.user_id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[derive(Serialize)]
pub struct UnreadCount {
    pub count: u64,
}

async fn unread_count(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
) -> Result<Json<UnreadCount>, StatusCode> {
    notification_service::unread_count(&state.db_conn, auth.0.user_id)
        .await
        .map(|count| Json(UnreadCount { count }))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn sse_handler(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = state.sse_tx.subscribe();
    let user_id = auth.0.user_id;

    let stream = BroadcastStream::new(rx)
        .filter_map(move |msg| {
            let Ok(event) = msg else { return None };
            if !event.user_ids.is_empty() && !event.user_ids.contains(&user_id) {
                return None;
            }
            let data = serde_json::to_string(&event.payload).unwrap_or_default();
            Some(Ok(Event::default().event(event.kind).data(data)))
        });

    Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::default())
}

async fn subscribe(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path((_org_id, _project_id, issue_id)): Path<(Uuid, Uuid, Uuid)>,
) -> StatusCode {
    match notification_service::subscribe(&state.db_conn, auth.0.user_id, issue_id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn unsubscribe(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path((_org_id, _project_id, issue_id)): Path<(Uuid, Uuid, Uuid)>,
) -> StatusCode {
    match notification_service::unsubscribe(&state.db_conn, auth.0.user_id, issue_id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
