use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(JournalDetails::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(JournalDetails::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(JournalDetails::JournalId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(JournalDetails::Property)
                            .string_len(30)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(JournalDetails::PropKey)
                            .string_len(40)
                            .not_null(),
                    )
                    .col(ColumnDef::new(JournalDetails::OldValue).text())
                    .col(ColumnDef::new(JournalDetails::Value).text())
                    .to_owned(),
            )
            .await?;

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_journal_details_journal_id")
                    .table(JournalDetails::Table)
                    .col(JournalDetails::JournalId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(JournalDetails::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum JournalDetails {
    Table,
    Id,
    JournalId,
    Property,
    PropKey,
    OldValue,
    Value,
}
