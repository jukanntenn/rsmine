use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(IssueStatuses::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IssueStatuses::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(IssueStatuses::Name)
                            .string_len(30)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IssueStatuses::IsClosed)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(IssueStatuses::IsDefault)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(IssueStatuses::Position).integer())
                    .col(ColumnDef::new(IssueStatuses::DefaultDoneRatio).integer())
                    .to_owned(),
            )
            .await?;

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_issue_statuses_name")
                    .table(IssueStatuses::Table)
                    .col(IssueStatuses::Name)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_issue_statuses_position")
                    .table(IssueStatuses::Table)
                    .col(IssueStatuses::Position)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(IssueStatuses::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum IssueStatuses {
    Table,
    Id,
    Name,
    IsClosed,
    IsDefault,
    Position,
    DefaultDoneRatio,
}
