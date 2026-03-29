use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(GitIntegration::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GitIntegration::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(GitIntegration::OrganizationId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(GitIntegration::Provider).string().not_null())
                    .col(
                        ColumnDef::new(GitIntegration::InstanceUrl)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(GitIntegration::DisplayName).string())
                    .col(ColumnDef::new(GitIntegration::AccessTokenEnc).text())
                    .col(ColumnDef::new(GitIntegration::RefreshTokenEnc).text())
                    .col(
                        ColumnDef::new(GitIntegration::TokenExpiresAt)
                            .timestamp_with_time_zone(),
                    )
                    .col(ColumnDef::new(GitIntegration::InstalledBy).uuid())
                    .col(ColumnDef::new(GitIntegration::WebhookSecretEnc).text())
                    .col(
                        ColumnDef::new(GitIntegration::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(GitIntegration::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(GitIntegration::Table, GitIntegration::OrganizationId)
                            .to(Organization::Table, Organization::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(GitIntegration::Table, GitIntegration::InstalledBy)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(GitIntegration::Table)
                    .name("idx_git_integration_org_provider_url")
                    .col(GitIntegration::OrganizationId)
                    .col(GitIntegration::Provider)
                    .col(GitIntegration::InstanceUrl)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(OauthState::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OauthState::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(OauthState::OrgId).uuid().not_null())
                    .col(ColumnDef::new(OauthState::Provider).string().not_null())
                    .col(ColumnDef::new(OauthState::InstanceUrl).string().not_null())
                    .col(ColumnDef::new(OauthState::Nonce).string().not_null())
                    .col(
                        ColumnDef::new(OauthState::ExpiresAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(OauthState::Table)
                    .name("idx_oauth_state_nonce")
                    .col(OauthState::Nonce)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(GitRepository::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GitRepository::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(GitRepository::IntegrationId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(GitRepository::ProjectId).uuid().not_null())
                    .col(
                        ColumnDef::new(GitRepository::ProviderRepoId)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(GitRepository::FullName).string().not_null())
                    .col(ColumnDef::new(GitRepository::DefaultBranch).string())
                    .col(
                        ColumnDef::new(GitRepository::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(GitRepository::Table, GitRepository::IntegrationId)
                            .to(GitIntegration::Table, GitIntegration::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(GitRepository::Table, GitRepository::ProjectId)
                            .to(Project::Table, Project::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(GitRepository::Table)
                    .name("idx_git_repository_integration_repo")
                    .col(GitRepository::IntegrationId)
                    .col(GitRepository::ProviderRepoId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(GitPullRequest::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GitPullRequest::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(GitPullRequest::RepositoryId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(GitPullRequest::ProviderPrId)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(GitPullRequest::Number).integer().not_null())
                    .col(ColumnDef::new(GitPullRequest::Title).string().not_null())
                    .col(ColumnDef::new(GitPullRequest::State).string().not_null())
                    .col(ColumnDef::new(GitPullRequest::Url).string().not_null())
                    .col(ColumnDef::new(GitPullRequest::Branch).string().not_null())
                    .col(ColumnDef::new(GitPullRequest::MergedAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(GitPullRequest::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(GitPullRequest::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(GitPullRequest::Table, GitPullRequest::RepositoryId)
                            .to(GitRepository::Table, GitRepository::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(GitPullRequest::Table)
                    .name("idx_git_pull_request_repo_pr")
                    .col(GitPullRequest::RepositoryId)
                    .col(GitPullRequest::ProviderPrId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(GitPullRequest::Table)
                    .name("idx_git_pull_request_branch")
                    .col(GitPullRequest::Branch)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(IssueGitLink::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(IssueGitLink::IssueId).uuid().not_null())
                    .col(ColumnDef::new(IssueGitLink::PrId).uuid().not_null())
                    .primary_key(
                        Index::create()
                            .col(IssueGitLink::IssueId)
                            .col(IssueGitLink::PrId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(IssueGitLink::Table, IssueGitLink::IssueId)
                            .to(Issue::Table, Issue::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(IssueGitLink::Table, IssueGitLink::PrId)
                            .to(GitPullRequest::Table, GitPullRequest::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(WebhookJob::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WebhookJob::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(WebhookJob::IntegrationId).uuid().not_null())
                    .col(ColumnDef::new(WebhookJob::Provider).string().not_null())
                    .col(ColumnDef::new(WebhookJob::EventType).string().not_null())
                    .col(
                        ColumnDef::new(WebhookJob::ProviderEventId)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(WebhookJob::Payload).text().not_null())
                    .col(
                        ColumnDef::new(WebhookJob::Status)
                            .string()
                            .not_null()
                            .default("pending"),
                    )
                    .col(
                        ColumnDef::new(WebhookJob::Attempts)
                            .small_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(WebhookJob::LastError).text())
                    .col(
                        ColumnDef::new(WebhookJob::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(WebhookJob::ProcessedAt).timestamp_with_time_zone())
                    .foreign_key(
                        ForeignKey::create()
                            .from(WebhookJob::Table, WebhookJob::IntegrationId)
                            .to(GitIntegration::Table, GitIntegration::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(WebhookJob::Table)
                    .name("idx_webhook_job_integration_event")
                    .col(WebhookJob::IntegrationId)
                    .col(WebhookJob::ProviderEventId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "CREATE INDEX IF NOT EXISTS idx_webhook_job_pending \
                 ON webhook_job (created_at) WHERE status = 'pending'",
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WebhookJob::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(IssueGitLink::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(GitPullRequest::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(GitRepository::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(OauthState::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(GitIntegration::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
enum GitIntegration {
    Table,
    Id,
    OrganizationId,
    Provider,
    InstanceUrl,
    DisplayName,
    AccessTokenEnc,
    RefreshTokenEnc,
    TokenExpiresAt,
    InstalledBy,
    WebhookSecretEnc,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum OauthState {
    Table,
    Id,
    OrgId,
    Provider,
    InstanceUrl,
    Nonce,
    ExpiresAt,
}

#[derive(Iden)]
enum GitRepository {
    Table,
    Id,
    IntegrationId,
    ProjectId,
    ProviderRepoId,
    FullName,
    DefaultBranch,
    CreatedAt,
}

#[derive(Iden)]
enum GitPullRequest {
    Table,
    Id,
    RepositoryId,
    ProviderPrId,
    Number,
    Title,
    State,
    Url,
    Branch,
    MergedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum IssueGitLink {
    Table,
    IssueId,
    PrId,
}

#[derive(Iden)]
enum WebhookJob {
    Table,
    Id,
    IntegrationId,
    Provider,
    EventType,
    ProviderEventId,
    Payload,
    Status,
    Attempts,
    LastError,
    CreatedAt,
    ProcessedAt,
}

#[derive(Iden)]
enum Organization {
    Table,
    Id,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
}

#[derive(Iden)]
enum Project {
    Table,
    Id,
}

#[derive(Iden)]
enum Issue {
    Table,
    Id,
}
