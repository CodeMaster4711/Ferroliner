use entity::{
    issue_subscription, notification,
    IssueSubscription, Notification,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DbConn, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter,
};
use uuid::Uuid;

pub struct CreateNotification {
    pub user_id: Uuid,
    pub issue_id: Uuid,
    pub kind: String,
    pub actor_id: Option<Uuid>,
}

pub async fn notify_subscribers(
    db: &DbConn,
    issue_id: Uuid,
    kind: &str,
    actor_id: Option<Uuid>,
) -> Result<(), DbErr> {
    let subscribers = IssueSubscription::find()
        .filter(issue_subscription::Column::IssueId.eq(issue_id))
        .all(db)
        .await?;

    for sub in subscribers {
        if actor_id == Some(sub.user_id) {
            continue;
        }
        create(
            db,
            CreateNotification {
                user_id: sub.user_id,
                issue_id,
                kind: kind.to_string(),
                actor_id,
            },
        )
        .await?;
    }
    Ok(())
}

pub async fn create(db: &DbConn, data: CreateNotification) -> Result<notification::Model, DbErr> {
    let now = chrono::Utc::now().fixed_offset();
    let model = notification::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(data.user_id),
        issue_id: Set(data.issue_id),
        kind: Set(data.kind),
        actor_id: Set(data.actor_id),
        read_at: Set(None),
        created_at: Set(now),
    };
    model.insert(db).await
}

pub async fn list_for_user(
    db: &DbConn,
    user_id: Uuid,
    unread_only: bool,
) -> Result<Vec<notification::Model>, DbErr> {
    let query = Notification::find().filter(notification::Column::UserId.eq(user_id));
    if unread_only {
        query
            .filter(notification::Column::ReadAt.is_null())
            .all(db)
            .await
    } else {
        query.all(db).await
    }
}

pub async fn mark_read(db: &DbConn, notification_id: Uuid, user_id: Uuid) -> Result<(), DbErr> {
    let notif = Notification::find_by_id(notification_id)
        .filter(notification::Column::UserId.eq(user_id))
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound("notification".into()))?;

    let mut active: notification::ActiveModel = notif.into();
    active.read_at = Set(Some(chrono::Utc::now().fixed_offset()));
    active.update(db).await?;
    Ok(())
}

pub async fn mark_all_read(db: &DbConn, user_id: Uuid) -> Result<(), DbErr> {
    use sea_orm::QuerySelect;
    let unread = Notification::find()
        .filter(notification::Column::UserId.eq(user_id))
        .filter(notification::Column::ReadAt.is_null())
        .all(db)
        .await?;

    let now = chrono::Utc::now().fixed_offset();
    for notif in unread {
        let mut active: notification::ActiveModel = notif.into();
        active.read_at = Set(Some(now));
        active.update(db).await?;
    }
    Ok(())
}

pub async fn unread_count(db: &DbConn, user_id: Uuid) -> Result<u64, DbErr> {
    Notification::find()
        .filter(notification::Column::UserId.eq(user_id))
        .filter(notification::Column::ReadAt.is_null())
        .count(db)
        .await
}

pub async fn subscribe(db: &DbConn, user_id: Uuid, issue_id: Uuid) -> Result<(), DbErr> {
    let exists = IssueSubscription::find_by_id((user_id, issue_id))
        .one(db)
        .await?;
    if exists.is_none() {
        issue_subscription::ActiveModel {
            user_id: Set(user_id),
            issue_id: Set(issue_id),
        }
        .insert(db)
        .await?;
    }
    Ok(())
}

pub async fn unsubscribe(db: &DbConn, user_id: Uuid, issue_id: Uuid) -> Result<(), DbErr> {
    IssueSubscription::delete_by_id((user_id, issue_id))
        .exec(db)
        .await?;
    Ok(())
}
