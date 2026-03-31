use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            "CREATE TABLE git_pull_request_new (
                id TEXT NOT NULL PRIMARY KEY,
                repository_id TEXT,
                provider_pr_id TEXT NOT NULL,
                number INTEGER NOT NULL,
                title TEXT NOT NULL,
                state TEXT NOT NULL,
                url TEXT NOT NULL,
                branch TEXT NOT NULL,
                merged_at TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (repository_id) REFERENCES git_repository (id) ON DELETE SET NULL
            )",
        )
        .await?;

        db.execute_unprepared(
            "INSERT INTO git_pull_request_new
                SELECT id, repository_id, provider_pr_id, number, title, state, url, branch, merged_at, created_at, updated_at
                FROM git_pull_request",
        )
        .await?;

        db.execute_unprepared("DROP TABLE git_pull_request").await?;
        db.execute_unprepared("ALTER TABLE git_pull_request_new RENAME TO git_pull_request").await?;

        db.execute_unprepared(
            "CREATE UNIQUE INDEX idx_git_pull_request_repo_pr ON git_pull_request (repository_id, provider_pr_id)",
        )
        .await?;

        db.execute_unprepared(
            "CREATE INDEX idx_git_pull_request_branch ON git_pull_request (branch)",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            "CREATE TABLE git_pull_request_old (
                id TEXT NOT NULL PRIMARY KEY,
                repository_id TEXT NOT NULL,
                provider_pr_id TEXT NOT NULL,
                number INTEGER NOT NULL,
                title TEXT NOT NULL,
                state TEXT NOT NULL,
                url TEXT NOT NULL,
                branch TEXT NOT NULL,
                merged_at TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (repository_id) REFERENCES git_repository (id) ON DELETE CASCADE
            )",
        )
        .await?;

        db.execute_unprepared(
            "INSERT INTO git_pull_request_old
                SELECT id, repository_id, provider_pr_id, number, title, state, url, branch, merged_at, created_at, updated_at
                FROM git_pull_request WHERE repository_id IS NOT NULL",
        )
        .await?;

        db.execute_unprepared("DROP TABLE git_pull_request").await?;
        db.execute_unprepared("ALTER TABLE git_pull_request_old RENAME TO git_pull_request").await?;

        db.execute_unprepared(
            "CREATE UNIQUE INDEX idx_git_pull_request_repo_pr ON git_pull_request (repository_id, provider_pr_id)",
        )
        .await?;

        db.execute_unprepared(
            "CREATE INDEX idx_git_pull_request_branch ON git_pull_request (branch)",
        )
        .await?;

        Ok(())
    }
}
