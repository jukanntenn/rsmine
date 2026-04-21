use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Enumerations::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Enumerations::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Enumerations::Name).string_len(30).not_null())
                    .col(ColumnDef::new(Enumerations::Position).integer())
                    .col(
                        ColumnDef::new(Enumerations::IsDefault)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Enumerations::Type).string().not_null())
                    .col(
                        ColumnDef::new(Enumerations::Active)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(ColumnDef::new(Enumerations::ProjectId).integer())
                    .col(ColumnDef::new(Enumerations::ParentId).integer())
                    .col(ColumnDef::new(Enumerations::PositionName).string_len(30))
                    .to_owned(),
            )
            .await?;

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_enumerations_type")
                    .table(Enumerations::Table)
                    .col(Enumerations::Type)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_enumerations_project_id")
                    .table(Enumerations::Table)
                    .col(Enumerations::ProjectId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_enumerations_parent_id")
                    .table(Enumerations::Table)
                    .col(Enumerations::ParentId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Enumerations::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Enumerations {
    Table,
    Id,
    Name,
    Position,
    IsDefault,
    Type,
    Active,
    ProjectId,
    ParentId,
    PositionName,
}
