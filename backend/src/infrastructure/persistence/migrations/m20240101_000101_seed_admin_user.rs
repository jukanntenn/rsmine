use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Insert default admin user (login: admin, password: admin123)
        // Password hash generated with Argon2id via: cargo run --bin hash_password
        // Use CURRENT_TIMESTAMP for cross-database compatibility (SQLite + PostgreSQL)
        db.execute_unprepared(
            "INSERT INTO users (login, hashed_password, firstname, lastname, admin, status, created_on, updated_on, mail_notification, salt, must_change_passwd, twofa_required) VALUES
            ('admin', '$argon2id$v=19$m=19456,t=2,p=1$0e7xorW/878dteNtdDEBxw$pvkgDu1C4/cHqz0VfN+w/FZqRu+rl0eDUGJqwS3DlaE', 'Admin', 'User', true, 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 'all', '', false, false)",
        )
        .await?;

        // Insert email address for admin
        db.execute_unprepared(
            "INSERT INTO email_addresses (user_id, address, is_default, notify, created_on, updated_on) VALUES
            (1, 'admin@example.com', true, true, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared("DELETE FROM email_addresses WHERE address = 'admin@example.com'")
            .await?;
        db.execute_unprepared("DELETE FROM users WHERE login = 'admin'")
            .await?;

        Ok(())
    }
}
