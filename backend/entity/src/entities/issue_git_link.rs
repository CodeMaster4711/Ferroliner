use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "issue_git_link")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub issue_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub pr_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::issue::Entity",
        from = "Column::IssueId",
        to = "super::issue::Column::Id",
        on_delete = "Cascade"
    )]
    Issue,
    #[sea_orm(
        belongs_to = "super::git_pull_request::Entity",
        from = "Column::PrId",
        to = "super::git_pull_request::Column::Id",
        on_delete = "Cascade"
    )]
    GitPullRequest,
}

impl Related<super::issue::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Issue.def()
    }
}

impl Related<super::git_pull_request::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GitPullRequest.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
