use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Projects::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Projects::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Projects::Name).string().not_null())
                    .col(ColumnDef::new(Projects::Description).text())
                    .col(ColumnDef::new(Projects::Homepage).string())
                    .col(
                        ColumnDef::new(Projects::IsPublic)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(ColumnDef::new(Projects::ParentId).integer())
                    .col(ColumnDef::new(Projects::CreatedOn).date_time())
                    .col(ColumnDef::new(Projects::UpdatedOn).date_time())
                    .col(ColumnDef::new(Projects::Identifier).string())
                    .col(
                        ColumnDef::new(Projects::Status)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .col(ColumnDef::new(Projects::Lft).integer())
                    .col(ColumnDef::new(Projects::Rgt).integer())
                    .col(
                        ColumnDef::new(Projects::InheritMembers)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Projects::DefaultVersionId).integer())
                    .col(ColumnDef::new(Projects::DefaultAssignedToId).integer())
                    .col(ColumnDef::new(Projects::DefaultIssueQueryId).integer())
                    .to_owned(),
            )
            .await?;

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_projects_parent_id")
                    .table(Projects::Table)
                    .col(Projects::ParentId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_projects_identifier")
                    .table(Projects::Table)
                    .col(Projects::Identifier)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_projects_lft_rgt")
                    .table(Projects::Table)
                    .col(Projects::Lft)
                    .col(Projects::Rgt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_projects_status")
                    .table(Projects::Table)
                    .col(Projects::Status)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Projects::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Projects {
    Table,
    Id,
    Name,
    Description,
    Homepage,
    IsPublic,
    ParentId,
    CreatedOn,
    UpdatedOn,
    Identifier,
    Status,
    Lft,
    Rgt,
    InheritMembers,
    DefaultVersionId,
    DefaultAssignedToId,
    DefaultIssueQueryId,
}
