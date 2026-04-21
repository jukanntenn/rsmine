use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Attachments::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Attachments::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Attachments::ContainerId).integer())
                    .col(ColumnDef::new(Attachments::ContainerType).string_len(30))
                    .col(ColumnDef::new(Attachments::Filename).string().not_null())
                    .col(
                        ColumnDef::new(Attachments::DiskFilename)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Attachments::Filesize)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Attachments::ContentType).string())
                    .col(ColumnDef::new(Attachments::Digest).string_len(64))
                    .col(
                        ColumnDef::new(Attachments::Downloads)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Attachments::AuthorId).integer().not_null())
                    .col(ColumnDef::new(Attachments::CreatedOn).date_time())
                    .col(ColumnDef::new(Attachments::Description).string())
                    .col(ColumnDef::new(Attachments::DiskDirectory).string())
                    .to_owned(),
            )
            .await?;

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_attachments_container")
                    .table(Attachments::Table)
                    .col(Attachments::ContainerId)
                    .col(Attachments::ContainerType)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_attachments_author_id")
                    .table(Attachments::Table)
                    .col(Attachments::AuthorId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_attachments_created_on")
                    .table(Attachments::Table)
                    .col(Attachments::CreatedOn)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Attachments::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Attachments {
    Table,
    Id,
    ContainerId,
    ContainerType,
    Filename,
    DiskFilename,
    Filesize,
    ContentType,
    Digest,
    Downloads,
    AuthorId,
    CreatedOn,
    Description,
    DiskDirectory,
}
