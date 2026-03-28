use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    attachment_service,
    auth::middleware::AuthenticatedUser,
    comment_service,
    issue_service::{IssueError, IssueFilter, IssuePatch, IssueService, IssueWithRelations},
    AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/organizations/{org_id}/projects/{project_id}/issues",
            axum::routing::get(list_issues).post(create_issue),
        )
        .route(
            "/organizations/{org_id}/projects/{project_id}/issues/{issue_id}",
            axum::routing::get(get_issue)
                .put(update_issue)
                .delete(delete_issue),
        )
        .route(
            "/organizations/{org_id}/projects/{project_id}/issues/{issue_id}/relationships",
            axum::routing::get(list_relationships).post(add_relationship),
        )
        .route(
            "/organizations/{org_id}/projects/{project_id}/issues/{issue_id}/relationships/{rel_id}",
            axum::routing::delete(remove_relationship),
        )
        .route(
            "/organizations/{org_id}/projects/{project_id}/issues/{issue_id}/comments",
            axum::routing::get(list_comments).post(create_comment),
        )
        .route(
            "/organizations/{org_id}/projects/{project_id}/issues/{issue_id}/comments/{comment_id}",
            axum::routing::put(update_comment).delete(delete_comment),
        )
        .route(
            "/organizations/{org_id}/projects/{project_id}/issues/{issue_id}/comments/{comment_id}/reactions/{emoji}",
            axum::routing::post(add_reaction).delete(remove_reaction),
        )
        .route(
            "/organizations/{org_id}/projects/{project_id}/issues/{issue_id}/activity",
            axum::routing::get(list_activity),
        )
        .route(
            "/organizations/{org_id}/projects/{project_id}/issues/{issue_id}/attachments",
            axum::routing::get(list_attachments).post(initiate_upload),
        )
        .route(
            "/organizations/{org_id}/projects/{project_id}/issues/{issue_id}/attachments/{attachment_id}",
            axum::routing::delete(delete_attachment),
        )
}

