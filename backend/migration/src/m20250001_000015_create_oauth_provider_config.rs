use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OauthProviderConfig::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OauthProviderConfig::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(OauthProviderConfig::Provider)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OauthProviderConfig::ClientId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OauthProviderConfig::ClientSecretEnc)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OauthProviderConfig::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OauthProviderConfig::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(OauthProviderConfig::Table)
                    .name("idx_oauth_provider_config_provider")
                    .col(OauthProviderConfig::Provider)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OauthProviderConfig::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
enum OauthProviderConfig {
    Table,
    Id,
    Provider,
    ClientId,
    ClientSecretEnc,
    CreatedAt,
    UpdatedAt,
}
