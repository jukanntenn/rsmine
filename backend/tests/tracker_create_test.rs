//! Integration tests for POST /api/v1/trackers.json - Admin creates tracker

mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

use common::{create_test_app, login_as_admin};

/// Test: Admin can create a new tracker successfully
#[tokio::test]
async fn admin_can_create_tracker() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/trackers.json")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "tracker": {
                            "name": "Bug Report",
                            "default_status_id": 1
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
        "Expected 201 Created. Response: {:?}",
        json
    );

    // Verify response structure
    assert!(
        json.get("tracker").is_some(),
        "Response should contain 'tracker' object"
    );
    let tracker = json.get("tracker").unwrap();
    assert!(tracker.get("id").is_some(), "Tracker should have 'id'");
    assert_eq!(tracker.get("name").unwrap().as_str().unwrap(), "Bug Report");
    assert!(
        tracker.get("default_status").is_some(),
        "Tracker should have 'default_status'"
    );
    assert_eq!(
        tracker
            .get("default_status")
            .unwrap()
            .get("id")
            .unwrap()
            .as_i64()
            .unwrap(),
        1
    );
}

/// Test: Tracker created with all optional fields
#[tokio::test]
async fn create_tracker_with_all_fields() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/trackers.json")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "tracker": {
                            "name": "Feature Request",
                            "default_status_id": 1,
                            "enabled_standard_fields": [
                                "assigned_to_id",
                                "category_id",
                                "start_date",
                                "due_date",
                                "description"
                            ]
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
        "Expected 201 Created. Response: {:?}",
        json
    );

    let tracker = json.get("tracker").unwrap();
    assert_eq!(
        tracker.get("name").unwrap().as_str().unwrap(),
        "Feature Request"
    );

    // Verify enabled_standard_fields
    let fields = tracker
        .get("enabled_standard_fields")
        .unwrap()
        .as_array()
        .unwrap();
    assert_eq!(fields.len(), 5, "Should have 5 enabled fields");
    assert!(fields.contains(&json!("assigned_to_id")));
    assert!(fields.contains(&json!("category_id")));
    assert!(fields.contains(&json!("start_date")));
    assert!(fields.contains(&json!("due_date")));
    assert!(fields.contains(&json!("description")));
}

/// Test: Creating tracker with duplicate name fails
#[tokio::test]
async fn create_tracker_duplicate_name_fails() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // First, create a tracker
    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/trackers.json")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "tracker": {
                            "name": "Unique Tracker",
                            "default_status_id": 1
                        }
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // Try to create another tracker with the same name
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/trackers.json")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "tracker": {
                            "name": "Unique Tracker",
                            "default_status_id": 1
                        }
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let status = response.status();
    assert_eq!(
        status,
        StatusCode::BAD_REQUEST,
        "Expected 400 for duplicate name"
    );
}

/// Test: Creating tracker with empty name fails
#[tokio::test]
async fn create_tracker_empty_name_fails() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/trackers.json")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "tracker": {
                            "name": "",
                            "default_status_id": 1
                        }
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let status = response.status();
    assert_eq!(
        status,
        StatusCode::BAD_REQUEST,
        "Expected 400 for empty name"
    );
}

/// Test: Creating tracker with invalid status_id fails
#[tokio::test]
async fn create_tracker_invalid_status_id_fails() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/trackers.json")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "tracker": {
                            "name": "Test Tracker",
                            "default_status_id": 99999
                        }
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let status = response.status();
    assert_eq!(
        status,
        StatusCode::BAD_REQUEST,
        "Expected 400 for invalid status_id"
    );
}

/// Test: Unauthenticated user cannot create tracker
#[tokio::test]
async fn unauthenticated_cannot_create_tracker() {
    let app = create_test_app().await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/trackers.json")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "tracker": {
                            "name": "Test Tracker",
                            "default_status_id": 1
                        }
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let status = response.status();
    assert_eq!(
        status,
        StatusCode::UNAUTHORIZED,
        "Expected 401 for unauthenticated request"
    );
}

