use entity::{
    comment, comment_reaction, issue_activity, Comment, CommentReaction, IssueActivity,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter,
};
use uuid::Uuid;

pub async fn list(db: &DbConn, issue_id: Uuid) -> Result<Vec<comment::Model>, DbErr> {
    Comment::find()
        .filter(comment::Column::IssueId.eq(issue_id))
        .all(db)
        .await
}

pub async fn create(
    db: &DbConn,
    issue_id: Uuid,
    author_id: Uuid,
    body: String,
) -> Result<comment::Model, DbErr> {
    let now = chrono::Utc::now().fixed_offset();
    let model = comment::ActiveModel {
        id: Set(Uuid::new_v4()),
        issue_id: Set(issue_id),
        author_id: Set(Some(author_id)),
        body: Set(body),
        edited_at: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
    };
    model.insert(db).await
}

pub async fn update(
    db: &DbConn,
    comment_id: Uuid,
    author_id: Uuid,
    body: String,
) -> Result<comment::Model, DbErr> {
    let comment = Comment::find_by_id(comment_id)
        .filter(comment::Column::AuthorId.eq(author_id))
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound("comment".into()))?;

    let now = chrono::Utc::now().fixed_offset();
    let mut active: comment::ActiveModel = comment.into();
    active.body = Set(body);
    active.edited_at = Set(Some(now));
    active.updated_at = Set(now);
    active.update(db).await
}

pub async fn delete(db: &DbConn, comment_id: Uuid, author_id: Uuid) -> Result<(), DbErr> {
    let comment = Comment::find_by_id(comment_id)
        .filter(comment::Column::AuthorId.eq(author_id))
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound("comment".into()))?;

    let active: comment::ActiveModel = comment.into();
    active.delete(db).await?;
    Ok(())
}

pub async fn add_reaction(
    db: &DbConn,
    comment_id: Uuid,
    user_id: Uuid,
    emoji: String,
) -> Result<(), DbErr> {
    let exists = CommentReaction::find_by_id((comment_id, user_id, emoji.clone()))
        .one(db)
        .await?;
    if exists.is_none() {
        comment_reaction::ActiveModel {
            comment_id: Set(comment_id),
            user_id: Set(user_id),
            emoji: Set(emoji),
        }
        .insert(db)
        .await?;
    }
    Ok(())
}

pub async fn remove_reaction(
    db: &DbConn,
    comment_id: Uuid,
    user_id: Uuid,
    emoji: String,
) -> Result<(), DbErr> {
    CommentReaction::delete_by_id((comment_id, user_id, emoji))
        .exec(db)
        .await?;
    Ok(())
}

pub async fn list_activity(
    db: &DbConn,
    issue_id: Uuid,
) -> Result<Vec<issue_activity::Model>, DbErr> {
    IssueActivity::find()
        .filter(issue_activity::Column::IssueId.eq(issue_id))
        .all(db)
        .await
}
