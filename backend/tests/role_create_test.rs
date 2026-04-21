//! Integration tests for POST /api/v1/roles.json - Admin creates role

mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

use common::{create_test_app, login_as_admin};

/// Test: Admin can create a new role successfully
#[tokio::test]
async fn admin_can_create_role() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/roles.json")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "role": {
                            "name": "Project Manager",
                            "permissions": [
                                "view_project",
                                "view_issues",
                                "add_issues",
                                "edit_issues"
                            ],
                            "issues_visibility": "all",
                            "assignable": true
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
        json.get("role").is_some(),
        "Response should contain 'role' object"
    );
    let role = json.get("role").unwrap();
    assert!(role.get("id").is_some(), "Role should have 'id'");
    assert_eq!(
        role.get("name").unwrap().as_str().unwrap(),
        "Project Manager"
    );
    assert!(
        role.get("permissions").is_some(),
        "Role should have 'permissions'"
    );
    assert_eq!(
        role.get("issues_visibility").unwrap().as_str().unwrap(),
        "all"
    );
}

/// Test: Role created with minimal fields (only name)
#[tokio::test]
async fn create_role_with_minimal_fields() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/roles.json")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "role": {
                            "name": "Minimal Role"
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

    let role = json.get("role").unwrap();
    assert_eq!(role.get("name").unwrap().as_str().unwrap(), "Minimal Role");
    // Default issues_visibility should be "default"
    assert_eq!(
        role.get("issues_visibility").unwrap().as_str().unwrap(),
        "default"
    );
    // Permissions should be empty array when not provided
    let permissions = role.get("permissions").unwrap().as_array().unwrap();
    assert_eq!(
        permissions.len(),
        0,
        "Permissions should be empty when not provided"
    );
}

/// Test: Creating role with duplicate name fails
#[tokio::test]
async fn create_role_duplicate_name_fails() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // First, create a role
    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/roles.json")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "role": {
                            "name": "Unique Role"
                        }
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // Try to create another role with the same name
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/roles.json")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "role": {
                            "name": "Unique Role"
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

/// Test: Creating role with empty name fails
#[tokio::test]
async fn create_role_empty_name_fails() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/roles.json")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "role": {
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

/// Test: Creating role with invalid issues_visibility fails
#[tokio::test]
async fn create_role_invalid_issues_visibility_fails() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/roles.json")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "role": {
                            "name": "Test Role",
                            "issues_visibility": "invalid_value"
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
        "Expected 400 for invalid issues_visibility"
    );
}

/// Test: Unauthenticated user cannot create role
#[tokio::test]
async fn unauthenticated_cannot_create_role() {
    let app = create_test_app().await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/roles.json")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "role": {
                            "name": "Test Role"
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

/// Test: Non-admin user cannot create role
#[tokio::test]
async fn non_admin_cannot_create_role() {
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

    // Try to create role as non-admin
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/roles.json")
                .header("Authorization", format!("Bearer {}", user_token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "role": {
                            "name": "Unauthorized Role"
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

/// Test: Role with all valid issues_visibility values
#[tokio::test]
async fn create_role_with_all_visibility_options() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Test "all" visibility
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/roles.json")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "role": {
                            "name": "All Visibility Role",
                            "issues_visibility": "all"
                        }
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    // Test "default" visibility
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/roles.json")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "role": {
                            "name": "Default Visibility Role",
                            "issues_visibility": "default"
                        }
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    // Test "own" visibility
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/roles.json")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "role": {
                            "name": "Own Visibility Role",
                            "issues_visibility": "own"
                        }
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
}

/// Test: Created role appears in list
#[tokio::test]
async fn created_role_appears_in_list() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create a role
    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/roles.json")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "role": {
                            "name": "List Test Role",
                            "permissions": ["view_project", "view_issues"]
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
        .get("role")
        .unwrap()
        .get("id")
        .unwrap()
        .as_i64()
        .unwrap();

    // List roles
    let list_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/roles.json")
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

    // Find the created role in the list
    let roles = list_json.get("roles").unwrap().as_array().unwrap();
    let found = roles
        .iter()
        .any(|r| r.get("id").unwrap().as_i64().unwrap() == created_id);

    assert!(found, "Created role should appear in list");
}

/// Test: Role with many permissions
#[tokio::test]
async fn create_role_with_many_permissions() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/roles.json")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "role": {
                            "name": "Full Permissions Role",
                            "permissions": [
                                "view_project",
                                "view_issues",
                                "add_issues",
                                "edit_issues",
                                "delete_issues",
                                "manage_members",
                                "manage_categories",
                                "manage_issue_relations",
                                "manage_files",
                                "view_files"
                            ],
                            "issues_visibility": "all",
                            "assignable": true
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

    let role = json.get("role").unwrap();
    let permissions = role.get("permissions").unwrap().as_array().unwrap();
    assert_eq!(permissions.len(), 10, "Should have 10 permissions");
}