/// Test: Non-admin user cannot create tracker
/// Note: This test creates a regular user via direct database manipulation
/// since the user creation API requires admin privileges
#[tokio::test]
async fn non_admin_cannot_create_tracker() {
    use rsmine::infrastructure::persistence::Migrator;
    use sea_orm::ConnectionTrait;
    use sea_orm_migration::MigratorTrait;

    // Create a test app with database access
    let db = sea_orm::Database::connect("sqlite::memory:")
        .await
        .expect("Failed to connect to in-memory database");

    // Run migrations
    Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations");

    // Create admin user first
    let password_hash = "$argon2id$v=19$m=19456,t=2,p=1$0e7xorW/878dteNtdDEBxw$pvkgDu1C4/cHqz0VfN+w/FZqRu+rl0eDUGJqwS3DlaE";
    db.execute_unprepared(&format!(
        r#"
        INSERT INTO users (login, hashed_password, firstname, lastname, admin, status, created_on, updated_on, mail_notification, salt, must_change_passwd, twofa_required)
        VALUES ('admin', '{}', 'Admin', 'User', 1, 1, datetime('now'), datetime('now'), 'all', '', 0, 0)
        "#,
        password_hash
    ))
    .await
    .expect("Failed to insert admin user");

    // Insert email address for admin
    db.execute_unprepared(
        r#"
        INSERT INTO email_addresses (user_id, address, is_default, notify, created_on, updated_on)
        VALUES (1, 'admin@example.com', 1, 1, datetime('now'), datetime('now'))
        "#,
    )
    .await
    .expect("Failed to insert admin email");

    // Create a non-admin user
    db.execute_unprepared(&format!(
        r#"
        INSERT INTO users (login, hashed_password, firstname, lastname, admin, status, created_on, updated_on, mail_notification, salt, must_change_passwd, twofa_required)
        VALUES ('regularuser', '{}', 'Regular', 'User', 0, 1, datetime('now'), datetime('now'), 'all', '', 0, 0)
        "#,
        password_hash
    ))
    .await
    .expect("Failed to insert regular user");

    // Insert email address for regular user
    db.execute_unprepared(
        r#"
        INSERT INTO email_addresses (user_id, address, is_default, notify, created_on, updated_on)
        VALUES (2, 'regular@example.com', 1, 1, datetime('now'), datetime('now'))
        "#,
    )
    .await
    .expect("Failed to insert regular user email");

    // Now create the app with the database
    let app = common::create_test_app_with_db(db).await;

    // Login as regular user
    let login_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/login")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "username": "regularuser",
                        "password": "admin123"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = login_response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    let user_token = json
        .get("token")
        .expect("Should get token from login")
        .as_str()
        .unwrap()
        .to_string();

    // Try to create tracker as non-admin
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/trackers.json")
                .header("Authorization", format!("Bearer {}", user_token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "tracker": {
                            "name": "Unauthorized Tracker",
                            "default_status_id": 1
                        }
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let status = response.status();
    assert_eq!(
        status,
        StatusCode::FORBIDDEN,
        "Expected 403 for non-admin request"
    );
}

/// Test: Default status is used when default_status_id not provided
#[tokio::test]
async fn create_tracker_uses_default_status_when_not_provided() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/trackers.json")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "tracker": {
                            "name": "Auto Status Tracker"
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
        "Expected 201 Created. Response: {:?}",
        json
    );

    let tracker = json.get("tracker").unwrap();
    // Should have a default status assigned
    assert!(
        tracker.get("default_status").is_some(),
        "Should have default_status assigned"
    );
}

/// Test: Created tracker appears in list
#[tokio::test]
async fn created_tracker_appears_in_list() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create a tracker
    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/trackers.json")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "tracker": {
                            "name": "List Test Tracker",
                            "default_status_id": 1
                        }
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let create_body = create_response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    let create_json: Value = serde_json::from_slice(&create_body).unwrap();
    let created_id = create_json
        .get("tracker")
        .unwrap()
        .get("id")
        .unwrap()
        .as_i64()
        .unwrap();

    // List trackers
    let list_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/trackers.json")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let list_body = list_response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    let list_json: Value = serde_json::from_slice(&list_body).unwrap();

    // Find the created tracker in the list
    let trackers = list_json.get("trackers").unwrap().as_array().unwrap();
    let found = trackers
        .iter()
        .any(|t| t.get("id").unwrap().as_i64().unwrap() == created_id);

    assert!(found, "Created tracker should appear in list");
}
