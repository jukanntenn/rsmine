use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(EmailAddresses::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(EmailAddresses::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(EmailAddresses::UserId).integer().not_null())
                    .col(ColumnDef::new(EmailAddresses::Address).string().not_null())
                    .col(
                        ColumnDef::new(EmailAddresses::IsDefault)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(EmailAddresses::Notify)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(ColumnDef::new(EmailAddresses::CreatedOn).date_time())
                    .col(ColumnDef::new(EmailAddresses::UpdatedOn).date_time())
                    .to_owned(),
            )
            .await?;

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_email_addresses_user_id")
                    .table(EmailAddresses::Table)
                    .col(EmailAddresses::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_email_addresses_address")
                    .table(EmailAddresses::Table)
                    .col(EmailAddresses::Address)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(EmailAddresses::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum EmailAddresses {
    Table,
    Id,
    UserId,
    Address,
    IsDefault,
    Notify,
    CreatedOn,
    UpdatedOn,
}
