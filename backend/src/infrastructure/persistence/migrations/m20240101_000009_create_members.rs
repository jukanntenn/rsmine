use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Members::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Members::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Members::UserId).integer().not_null())
                    .col(ColumnDef::new(Members::ProjectId).integer().not_null())
                    .col(ColumnDef::new(Members::CreatedOn).date_time())
                    .col(
                        ColumnDef::new(Members::MailNotification)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await?;

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_members_user_id")
                    .table(Members::Table)
                    .col(Members::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_members_project_id")
                    .table(Members::Table)
                    .col(Members::ProjectId)
                    .to_owned(),
            )
            .await?;

        // Create unique index for user_id + project_id
        manager
            .create_index(
                Index::create()
                    .name("idx_members_user_project")
                    .table(Members::Table)
                    .col(Members::UserId)
                    .col(Members::ProjectId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Members::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Members {
    Table,
    Id,
    UserId,
    ProjectId,
    CreatedOn,
    MailNotification,
}
