//! Integration tests for DELETE /api/v1/issue_statuses/:id - Admin deletes issue status

mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

use common::{create_test_app, create_test_issue, create_test_project, login_as_admin};

/// Helper function to create a test issue status
async fn create_test_status(app: &axum::Router, token: &str, name: &str) -> i32 {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/issue_statuses.json")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "issue_status": {
                            "name": name
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
    json.get("issue_status")
        .unwrap()
        .get("id")
        .unwrap()
        .as_i64()
        .unwrap() as i32
}

/// Test: Admin can delete an unused issue status
#[tokio::test]
async fn admin_can_delete_unused_status() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create a new status
    let status_id = create_test_status(&app, &token, "Status To Delete").await;

    // Verify status exists by getting it
    let get_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/issue_statuses/{}", status_id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        get_response.status(),
        StatusCode::OK,
        "Status should exist before delete"
    );

    // Delete the status
    let delete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/v1/issue_statuses/{}", status_id))
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

    // Verify status no longer exists
    let get_after_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/issue_statuses/{}", status_id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        get_after_response.status(),
        StatusCode::NOT_FOUND,
        "Status should not exist after delete"
    );
}

/// Test: Cannot delete status with issues without reassignment
#[tokio::test]
async fn cannot_delete_status_with_issues_without_reassignment() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create a project
    let project_id = create_test_project(&app, &token, "test-project-status").await;

    // Create an issue (using default status_id=1)
    let _issue_id = create_test_issue(&app, &token, project_id, "Test Issue with Status").await;

    // Try to delete status_id=1 which is used by the issue
    let delete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/v1/issue_statuses/1")
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
        "Expected 400 Bad Request when deleting status with issues. Response: {:?}",
        json
    );
    assert!(
        json.get("errors").is_some(),
        "Response should contain 'errors'"
    );
}

/// Test: Can delete status with reassignment
#[tokio::test]
async fn can_delete_status_with_reassignment() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create a project
    let project_id = create_test_project(&app, &token, "test-project-reassign").await;

    // Create a new status to reassign to
    let new_status_id = create_test_status(&app, &token, "Target Status").await;

    // Create an issue (using default status_id=1)
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
                            "subject": "Test Issue for Reassign",
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

    // Delete status_id=1 with reassignment to new_status_id
    let delete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/v1/issue_statuses/1")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "reassign_to_id": new_status_id
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        delete_response.status(),
        StatusCode::NO_CONTENT,
        "Expected 204 No Content with reassignment"
    );

    // Verify the issue now has the new status
    let get_issue_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/issues/{}", issue_id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = get_issue_response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    let status_id = json
        .get("issue")
        .unwrap()
        .get("status")
        .unwrap()
        .get("id")
        .unwrap()
        .as_i64()
        .unwrap() as i32;

    assert_eq!(
        status_id, new_status_id,
        "Issue should be reassigned to new status"
    );
}

/// Test: Cannot reassign to same status being deleted
#[tokio::test]
async fn cannot_reassign_to_same_status() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create a project and issue with status_id=1
    let project_id = create_test_project(&app, &token, "test-project-same").await;
    let _issue_id = create_test_issue(&app, &token, project_id, "Test Issue").await;

    // Try to delete status with reassignment to same status
    let delete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/v1/issue_statuses/1")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "reassign_to_id": 1
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let status = delete_response.status();
    assert_eq!(
        status,
        StatusCode::BAD_REQUEST,
        "Expected 400 when reassigning to same status"
    );
}

/// Test: Cannot reassign to non-existent status
#[tokio::test]
async fn cannot_reassign_to_non_existent_status() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create a project and issue with status_id=1
    let project_id = create_test_project(&app, &token, "test-project-nonexist").await;
    let _issue_id = create_test_issue(&app, &token, project_id, "Test Issue").await;

    // Try to delete status with reassignment to non-existent status
    let delete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/v1/issue_statuses/1")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "reassign_to_id": 99999
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let status = delete_response.status();
    assert_eq!(
        status,
        StatusCode::NOT_FOUND,
        "Expected 404 when reassigning to non-existent status"
    );
}

/// Test: Deleting non-existent status returns 404
#[tokio::test]
async fn delete_non_existent_status_returns_404() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/v1/issue_statuses/99999")
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
        "Expected 404 for non-existent status"
    );
}

/// Test: Unauthenticated user cannot delete status
#[tokio::test]
async fn unauthenticated_cannot_delete_status() {
    let app = create_test_app().await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/v1/issue_statuses/1")
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

/// Test: Non-admin user cannot delete status
#[tokio::test]
async fn non_admin_cannot_delete_status() {
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

    // Try to delete status as non-admin
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/v1/issue_statuses/1")
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

/// Test: Deleted status is removed from list
#[tokio::test]
async fn deleted_status_removed_from_list() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create a new status
    let status_id = create_test_status(&app, &token, "Temporary Status").await;

    // Verify it appears in list
    let list_response_before = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/issue_statuses.json")
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
    let statuses_before = json.get("issue_statuses").unwrap().as_array().unwrap();
    let count_before = statuses_before.len();

    // Delete the status
    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/v1/issue_statuses/{}", status_id))
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
                .uri("/api/v1/issue_statuses.json")
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
    let statuses_after = json.get("issue_statuses").unwrap().as_array().unwrap();
    let count_after = statuses_after.len();

    assert_eq!(
        count_after,
        count_before - 1,
        "Status count should decrease by 1 after delete"
    );

    // Verify the specific status is not in the list
    let found = statuses_after
        .iter()
        .any(|s| s.get("id").unwrap().as_i64().unwrap() == status_id as i64);
    assert!(!found, "Deleted status should not appear in list");
}
