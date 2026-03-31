use base64::Engine;
use entity::{
    git_integration, git_pull_request, git_repository, issue, issue_activity, issue_git_link,
    issue_status, oauth_provider_config, oauth_state, project, webhook_job, GitIntegration,
    GitPullRequest, GitRepository, Issue, IssueGitLink, IssueStatus, OauthProviderConfig,
    OauthState, Project, WebhookJob,
};
use regex::Regex;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::OnceLock;
use uuid::Uuid;

use crate::auth::crypto::{decrypt_secret, encrypt_secret};

#[derive(Debug, thiserror::Error)]
pub enum GitError {
    #[error("database error: {0}")]
    Db(#[from] sea_orm::DbErr),
    #[error("integration not found")]
    NotFound,
    #[error("oauth state expired or invalid")]
    InvalidOauthState,
    #[error("provider API error: {0}")]
    ProviderError(String),
    #[error("crypto error: {0}")]
    Crypto(String),
}

pub type GitResult<T> = Result<T, GitError>;

#[derive(Debug, Serialize)]
pub struct GitIntegrationPublic {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub provider: String,
    pub instance_url: String,
    pub display_name: Option<String>,
    pub token_expires_at: Option<String>,
    pub installed_by: Option<Uuid>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<git_integration::Model> for GitIntegrationPublic {
    fn from(m: git_integration::Model) -> Self {
        Self {
            id: m.id,
            organization_id: m.organization_id,
            provider: m.provider,
            instance_url: m.instance_url,
            display_name: m.display_name,
            token_expires_at: m.token_expires_at.map(|t| t.to_rfc3339()),
            installed_by: m.installed_by,
            created_at: m.created_at.to_rfc3339(),
            updated_at: m.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct GitRepositoryPublic {
    pub id: Uuid,
    pub integration_id: Uuid,
    pub project_id: Uuid,
    pub provider_repo_id: String,
    pub full_name: String,
    pub default_branch: Option<String>,
    pub created_at: String,
}

impl From<git_repository::Model> for GitRepositoryPublic {
    fn from(m: git_repository::Model) -> Self {
        Self {
            id: m.id,
            integration_id: m.integration_id,
            project_id: m.project_id,
            provider_repo_id: m.provider_repo_id,
            full_name: m.full_name,
            default_branch: m.default_branch,
            created_at: m.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct GitPullRequestPublic {
    pub id: Uuid,
    pub repository_id: Uuid,
    pub provider_pr_id: String,
    pub number: i32,
    pub title: String,
    pub state: String,
    pub url: String,
    pub branch: String,
    pub merged_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<git_pull_request::Model> for GitPullRequestPublic {
    fn from(m: git_pull_request::Model) -> Self {
        Self {
            id: m.id,
            repository_id: m.repository_id,
            provider_pr_id: m.provider_pr_id,
            number: m.number,
            title: m.title,
            state: m.state,
            url: m.url,
            branch: m.branch,
            merged_at: m.merged_at.map(|t| t.to_rfc3339()),
            created_at: m.created_at.to_rfc3339(),
            updated_at: m.updated_at.to_rfc3339(),
        }
    }
}

pub struct OauthTokenResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: Option<i64>,
}

fn aes_key_b64(key: &[u8; 32]) -> String {
    base64::engine::general_purpose::STANDARD.encode(key)
}

pub async fn create_integration(
    db: &DatabaseConnection,
    aes_key: &[u8; 32],
    org_id: Uuid,
    provider: String,
    instance_url: String,
    display_name: Option<String>,
    access_token: String,
    refresh_token: Option<String>,
    token_expires_at: Option<chrono::DateTime<chrono::Utc>>,
    installed_by: Uuid,
    webhook_secret: String,
) -> GitResult<GitIntegrationPublic> {
    let key_b64 = aes_key_b64(aes_key);
    let access_token_enc = encrypt_secret(&access_token, &key_b64)
        .map_err(|e| GitError::Crypto(e.to_string()))?;
    let refresh_token_enc = refresh_token
        .as_deref()
        .map(|t| encrypt_secret(t, &key_b64))
        .transpose()
        .map_err(|e| GitError::Crypto(e.to_string()))?;
    let webhook_secret_enc = encrypt_secret(&webhook_secret, &key_b64)
        .map_err(|e| GitError::Crypto(e.to_string()))?;

    let now = chrono::Utc::now().fixed_offset();

    if let Some(existing) = GitIntegration::find()
        .filter(git_integration::Column::OrganizationId.eq(org_id))
        .filter(git_integration::Column::Provider.eq(&provider))
        .filter(git_integration::Column::InstanceUrl.eq(&instance_url))
        .one(db)
        .await?
    {
        let mut active = existing.into_active_model();
        active.access_token_enc = Set(Some(access_token_enc));
        active.refresh_token_enc = Set(refresh_token_enc);
        active.token_expires_at = Set(token_expires_at.map(|t| t.fixed_offset()));
        active.updated_at = Set(now);
        let updated = active.update(db).await?;
        return Ok(updated.into());
    }

    let model = git_integration::ActiveModel {
        id: Set(Uuid::new_v4()),
        organization_id: Set(org_id),
        provider: Set(provider),
        instance_url: Set(instance_url),
        display_name: Set(display_name),
        access_token_enc: Set(Some(access_token_enc)),
        refresh_token_enc: Set(refresh_token_enc),
        token_expires_at: Set(token_expires_at.map(|t| t.fixed_offset())),
        installed_by: Set(Some(installed_by)),
        webhook_secret_enc: Set(Some(webhook_secret_enc)),
        created_at: Set(now),
        updated_at: Set(now),
    };
    let inserted = model.insert(db).await?;
    Ok(inserted.into())
}

pub async fn list_integrations(
    db: &DatabaseConnection,
    org_id: Uuid,
) -> GitResult<Vec<GitIntegrationPublic>> {
    let rows = GitIntegration::find()
        .filter(git_integration::Column::OrganizationId.eq(org_id))
        .all(db)
        .await?;
    Ok(rows.into_iter().map(Into::into).collect())
}

pub async fn delete_integration(
    db: &DatabaseConnection,
    org_id: Uuid,
    integration_id: Uuid,
) -> GitResult<()> {
    let integration = GitIntegration::find_by_id(integration_id)
        .filter(git_integration::Column::OrganizationId.eq(org_id))
        .one(db)
        .await?
        .ok_or(GitError::NotFound)?;
    let active: git_integration::ActiveModel = integration.into();
    active.delete(db).await?;
    Ok(())
}

pub async fn generate_oauth_state(
    db: &DatabaseConnection,
    org_id: Uuid,
    provider: &str,
    instance_url: &str,
) -> GitResult<String> {
    let nonce = Uuid::new_v4().to_string();
    let expires_at = (chrono::Utc::now() + chrono::Duration::minutes(10)).fixed_offset();
    let model = oauth_state::ActiveModel {
        id: Set(Uuid::new_v4()),
        org_id: Set(org_id),
        provider: Set(provider.to_string()),
        instance_url: Set(instance_url.to_string()),
        nonce: Set(nonce.clone()),
        expires_at: Set(expires_at),
    };
    model.insert(db).await?;
    Ok(nonce)
}

pub async fn consume_oauth_state(
    db: &DatabaseConnection,
    nonce: &str,
) -> GitResult<oauth_state::Model> {
    let state = OauthState::find()
        .filter(oauth_state::Column::Nonce.eq(nonce))
        .one(db)
        .await?
        .ok_or(GitError::InvalidOauthState)?;

    if state.expires_at < chrono::Utc::now().fixed_offset() {
        let active: oauth_state::ActiveModel = state.into();
        active.delete(db).await?;
        return Err(GitError::InvalidOauthState);
    }

    let row = state.clone();
    let active: oauth_state::ActiveModel = state.into();
    active.delete(db).await?;
    Ok(row)
}

#[derive(Deserialize)]
struct OauthTokenRaw {
    access_token: String,
    refresh_token: Option<String>,
    expires_in: Option<i64>,
}

pub async fn exchange_oauth_code(
    http: &reqwest::Client,
    provider: &str,
    instance_url: &str,
    code: &str,
    redirect_uri: &str,
    client_id: &str,
    client_secret: &str,
) -> GitResult<OauthTokenResponse> {
    let token_url = match provider {
        "gitlab" => format!("{}/oauth/token", instance_url.trim_end_matches('/')),
        "forgejo" => format!(
            "{}/login/oauth/access_token",
            instance_url.trim_end_matches('/')
        ),
        p => return Err(GitError::ProviderError(format!("unknown provider: {p}"))),
    };

    let resp = http
        .post(&token_url)
        .header("Accept", "application/json")
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", redirect_uri),
            ("client_id", client_id),
            ("client_secret", client_secret),
        ])
        .send()
        .await
        .map_err(|e| GitError::ProviderError(e.to_string()))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(GitError::ProviderError(format!("status={status} body={body}")));
    }

    let raw: OauthTokenRaw = resp
        .json()
        .await
        .map_err(|e| GitError::ProviderError(e.to_string()))?;

    Ok(OauthTokenResponse {
        access_token: raw.access_token,
        refresh_token: raw.refresh_token,
        expires_in: raw.expires_in,
    })
}

async fn delete_existing_hooks(
    http: &reqwest::Client,
    provider: &str,
    base: &str,
    repo_full_name: &str,
    access_token: &str,
    webhook_url: &str,
) -> GitResult<()> {
    let list_url = match provider {
        "gitlab" => {
            let encoded = urlencoding::encode(repo_full_name);
            format!("{}/api/v4/projects/{}/hooks", base, encoded)
        }
        "forgejo" => {
            let parts: Vec<&str> = repo_full_name.splitn(2, '/').collect();
            if parts.len() != 2 {
                return Ok(());
            }
            format!("{}/api/v1/repos/{}/{}/hooks", base, parts[0], parts[1])
        }
        _ => return Ok(()),
    };

    let auth_header = match provider {
        "gitlab" => format!("Bearer {access_token}"),
        _ => format!("token {access_token}"),
    };

    let resp = http
        .get(&list_url)
        .header("Authorization", &auth_header)
        .send()
        .await
        .map_err(|e| GitError::ProviderError(e.to_string()))?;

    if !resp.status().is_success() {
        return Ok(());
    }

    let hooks: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| GitError::ProviderError(e.to_string()))?;

    let hooks = match hooks.as_array() {
        Some(a) => a.clone(),
        None => return Ok(()),
    };

    for hook in hooks {
        let hook_url = match provider {
            "gitlab" => hook["url"].as_str().unwrap_or("").to_string(),
            _ => hook["config"]["url"].as_str().unwrap_or("").to_string(),
        };
        if hook_url != webhook_url {
            continue;
        }
        let hook_id = match hook["id"].as_i64() {
            Some(id) => id,
            None => continue,
        };
        let delete_url = match provider {
            "gitlab" => {
                let encoded = urlencoding::encode(repo_full_name);
                format!("{}/api/v4/projects/{}/hooks/{}", base, encoded, hook_id)
            }
            _ => {
                let parts: Vec<&str> = repo_full_name.splitn(2, '/').collect();
                format!("{}/api/v1/repos/{}/{}/hooks/{}", base, parts[0], parts[1], hook_id)
            }
        };
        let _ = http
            .delete(&delete_url)
            .header("Authorization", &auth_header)
            .send()
            .await;
    }

    Ok(())
}

pub async fn register_webhook(
    http: &reqwest::Client,
    provider: &str,
    instance_url: &str,
    access_token: &str,
    repo_full_name: &str,
    webhook_url: &str,
    webhook_secret: &str,
) -> GitResult<()> {
    let base = instance_url.trim_end_matches('/');

    delete_existing_hooks(http, provider, base, repo_full_name, access_token, webhook_url).await?;

    let (url, body) = match provider {
        "gitlab" => {
            let encoded = urlencoding::encode(repo_full_name);
            let url = format!("{}/api/v4/projects/{}/hooks", base, encoded);
            let body = json!({
                "url": webhook_url,
                "token": webhook_secret,
                "merge_requests_events": true,
                "push_events": false
            });
            (url, body)
        }
        "forgejo" => {
            let parts: Vec<&str> = repo_full_name.splitn(2, '/').collect();
            if parts.len() != 2 {
                return Err(GitError::ProviderError("invalid repo full_name".into()));
            }
            let url = format!("{}/api/v1/repos/{}/{}/hooks", base, parts[0], parts[1]);
            let body = json!({
                "type": "gitea",
                "config": {
                    "url": webhook_url,
                    "content_type": "json",
                    "secret": webhook_secret
                },
                "events": ["pull_request"],
                "active": true
            });
            (url, body)
        }
        p => return Err(GitError::ProviderError(format!("unknown provider: {p}"))),
    };

    let mut req = http.post(&url).json(&body);
    req = match provider {
        "gitlab" => req.header("Authorization", format!("Bearer {access_token}")),
        _ => req.header("Authorization", format!("token {access_token}")),
    };

    let resp = req
        .send()
        .await
        .map_err(|e| GitError::ProviderError(e.to_string()))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(GitError::ProviderError(format!("status={status} body={body}")));
    }
    Ok(())
}

pub async fn enqueue_webhook_job(
    db: &DatabaseConnection,
    integration_id: Uuid,
    provider: &str,
    event_type: &str,
    provider_event_id: &str,
    payload: serde_json::Value,
) -> GitResult<Option<Uuid>> {
    let existing = WebhookJob::find()
        .filter(webhook_job::Column::IntegrationId.eq(integration_id))
        .filter(webhook_job::Column::ProviderEventId.eq(provider_event_id))
        .one(db)
        .await?;

    if existing.is_some() {
        return Ok(None);
    }

    let id = Uuid::new_v4();
    let now = chrono::Utc::now().fixed_offset();
    let model = webhook_job::ActiveModel {
        id: Set(id),
        integration_id: Set(integration_id),
        provider: Set(provider.to_string()),
        event_type: Set(event_type.to_string()),
        provider_event_id: Set(provider_event_id.to_string()),
        payload: Set(payload.to_string()),
        status: Set("pending".to_string()),
        attempts: Set(0),
        last_error: Set(None),
        created_at: Set(now),
        processed_at: Set(None),
    };
    model.insert(db).await?;
    Ok(Some(id))
}

fn issue_identifier_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?i)(?:^|[^A-Z0-9])([A-Z]{2,10}-\d+)").unwrap())
}

fn extract_identifiers(text: &str) -> Vec<String> {
    issue_identifier_regex()
        .captures_iter(text)
        .filter_map(|c| c.get(1))
        .map(|m| m.as_str().to_uppercase())
        .collect()
}

fn parse_identifier(raw: &str) -> Option<(String, i32)> {
    let dash = raw.rfind('-')?;
    let prefix = raw[..dash].to_string();
    let number: i32 = raw[dash + 1..].parse().ok()?;
    Some((prefix, number))
}

pub async fn process_webhook_mr(
    db: &DatabaseConnection,
    aes_key: &[u8; 32],
    job: &webhook_job::Model,
) -> GitResult<()> {
    let payload: serde_json::Value = serde_json::from_str(&job.payload)
        .map_err(|e| GitError::ProviderError(e.to_string()))?;

    let (title, branch, pr_number, pr_url, state, pr_action) = match job.provider.as_str() {
        "gitlab" => {
            let attrs = &payload["object_attributes"];
            let title = attrs["title"].as_str().unwrap_or("").to_string();
            let branch = attrs["source_branch"].as_str().unwrap_or("").to_string();
            let number = attrs["iid"].as_i64().unwrap_or(0) as i32;
            let url = attrs["url"].as_str().unwrap_or("").to_string();
            let action = attrs["action"].as_str().unwrap_or("").to_string();
            let is_draft = attrs["draft"].as_bool().unwrap_or(false)
                || attrs["work_in_progress"].as_bool().unwrap_or(false);
            let state = match attrs["state"].as_str().unwrap_or("") {
                "merged" => "merged",
                "closed" => "closed",
                _ => "open",
            };
            let pr_action = if is_draft && (action == "open" || action == "update") {
                "draft"
            } else {
                match action.as_str() {
                    "close" => "closed",
                    "merge" => "merged",
                    "reopen" => "reopened",
                    _ => state,
                }
            };
            (title, branch, number, url, state.to_string(), pr_action.to_string())
        }
        "forgejo" => {
            let pr = &payload["pull_request"];
            let title = pr["title"].as_str().unwrap_or("").to_string();
            let branch = pr["head"]["ref"].as_str().unwrap_or("").to_string();
            let number = pr["number"].as_i64().unwrap_or(0) as i32;
            let url = pr["html_url"].as_str().unwrap_or("").to_string();
            let merged = pr["merged"].as_bool().unwrap_or(false);
            let is_draft = pr["draft"].as_bool().unwrap_or(false);
            let state_str = pr["state"].as_str().unwrap_or("open");
            let action = payload["action"].as_str().unwrap_or("").to_string();
            let state = if merged {
                "merged"
            } else if state_str == "closed" {
                "closed"
            } else {
                "open"
            };
            let pr_action = if merged {
                "merged"
            } else {
                match action.as_str() {
                    "closed" => "closed",
                    "reopened" => "reopened",
                    "converted_to_draft" => "draft",
                    "ready_for_review" => "ready_for_review",
                    _ if is_draft => "draft",
                    _ => state,
                }
            };
            (title, branch, number, url, state.to_string(), pr_action.to_string())
        }
        p => return Err(GitError::ProviderError(format!("unknown provider: {p}"))),
    };

    let mut identifiers: Vec<String> = extract_identifiers(&title);
    identifiers.extend(extract_identifiers(&branch));
    identifiers.sort();
    identifiers.dedup();

    let repo = GitRepository::find()
        .filter(git_repository::Column::IntegrationId.eq(job.integration_id))
        .one(db)
        .await?;

    let repo = match repo {
        Some(r) => r,
        None => return Ok(()),
    };

    let provider_pr_id = format!("{}:{}", repo.id, pr_number);
    let now = chrono::Utc::now().fixed_offset();

    let existing_pr = GitPullRequest::find()
        .filter(git_pull_request::Column::RepositoryId.eq(repo.id))
        .filter(git_pull_request::Column::ProviderPrId.eq(&provider_pr_id))
        .one(db)
        .await?;

    let is_new = existing_pr.is_none();

    let pr = if let Some(pr) = existing_pr {
        let mut active: git_pull_request::ActiveModel = pr.into();
        active.state = Set(state.clone());
        active.title = Set(title.clone());
        active.updated_at = Set(now);
        if state == "merged" {
            active.merged_at = Set(Some(now));
        }
        active.update(db).await?
    } else {
        let merged_at = if state == "merged" { Some(now) } else { None };
        git_pull_request::ActiveModel {
            id: Set(Uuid::new_v4()),
            repository_id: Set(repo.id),
            provider_pr_id: Set(provider_pr_id),
            number: Set(pr_number),
            title: Set(title.clone()),
            state: Set(state.clone()),
            url: Set(pr_url.clone()),
            branch: Set(branch.clone()),
            merged_at: Set(merged_at),
            created_at: Set(now),
            updated_at: Set(now),
        }
        .insert(db)
        .await?
    };

    let pr_meta = json!({
        "url": pr.url,
        "title": pr.title,
        "number": pr.number,
        "provider": job.provider
    })
    .to_string();

    for raw_id in &identifiers {
        let Some((prefix, number)) = parse_identifier(raw_id) else {
            continue;
        };

        let project = Project::find()
            .filter(project::Column::Identifier.eq(&prefix))
            .one(db)
            .await?;

        let Some(project) = project else { continue };

        let issue = Issue::find()
            .filter(issue::Column::ProjectId.eq(project.id))
            .filter(issue::Column::Number.eq(number))
            .one(db)
            .await?;

        let Some(issue) = issue else { continue };

        let link_exists = IssueGitLink::find_by_id((issue.id, pr.id))
            .one(db)
            .await?
            .is_some();

        if !link_exists {
            issue_git_link::ActiveModel {
                issue_id: Set(issue.id),
                pr_id: Set(pr.id),
            }
            .insert(db)
            .await?;
        }

        if is_new {
            write_activity(db, issue.id, "git.pr_linked", &pr_meta).await?;
            write_activity(db, issue.id, "git.branch_linked", &branch).await?;
        }

        let activity_kind = match pr_action.as_str() {
            "merged" => Some("git.pr_merged"),
            "closed" => Some("git.pr_closed"),
            "reopened" => Some("git.pr_reopened"),
            "draft" => Some("git.pr_draft"),
            "ready_for_review" => Some("git.pr_ready"),
            "open" if is_new => Some("git.pr_opened"),
            _ => None,
        };

        if let Some(kind) = activity_kind {
            write_activity(db, issue.id, kind, &pr_meta).await?;
        }

        match pr_action.as_str() {
            "merged" => transition_issue_to(db, &issue, project.id, "done").await?,
            "closed" => transition_issue_to(db, &issue, project.id, "cancelled").await?,
            "open" | "reopened" | "ready_for_review" | "draft" => {
                transition_issue_to(db, &issue, project.id, "in_progress").await?
            }
            _ => {}
        }
    }

    Ok(())
}

async fn write_activity(
    db: &DatabaseConnection,
    issue_id: Uuid,
    kind: &str,
    to_value: &str,
) -> GitResult<()> {
    let now = chrono::Utc::now().fixed_offset();
    issue_activity::ActiveModel {
        id: Set(Uuid::new_v4()),
        issue_id: Set(issue_id),
        actor_id: Set(None),
        kind: Set(kind.to_string()),
        from_value: Set(None),
        to_value: Set(Some(to_value.to_string())),
        created_at: Set(now),
    }
    .insert(db)
    .await?;
    Ok(())
}

async fn transition_issue_to(
    db: &DatabaseConnection,
    issue: &issue::Model,
    project_id: Uuid,
    status_type: &str,
) -> GitResult<()> {
    let target_status = IssueStatus::find()
        .filter(issue_status::Column::ProjectId.eq(project_id))
        .filter(issue_status::Column::StatusType.eq(status_type))
        .one(db)
        .await?;

    let Some(target_status) = target_status else {
        return Ok(());
    };

    if issue.status_id == target_status.id {
        return Ok(());
    }

    let mut active: issue::ActiveModel = issue.clone().into();
    active.status_id = Set(target_status.id);
    active.updated_at = Set(chrono::Utc::now().naive_utc());
    active.update(db).await?;

    Ok(())
}

pub async fn list_repositories(
    db: &DatabaseConnection,
    org_id: Uuid,
) -> GitResult<Vec<GitRepositoryPublic>> {
    let integrations = GitIntegration::find()
        .filter(git_integration::Column::OrganizationId.eq(org_id))
        .all(db)
        .await?;

    let mut repos = Vec::new();
    for integration in integrations {
        let rows = GitRepository::find()
            .filter(git_repository::Column::IntegrationId.eq(integration.id))
            .all(db)
            .await?;
        repos.extend(rows.into_iter().map(GitRepositoryPublic::from));
    }
    Ok(repos)
}

pub async fn get_integration_provider(
    db: &DatabaseConnection,
    integration_id: Uuid,
) -> GitResult<String> {
    let integration = GitIntegration::find_by_id(integration_id)
        .one(db)
        .await?
        .ok_or(GitError::NotFound)?;
    Ok(integration.provider)
}

pub async fn register_webhook_for_integration(
    db: &DatabaseConnection,
    aes_key: &[u8; 32],
    http: &reqwest::Client,
    integration_id: Uuid,
    repo_full_name: &str,
    webhook_url: &str,
) -> GitResult<()> {
    let integration = GitIntegration::find_by_id(integration_id)
        .one(db)
        .await?
        .ok_or(GitError::NotFound)?;

    let key_b64 = aes_key_b64(aes_key);
    let access_token = integration
        .access_token_enc
        .as_deref()
        .map(|enc| decrypt_secret(enc, &key_b64))
        .transpose()
        .map_err(|e| GitError::Crypto(e.to_string()))?
        .ok_or(GitError::NotFound)?;

    let webhook_secret = integration
        .webhook_secret_enc
        .as_deref()
        .map(|enc| decrypt_secret(enc, &key_b64))
        .transpose()
        .map_err(|e| GitError::Crypto(e.to_string()))?
        .ok_or(GitError::NotFound)?;

    let instance_url = rewrite_for_backend(&integration.instance_url);

    register_webhook(
        http,
        &integration.provider,
        &instance_url,
        &access_token,
        repo_full_name,
        webhook_url,
        &webhook_secret,
    )
    .await
}

fn rewrite_for_backend(url: &str) -> String {
    let internal = std::env::var("GIT_INTERNAL_HOST").unwrap_or_default();
    if internal.is_empty() {
        return url.to_string();
    }
    url.replace("localhost", &internal).replace("127.0.0.1", &internal)
}

pub async fn link_repository(
    db: &DatabaseConnection,
    integration_id: Uuid,
    project_id: Uuid,
    provider_repo_id: String,
    full_name: String,
    default_branch: Option<String>,
) -> GitResult<GitRepositoryPublic> {
    let now = chrono::Utc::now().fixed_offset();
    let model = git_repository::ActiveModel {
        id: Set(Uuid::new_v4()),
        integration_id: Set(integration_id),
        project_id: Set(project_id),
        provider_repo_id: Set(provider_repo_id),
        full_name: Set(full_name),
        default_branch: Set(default_branch),
        created_at: Set(now),
    };
    let inserted = model.insert(db).await?;
    Ok(inserted.into())
}

pub async fn list_issue_prs(
    db: &DatabaseConnection,
    issue_id: Uuid,
) -> GitResult<Vec<GitPullRequestPublic>> {
    let links = IssueGitLink::find()
        .filter(issue_git_link::Column::IssueId.eq(issue_id))
        .all(db)
        .await?;

    let mut prs = Vec::new();
    for link in links {
        if let Some(pr) = GitPullRequest::find_by_id(link.pr_id).one(db).await? {
            prs.push(pr.into());
        }
    }
    Ok(prs)
}

#[derive(Debug, Serialize)]
pub struct OauthProviderConfigPublic {
    pub id: Uuid,
    pub provider: String,
    pub client_id: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<oauth_provider_config::Model> for OauthProviderConfigPublic {
    fn from(m: oauth_provider_config::Model) -> Self {
        Self {
            id: m.id,
            provider: m.provider,
            client_id: m.client_id,
            created_at: m.created_at.to_rfc3339(),
            updated_at: m.updated_at.to_rfc3339(),
        }
    }
}

pub async fn upsert_oauth_provider_config(
    db: &DatabaseConnection,
    aes_key: &[u8; 32],
    provider: String,
    client_id: String,
    client_secret: String,
) -> GitResult<OauthProviderConfigPublic> {
    let key_b64 = aes_key_b64(aes_key);
    let now = chrono::Utc::now().fixed_offset();

    let existing = OauthProviderConfig::find()
        .filter(oauth_provider_config::Column::Provider.eq(&provider))
        .one(db)
        .await?;

    let model = if let Some(row) = existing {
        let mut active: oauth_provider_config::ActiveModel = row.into();
        active.client_id = Set(client_id);
        if !client_secret.is_empty() {
            let enc = encrypt_secret(&client_secret, &key_b64)
                .map_err(|e| GitError::Crypto(e.to_string()))?;
            active.client_secret_enc = Set(enc);
        }
        active.updated_at = Set(now);
        active.update(db).await?
    } else {
        if client_secret.is_empty() {
            return Err(GitError::Crypto("client_secret required for new provider".into()));
        }
        let enc = encrypt_secret(&client_secret, &key_b64)
            .map_err(|e| GitError::Crypto(e.to_string()))?;
        oauth_provider_config::ActiveModel {
            id: Set(Uuid::new_v4()),
            provider: Set(provider),
            client_id: Set(client_id),
            client_secret_enc: Set(enc),
            created_at: Set(now),
            updated_at: Set(now),
        }
        .insert(db)
        .await?
    };

    Ok(model.into())
}

pub async fn list_oauth_provider_configs(
    db: &DatabaseConnection,
) -> GitResult<Vec<OauthProviderConfigPublic>> {
    let rows = OauthProviderConfig::find().all(db).await?;
    Ok(rows.into_iter().map(Into::into).collect())
}

pub async fn delete_oauth_provider_config(
    db: &DatabaseConnection,
    provider: &str,
) -> GitResult<()> {
    let row = OauthProviderConfig::find()
        .filter(oauth_provider_config::Column::Provider.eq(provider))
        .one(db)
        .await?
        .ok_or(GitError::NotFound)?;
    let active: oauth_provider_config::ActiveModel = row.into();
    active.delete(db).await?;
    Ok(())
}

pub async fn get_oauth_provider_credentials(
    db: &DatabaseConnection,
    aes_key: &[u8; 32],
    provider: &str,
) -> GitResult<(String, String)> {
    let row = OauthProviderConfig::find()
        .filter(oauth_provider_config::Column::Provider.eq(provider))
        .one(db)
        .await?
        .ok_or(GitError::NotFound)?;

    let key_b64 = aes_key_b64(aes_key);
    let client_secret = decrypt_secret(&row.client_secret_enc, &key_b64)
        .map_err(|e| GitError::Crypto(e.to_string()))?;

    Ok((row.client_id, client_secret))
}
