use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MemberRoles::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MemberRoles::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(MemberRoles::MemberId).integer().not_null())
                    .col(ColumnDef::new(MemberRoles::RoleId).integer().not_null())
                    .col(ColumnDef::new(MemberRoles::InheritedFrom).integer())
                    .to_owned(),
            )
            .await?;

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_member_roles_member_id")
                    .table(MemberRoles::Table)
                    .col(MemberRoles::MemberId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_member_roles_role_id")
                    .table(MemberRoles::Table)
                    .col(MemberRoles::RoleId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MemberRoles::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum MemberRoles {
    Table,
    Id,
    MemberId,
    RoleId,
    InheritedFrom,
}
