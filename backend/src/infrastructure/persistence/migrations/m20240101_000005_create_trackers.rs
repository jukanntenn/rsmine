use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Trackers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Trackers::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Trackers::Name).string_len(30).not_null())
                    .col(ColumnDef::new(Trackers::Position).integer())
                    .col(
                        ColumnDef::new(Trackers::IsInRoadmap)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Trackers::FieldsBits).big_integer())
                    .col(
                        ColumnDef::new(Trackers::DefaultStatusId)
                            .integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_trackers_name")
                    .table(Trackers::Table)
                    .col(Trackers::Name)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_trackers_position")
                    .table(Trackers::Table)
                    .col(Trackers::Position)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Trackers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Trackers {
    Table,
    Id,
    Name,
    Position,
    IsInRoadmap,
    FieldsBits,
    DefaultStatusId,
}
