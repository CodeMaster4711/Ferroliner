use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use entity::User;
use sea_orm::EntityTrait;

use crate::{
    auth::middleware::AuthenticatedUser,
    project_service::{ProjectError, ProjectService},
    AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/organizations/{org_id}/projects",
            axum::routing::get(list_projects).post(create_project),
        )
        .route(
            "/organizations/{org_id}/projects/{project_id}",
            axum::routing::get(get_project)
                .put(update_project)
                .delete(delete_project),
        )
        .route(
            "/organizations/{org_id}/projects/{project_id}/members",
            axum::routing::get(list_members).post(add_member),
        )
        .route(
            "/organizations/{org_id}/projects/{project_id}/members/{user_id}",
            axum::routing::delete(remove_member),
        )
        .route(
            "/organizations/{org_id}/projects/{project_id}/statuses",
            axum::routing::get(list_statuses).post(create_status),
        )
        .route(
            "/organizations/{org_id}/projects/{project_id}/statuses/{status_id}",
            axum::routing::put(update_status).delete(delete_status),
        )
        .route(
            "/organizations/{org_id}/projects/{project_id}/labels",
            axum::routing::get(list_labels).post(create_label),
        )
        .route(
            "/organizations/{org_id}/projects/{project_id}/labels/{label_id}",
            axum::routing::put(update_label).delete(delete_label),
        )
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub identifier: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProjectRequest {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub color: Option<Option<String>>,
    pub icon: Option<Option<String>>,
}

