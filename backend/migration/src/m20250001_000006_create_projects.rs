use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Project::Table)
                    .if_not_exists()
                    .col(uuid(Project::Id).primary_key())
                    .col(uuid(Project::OrganizationId))
                    .col(string(Project::Name))
                    .col(string(Project::Identifier))
                    .col(string_null(Project::Description))
                    .col(string_null(Project::Color))
                    .col(string_null(Project::Icon))
                    .col(uuid_null(Project::CreatedBy))
                    .col(timestamp(Project::CreatedAt))
                    .col(timestamp(Project::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_project_organization")
                            .from(Project::Table, Project::OrganizationId)
                            .to(Organization::Table, Organization::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_project_created_by")
                            .from(Project::Table, Project::CreatedBy)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_project_org_identifier")
                    .table(Project::Table)
                    .col(Project::OrganizationId)
                    .col(Project::Identifier)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_project_org")
                    .table(Project::Table)
                    .col(Project::OrganizationId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ProjectMember::Table)
                    .if_not_exists()
                    .col(uuid(ProjectMember::Id).primary_key())
                    .col(uuid(ProjectMember::ProjectId))
                    .col(uuid(ProjectMember::UserId))
                    .col(string(ProjectMember::Role))
                    .col(timestamp(ProjectMember::JoinedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_project_member_project")
                            .from(ProjectMember::Table, ProjectMember::ProjectId)
                            .to(Project::Table, Project::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_project_member_user")
                            .from(ProjectMember::Table, ProjectMember::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_project_member_unique")
                    .table(ProjectMember::Table)
                    .col(ProjectMember::ProjectId)
                    .col(ProjectMember::UserId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_project_member_user")
                    .table(ProjectMember::Table)
                    .col(ProjectMember::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ProjectIssueCounter::Table)
                    .if_not_exists()
                    .col(uuid(ProjectIssueCounter::ProjectId).primary_key())
                    .col(integer(ProjectIssueCounter::NextNumber).default(1))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_issue_counter_project")
                            .from(ProjectIssueCounter::Table, ProjectIssueCounter::ProjectId)
                            .to(Project::Table, Project::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProjectIssueCounter::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ProjectMember::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Project::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Project {
    Table,
    Id,
    OrganizationId,
    Name,
    Identifier,
    Description,
    Color,
    Icon,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum ProjectMember {
    Table,
    Id,
    ProjectId,
    UserId,
    Role,
    JoinedAt,
}

#[derive(DeriveIden)]
pub enum ProjectIssueCounter {
    Table,
    ProjectId,
    NextNumber,
}

#[derive(DeriveIden)]
enum Organization {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}
