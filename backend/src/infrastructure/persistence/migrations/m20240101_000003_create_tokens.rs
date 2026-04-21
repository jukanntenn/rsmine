use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Tokens::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Tokens::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Tokens::UserId).integer().not_null())
                    .col(ColumnDef::new(Tokens::Action).string().not_null())
                    .col(ColumnDef::new(Tokens::Value).string_len(64).not_null())
                    .col(ColumnDef::new(Tokens::ValidityExpiresOn).date_time())
                    .col(ColumnDef::new(Tokens::CreatedOn).date_time())
                    .col(ColumnDef::new(Tokens::UpdatedOn).date_time())
                    .to_owned(),
            )
            .await?;

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_tokens_user_id")
                    .table(Tokens::Table)
                    .col(Tokens::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_tokens_value")
                    .table(Tokens::Table)
                    .col(Tokens::Value)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tokens::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Tokens {
    Table,
    Id,
    UserId,
    Action,
    Value,
    ValidityExpiresOn,
    CreatedOn,
    UpdatedOn,
}
