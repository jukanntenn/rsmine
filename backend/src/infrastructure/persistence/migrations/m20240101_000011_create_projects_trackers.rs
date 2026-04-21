use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProjectsTrackers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProjectsTrackers::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ProjectsTrackers::ProjectId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProjectsTrackers::TrackerId)
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
                    .name("idx_projects_trackers_project_id")
                    .table(ProjectsTrackers::Table)
                    .col(ProjectsTrackers::ProjectId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_projects_trackers_tracker_id")
                    .table(ProjectsTrackers::Table)
                    .col(ProjectsTrackers::TrackerId)
                    .to_owned(),
            )
            .await?;

        // Create unique index for project_id + tracker_id
        manager
            .create_index(
                Index::create()
                    .name("idx_projects_trackers_unique")
                    .table(ProjectsTrackers::Table)
                    .col(ProjectsTrackers::ProjectId)
                    .col(ProjectsTrackers::TrackerId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProjectsTrackers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ProjectsTrackers {
    Table,
    Id,
    ProjectId,
    TrackerId,
}
