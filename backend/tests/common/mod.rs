#![allow(dead_code)]

//! Common test utilities for integration tests

use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use http_body_util::BodyExt;
use sea_orm::DatabaseConnection;
use serde_json::{json, Value};
use tower::ServiceExt;

use rsmine::config::{
    AppConfig, DatabaseConfig, JwtConfig, LoggingConfig, PasswordConfig, ServerConfig,
    StorageConfig,
};
use rsmine::infrastructure::persistence::Migrator;
use rsmine::presentation::api::create_routes;
use sea_orm_migration::MigratorTrait;

/// Create a test application with in-memory SQLite database
pub async fn create_test_app() -> Router {
    // Create in-memory SQLite database
    let db = sea_orm::Database::connect("sqlite::memory:")
        .await
        .expect("Failed to connect to in-memory database");

    // Run migrations
    Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations");

    // Seed the database with initial data
    seed_test_data(&db).await;

    // Build app using the actual create_routes function
    create_test_app_with_db(db).await
}

/// Create a test application with an existing database connection
pub async fn create_test_app_with_db(db: DatabaseConnection) -> Router {
    // Create minimal config
    let config = AppConfig {
        server: ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 3000,
            base_url: "http://localhost:3000".to_string(),
        },
        database: DatabaseConfig {
            url: "sqlite::memory:".to_string(),
            max_connections: 10,
        },
        jwt: JwtConfig {
            secret: "test-secret-key-for-integration-tests".to_string(),
            expiration: 3600,
        },
        logging: LoggingConfig {
            level: "debug".to_string(),
            format: "text".to_string(),
        },
        storage: StorageConfig {
            path: "/tmp/rsmine-test-uploads".to_string(),
            max_file_size: 10485760,
        },
        password: PasswordConfig {
            min_length: 8,
            required_char_classes: vec![],
            max_age: 0,
        },
    };

    // Build app using the actual create_routes function
    create_routes(db, config)
}

/// Seed the database with test data
async fn seed_test_data(db: &DatabaseConnection) {
    use sea_orm::ConnectionTrait;

    // Insert admin user with hashed password 'admin123'
    // Password hash generated with Argon2 using: cargo run --bin hash_password
    // Password: admin123
    // Salt: 0e7xorW/878dteNtdDEBxw
    // Hash: $argon2id$v=19$m=19456,t=2,p=1$0e7xorW/878dteNtdDEBxw$pvkgDu1C4/cHqz0VfN+w/FZqRu+rl0eDUGJqwS3DlaE
    let password_hash = "$argon2id$v=19$m=19456,t=2,p=1$0e7xorW/878dteNtdDEBxw$pvkgDu1C4/cHqz0VfN+w/FZqRu+rl0eDUGJqwS3DlaE";

    let sql = format!(
        r#"
        INSERT OR IGNORE INTO users (login, hashed_password, firstname, lastname, admin, status, created_on, updated_on, mail_notification, salt, must_change_passwd, twofa_required)
        VALUES ('admin', '{}', 'Admin', 'User', 1, 1, datetime('now'), datetime('now'), 'all', '', 0, 0)
        "#,
        password_hash
    );

    db.execute_unprepared(&sql)
        .await
        .expect("Failed to insert admin user");

    // Insert email address for admin
    db.execute_unprepared(
        r#"
        INSERT OR IGNORE INTO email_addresses (user_id, address, is_default, notify, created_on, updated_on)
        VALUES (1, 'admin@example.com', 1, 1, datetime('now'), datetime('now'))
        "#,
    )
    .await
    .expect("Failed to insert admin email");

    // Note: trackers, issue_statuses, enumerations (priorities), and roles are seeded by migrations
}

/// Login as admin user and return JWT token
pub async fn login_as_admin(app: &Router) -> String {
    let login_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/login")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "username": "admin",
                        "password": "admin123"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        login_response.status(),
        StatusCode::OK,
        "Admin login failed"
    );

    let body = login_response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    json.get("token").unwrap().as_str().unwrap().to_string()
}

/// Create a test project and return its ID
///
/// This also associates all available trackers with the project,
/// which is required for issue creation to succeed.
pub async fn create_test_project(app: &Router, _token: &str, identifier: &str) -> i32 {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/projects.json")
                .header("Authorization", format!("Bearer {}", _token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "project": {
                            "name": format!("Test Project {}", identifier),
                            "identifier": identifier,
                            "is_public": true,
                            "tracker_ids": [1, 2, 3]  // Associate trackers via API
                        }
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(
        status,
        StatusCode::CREATED,
        "Failed to create test project. Response: {:?}",
        json
    );

    json.get("project")
        .unwrap()
        .get("id")
        .unwrap()
        .as_i64()
        .unwrap() as i32
}

/// Create a test issue and return its ID
pub async fn create_test_issue(app: &Router, token: &str, project_id: i32, subject: &str) -> i32 {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/issues.json")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "issue": {
                            "project_id": project_id,
                            "subject": subject,
                            "tracker_id": 1,
                            "status_id": 1,
                            "priority_id": 2
                        }
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(
        status,
        StatusCode::CREATED,
        "Failed to create test issue. Status: {:?}, Response: {:?}",
        status,
        json
    );

    json.get("issue")
        .unwrap()
        .get("id")
        .unwrap()
        .as_i64()
        .unwrap() as i32
}

/// Update an issue with notes to create a journal entry
pub async fn update_issue_with_notes(app: &Router, token: &str, issue_id: i32, notes: &str) {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/v1/issues/{}", issue_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "issue": {
                            "notes": notes
                        }
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8_lossy(&body);

    let json: Value = serde_json::from_slice(&body)
        .unwrap_or_else(|_| json!({ "raw_body": body_str.to_string() }));

    assert_eq!(
        status,
        StatusCode::OK,
        "Failed to update issue with notes. Status: {:?}, Response: {:?}",
        status,
        json
    );
}