#[derive(Debug, Serialize)]
pub struct ProjectResponse {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub identifier: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub created_by: Option<Uuid>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddMemberRequest {
    pub user_id: Uuid,
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct MemberResponse {
    pub id: Uuid,
    pub project_id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub role: String,
    pub joined_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateStatusRequest {
    pub name: String,
    pub color: String,
    pub status_type: String,
    pub position: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateStatusRequest {
    pub name: Option<String>,
    pub color: Option<String>,
    pub position: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct StatusResponse {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub color: String,
    pub status_type: String,
    pub position: i32,
    pub is_default: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLabelRequest {
    pub name: String,
    pub color: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateLabelRequest {
    pub name: Option<String>,
    pub color: Option<String>,
    pub description: Option<Option<String>>,
}

#[derive(Debug, Serialize)]
pub struct LabelResponse {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub color: String,
    pub description: Option<String>,
}

fn project_err(e: ProjectError) -> StatusCode {
    match e {
        ProjectError::NotFound => StatusCode::NOT_FOUND,
        ProjectError::IdentifierTaken => StatusCode::CONFLICT,
        ProjectError::Db(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn list_projects(
    AuthenticatedUser(_claims): AuthenticatedUser,
    State(state): State<AppState>,
    Path(org_id): Path<Uuid>,
) -> Result<Json<Vec<ProjectResponse>>, StatusCode> {
    let svc = ProjectService::new(state.db_conn.clone());
    let projects = svc.list_projects(org_id).await.map_err(project_err)?;
    Ok(Json(
        projects
            .into_iter()
            .map(|p| ProjectResponse {
                id: p.id,
                organization_id: p.organization_id,
                name: p.name,
                identifier: p.identifier,
                description: p.description,
                color: p.color,
                icon: p.icon,
                created_by: p.created_by,
                created_at: p.created_at.to_string(),
                updated_at: p.updated_at.to_string(),
            })
            .collect(),
    ))
}

async fn create_project(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(state): State<AppState>,
    Path(org_id): Path<Uuid>,
    Json(req): Json<CreateProjectRequest>,
) -> Result<Json<ProjectResponse>, StatusCode> {
    let svc = ProjectService::new(state.db_conn.clone());
    let p = svc
        .create_project(
            org_id,
            req.name,
            req.identifier,
            req.description,
            req.color,
            req.icon,
            claims.user_id,
        )
        .await
        .map_err(project_err)?;
    Ok(Json(ProjectResponse {
        id: p.id,
        organization_id: p.organization_id,
        name: p.name,
        identifier: p.identifier,
        description: p.description,
        color: p.color,
        icon: p.icon,
        created_by: p.created_by,
        created_at: p.created_at.to_string(),
        updated_at: p.updated_at.to_string(),
    }))
}

async fn get_project(
    AuthenticatedUser(_claims): AuthenticatedUser,
    State(state): State<AppState>,
    Path((_org_id, project_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ProjectResponse>, StatusCode> {
    let svc = ProjectService::new(state.db_conn.clone());
    let p = svc.get_project(project_id).await.map_err(project_err)?;
    Ok(Json(ProjectResponse {
        id: p.id,
        organization_id: p.organization_id,
        name: p.name,
        identifier: p.identifier,
        description: p.description,
        color: p.color,
        icon: p.icon,
        created_by: p.created_by,
        created_at: p.created_at.to_string(),
        updated_at: p.updated_at.to_string(),
    }))
}

async fn update_project(
    AuthenticatedUser(_claims): AuthenticatedUser,
    State(state): State<AppState>,
    Path((_org_id, project_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<UpdateProjectRequest>,
) -> Result<Json<ProjectResponse>, StatusCode> {
    let svc = ProjectService::new(state.db_conn.clone());
    let p = svc
        .update_project(project_id, req.name, req.description, req.color, req.icon)
        .await
        .map_err(project_err)?;
    Ok(Json(ProjectResponse {
        id: p.id,
        organization_id: p.organization_id,
        name: p.name,
        identifier: p.identifier,
        description: p.description,
        color: p.color,
        icon: p.icon,
        created_by: p.created_by,
        created_at: p.created_at.to_string(),
        updated_at: p.updated_at.to_string(),
    }))
}

async fn delete_project(
    AuthenticatedUser(_claims): AuthenticatedUser,
    State(state): State<AppState>,
    Path((_org_id, project_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, StatusCode> {
    let svc = ProjectService::new(state.db_conn.clone());
    svc.delete_project(project_id).await.map_err(project_err)?;
    Ok(StatusCode::NO_CONTENT)
}

async fn list_members(
    AuthenticatedUser(_claims): AuthenticatedUser,
    State(state): State<AppState>,
    Path((_org_id, project_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Vec<MemberResponse>>, StatusCode> {
    let svc = ProjectService::new(state.db_conn.clone());
    let members = svc.list_members(project_id).await.map_err(project_err)?;
    let mut response = Vec::with_capacity(members.len());
    for m in members {
        let user = User::find_by_id(m.user_id)
            .one(&state.db_conn)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?;
        response.push(MemberResponse {
            id: m.id,
            project_id: m.project_id,
            user_id: m.user_id,
            username: user.name,
            role: m.role,
            joined_at: m.joined_at.to_string(),
        });
    }
    Ok(Json(response))
}

async fn add_member(
    AuthenticatedUser(_claims): AuthenticatedUser,
    State(state): State<AppState>,
    Path((_org_id, project_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<AddMemberRequest>,
) -> Result<Json<MemberResponse>, StatusCode> {
    let svc = ProjectService::new(state.db_conn.clone());
    let m = svc
        .add_member(project_id, req.user_id, req.role)
        .await
        .map_err(project_err)?;
    let user = User::find_by_id(m.user_id)
        .one(&state.db_conn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(MemberResponse {
        id: m.id,
        project_id: m.project_id,
        user_id: m.user_id,
        username: user.name,
        role: m.role,
        joined_at: m.joined_at.to_string(),
    }))
}

async fn remove_member(
    AuthenticatedUser(_claims): AuthenticatedUser,
    State(state): State<AppState>,
    Path((_org_id, project_id, user_id)): Path<(Uuid, Uuid, Uuid)>,
) -> Result<StatusCode, StatusCode> {
    let svc = ProjectService::new(state.db_conn.clone());
    svc.remove_member(project_id, user_id)
        .await
        .map_err(project_err)?;
    Ok(StatusCode::NO_CONTENT)
}

async fn list_statuses(
    AuthenticatedUser(_claims): AuthenticatedUser,
    State(state): State<AppState>,
    Path((_org_id, project_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Vec<StatusResponse>>, StatusCode> {
    let svc = ProjectService::new(state.db_conn.clone());
    let statuses = svc.list_statuses(project_id).await.map_err(project_err)?;
    Ok(Json(
        statuses
            .into_iter()
            .map(|s| StatusResponse {
                id: s.id,
                project_id: s.project_id,
                name: s.name,
                color: s.color,
                status_type: s.status_type,
                position: s.position,
                is_default: s.is_default,
            })
            .collect(),
    ))
}

async fn create_status(
    AuthenticatedUser(_claims): AuthenticatedUser,
    State(state): State<AppState>,
    Path((_org_id, project_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<CreateStatusRequest>,
) -> Result<Json<StatusResponse>, StatusCode> {
    let svc = ProjectService::new(state.db_conn.clone());
    let s = svc
        .create_status(project_id, req.name, req.color, req.status_type, req.position)
        .await
        .map_err(project_err)?;
    Ok(Json(StatusResponse {
        id: s.id,
        project_id: s.project_id,
        name: s.name,
        color: s.color,
        status_type: s.status_type,
        position: s.position,
        is_default: s.is_default,
    }))
}

async fn update_status(
    AuthenticatedUser(_claims): AuthenticatedUser,
    State(state): State<AppState>,
    Path((_org_id, _project_id, status_id)): Path<(Uuid, Uuid, Uuid)>,
    Json(req): Json<UpdateStatusRequest>,
) -> Result<Json<StatusResponse>, StatusCode> {
    let svc = ProjectService::new(state.db_conn.clone());
    let s = svc
        .update_status(status_id, req.name, req.color, req.position)
        .await
        .map_err(project_err)?;
    Ok(Json(StatusResponse {
        id: s.id,
        project_id: s.project_id,
        name: s.name,
        color: s.color,
        status_type: s.status_type,
        position: s.position,
        is_default: s.is_default,
    }))
}

async fn delete_status(
    AuthenticatedUser(_claims): AuthenticatedUser,
    State(state): State<AppState>,
    Path((_org_id, _project_id, status_id)): Path<(Uuid, Uuid, Uuid)>,
) -> Result<StatusCode, StatusCode> {
    let svc = ProjectService::new(state.db_conn.clone());
    svc.delete_status(status_id).await.map_err(project_err)?;
    Ok(StatusCode::NO_CONTENT)
}

async fn list_labels(
    AuthenticatedUser(_claims): AuthenticatedUser,
    State(state): State<AppState>,
    Path((_org_id, project_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Vec<LabelResponse>>, StatusCode> {
    let svc = ProjectService::new(state.db_conn.clone());
    let labels = svc.list_labels(project_id).await.map_err(project_err)?;
    Ok(Json(
        labels
            .into_iter()
            .map(|l| LabelResponse {
                id: l.id,
                project_id: l.project_id,
                name: l.name,
                color: l.color,
                description: l.description,
            })
            .collect(),
    ))
}

async fn create_label(
    AuthenticatedUser(_claims): AuthenticatedUser,
    State(state): State<AppState>,
    Path((_org_id, project_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<CreateLabelRequest>,
) -> Result<Json<LabelResponse>, StatusCode> {
    let svc = ProjectService::new(state.db_conn.clone());
    let l = svc
        .create_label(project_id, req.name, req.color, req.description)
        .await
        .map_err(project_err)?;
    Ok(Json(LabelResponse {
        id: l.id,
        project_id: l.project_id,
        name: l.name,
        color: l.color,
        description: l.description,
    }))
}

async fn update_label(
    AuthenticatedUser(_claims): AuthenticatedUser,
    State(state): State<AppState>,
    Path((_org_id, _project_id, label_id)): Path<(Uuid, Uuid, Uuid)>,
    Json(req): Json<UpdateLabelRequest>,
) -> Result<Json<LabelResponse>, StatusCode> {
    let svc = ProjectService::new(state.db_conn.clone());
    let l = svc
        .update_label(label_id, req.name, req.color, req.description)
        .await
        .map_err(project_err)?;
    Ok(Json(LabelResponse {
        id: l.id,
        project_id: l.project_id,
        name: l.name,
        color: l.color,
        description: l.description,
    }))
}

async fn delete_label(
    AuthenticatedUser(_claims): AuthenticatedUser,
    State(state): State<AppState>,
    Path((_org_id, _project_id, label_id)): Path<(Uuid, Uuid, Uuid)>,
) -> Result<StatusCode, StatusCode> {
    let svc = ProjectService::new(state.db_conn.clone());
    svc.delete_label(label_id).await.map_err(project_err)?;
    Ok(StatusCode::NO_CONTENT)
}
