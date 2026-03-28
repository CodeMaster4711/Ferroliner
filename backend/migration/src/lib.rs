pub use sea_orm_migration::prelude::*;

mod m20240101_000001_create_key;
mod m20240101_000002_create_invalid_jwt;
mod m20240101_000003_create_user;
mod m20240101_000004_add_2fa_fields;
mod m20240101_000005_add_rbac_tables;
mod m20250001_000006_create_projects;
mod m20250001_000007_create_issue_workflow;
mod m20250001_000008_create_issues;
mod m20250001_000009_create_comments_and_activity;
mod m20250001_000011_create_notifications;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240101_000001_create_key::Migration),
            Box::new(m20240101_000002_create_invalid_jwt::Migration),
            Box::new(m20240101_000003_create_user::Migration),
            Box::new(m20240101_000004_add_2fa_fields::Migration),
            Box::new(m20240101_000005_add_rbac_tables::Migration),
            Box::new(m20250001_000006_create_projects::Migration),
            Box::new(m20250001_000007_create_issue_workflow::Migration),
            Box::new(m20250001_000008_create_issues::Migration),
            Box::new(m20250001_000009_create_comments_and_activity::Migration),
            Box::new(m20250001_000011_create_notifications::Migration),
        ]
    }
}
