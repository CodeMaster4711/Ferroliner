use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Comment::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Comment::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Comment::IssueId).uuid().not_null())
                    .col(ColumnDef::new(Comment::AuthorId).uuid())
                    .col(ColumnDef::new(Comment::Body).text().not_null())
                    .col(ColumnDef::new(Comment::EditedAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(Comment::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Comment::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Comment::Table, Comment::IssueId)
                            .to(Issue::Table, Issue::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Comment::Table, Comment::AuthorId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(CommentReaction::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(CommentReaction::CommentId).uuid().not_null())
                    .col(ColumnDef::new(CommentReaction::UserId).uuid().not_null())
                    .col(ColumnDef::new(CommentReaction::Emoji).string().not_null())
                    .primary_key(
                        Index::create()
                            .col(CommentReaction::CommentId)
                            .col(CommentReaction::UserId)
                            .col(CommentReaction::Emoji),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(CommentReaction::Table, CommentReaction::CommentId)
                            .to(Comment::Table, Comment::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(CommentReaction::Table, CommentReaction::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(IssueActivity::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IssueActivity::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(IssueActivity::IssueId).uuid().not_null())
                    .col(ColumnDef::new(IssueActivity::ActorId).uuid())
                    .col(ColumnDef::new(IssueActivity::Kind).string().not_null())
                    .col(ColumnDef::new(IssueActivity::FromValue).text())
                    .col(ColumnDef::new(IssueActivity::ToValue).text())
                    .col(
                        ColumnDef::new(IssueActivity::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(IssueActivity::Table, IssueActivity::IssueId)
                            .to(Issue::Table, Issue::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(IssueActivity::Table, IssueActivity::ActorId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Attachment::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Attachment::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Attachment::IssueId).uuid().not_null())
                    .col(ColumnDef::new(Attachment::UploadedBy).uuid())
                    .col(ColumnDef::new(Attachment::Filename).text().not_null())
                    .col(ColumnDef::new(Attachment::MimeType).text().not_null())
                    .col(ColumnDef::new(Attachment::SizeBytes).big_integer().not_null())
                    .col(ColumnDef::new(Attachment::StorageKey).text().not_null())
                    .col(
                        ColumnDef::new(Attachment::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Attachment::Table, Attachment::IssueId)
                            .to(Issue::Table, Issue::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Attachment::Table, Attachment::UploadedBy)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Attachment::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(IssueActivity::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(CommentReaction::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Comment::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
enum Comment {
    Table,
    Id,
    IssueId,
    AuthorId,
    Body,
    EditedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum CommentReaction {
    Table,
    CommentId,
    UserId,
    Emoji,
}

#[derive(Iden)]
enum IssueActivity {
    Table,
    Id,
    IssueId,
    ActorId,
    Kind,
    FromValue,
    ToValue,
    CreatedAt,
}

#[derive(Iden)]
enum Attachment {
    Table,
    Id,
    IssueId,
    UploadedBy,
    Filename,
    MimeType,
    SizeBytes,
    StorageKey,
    CreatedAt,
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
