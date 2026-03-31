use axum::{
    body::Bytes,
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    Json, Router,
};
use base64::Engine;
use constant_time_eq::constant_time_eq;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use uuid::Uuid;

use crate::{
    auth::rbac::{CanConnectGit, CanDisconnectGit, CanManageOrganization, CanViewIssues},
    git_service,
    AppState,
};

fn rewrite_for_backend(url: &str) -> String {
    let internal = std::env::var("GIT_INTERNAL_HOST").unwrap_or_default();
    if internal.is_empty() {
        return url.to_string();
    }
    url.replace("localhost", &internal)
        .replace("127.0.0.1", &internal)
}

pub fn jwt_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/organizations/{org_id}/git/integrations",
            axum::routing::get(list_integrations).post(create_integration_manual),
        )
        .route(
            "/organizations/{org_id}/git/integrations/{id}",
            axum::routing::delete(delete_integration),
        )
        .route(
            "/organizations/{org_id}/git/integrations/oauth/authorize",
            axum::routing::get(oauth_authorize),
        )
        .route(
            "/organizations/{org_id}/git/integrations/oauth/callback",
            axum::routing::post(oauth_callback),
        )
        .route(
            "/organizations/{org_id}/git/repositories",
            axum::routing::get(list_repositories),
        )
        .route(
            "/organizations/{org_id}/git/integrations/{id}/repositories",
            axum::routing::get(list_integration_repositories).post(link_repository),
        )
        .route(
            "/organizations/{org_id}/git/integrations/{id}/repositories/{repo_id}",
            axum::routing::delete(unlink_repository),
        )
        .route(
            "/organizations/{org_id}/git/integrations/{id}/sync-webhook",
            axum::routing::post(sync_webhook),
        )
        .route(
            "/organizations/{org_id}/projects/{project_id}/issues/{issue_id}/git/prs",
            axum::routing::get(list_issue_prs),
        )
        .route(
            "/git/oauth-providers",
            axum::routing::get(list_oauth_providers).post(upsert_oauth_provider),
        )
        .route(
            "/git/oauth-providers/{provider}",
            axum::routing::delete(delete_oauth_provider),
        )
}

pub fn webhook_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/webhooks/gitlab/{integration_id}",
            axum::routing::post(gitlab_webhook),
        )
        .route(
            "/webhooks/forgejo/{integration_id}",
            axum::routing::post(forgejo_webhook),
        )
}

