use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(IssueRelations::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IssueRelations::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(IssueRelations::IssueFromId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IssueRelations::IssueToId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IssueRelations::RelationType)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(IssueRelations::Delay).integer())
                    .to_owned(),
            )
            .await?;

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_issue_relations_issue_from_id")
                    .table(IssueRelations::Table)
                    .col(IssueRelations::IssueFromId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_issue_relations_issue_to_id")
                    .table(IssueRelations::Table)
                    .col(IssueRelations::IssueToId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(IssueRelations::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum IssueRelations {
    Table,
    Id,
    IssueFromId,
    IssueToId,
    RelationType,
    Delay,
}