#[derive(Debug, Deserialize)]
pub struct ListIssuesQuery {
    pub status_id: Option<Uuid>,
    pub assignee_id: Option<Uuid>,
    pub priority: Option<i16>,
    pub parent_id: Option<Uuid>,
    pub search: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateIssueRequest {
    pub title: String,
    pub description: Option<String>,
    pub status_id: Option<Uuid>,
    pub priority: Option<i16>,
    pub assignee_id: Option<Uuid>,
    pub label_ids: Option<Vec<Uuid>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateIssueRequest {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct AddRelationshipRequest {
    pub target_issue_id: Uuid,
    pub kind: String,
}

#[derive(Debug, Serialize)]
pub struct RelationshipResponse {
    pub id: Uuid,
    pub source_issue_id: Uuid,
    pub target_issue_id: Uuid,
    pub kind: String,
}

#[derive(Debug, Serialize)]
pub struct LabelRef {
    pub id: Uuid,
    pub name: String,
    pub color: String,
}

#[derive(Debug, Serialize)]
pub struct AssigneeRef {
    pub id: Uuid,
    pub username: String,
}

#[derive(Debug, Serialize)]
pub struct StatusRef {
    pub id: Uuid,
    pub name: String,
    pub color: String,
    pub status_type: String,
}

#[derive(Debug, Serialize)]
pub struct IssueResponse {
    pub id: Uuid,
    pub project_id: Uuid,
    pub number: i32,
    pub identifier: String,
    pub title: String,
    pub description: Option<String>,
    pub status: StatusRef,
    pub priority: i16,
    pub assignee: Option<AssigneeRef>,
    pub parent_id: Option<Uuid>,
    pub due_date: Option<String>,
    pub estimate: Option<i32>,
    pub labels: Vec<LabelRef>,
    pub created_by: Option<Uuid>,
    pub created_at: String,
    pub updated_at: String,
}

fn issue_err(e: IssueError) -> StatusCode {
    match e {
        IssueError::NotFound => StatusCode::NOT_FOUND,
        IssueError::NoDefaultStatus => StatusCode::UNPROCESSABLE_ENTITY,
        IssueError::Db(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

fn to_response(r: IssueWithRelations, identifier_prefix: &str) -> IssueResponse {
    IssueResponse {
        identifier: format!("{}-{}", identifier_prefix, r.issue.number),
        id: r.issue.id,
        project_id: r.issue.project_id,
        number: r.issue.number,
        title: r.issue.title,
        description: r.issue.description,
        status: StatusRef {
            id: r.status.id,
            name: r.status.name,
            color: r.status.color,
            status_type: r.status.status_type,
        },
        priority: r.issue.priority,
        assignee: r.assignee.map(|a| AssigneeRef {
            id: a.id,
            username: a.name,
        }),
        parent_id: r.issue.parent_id,
        due_date: r.issue.due_date.map(|d| d.to_string()),
        estimate: r.issue.estimate,
        labels: r
            .labels
            .into_iter()
            .map(|l| LabelRef {
                id: l.id,
                name: l.name,
                color: l.color,
            })
            .collect(),
        created_by: r.issue.created_by,
        created_at: r.issue.created_at.to_string(),
        updated_at: r.issue.updated_at.to_string(),
    }
}

async fn get_project_identifier(state: &AppState, project_id: Uuid) -> Result<String, StatusCode> {
    use entity::Project;
    use sea_orm::EntityTrait;
    let p = Project::find_by_id(project_id)
        .one(&state.db_conn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(p.identifier)
}

async fn list_issues(
    AuthenticatedUser(_claims): AuthenticatedUser,
    State(state): State<AppState>,
    Path((_org_id, project_id)): Path<(Uuid, Uuid)>,
    Query(q): Query<ListIssuesQuery>,
) -> Result<Json<Vec<IssueResponse>>, StatusCode> {
    let identifier = get_project_identifier(&state, project_id).await?;
    let svc = IssueService::new(state.db_conn.clone());
    let filter = IssueFilter {
        status_ids: q.status_id.map(|id| vec![id]),
        assignee_ids: q.assignee_id.map(|id| vec![id]),
        priority: q.priority,
        parent_id: q.parent_id,
        search: q.search,
        ..Default::default()
    };
    let issues = svc.list_issues(project_id, filter).await.map_err(issue_err)?;
    Ok(Json(
        issues
            .into_iter()
            .map(|r| to_response(r, &identifier))
            .collect(),
    ))
}

async fn create_issue(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(state): State<AppState>,
    Path((_org_id, project_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<CreateIssueRequest>,
) -> Result<Json<IssueResponse>, StatusCode> {
    let identifier = get_project_identifier(&state, project_id).await?;
    let svc = IssueService::new(state.db_conn.clone());
    let issue = svc
        .create_issue(
            project_id,
            req.title,
            req.description,
            req.status_id,
            req.priority.unwrap_or(0),
            req.assignee_id,
            req.label_ids.unwrap_or_default(),
            claims.user_id,
        )
        .await
        .map_err(issue_err)?;
    Ok(Json(to_response(issue, &identifier)))
}

async fn get_issue(
    AuthenticatedUser(_claims): AuthenticatedUser,
    State(state): State<AppState>,
    Path((_org_id, project_id, issue_id)): Path<(Uuid, Uuid, Uuid)>,
) -> Result<Json<IssueResponse>, StatusCode> {
    let identifier = get_project_identifier(&state, project_id).await?;
    let svc = IssueService::new(state.db_conn.clone());
    let issue = svc.get_issue(issue_id).await.map_err(issue_err)?;
    Ok(Json(to_response(issue, &identifier)))
}

async fn update_issue(
    AuthenticatedUser(_claims): AuthenticatedUser,
    State(state): State<AppState>,
    Path((_org_id, project_id, issue_id)): Path<(Uuid, Uuid, Uuid)>,
    Json(req): Json<UpdateIssueRequest>,
) -> Result<Json<IssueResponse>, StatusCode> {
    let identifier = get_project_identifier(&state, project_id).await?;
    let svc = IssueService::new(state.db_conn.clone());
    let patch = IssuePatch {
        title: req.title,
        description: req.description,
        status_id: req.status_id,
        priority: req.priority,
        assignee_id: req.assignee_id,
        parent_id: req.parent_id,
        due_date: req.due_date,
        estimate: req.estimate,
        label_ids: req.label_ids,
    };
    let issue = svc.update_issue(issue_id, patch).await.map_err(issue_err)?;
    Ok(Json(to_response(issue, &identifier)))
}

async fn delete_issue(
    AuthenticatedUser(_claims): AuthenticatedUser,
    State(state): State<AppState>,
    Path((_org_id, _project_id, issue_id)): Path<(Uuid, Uuid, Uuid)>,
) -> Result<StatusCode, StatusCode> {
    let svc = IssueService::new(state.db_conn.clone());
    svc.delete_issue(issue_id).await.map_err(issue_err)?;
    Ok(StatusCode::NO_CONTENT)
}

async fn list_relationships(
    AuthenticatedUser(_claims): AuthenticatedUser,
    State(state): State<AppState>,
    Path((_org_id, _project_id, issue_id)): Path<(Uuid, Uuid, Uuid)>,
) -> Result<Json<Vec<RelationshipResponse>>, StatusCode> {
    let svc = IssueService::new(state.db_conn.clone());
    let rels = svc
        .list_relationships(issue_id)
        .await
        .map_err(issue_err)?;
    Ok(Json(
        rels.into_iter()
            .map(|r| RelationshipResponse {
                id: r.id,
                source_issue_id: r.source_issue_id,
                target_issue_id: r.target_issue_id,
                kind: r.kind,
            })
            .collect(),
    ))
}

async fn add_relationship(
    AuthenticatedUser(_claims): AuthenticatedUser,
    State(state): State<AppState>,
    Path((_org_id, _project_id, issue_id)): Path<(Uuid, Uuid, Uuid)>,
    Json(req): Json<AddRelationshipRequest>,
) -> Result<Json<RelationshipResponse>, StatusCode> {
    let svc = IssueService::new(state.db_conn.clone());
    let r = svc
        .add_relationship(issue_id, req.target_issue_id, req.kind)
        .await
        .map_err(issue_err)?;
    Ok(Json(RelationshipResponse {
        id: r.id,
        source_issue_id: r.source_issue_id,
        target_issue_id: r.target_issue_id,
        kind: r.kind,
    }))
}

async fn remove_relationship(
    AuthenticatedUser(_claims): AuthenticatedUser,
    State(state): State<AppState>,
    Path((_org_id, _project_id, _issue_id, rel_id)): Path<(Uuid, Uuid, Uuid, Uuid)>,
) -> Result<StatusCode, StatusCode> {
    let svc = IssueService::new(state.db_conn.clone());
    svc.remove_relationship(rel_id).await.map_err(issue_err)?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentBody {
    pub body: String,
}

async fn list_comments(
    AuthenticatedUser(_claims): AuthenticatedUser,
    State(state): State<AppState>,
    Path((_org_id, _project_id, issue_id)): Path<(Uuid, Uuid, Uuid)>,
) -> Result<Json<Vec<entity::comment::Model>>, StatusCode> {
    comment_service::list(&state.db_conn, issue_id)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn create_comment(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(state): State<AppState>,
    Path((_org_id, _project_id, issue_id)): Path<(Uuid, Uuid, Uuid)>,
    Json(req): Json<CommentBody>,
) -> Result<Json<entity::comment::Model>, StatusCode> {
    comment_service::create(&state.db_conn, issue_id, claims.user_id, req.body)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn update_comment(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(state): State<AppState>,
    Path((_org_id, _project_id, _issue_id, comment_id)): Path<(Uuid, Uuid, Uuid, Uuid)>,
    Json(req): Json<CommentBody>,
) -> Result<Json<entity::comment::Model>, StatusCode> {
    comment_service::update(&state.db_conn, comment_id, claims.user_id, req.body)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn delete_comment(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(state): State<AppState>,
    Path((_org_id, _project_id, _issue_id, comment_id)): Path<(Uuid, Uuid, Uuid, Uuid)>,
) -> StatusCode {
    match comment_service::delete(&state.db_conn, comment_id, claims.user_id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn add_reaction(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(state): State<AppState>,
    Path((_org_id, _project_id, _issue_id, comment_id, emoji)): Path<(Uuid, Uuid, Uuid, Uuid, String)>,
) -> StatusCode {
    match comment_service::add_reaction(&state.db_conn, comment_id, claims.user_id, emoji).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn remove_reaction(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(state): State<AppState>,
    Path((_org_id, _project_id, _issue_id, comment_id, emoji)): Path<(Uuid, Uuid, Uuid, Uuid, String)>,
) -> StatusCode {
    match comment_service::remove_reaction(&state.db_conn, comment_id, claims.user_id, emoji).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn list_activity(
    AuthenticatedUser(_claims): AuthenticatedUser,
    State(state): State<AppState>,
    Path((_org_id, _project_id, issue_id)): Path<(Uuid, Uuid, Uuid)>,
) -> Result<Json<Vec<entity::issue_activity::Model>>, StatusCode> {
    comment_service::list_activity(&state.db_conn, issue_id)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

#[derive(Debug, Deserialize)]
pub struct InitiateUploadRequest {
    pub filename: String,
    pub mime_type: String,
    pub size_bytes: i64,
}

#[derive(Debug, Serialize)]
pub struct InitiateUploadResponse {
    pub attachment_id: Uuid,
    pub storage_key: String,
}

async fn list_attachments(
    AuthenticatedUser(_claims): AuthenticatedUser,
    State(state): State<AppState>,
    Path((_org_id, _project_id, issue_id)): Path<(Uuid, Uuid, Uuid)>,
) -> Result<Json<Vec<entity::attachment::Model>>, StatusCode> {
    attachment_service::list(&state.db_conn, issue_id)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn initiate_upload(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(state): State<AppState>,
    Path((_org_id, _project_id, issue_id)): Path<(Uuid, Uuid, Uuid)>,
    Json(req): Json<InitiateUploadRequest>,
) -> Result<Json<InitiateUploadResponse>, StatusCode> {
    attachment_service::initiate_upload(
        &state.db_conn,
        issue_id,
        claims.user_id,
        req.filename,
        req.mime_type,
        req.size_bytes,
    )
    .await
    .map(|r| {
        Json(InitiateUploadResponse {
            attachment_id: r.attachment_id,
            storage_key: r.storage_key,
        })
    })
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn delete_attachment(
    AuthenticatedUser(_claims): AuthenticatedUser,
    State(state): State<AppState>,
    Path((_org_id, _project_id, _issue_id, attachment_id)): Path<(Uuid, Uuid, Uuid, Uuid)>,
) -> StatusCode {
    match attachment_service::delete(&state.db_conn, &state.storage, attachment_id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
