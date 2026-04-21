use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Journals::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Journals::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Journals::JournalizedId).integer().not_null())
                    .col(
                        ColumnDef::new(Journals::JournalizedType)
                            .string_len(30)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Journals::UserId).integer().not_null())
                    .col(ColumnDef::new(Journals::Notes).text())
                    .col(ColumnDef::new(Journals::CreatedOn).date_time().not_null())
                    .col(
                        ColumnDef::new(Journals::PrivateNotes)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Journals::UpdatedOn).date_time())
                    .col(ColumnDef::new(Journals::UpdatedById).integer())
                    .to_owned(),
            )
            .await?;

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_journals_journalized")
                    .table(Journals::Table)
                    .col(Journals::JournalizedId)
                    .col(Journals::JournalizedType)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_journals_user_id")
                    .table(Journals::Table)
                    .col(Journals::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_journals_created_on")
                    .table(Journals::Table)
                    .col(Journals::CreatedOn)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Journals::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Journals {
    Table,
    Id,
    JournalizedId,
    JournalizedType,
    UserId,
    Notes,
    CreatedOn,
    PrivateNotes,
    UpdatedOn,
    UpdatedById,
}
