use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "git_repository")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub integration_id: Uuid,
    pub project_id: Uuid,
    pub provider_repo_id: String,
    pub full_name: String,
    pub default_branch: Option<String>,
    pub created_at: DateTimeWithTimeZone,
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
    #[sea_orm(
        belongs_to = "super::project::Entity",
        from = "Column::ProjectId",
        to = "super::project::Column::Id",
        on_delete = "Cascade"
    )]
    Project,
}

impl ActiveModelBehavior for ActiveModel {}
