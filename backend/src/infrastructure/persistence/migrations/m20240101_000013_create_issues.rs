use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Issues::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Issues::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Issues::TrackerId).integer().not_null())
                    .col(ColumnDef::new(Issues::ProjectId).integer().not_null())
                    .col(ColumnDef::new(Issues::Subject).string().not_null())
                    .col(ColumnDef::new(Issues::Description).text())
                    .col(ColumnDef::new(Issues::DueDate).date())
                    .col(ColumnDef::new(Issues::CategoryId).integer())
                    .col(ColumnDef::new(Issues::StatusId).integer().not_null())
                    .col(ColumnDef::new(Issues::AssignedToId).integer())
                    .col(ColumnDef::new(Issues::PriorityId).integer().not_null())
                    .col(ColumnDef::new(Issues::FixedVersionId).integer())
                    .col(ColumnDef::new(Issues::AuthorId).integer().not_null())
                    .col(
                        ColumnDef::new(Issues::LockVersion)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Issues::CreatedOn).date_time())
                    .col(ColumnDef::new(Issues::UpdatedOn).date_time())
                    .col(ColumnDef::new(Issues::StartDate).date())
                    .col(
                        ColumnDef::new(Issues::DoneRatio)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Issues::EstimatedHours).double())
                    .col(ColumnDef::new(Issues::ParentId).integer())
                    .col(ColumnDef::new(Issues::RootId).integer())
                    .col(ColumnDef::new(Issues::Lft).integer())
                    .col(ColumnDef::new(Issues::Rgt).integer())
                    .col(
                        ColumnDef::new(Issues::IsPrivate)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Issues::ClosedOn).date_time())
                    .to_owned(),
            )
            .await?;

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_issues_project_id")
                    .table(Issues::Table)
                    .col(Issues::ProjectId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_issues_status_id")
                    .table(Issues::Table)
                    .col(Issues::StatusId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_issues_assigned_to_id")
                    .table(Issues::Table)
                    .col(Issues::AssignedToId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_issues_author_id")
                    .table(Issues::Table)
                    .col(Issues::AuthorId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_issues_tracker_id")
                    .table(Issues::Table)
                    .col(Issues::TrackerId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_issues_priority_id")
                    .table(Issues::Table)
                    .col(Issues::PriorityId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_issues_category_id")
                    .table(Issues::Table)
                    .col(Issues::CategoryId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_issues_parent_id")
                    .table(Issues::Table)
                    .col(Issues::ParentId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_issues_root_id")
                    .table(Issues::Table)
                    .col(Issues::RootId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_issues_root_id_lft_rgt")
                    .table(Issues::Table)
                    .col(Issues::RootId)
                    .col(Issues::Lft)
                    .col(Issues::Rgt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_issues_created_on")
                    .table(Issues::Table)
                    .col(Issues::CreatedOn)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_issues_closed_on")
                    .table(Issues::Table)
                    .col(Issues::ClosedOn)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Issues::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Issues {
    Table,
    Id,
    TrackerId,
    ProjectId,
    Subject,
    Description,
    DueDate,
    CategoryId,
    StatusId,
    AssignedToId,
    PriorityId,
    FixedVersionId,
    AuthorId,
    LockVersion,
    CreatedOn,
    UpdatedOn,
    StartDate,
    DoneRatio,
    EstimatedHours,
    ParentId,
    RootId,
    Lft,
    Rgt,
    IsPrivate,
    ClosedOn,
}
