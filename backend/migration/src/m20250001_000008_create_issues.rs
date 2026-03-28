use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Issue::Table)
                    .if_not_exists()
                    .col(uuid(Issue::Id).primary_key())
                    .col(uuid(Issue::ProjectId))
                    .col(integer(Issue::Number))
                    .col(string(Issue::Title))
                    .col(text_null(Issue::Description))
                    .col(uuid(Issue::StatusId))
                    .col(small_integer(Issue::Priority).default(0))
                    .col(uuid_null(Issue::AssigneeId))
                    .col(uuid_null(Issue::ParentId))
                    .col(date_null(Issue::DueDate))
                    .col(integer_null(Issue::Estimate))
                    .col(uuid_null(Issue::CreatedBy))
                    .col(timestamp(Issue::CreatedAt))
                    .col(timestamp(Issue::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_issue_project")
                            .from(Issue::Table, Issue::ProjectId)
                            .to(Project::Table, Project::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_issue_status")
                            .from(Issue::Table, Issue::StatusId)
                            .to(IssueStatus::Table, IssueStatus::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_issue_assignee")
                            .from(Issue::Table, Issue::AssigneeId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_issue_parent")
                            .from(Issue::Table, Issue::ParentId)
                            .to(Issue::Table, Issue::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_issue_created_by")
                            .from(Issue::Table, Issue::CreatedBy)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_issue_project_number")
                    .table(Issue::Table)
                    .col(Issue::ProjectId)
                    .col(Issue::Number)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_issue_project_status")
                    .table(Issue::Table)
                    .col(Issue::ProjectId)
                    .col(Issue::StatusId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_issue_project_priority")
                    .table(Issue::Table)
                    .col(Issue::ProjectId)
                    .col(Issue::Priority)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_issue_assignee")
                    .table(Issue::Table)
                    .col(Issue::AssigneeId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_issue_parent")
                    .table(Issue::Table)
                    .col(Issue::ParentId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(IssueLabel::Table)
                    .if_not_exists()
                    .col(uuid(IssueLabel::IssueId))
                    .col(uuid(IssueLabel::LabelId))
                    .primary_key(
                        Index::create()
                            .col(IssueLabel::IssueId)
                            .col(IssueLabel::LabelId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_issue_label_issue")
                            .from(IssueLabel::Table, IssueLabel::IssueId)
                            .to(Issue::Table, Issue::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_issue_label_label")
                            .from(IssueLabel::Table, IssueLabel::LabelId)
                            .to(Label::Table, Label::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(IssueRelationship::Table)
                    .if_not_exists()
                    .col(uuid(IssueRelationship::Id).primary_key())
                    .col(uuid(IssueRelationship::SourceIssueId))
                    .col(uuid(IssueRelationship::TargetIssueId))
                    .col(string(IssueRelationship::Kind))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_relationship_source")
                            .from(IssueRelationship::Table, IssueRelationship::SourceIssueId)
                            .to(Issue::Table, Issue::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_relationship_target")
                            .from(IssueRelationship::Table, IssueRelationship::TargetIssueId)
                            .to(Issue::Table, Issue::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_issue_relationship_unique")
                    .table(IssueRelationship::Table)
                    .col(IssueRelationship::SourceIssueId)
                    .col(IssueRelationship::TargetIssueId)
                    .col(IssueRelationship::Kind)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(IssueRelationship::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(IssueLabel::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Issue::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Issue {
    Table,
    Id,
    ProjectId,
    Number,
    Title,
    Description,
    StatusId,
    Priority,
    AssigneeId,
    ParentId,
    DueDate,
    Estimate,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum IssueLabel {
    Table,
    IssueId,
    LabelId,
}

#[derive(DeriveIden)]
enum IssueRelationship {
    Table,
    Id,
    SourceIssueId,
    TargetIssueId,
    Kind,
}

#[derive(DeriveIden)]
enum Project {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum IssueStatus {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Label {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}
