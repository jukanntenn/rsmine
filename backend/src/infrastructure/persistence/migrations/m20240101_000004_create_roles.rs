use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Roles::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Roles::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Roles::Name).string_len(30).not_null())
                    .col(ColumnDef::new(Roles::Position).integer())
                    .col(
                        ColumnDef::new(Roles::Assignable)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(Roles::Builtin)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Roles::Permissions).text())
                    .col(
                        ColumnDef::new(Roles::IssuesVisibility)
                            .string_len(30)
                            .not_null()
                            .default("default"),
                    )
                    .col(
                        ColumnDef::new(Roles::UsersVisibility)
                            .string_len(30)
                            .not_null()
                            .default("all"),
                    )
                    .col(
                        ColumnDef::new(Roles::TimeEntriesVisibility)
                            .string_len(30)
                            .not_null()
                            .default("all"),
                    )
                    .col(
                        ColumnDef::new(Roles::AllRolesManaged)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(ColumnDef::new(Roles::Settings).text())
                    .col(ColumnDef::new(Roles::DefaultTimeEntryActivityId).integer())
                    .to_owned(),
            )
            .await?;

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_roles_name")
                    .table(Roles::Table)
                    .col(Roles::Name)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_roles_builtin")
                    .table(Roles::Table)
                    .col(Roles::Builtin)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Roles::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Roles {
    Table,
    Id,
    Name,
    Position,
    Assignable,
    Builtin,
    Permissions,
    IssuesVisibility,
    UsersVisibility,
    TimeEntriesVisibility,
    AllRolesManaged,
    Settings,
    DefaultTimeEntryActivityId,
}
