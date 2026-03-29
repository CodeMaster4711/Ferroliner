use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "git_integration")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub organization_id: Uuid,
    pub provider: String,
    pub instance_url: String,
    pub display_name: Option<String>,
    pub access_token_enc: Option<String>,
    pub refresh_token_enc: Option<String>,
    pub token_expires_at: Option<DateTimeWithTimeZone>,
    pub installed_by: Option<Uuid>,
    pub webhook_secret_enc: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::organization::Entity",
        from = "Column::OrganizationId",
        to = "super::organization::Column::Id",
        on_delete = "Cascade"
    )]
    Organization,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::InstalledBy",
        to = "super::user::Column::Id",
        on_delete = "SetNull"
    )]
    InstalledBy,
}

impl ActiveModelBehavior for ActiveModel {}
