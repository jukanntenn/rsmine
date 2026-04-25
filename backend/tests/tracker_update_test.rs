//! Integration tests for PUT /api/v1/trackers/:id - Admin updates tracker

mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

use common::{create_test_app, login_as_admin};

/// Helper function to create a test tracker
async fn create_test_tracker(app: &axum::Router, token: &str, name: &str) -> i32 {
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
                            "name": name,
                            "default_status_id": 1
                        }
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    json.get("tracker")
        .unwrap()
        .get("id")
        .unwrap()
        .as_i64()
        .unwrap() as i32
}

/// Test: Admin can update tracker name
#[tokio::test]
async fn admin_can_update_tracker_name() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create a tracker first
    let tracker_id = create_test_tracker(&app, &token, "Original Name").await;

    // Update the tracker
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/v1/trackers/{}", tracker_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "tracker": {
                            "name": "Updated Name"
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
        StatusCode::OK,
        "Expected 200 OK. Response: {:?}",
        json
    );

    let tracker = json.get("tracker").unwrap();
    assert_eq!(
        tracker.get("name").unwrap().as_str().unwrap(),
        "Updated Name"
    );
}

/// Test: Admin can update tracker position
#[tokio::test]
async fn admin_can_update_tracker_position() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create a tracker first
    let tracker_id = create_test_tracker(&app, &token, "Position Test Tracker").await;

    // Update the tracker position
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/v1/trackers/{}", tracker_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "tracker": {
                            "position": 5
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
        StatusCode::OK,
        "Expected 200 OK. Response: {:?}",
        json
    );

    let tracker = json.get("tracker").unwrap();
    assert_eq!(tracker.get("position").unwrap().as_i64().unwrap(), 5);
}

/// Test: Admin can update tracker is_in_roadmap
#[tokio::test]
async fn admin_can_update_tracker_is_in_roadmap() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create a tracker first
    let tracker_id = create_test_tracker(&app, &token, "Roadmap Test Tracker").await;

    // Update the tracker is_in_roadmap to false
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/v1/trackers/{}", tracker_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "tracker": {
                            "is_in_roadmap": false
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
        StatusCode::OK,
        "Expected 200 OK. Response: {:?}",
        json
    );

    let tracker = json.get("tracker").unwrap();
    assert!(
        !tracker.get("is_in_roadmap").unwrap().as_bool().unwrap()
    );
}

/// Test: Admin can update tracker default_status_id
#[tokio::test]
async fn admin_can_update_tracker_default_status_id() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create a tracker first
    let tracker_id = create_test_tracker(&app, &token, "Status Test Tracker").await;

    // Update the tracker default_status_id
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/v1/trackers/{}", tracker_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "tracker": {
                            "default_status_id": 2
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
        StatusCode::OK,
        "Expected 200 OK. Response: {:?}",
        json
    );

    let tracker = json.get("tracker").unwrap();
    assert_eq!(
        tracker
            .get("default_status")
            .unwrap()
            .get("id")
            .unwrap()
            .as_i64()
            .unwrap(),
        2
    );
}

/// Test: Admin can update multiple fields at once
#[tokio::test]
async fn admin_can_update_multiple_fields() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create a tracker first
    let tracker_id = create_test_tracker(&app, &token, "Multi Update Tracker").await;

    // Update multiple fields
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/v1/trackers/{}", tracker_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "tracker": {
                            "name": "Multi Updated",
                            "position": 10,
                            "is_in_roadmap": false,
                            "default_status_id": 2
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
        StatusCode::OK,
        "Expected 200 OK. Response: {:?}",
        json
    );

    let tracker = json.get("tracker").unwrap();
    assert_eq!(
        tracker.get("name").unwrap().as_str().unwrap(),
        "Multi Updated"
    );
    assert_eq!(tracker.get("position").unwrap().as_i64().unwrap(), 10);
    assert!(
        !tracker.get("is_in_roadmap").unwrap().as_bool().unwrap()
    );
    assert_eq!(
        tracker
            .get("default_status")
            .unwrap()
            .get("id")
            .unwrap()
            .as_i64()
            .unwrap(),
        2
    );
}

