use crate::config::DatabaseConfig;
use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn connect_database(config: &DatabaseConfig) -> Result<DatabaseConnection, DbErr> {
    let db_url = &config.url;

    let mut db_options = sea_orm::ConnectOptions::new(db_url.clone());
    db_options.max_connections(config.max_connections);

    Database::connect(db_options).await
}
