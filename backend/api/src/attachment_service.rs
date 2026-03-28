use entity::{attachment, Attachment};
use object_store::{ObjectStore, path::Path};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DbConn, DbErr, EntityTrait};
use std::sync::Arc;
use uuid::Uuid;

pub struct InitiateUploadResult {
    pub attachment_id: Uuid,
    pub storage_key: String,
}

pub async fn initiate_upload(
    db: &DbConn,
    issue_id: Uuid,
    uploaded_by: Uuid,
    filename: String,
    mime_type: String,
    size_bytes: i64,
) -> Result<InitiateUploadResult, DbErr> {
    let attachment_id = Uuid::new_v4();
    let storage_key = format!("attachments/{}/{}", issue_id, attachment_id);

    attachment::ActiveModel {
        id: Set(attachment_id),
        issue_id: Set(issue_id),
        uploaded_by: Set(Some(uploaded_by)),
        filename: Set(filename),
        mime_type: Set(mime_type),
        size_bytes: Set(size_bytes),
        storage_key: Set(storage_key.clone()),
        created_at: Set(chrono::Utc::now().fixed_offset()),
    }
    .insert(db)
    .await?;

    Ok(InitiateUploadResult {
        attachment_id,
        storage_key,
    })
}

pub async fn delete(
    db: &DbConn,
    storage: &Arc<dyn ObjectStore>,
    attachment_id: Uuid,
) -> Result<(), anyhow::Error> {
    let att = Attachment::find_by_id(attachment_id)
        .one(db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("attachment not found"))?;

    let path = Path::from(att.storage_key.as_str());
    storage.delete(&path).await?;

    let active: attachment::ActiveModel = att.into();
    active.delete(db).await?;
    Ok(())
}

pub async fn list(db: &DbConn, issue_id: Uuid) -> Result<Vec<attachment::Model>, DbErr> {
    use sea_orm::{ColumnTrait, QueryFilter};
    Attachment::find()
        .filter(entity::entities::attachment::Column::IssueId.eq(issue_id))
        .all(db)
        .await
}
