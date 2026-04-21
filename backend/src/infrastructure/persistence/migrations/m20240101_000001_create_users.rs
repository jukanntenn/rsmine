use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::Login).string().not_null())
                    .col(ColumnDef::new(Users::HashedPassword).string_len(40))
                    .col(ColumnDef::new(Users::Firstname).string_len(30).not_null())
                    .col(ColumnDef::new(Users::Lastname).string_len(255).not_null())
                    .col(
                        ColumnDef::new(Users::Admin)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Users::Status)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .col(ColumnDef::new(Users::LastLoginOn).date_time())
                    .col(ColumnDef::new(Users::Language).string_len(5))
                    .col(ColumnDef::new(Users::AuthSourceId).integer())
                    .col(ColumnDef::new(Users::CreatedOn).date_time())
                    .col(ColumnDef::new(Users::UpdatedOn).date_time())
                    .col(ColumnDef::new(Users::Type).string())
                    .col(
                        ColumnDef::new(Users::MailNotification)
                            .string()
                            .not_null()
                            .default("only_my_events"),
                    )
                    .col(ColumnDef::new(Users::Salt).string_len(64))
                    .col(
                        ColumnDef::new(Users::MustChangePasswd)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Users::PasswdChangedOn).date_time())
                    .col(ColumnDef::new(Users::TwofaScheme).string())
                    .col(ColumnDef::new(Users::TwofaTotpKey).string())
                    .col(ColumnDef::new(Users::TwofaTotpLastUsedAt).integer())
                    .col(
                        ColumnDef::new(Users::TwofaRequired)
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
                    .name("idx_users_login")
                    .table(Users::Table)
                    .col(Users::Login)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_users_type")
                    .table(Users::Table)
                    .col(Users::Type)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_users_status")
                    .table(Users::Table)
                    .col(Users::Status)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Users {
    Table,
    Id,
    Login,
    HashedPassword,
    Firstname,
    Lastname,
    Admin,
    Status,
    LastLoginOn,
    Language,
    AuthSourceId,
    CreatedOn,
    UpdatedOn,
    Type,
    MailNotification,
    Salt,
    MustChangePasswd,
    PasswdChangedOn,
    TwofaScheme,
    TwofaTotpKey,
    TwofaTotpLastUsedAt,
    TwofaRequired,
}
