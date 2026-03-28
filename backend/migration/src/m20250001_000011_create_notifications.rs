use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Notification::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Notification::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Notification::UserId).uuid().not_null())
                    .col(ColumnDef::new(Notification::IssueId).uuid().not_null())
                    .col(ColumnDef::new(Notification::Kind).string().not_null())
                    .col(ColumnDef::new(Notification::ActorId).uuid())
                    .col(ColumnDef::new(Notification::ReadAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(Notification::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Notification::Table, Notification::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Notification::Table, Notification::IssueId)
                            .to(Issue::Table, Issue::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Notification::Table, Notification::ActorId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(Notification::Table)
                    .name("idx_notification_user_unread")
                    .col(Notification::UserId)
                    .col(Notification::ReadAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(IssueSubscription::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IssueSubscription::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IssueSubscription::IssueId)
                            .uuid()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(IssueSubscription::UserId)
                            .col(IssueSubscription::IssueId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(IssueSubscription::Table, IssueSubscription::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(IssueSubscription::Table, IssueSubscription::IssueId)
                            .to(Issue::Table, Issue::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(IssueSubscription::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Notification::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
enum Notification {
    Table,
    Id,
    UserId,
    IssueId,
    Kind,
    ActorId,
    ReadAt,
    CreatedAt,
}

#[derive(Iden)]
enum IssueSubscription {
    Table,
    UserId,
    IssueId,
}

#[derive(Iden)]
enum Issue {
    Table,
    Id,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
}
