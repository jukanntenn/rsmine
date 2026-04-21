//! Integration tests for DELETE /api/v1/trackers/:id - Admin deletes tracker

mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

use common::{create_test_app, create_test_issue, create_test_project, login_as_admin};

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

/// Test: Admin can delete an unused tracker
#[tokio::test]
async fn admin_can_delete_unused_tracker() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create a tracker
    let tracker_id = create_test_tracker(&app, &token, "Tracker To Delete").await;

    // Verify tracker exists by getting it
    let get_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/trackers/{}", tracker_id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        get_response.status(),
        StatusCode::OK,
        "Tracker should exist before delete"
    );

    // Delete the tracker
    let delete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/v1/trackers/{}", tracker_id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        delete_response.status(),
        StatusCode::NO_CONTENT,
        "Expected 204 No Content on successful delete"
    );

    // Verify tracker no longer exists
    let get_after_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/trackers/{}", tracker_id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        get_after_response.status(),
        StatusCode::NOT_FOUND,
        "Tracker should not exist after delete"
    );
}

/// Test: Cannot delete a tracker that is in use by issues
#[tokio::test]
async fn cannot_delete_tracker_in_use() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create a project with trackers
    let project_id = create_test_project(&app, &token, "test-project-in-use").await;

    // Create an issue using tracker_id=1 (existing tracker)
    let _issue_id = create_test_issue(&app, &token, project_id, "Test Issue with Tracker").await;

    // Try to delete the tracker (tracker_id=1 is used by the issue)
    let delete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/v1/trackers/1")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let status = delete_response.status();
    let body = delete_response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(
        status,
        StatusCode::BAD_REQUEST,
        "Expected 400 Bad Request when deleting tracker in use. Response: {:?}",
        json
    );
    assert!(
        json.get("errors").is_some(),
        "Response should contain 'errors'"
    );
}

/// Test: Deleting non-existent tracker returns 404
#[tokio::test]
async fn delete_non_existent_tracker_returns_404() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/v1/trackers/99999")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
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

/// Test: Unauthenticated user cannot delete tracker
#[tokio::test]
async fn unauthenticated_cannot_delete_tracker() {
    let app = create_test_app().await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/v1/trackers/1")
                .body(Body::empty())
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

/// Test: Non-admin user cannot delete tracker
#[tokio::test]
async fn non_admin_cannot_delete_tracker() {
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

    // Try to delete tracker as non-admin
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/v1/trackers/1")
                .header("Authorization", format!("Bearer {}", user_token))
                .body(Body::empty())
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

/// Test: Deleted tracker is removed from list
#[tokio::test]
async fn deleted_tracker_removed_from_list() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create a new tracker
    let tracker_id = create_test_tracker(&app, &token, "Temporary Tracker").await;

    // Verify it appears in list
    let list_response_before = app
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

    let body = list_response_before
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    let trackers_before = json.get("trackers").unwrap().as_array().unwrap();
    let count_before = trackers_before.len();

    // Delete the tracker
    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/v1/trackers/{}", tracker_id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Verify it's removed from list
    let list_response_after = app
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

    let body = list_response_after
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    let trackers_after = json.get("trackers").unwrap().as_array().unwrap();
    let count_after = trackers_after.len();

    assert_eq!(
        count_after,
        count_before - 1,
        "Tracker count should decrease by 1 after delete"
    );

    // Verify the specific tracker is not in the list
    let found = trackers_after
        .iter()
        .any(|t| t.get("id").unwrap().as_i64().unwrap() == tracker_id as i64);
    assert!(!found, "Deleted tracker should not appear in list");
}

/// Test: Can delete tracker after all issues using it are deleted
#[tokio::test]
async fn can_delete_tracker_after_issues_deleted() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create a new tracker
    let tracker_id = create_test_tracker(&app, &token, "Reusable Tracker").await;

    // Create a project with this tracker (use tracker_ids to associate)
    let project_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/projects.json")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "project": {
                            "name": "Test Project Reuse",
                            "identifier": "test-project-reuse",
                            "is_public": true,
                            "tracker_ids": [tracker_id]
                        }
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = project_response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    let project_id = json
        .get("project")
        .unwrap()
        .get("id")
        .unwrap()
        .as_i64()
        .unwrap() as i32;

    // Create an issue using this new tracker
    let issue_response = app
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
                            "subject": "Test Issue",
                            "tracker_id": tracker_id,
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

    let body = issue_response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    let issue_id = json
        .get("issue")
        .unwrap()
        .get("id")
        .unwrap()
        .as_i64()
        .unwrap() as i32;

    // Try to delete the tracker - should fail
    let delete_tracker_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/v1/trackers/{}", tracker_id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        delete_tracker_response.status(),
        StatusCode::BAD_REQUEST,
        "Should not be able to delete tracker with issues"
    );

    // Delete the issue
    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/v1/issues/{}", issue_id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Now try to delete the tracker again - should succeed
    let delete_tracker_response2 = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/v1/trackers/{}", tracker_id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        delete_tracker_response2.status(),
        StatusCode::NO_CONTENT,
        "Should be able to delete tracker after issues are deleted"
    );
}
