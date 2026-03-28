use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(IssueStatus::Table)
                    .if_not_exists()
                    .col(uuid(IssueStatus::Id).primary_key())
                    .col(uuid(IssueStatus::ProjectId))
                    .col(string(IssueStatus::Name))
                    .col(string(IssueStatus::Color))
                    .col(string(IssueStatus::StatusType))
                    .col(integer(IssueStatus::Position))
                    .col(boolean(IssueStatus::IsDefault).default(false))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_issue_status_project")
                            .from(IssueStatus::Table, IssueStatus::ProjectId)
                            .to(Project::Table, Project::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_issue_status_project")
                    .table(IssueStatus::Table)
                    .col(IssueStatus::ProjectId)
                    .col(IssueStatus::Position)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Label::Table)
                    .if_not_exists()
                    .col(uuid(Label::Id).primary_key())
                    .col(uuid(Label::ProjectId))
                    .col(string(Label::Name))
                    .col(string(Label::Color))
                    .col(string_null(Label::Description))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_label_project")
                            .from(Label::Table, Label::ProjectId)
                            .to(Project::Table, Project::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_label_project_name")
                    .table(Label::Table)
                    .col(Label::ProjectId)
                    .col(Label::Name)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Label::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(IssueStatus::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum IssueStatus {
    Table,
    Id,
    ProjectId,
    Name,
    Color,
    StatusType,
    Position,
    IsDefault,
}

#[derive(DeriveIden)]
pub enum Label {
    Table,
    Id,
    ProjectId,
    Name,
    Color,
    Description,
}

#[derive(DeriveIden)]
enum Project {
    Table,
    Id,
}
