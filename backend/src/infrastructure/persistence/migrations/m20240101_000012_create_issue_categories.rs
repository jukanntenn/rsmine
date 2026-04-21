use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(IssueCategories::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IssueCategories::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(IssueCategories::ProjectId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IssueCategories::Name)
                            .string_len(60)
                            .not_null(),
                    )
                    .col(ColumnDef::new(IssueCategories::AssignedToId).integer())
                    .to_owned(),
            )
            .await?;

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_issue_categories_project_id")
                    .table(IssueCategories::Table)
                    .col(IssueCategories::ProjectId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_issue_categories_assigned_to_id")
                    .table(IssueCategories::Table)
                    .col(IssueCategories::AssignedToId)
                    .to_owned(),
            )
            .await?;

        // Unique index for project_id + name
        manager
            .create_index(
                Index::create()
                    .name("idx_issue_categories_project_name")
                    .table(IssueCategories::Table)
                    .col(IssueCategories::ProjectId)
                    .col(IssueCategories::Name)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(IssueCategories::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum IssueCategories {
    Table,
    Id,
    ProjectId,
    Name,
    AssignedToId,
}