async fn list_integrations(
    _auth: CanViewIssues,
    Path(org_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Vec<git_service::GitIntegrationPublic>>, StatusCode> {
    git_service::list_integrations(&state.db_conn, org_id)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

#[derive(Deserialize)]
struct CreateManualRequest {
    provider: String,
    instance_url: String,
    display_name: Option<String>,
    access_token: String,
    webhook_secret: String,
}

async fn create_integration_manual(
    auth: CanConnectGit,
    Path(org_id): Path<Uuid>,
    State(state): State<AppState>,
    Json(body): Json<CreateManualRequest>,
) -> Result<Json<git_service::GitIntegrationPublic>, StatusCode> {
    git_service::create_integration(
        &state.db_conn,
        &state.aes_key,
        org_id,
        body.provider,
        body.instance_url,
        body.display_name,
        body.access_token,
        None,
        None,
        auth.0.user_id,
        body.webhook_secret,
    )
    .await
    .map(Json)
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn delete_integration(
    _auth: CanDisconnectGit,
    Path((org_id, id)): Path<(Uuid, Uuid)>,
    State(state): State<AppState>,
) -> StatusCode {
    match git_service::delete_integration(&state.db_conn, org_id, id).await {
        Ok(()) => StatusCode::NO_CONTENT,
        Err(git_service::GitError::NotFound) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[derive(Deserialize)]
struct OauthAuthorizeQuery {
    provider: String,
    instance_url: String,
}

#[derive(Serialize)]
struct OauthAuthorizeResponse {
    oauth_url: String,
}

async fn oauth_authorize(
    _auth: CanConnectGit,
    Path(org_id): Path<Uuid>,
    Query(q): Query<OauthAuthorizeQuery>,
    State(state): State<AppState>,
) -> Result<Json<OauthAuthorizeResponse>, StatusCode> {
    let (client_id, _) =
        git_service::get_oauth_provider_credentials(&state.db_conn, &state.aes_key, &q.provider)
            .await
            .map_err(|e| match e {
                git_service::GitError::NotFound => StatusCode::UNPROCESSABLE_ENTITY,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            })?;

    let frontend_url =
        std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:1420".to_string());
    let redirect_uri = format!("{}/oauth-callback", frontend_url.trim_end_matches('/'));

    let nonce = git_service::generate_oauth_state(
        &state.db_conn,
        org_id,
        &q.provider,
        &q.instance_url,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let base = q.instance_url.trim_end_matches('/');
    let auth_url = match q.provider.as_str() {
        "gitlab" => format!(
            "{}/oauth/authorize?client_id={}&redirect_uri={}&response_type=code&scope=api&state={}",
            base,
            urlencoding::encode(&client_id),
            urlencoding::encode(&redirect_uri),
            urlencoding::encode(&nonce)
        ),
        "forgejo" => format!(
            "{}/login/oauth/authorize?client_id={}&redirect_uri={}&response_type=code&scope=repository&state={}",
            base,
            urlencoding::encode(&client_id),
            urlencoding::encode(&redirect_uri),
            urlencoding::encode(&nonce)
        ),
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    Ok(Json(OauthAuthorizeResponse { oauth_url: auth_url }))
}

#[derive(Deserialize)]
struct OauthCallbackQuery {
    code: String,
    state: String,
}

async fn oauth_callback(
    auth: CanConnectGit,
    Path(org_id): Path<Uuid>,
    Query(q): Query<OauthCallbackQuery>,
    State(state): State<AppState>,
) -> Result<Json<git_service::GitIntegrationPublic>, StatusCode> {
    let oauth_state = git_service::consume_oauth_state(&state.db_conn, &q.state)
        .await
        .map_err(|e| match e {
            git_service::GitError::InvalidOauthState => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    if oauth_state.org_id != org_id {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let (client_id, client_secret) = git_service::get_oauth_provider_credentials(
        &state.db_conn,
        &state.aes_key,
        &oauth_state.provider,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let frontend_url =
        std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:1420".to_string());
    let redirect_uri = format!("{}/oauth-callback", frontend_url.trim_end_matches('/'));

    let backend_instance_url = rewrite_for_backend(&oauth_state.instance_url);
    let tokens = git_service::exchange_oauth_code(
        &state.http_client,
        &oauth_state.provider,
        &backend_instance_url,
        &q.code,
        &redirect_uri,
        &client_id,
        &client_secret,
    )
    .await
    .map_err(|_| StatusCode::BAD_GATEWAY)?;

    let expires_at = tokens.expires_in.map(|secs| {
        chrono::Utc::now() + chrono::Duration::seconds(secs)
    });

    let webhook_secret: String = {
        use rand::Rng;
        rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(32)
            .map(char::from)
            .collect()
    };

    git_service::create_integration(
        &state.db_conn,
        &state.aes_key,
        org_id,
        oauth_state.provider,
        oauth_state.instance_url,
        None,
        tokens.access_token,
        tokens.refresh_token,
        expires_at,
        auth.0.user_id,
        webhook_secret,
    )
    .await
    .map(Json)
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn list_repositories(
    _auth: CanViewIssues,
    Path(org_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Vec<git_service::GitRepositoryPublic>>, StatusCode> {
    git_service::list_repositories(&state.db_conn, org_id)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

#[derive(Deserialize)]
struct LinkRepositoryRequest {
    provider_repo_id: String,
    full_name: String,
    project_id: Uuid,
    default_branch: Option<String>,
}

async fn link_repository(
    _auth: CanConnectGit,
    Path((_org_id, id)): Path<(Uuid, Uuid)>,
    State(state): State<AppState>,
    Json(body): Json<LinkRepositoryRequest>,
) -> Result<Json<git_service::GitRepositoryPublic>, StatusCode> {
    let repo = git_service::link_repository(
        &state.db_conn,
        id,
        body.project_id,
        body.provider_repo_id,
        body.full_name.clone(),
        body.default_branch,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let webhook_base = std::env::var("WEBHOOK_BASE_URL")
        .or_else(|_| std::env::var("BACKEND_URL"))
        .unwrap_or_else(|_| "http://localhost:8000".to_string());
    let provider = git_service::get_integration_provider(&state.db_conn, id)
        .await
        .unwrap_or_else(|_| "forgejo".to_string());
    let webhook_url = format!(
        "{}/api/webhooks/{}/{}",
        webhook_base.trim_end_matches('/'),
        provider,
        id
    );

    if let Err(e) = git_service::register_webhook_for_integration(
        &state.db_conn,
        &state.aes_key,
        &state.http_client,
        id,
        &body.full_name,
        &webhook_url,
    )
    .await
    {
        tracing::warn!(error = %e, integration_id = %id, "failed to register webhook");
    }

    Ok(Json(repo))
}

async fn list_integration_repositories(
    _auth: CanViewIssues,
    Path((_org_id, id)): Path<(Uuid, Uuid)>,
    State(state): State<AppState>,
) -> Result<Json<Vec<git_service::GitRepositoryPublic>>, StatusCode> {
    git_service::list_integration_repositories(&state.db_conn, id)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn unlink_repository(
    _auth: CanConnectGit,
    Path((_org_id, _integration_id, repo_id)): Path<(Uuid, Uuid, Uuid)>,
    State(state): State<AppState>,
) -> StatusCode {
    match git_service::unlink_repository(&state.db_conn, repo_id).await {
        Ok(()) => StatusCode::NO_CONTENT,
        Err(git_service::GitError::NotFound) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn sync_webhook(
    _auth: CanConnectGit,
    Path((org_id, id)): Path<(Uuid, Uuid)>,
    State(state): State<AppState>,
) -> StatusCode {
    let repos = match git_service::list_integration_repositories(&state.db_conn, id).await {
        Ok(r) => r,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    let webhook_base = std::env::var("WEBHOOK_BASE_URL")
        .or_else(|_| std::env::var("BACKEND_URL"))
        .unwrap_or_else(|_| "http://localhost:8000".to_string());
    let provider = match git_service::get_integration_provider(&state.db_conn, id).await {
        Ok(p) => p,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };
    let webhook_url = format!(
        "{}/api/webhooks/{}/{}",
        webhook_base.trim_end_matches('/'),
        provider,
        id
    );
    let _ = org_id;

    let mut any_err = false;
    for repo in repos {
        if let Err(e) = git_service::register_webhook_for_integration(
            &state.db_conn,
            &state.aes_key,
            &state.http_client,
            id,
            &repo.full_name,
            &webhook_url,
        )
        .await
        {
            tracing::warn!(error = %e, repo = %repo.full_name, "failed to sync webhook");
            any_err = true;
        }
    }

    if any_err {
        StatusCode::BAD_GATEWAY
    } else {
        StatusCode::NO_CONTENT
    }
}

async fn list_issue_prs(
    _auth: CanViewIssues,
    Path((_org_id, _project_id, issue_id)): Path<(Uuid, Uuid, Uuid)>,
    State(state): State<AppState>,
) -> Result<Json<Vec<git_service::GitPullRequestPublic>>, StatusCode> {
    git_service::list_issue_prs(&state.db_conn, issue_id)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn gitlab_webhook(
    Path(integration_id): Path<Uuid>,
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> StatusCode {
    use entity::{git_integration, GitIntegration};
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    let integration = match GitIntegration::find_by_id(integration_id)
        .one(&state.db_conn)
        .await
    {
        Ok(Some(i)) => i,
        Ok(None) => return StatusCode::NOT_FOUND,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    let secret = match integration.webhook_secret_enc.as_deref() {
        Some(enc) => {
            let key_b64 = base64::engine::general_purpose::STANDARD.encode(state.aes_key.as_ref());
            match crate::auth::crypto::decrypt_secret(enc, &key_b64) {
                Ok(s) => s,
                Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
            }
        }
        None => return StatusCode::UNAUTHORIZED,
    };

    let token = match headers.get("X-Gitlab-Token").and_then(|v| v.to_str().ok()) {
        Some(t) => t,
        None => return StatusCode::UNAUTHORIZED,
    };

    if !constant_time_eq(token.as_bytes(), secret.as_bytes()) {
        return StatusCode::UNAUTHORIZED;
    }

    let payload: serde_json::Value = match serde_json::from_slice(&body) {
        Ok(v) => v,
        Err(_) => return StatusCode::BAD_REQUEST,
    };

    let event_type = headers
        .get("X-Gitlab-Event")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");

    let gl_action = payload["object_attributes"]["action"].as_str().unwrap_or("unknown");
    let ts = chrono::Utc::now().timestamp();
    let provider_event_id = payload["object_attributes"]["iid"]
        .as_i64()
        .map(|n| format!("{integration_id}:{n}:{gl_action}:{ts}"))
        .unwrap_or_else(|| format!("{integration_id}:{}", uuid::Uuid::new_v4()));

    match git_service::enqueue_webhook_job(
        &state.db_conn,
        integration_id,
        "gitlab",
        event_type,
        &provider_event_id,
        payload,
    )
    .await
    {
        Ok(Some(job_id)) => {
            let _ = state.job_tx.send(job_id).await;
        }
        Ok(None) => {}
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    }

    StatusCode::OK
}

async fn forgejo_webhook(
    Path(integration_id): Path<Uuid>,
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> StatusCode {
    use entity::GitIntegration;
    use sea_orm::EntityTrait;

    let integration = match GitIntegration::find_by_id(integration_id)
        .one(&state.db_conn)
        .await
    {
        Ok(Some(i)) => i,
        Ok(None) => return StatusCode::NOT_FOUND,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    let secret = match integration.webhook_secret_enc.as_deref() {
        Some(enc) => {
            let key_b64 = base64::engine::general_purpose::STANDARD.encode(state.aes_key.as_ref());
            match crate::auth::crypto::decrypt_secret(enc, &key_b64) {
                Ok(s) => s,
                Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
            }
        }
        None => return StatusCode::UNAUTHORIZED,
    };

    let sig_header = headers
        .get("X-Gitea-Signature")
        .or_else(|| headers.get("X-Hub-Signature-256"))
        .and_then(|v| v.to_str().ok());

    if let Some(sig) = sig_header {
        let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
            .expect("HMAC accepts any key size");
        mac.update(&body);
        let computed = hex::encode(mac.finalize().into_bytes());
        let expected = sig.strip_prefix("sha256=").unwrap_or(sig);
        if !constant_time_eq(computed.as_bytes(), expected.as_bytes()) {
            return StatusCode::UNAUTHORIZED;
        }
    }

    let payload: serde_json::Value = match serde_json::from_slice(&body) {
        Ok(v) => v,
        Err(_) => return StatusCode::BAD_REQUEST,
    };

    let event_type = headers
        .get("X-Gitea-Event")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");

    let action = payload["action"].as_str().unwrap_or("unknown");
    let ts = chrono::Utc::now().timestamp();
    let provider_event_id = payload["pull_request"]["number"]
        .as_i64()
        .map(|n| format!("{integration_id}:{n}:{action}:{ts}"))
        .unwrap_or_else(|| format!("{integration_id}:{}", uuid::Uuid::new_v4()));

    match git_service::enqueue_webhook_job(
        &state.db_conn,
        integration_id,
        "forgejo",
        event_type,
        &provider_event_id,
        payload,
    )
    .await
    {
        Ok(Some(job_id)) => {
            let _ = state.job_tx.send(job_id).await;
        }
        Ok(None) => {}
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    }

    StatusCode::OK
}

async fn list_oauth_providers(
    _auth: CanManageOrganization,
    State(state): State<AppState>,
) -> Result<Json<Vec<git_service::OauthProviderConfigPublic>>, StatusCode> {
    git_service::list_oauth_provider_configs(&state.db_conn)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

#[derive(Deserialize)]
struct UpsertOauthProviderRequest {
    provider: String,
    client_id: String,
    client_secret: String,
}

async fn upsert_oauth_provider(
    _auth: CanManageOrganization,
    State(state): State<AppState>,
    Json(body): Json<UpsertOauthProviderRequest>,
) -> Result<Json<git_service::OauthProviderConfigPublic>, StatusCode> {
    git_service::upsert_oauth_provider_config(
        &state.db_conn,
        &state.aes_key,
        body.provider,
        body.client_id,
        body.client_secret,
    )
    .await
    .map(Json)
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn delete_oauth_provider(
    _auth: CanManageOrganization,
    Path(provider): Path<String>,
    State(state): State<AppState>,
) -> StatusCode {
    match git_service::delete_oauth_provider_config(&state.db_conn, &provider).await {
        Ok(()) => StatusCode::NO_CONTENT,
        Err(git_service::GitError::NotFound) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
