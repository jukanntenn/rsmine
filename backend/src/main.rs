#![allow(dead_code, unused_imports, unused_variables)]

use axum::Router;
use sea_orm_migration::MigratorTrait;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod application;
mod config;
mod domain;
mod infrastructure;
mod presentation;

use config::AppConfig;
use infrastructure::persistence::db::connect_database;
use infrastructure::persistence::Migrator;
use presentation::api::create_routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Load configuration
    let config = AppConfig::load().expect("Failed to load configuration");

    // Initialize logging based on format
    if config.logging.format == "json" {
        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::new(&config.logging.level))
            .with(tracing_subscriber::fmt::layer().json())
            .init();
    } else {
        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::new(&config.logging.level))
            .with(tracing_subscriber::fmt::layer())
            .init();
    }

    // Connect to database
    let db = connect_database(&config.database).await?;

    tracing::info!("Database connected successfully");

    // Run migrations
    tracing::info!("Running database migrations...");
    Migrator::up(&db, None).await?;
    tracing::info!("Database migrations completed successfully");

    // Build router
    let app = create_routes(db.clone(), config.clone());

    // Start server
    let addr = SocketAddr::new(config.server.host.parse()?, config.server.port);

    tracing::info!("Server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
