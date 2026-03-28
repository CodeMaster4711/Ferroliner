use entity::{
    issue, issue_label, issue_relationship, issue_status, label, user, Issue, IssueLabel,
    IssueRelationship, IssueStatus, Label, User,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait,
    QueryFilter, QueryOrder, Statement,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum IssueError {
    #[error("database error: {0}")]
    Db(#[from] sea_orm::DbErr),
    #[error("issue not found")]
    NotFound,
    #[error("project has no default status")]
    NoDefaultStatus,
}

pub type IssueResult<T> = Result<T, IssueError>;

#[derive(Debug, Serialize)]
pub struct IssueWithRelations {
    pub issue: issue::Model,
    pub status: issue_status::Model,
    pub assignee: Option<user::Model>,
    pub labels: Vec<label::Model>,
}

#[derive(Debug, Deserialize, Default)]
pub struct IssueFilter {
    pub status_ids: Option<Vec<Uuid>>,
    pub assignee_ids: Option<Vec<Uuid>>,
    pub label_ids: Option<Vec<Uuid>>,
    pub priority: Option<i16>,
    pub parent_id: Option<Uuid>,
    pub search: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct IssuePatch {
    pub title: Option<String>,
    pub description: Option<Option<String>>,
    pub status_id: Option<Uuid>,
    pub priority: Option<i16>,
    pub assignee_id: Option<Option<Uuid>>,
    pub parent_id: Option<Option<Uuid>>,
    pub due_date: Option<Option<chrono::NaiveDate>>,
    pub estimate: Option<Option<i32>>,
    pub label_ids: Option<Vec<Uuid>>,
}

pub struct IssueService {
    db: DatabaseConnection,
}

impl IssueService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_issue(
        &self,
        project_id: Uuid,
        title: String,
        description: Option<String>,
        status_id: Option<Uuid>,
        priority: i16,
        assignee_id: Option<Uuid>,
        label_ids: Vec<Uuid>,
        created_by: Uuid,
    ) -> IssueResult<IssueWithRelations> {
        let status_id = match status_id {
            Some(id) => id,
            None => {
                IssueStatus::find()
                    .filter(issue_status::Column::ProjectId.eq(project_id))
                    .filter(issue_status::Column::IsDefault.eq(true))
                    .one(&self.db)
                    .await?
                    .ok_or(IssueError::NoDefaultStatus)?
                    .id
            }
        };

        let number = self.next_issue_number(project_id).await?;
        let now = chrono::Utc::now().naive_utc();
        let id = Uuid::new_v4();

        let model = issue::ActiveModel {
            id: ActiveValue::Set(id),
            project_id: ActiveValue::Set(project_id),
            number: ActiveValue::Set(number),
            title: ActiveValue::Set(title),
            description: ActiveValue::Set(description),
            status_id: ActiveValue::Set(status_id),
            priority: ActiveValue::Set(priority),
            assignee_id: ActiveValue::Set(assignee_id),
            parent_id: ActiveValue::Set(None),
            due_date: ActiveValue::Set(None),
            estimate: ActiveValue::Set(None),
            created_by: ActiveValue::Set(Some(created_by)),
            created_at: ActiveValue::Set(now),
            updated_at: ActiveValue::Set(now),
        };

        let issue = model.insert(&self.db).await?;

        for label_id in &label_ids {
            let lnk = issue_label::ActiveModel {
                issue_id: ActiveValue::Set(id),
                label_id: ActiveValue::Set(*label_id),
            };
            IssueLabel::insert(lnk)
                .exec_without_returning(&self.db)
                .await?;
        }

        self.load_with_relations(issue).await
    }

    async fn next_issue_number(&self, project_id: Uuid) -> IssueResult<i32> {
        let db = &self.db;
        let result = db
            .query_one(Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Postgres,
                "UPDATE project_issue_counter SET next_number = next_number + 1 WHERE project_id = $1 RETURNING (next_number - 1) AS assigned_number",
                [sea_orm::Value::Uuid(Some(Box::new(project_id)))],
            ))
            .await?
            .ok_or_else(|| sea_orm::DbErr::Custom("project counter not found".into()))?;

        Ok(result.try_get::<i32>("", "assigned_number")?)
    }

    pub async fn get_issue(&self, issue_id: Uuid) -> IssueResult<IssueWithRelations> {
        let issue = Issue::find_by_id(issue_id)
            .one(&self.db)
            .await?
            .ok_or(IssueError::NotFound)?;
        self.load_with_relations(issue).await
    }

    pub async fn list_issues(
        &self,
        project_id: Uuid,
        filter: IssueFilter,
    ) -> IssueResult<Vec<IssueWithRelations>> {
        let mut query = Issue::find().filter(issue::Column::ProjectId.eq(project_id));

        if let Some(status_ids) = filter.status_ids {
            query = query.filter(issue::Column::StatusId.is_in(status_ids));
        }
        if let Some(assignee_ids) = filter.assignee_ids {
            query = query.filter(issue::Column::AssigneeId.is_in(assignee_ids));
        }
        if let Some(priority) = filter.priority {
            query = query.filter(issue::Column::Priority.eq(priority));
        }
        if let Some(parent_id) = filter.parent_id {
            query = query.filter(issue::Column::ParentId.eq(parent_id));
        }

        let issues = query
            .order_by_asc(issue::Column::Number)
            .all(&self.db)
            .await?;

        let mut result = Vec::with_capacity(issues.len());
        for issue in issues {
            result.push(self.load_with_relations(issue).await?);
        }
        Ok(result)
    }

    pub async fn update_issue(
        &self,
        issue_id: Uuid,
        patch: IssuePatch,
    ) -> IssueResult<IssueWithRelations> {
        let issue = Issue::find_by_id(issue_id)
            .one(&self.db)
            .await?
            .ok_or(IssueError::NotFound)?;

        let mut active: issue::ActiveModel = issue.into();

        if let Some(t) = patch.title {
            active.title = ActiveValue::Set(t);
        }
        if let Some(d) = patch.description {
            active.description = ActiveValue::Set(d);
        }
        if let Some(s) = patch.status_id {
            active.status_id = ActiveValue::Set(s);
        }
        if let Some(p) = patch.priority {
            active.priority = ActiveValue::Set(p);
        }
        if let Some(a) = patch.assignee_id {
            active.assignee_id = ActiveValue::Set(a);
        }
        if let Some(p) = patch.parent_id {
            active.parent_id = ActiveValue::Set(p);
        }
        if let Some(d) = patch.due_date {
            active.due_date = ActiveValue::Set(d);
        }
        if let Some(e) = patch.estimate {
            active.estimate = ActiveValue::Set(e);
        }

        active.updated_at = ActiveValue::Set(chrono::Utc::now().naive_utc());
        let updated = active.update(&self.db).await?;

        if let Some(label_ids) = patch.label_ids {
            IssueLabel::delete_many()
                .filter(issue_label::Column::IssueId.eq(issue_id))
                .exec(&self.db)
                .await?;
            for label_id in label_ids {
                let lnk = issue_label::ActiveModel {
                    issue_id: ActiveValue::Set(issue_id),
                    label_id: ActiveValue::Set(label_id),
                };
                IssueLabel::insert(lnk)
                    .exec_without_returning(&self.db)
                    .await?;
            }
        }

        self.load_with_relations(updated).await
    }

    pub async fn delete_issue(&self, issue_id: Uuid) -> IssueResult<()> {
        Issue::delete_by_id(issue_id).exec(&self.db).await?;
        Ok(())
    }

    pub async fn add_relationship(
        &self,
        source_id: Uuid,
        target_id: Uuid,
        kind: String,
    ) -> IssueResult<issue_relationship::Model> {
        let model = issue_relationship::ActiveModel {
            id: ActiveValue::Set(Uuid::new_v4()),
            source_issue_id: ActiveValue::Set(source_id),
            target_issue_id: ActiveValue::Set(target_id),
            kind: ActiveValue::Set(kind),
        };
        Ok(model.insert(&self.db).await?)
    }

    pub async fn remove_relationship(&self, relationship_id: Uuid) -> IssueResult<()> {
        IssueRelationship::delete_by_id(relationship_id)
            .exec(&self.db)
            .await?;
        Ok(())
    }

    pub async fn list_relationships(
        &self,
        issue_id: Uuid,
    ) -> IssueResult<Vec<issue_relationship::Model>> {
        Ok(IssueRelationship::find()
            .filter(issue_relationship::Column::SourceIssueId.eq(issue_id))
            .all(&self.db)
            .await?)
    }

    async fn load_with_relations(
        &self,
        issue: issue::Model,
    ) -> IssueResult<IssueWithRelations> {
        let status = IssueStatus::find_by_id(issue.status_id)
            .one(&self.db)
            .await?
            .ok_or(IssueError::NotFound)?;

        let assignee = if let Some(aid) = issue.assignee_id {
            User::find_by_id(aid).one(&self.db).await?
        } else {
            None
        };

        let label_links = IssueLabel::find()
            .filter(issue_label::Column::IssueId.eq(issue.id))
            .all(&self.db)
            .await?;

        let label_ids: Vec<Uuid> = label_links.into_iter().map(|l| l.label_id).collect();
        let labels = if label_ids.is_empty() {
            vec![]
        } else {
            Label::find()
                .filter(label::Column::Id.is_in(label_ids))
                .all(&self.db)
                .await?
        };

        Ok(IssueWithRelations {
            issue,
            status,
            assignee,
            labels,
        })
    }
}
