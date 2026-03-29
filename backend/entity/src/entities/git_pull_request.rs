use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "git_pull_request")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub repository_id: Uuid,
    pub provider_pr_id: String,
    pub number: i32,
    pub title: String,
    pub state: String,
    pub url: String,
    pub branch: String,
    pub merged_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::git_repository::Entity",
        from = "Column::RepositoryId",
        to = "super::git_repository::Column::Id",
        on_delete = "Cascade"
    )]
    GitRepository,
}

impl ActiveModelBehavior for ActiveModel {}