/// Test: Updating tracker with duplicate name fails
#[tokio::test]
async fn update_tracker_duplicate_name_fails() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create two trackers
    let _tracker1_id = create_test_tracker(&app, &token, "Tracker One").await;
    let tracker2_id = create_test_tracker(&app, &token, "Tracker Two").await;

    // Try to update tracker2's name to tracker1's name
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/v1/trackers/{}", tracker2_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "tracker": {
                            "name": "Tracker One"
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

/// Test: Updating tracker with empty name fails
#[tokio::test]
async fn update_tracker_empty_name_fails() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create a tracker first
    let tracker_id = create_test_tracker(&app, &token, "Empty Name Test").await;

    // Try to update with empty name
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/v1/trackers/{}", tracker_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "tracker": {
                            "name": ""
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

/// Test: Updating non-existent tracker returns 404
#[tokio::test]
async fn update_non_existent_tracker_returns_404() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/api/v1/trackers/99999")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "tracker": {
                            "name": "Non-existent"
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
        StatusCode::NOT_FOUND,
        "Expected 404 for non-existent tracker"
    );
}

/// Test: Updating with invalid default_status_id fails
#[tokio::test]
async fn update_tracker_invalid_status_id_fails() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create a tracker first
    let tracker_id = create_test_tracker(&app, &token, "Invalid Status Test").await;

    // Try to update with invalid status_id
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/v1/trackers/{}", tracker_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "tracker": {
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

/// Test: Unauthenticated user cannot update tracker
#[tokio::test]
async fn unauthenticated_cannot_update_tracker() {
    let app = create_test_app().await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/api/v1/trackers/1")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "tracker": {
                            "name": "Unauthorized Update"
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

/// Test: Non-admin user cannot update tracker
#[tokio::test]
async fn non_admin_cannot_update_tracker() {
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
        INSERT OR IGNORE INTO users (login, hashed_password, firstname, lastname, admin, status, created_on, updated_on, mail_notification, salt, must_change_passwd, twofa_required)
        VALUES ('admin', '{}', 'Admin', 'User', 1, 1, datetime('now'), datetime('now'), 'all', '', 0, 0)
        "#,
        password_hash
    ))
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

    // Create a non-admin user
    db.execute_unprepared(&format!(
        r#"
        INSERT OR IGNORE INTO users (login, hashed_password, firstname, lastname, admin, status, created_on, updated_on, mail_notification, salt, must_change_passwd, twofa_required)
        VALUES ('regularuser', '{}', 'Regular', 'User', 0, 1, datetime('now'), datetime('now'), 'all', '', 0, 0)
        "#,
        password_hash
    ))
    .await
    .expect("Failed to insert regular user");

    // Insert email address for regular user
    db.execute_unprepared(
        r#"
        INSERT OR IGNORE INTO email_addresses (user_id, address, is_default, notify, created_on, updated_on)
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

    // Try to update tracker as non-admin
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/api/v1/trackers/1")
                .header("Authorization", format!("Bearer {}", user_token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "tracker": {
                            "name": "Unauthorized Update"
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

/// Test: Update enabled_standard_fields
#[tokio::test]
async fn admin_can_update_enabled_standard_fields() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create a tracker first
    let tracker_id = create_test_tracker(&app, &token, "Fields Test Tracker").await;

    // Update enabled_standard_fields
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/v1/trackers/{}", tracker_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "tracker": {
                            "enabled_standard_fields": [
                                "assigned_to_id",
                                "category_id",
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
        StatusCode::OK,
        "Expected 200 OK. Response: {:?}",
        json
    );

    let tracker = json.get("tracker").unwrap();
    let fields = tracker
        .get("enabled_standard_fields")
        .unwrap()
        .as_array()
        .unwrap();
    assert_eq!(fields.len(), 3, "Should have 3 enabled fields");
    assert!(fields.contains(&json!("assigned_to_id")));
    assert!(fields.contains(&json!("category_id")));
    assert!(fields.contains(&json!("description")));
}
