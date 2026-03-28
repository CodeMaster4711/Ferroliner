use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "issue")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub project_id: Uuid,
    pub number: i32,
    pub title: String,
    pub description: Option<String>,
    pub status_id: Uuid,
    pub priority: i16,
    pub assignee_id: Option<Uuid>,
    pub parent_id: Option<Uuid>,
    pub due_date: Option<chrono::NaiveDate>,
    pub estimate: Option<i32>,
    pub created_by: Option<Uuid>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::project::Entity",
        from = "Column::ProjectId",
        to = "super::project::Column::Id",
        on_delete = "Cascade"
    )]
    Project,
    #[sea_orm(
        belongs_to = "super::issue_status::Entity",
        from = "Column::StatusId",
        to = "super::issue_status::Column::Id",
        on_delete = "Restrict"
    )]
    IssueStatus,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::AssigneeId",
        to = "super::user::Column::Id",
        on_delete = "SetNull"
    )]
    Assignee,
}

impl Related<super::project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Project.def()
    }
}

impl Related<super::issue_status::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IssueStatus.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
