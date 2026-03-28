use entity::{
    issue_status, label, project, project_issue_counter, project_member, IssueStatus, Label,
    Project, ProjectIssueCounter, ProjectMember,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
    QueryOrder,
};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("database error: {0}")]
    Db(#[from] sea_orm::DbErr),
    #[error("project not found")]
    NotFound,
    #[error("identifier already taken")]
    IdentifierTaken,
}

pub type ProjectResult<T> = Result<T, ProjectError>;

pub struct ProjectService {
    db: DatabaseConnection,
}

impl ProjectService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_project(
        &self,
        org_id: Uuid,
        name: String,
        identifier: String,
        description: Option<String>,
        color: Option<String>,
        icon: Option<String>,
        created_by: Uuid,
    ) -> ProjectResult<project::Model> {
        let exists = Project::find()
            .filter(project::Column::OrganizationId.eq(org_id))
            .filter(project::Column::Identifier.eq(&identifier))
            .one(&self.db)
            .await?;

        if exists.is_some() {
            return Err(ProjectError::IdentifierTaken);
        }

        let now = chrono::Utc::now().naive_utc();
        let id = Uuid::new_v4();

        let model = project::ActiveModel {
            id: ActiveValue::Set(id),
            organization_id: ActiveValue::Set(org_id),
            name: ActiveValue::Set(name),
            identifier: ActiveValue::Set(identifier.to_uppercase()),
            description: ActiveValue::Set(description),
            color: ActiveValue::Set(color),
            icon: ActiveValue::Set(icon),
            created_by: ActiveValue::Set(Some(created_by)),
            created_at: ActiveValue::Set(now),
            updated_at: ActiveValue::Set(now),
        };

        let project = model.insert(&self.db).await?;

        let counter = project_issue_counter::ActiveModel {
            project_id: ActiveValue::Set(id),
            next_number: ActiveValue::Set(1),
        };
        ProjectIssueCounter::insert(counter)
            .exec_without_returning(&self.db)
            .await?;

        self.seed_default_statuses(id).await?;

        Ok(project)
    }

    async fn seed_default_statuses(&self, project_id: Uuid) -> ProjectResult<()> {
        let defaults = [
            ("Backlog", "#6b7280", "backlog", 0),
            ("Todo", "#3b82f6", "todo", 1),
            ("In Progress", "#f59e0b", "in_progress", 2),
            ("Done", "#10b981", "done", 3),
            ("Cancelled", "#ef4444", "cancelled", 4),
        ];

        for (name, color, status_type, position) in defaults {
            let id = Uuid::new_v4();
            let model = issue_status::ActiveModel {
                id: ActiveValue::Set(id),
                project_id: ActiveValue::Set(project_id),
                name: ActiveValue::Set(name.to_string()),
                color: ActiveValue::Set(color.to_string()),
                status_type: ActiveValue::Set(status_type.to_string()),
                position: ActiveValue::Set(position),
                is_default: ActiveValue::Set(position == 1),
            };
            IssueStatus::insert(model)
                .exec_without_returning(&self.db)
                .await?;
        }
        Ok(())
    }

    pub async fn get_project(&self, project_id: Uuid) -> ProjectResult<project::Model> {
        Project::find_by_id(project_id)
            .one(&self.db)
            .await?
            .ok_or(ProjectError::NotFound)
    }

    pub async fn list_projects(&self, org_id: Uuid) -> ProjectResult<Vec<project::Model>> {
        Ok(Project::find()
            .filter(project::Column::OrganizationId.eq(org_id))
            .order_by_asc(project::Column::CreatedAt)
            .all(&self.db)
            .await?)
    }

    pub async fn update_project(
        &self,
        project_id: Uuid,
        name: Option<String>,
        description: Option<Option<String>>,
        color: Option<Option<String>>,
        icon: Option<Option<String>>,
    ) -> ProjectResult<project::Model> {
        let project = self.get_project(project_id).await?;
        let mut active: project::ActiveModel = project.into();
        if let Some(n) = name {
            active.name = ActiveValue::Set(n);
        }
        if let Some(d) = description {
            active.description = ActiveValue::Set(d);
        }
        if let Some(c) = color {
            active.color = ActiveValue::Set(c);
        }
        if let Some(i) = icon {
            active.icon = ActiveValue::Set(i);
        }
        active.updated_at = ActiveValue::Set(chrono::Utc::now().naive_utc());
        Ok(active.update(&self.db).await?)
    }

    pub async fn delete_project(&self, project_id: Uuid) -> ProjectResult<()> {
        Project::delete_by_id(project_id).exec(&self.db).await?;
        Ok(())
    }

    pub async fn add_member(
        &self,
        project_id: Uuid,
        user_id: Uuid,
        role: String,
    ) -> ProjectResult<project_member::Model> {
        let existing = ProjectMember::find()
            .filter(project_member::Column::ProjectId.eq(project_id))
            .filter(project_member::Column::UserId.eq(user_id))
            .one(&self.db)
            .await?;

        if let Some(m) = existing {
            let mut active: project_member::ActiveModel = m.into();
            active.role = ActiveValue::Set(role);
            return Ok(active.update(&self.db).await?);
        }

        let model = project_member::ActiveModel {
            id: ActiveValue::Set(Uuid::new_v4()),
            project_id: ActiveValue::Set(project_id),
            user_id: ActiveValue::Set(user_id),
            role: ActiveValue::Set(role),
            joined_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
        };
        Ok(model.insert(&self.db).await?)
    }

    pub async fn remove_member(&self, project_id: Uuid, user_id: Uuid) -> ProjectResult<()> {
        ProjectMember::delete_many()
            .filter(project_member::Column::ProjectId.eq(project_id))
            .filter(project_member::Column::UserId.eq(user_id))
            .exec(&self.db)
            .await?;
        Ok(())
    }

    pub async fn list_members(
        &self,
        project_id: Uuid,
    ) -> ProjectResult<Vec<project_member::Model>> {
        Ok(ProjectMember::find()
            .filter(project_member::Column::ProjectId.eq(project_id))
            .all(&self.db)
            .await?)
    }

    pub async fn list_statuses(
        &self,
        project_id: Uuid,
    ) -> ProjectResult<Vec<issue_status::Model>> {
        Ok(IssueStatus::find()
            .filter(issue_status::Column::ProjectId.eq(project_id))
            .order_by_asc(issue_status::Column::Position)
            .all(&self.db)
            .await?)
    }

    pub async fn create_status(
        &self,
        project_id: Uuid,
        name: String,
        color: String,
        status_type: String,
        position: i32,
    ) -> ProjectResult<issue_status::Model> {
        let model = issue_status::ActiveModel {
            id: ActiveValue::Set(Uuid::new_v4()),
            project_id: ActiveValue::Set(project_id),
            name: ActiveValue::Set(name),
            color: ActiveValue::Set(color),
            status_type: ActiveValue::Set(status_type),
            position: ActiveValue::Set(position),
            is_default: ActiveValue::Set(false),
        };
        Ok(model.insert(&self.db).await?)
    }

    pub async fn update_status(
        &self,
        status_id: Uuid,
        name: Option<String>,
        color: Option<String>,
        position: Option<i32>,
    ) -> ProjectResult<issue_status::Model> {
        let status = IssueStatus::find_by_id(status_id)
            .one(&self.db)
            .await?
            .ok_or(ProjectError::NotFound)?;

        let mut active: issue_status::ActiveModel = status.into();
        if let Some(n) = name {
            active.name = ActiveValue::Set(n);
        }
        if let Some(c) = color {
            active.color = ActiveValue::Set(c);
        }
        if let Some(p) = position {
            active.position = ActiveValue::Set(p);
        }
        Ok(active.update(&self.db).await?)
    }

    pub async fn delete_status(&self, status_id: Uuid) -> ProjectResult<()> {
        IssueStatus::delete_by_id(status_id).exec(&self.db).await?;
        Ok(())
    }

    pub async fn list_labels(&self, project_id: Uuid) -> ProjectResult<Vec<label::Model>> {
        Ok(Label::find()
            .filter(label::Column::ProjectId.eq(project_id))
            .all(&self.db)
            .await?)
    }

    pub async fn create_label(
        &self,
        project_id: Uuid,
        name: String,
        color: String,
        description: Option<String>,
    ) -> ProjectResult<label::Model> {
        let model = label::ActiveModel {
            id: ActiveValue::Set(Uuid::new_v4()),
            project_id: ActiveValue::Set(project_id),
            name: ActiveValue::Set(name),
            color: ActiveValue::Set(color),
            description: ActiveValue::Set(description),
        };
        Ok(model.insert(&self.db).await?)
    }

    pub async fn update_label(
        &self,
        label_id: Uuid,
        name: Option<String>,
        color: Option<String>,
        description: Option<Option<String>>,
    ) -> ProjectResult<label::Model> {
        let lbl = Label::find_by_id(label_id)
            .one(&self.db)
            .await?
            .ok_or(ProjectError::NotFound)?;

        let mut active: label::ActiveModel = lbl.into();
        if let Some(n) = name {
            active.name = ActiveValue::Set(n);
        }
        if let Some(c) = color {
            active.color = ActiveValue::Set(c);
        }
        if let Some(d) = description {
            active.description = ActiveValue::Set(d);
        }
        Ok(active.update(&self.db).await?)
    }

    pub async fn delete_label(&self, label_id: Uuid) -> ProjectResult<()> {
        Label::delete_by_id(label_id).exec(&self.db).await?;
        Ok(())
    }
}
