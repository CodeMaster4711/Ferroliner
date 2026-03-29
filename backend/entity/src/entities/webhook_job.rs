use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "webhook_job")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub integration_id: Uuid,
    pub provider: String,
    pub event_type: String,
    pub provider_event_id: String,
    pub payload: String,
    pub status: String,
    pub attempts: i16,
    pub last_error: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub processed_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::git_integration::Entity",
        from = "Column::IntegrationId",
        to = "super::git_integration::Column::Id",
        on_delete = "Cascade"
    )]
    GitIntegration,
}

impl ActiveModelBehavior for ActiveModel {}
