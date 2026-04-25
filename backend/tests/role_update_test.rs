//! Integration tests for PUT /api/v1/roles/:id - Admin updates role

mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

use common::{create_test_app, login_as_admin};

/// Helper to create a role and return its ID
async fn create_test_role(app: &Router, token: &str, name: &str) -> i32 {
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
                            "name": name,
                            "permissions": ["view_project", "view_issues"],
                            "issues_visibility": "default",
                            "assignable": true
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
    json.get("role")
        .unwrap()
        .get("id")
        .unwrap()
        .as_i64()
        .unwrap() as i32
}

/// Test: Admin can update a role successfully
#[tokio::test]
async fn admin_can_update_role() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create a role first
    let role_id = create_test_role(&app, &token, "Role To Update").await;

    // Update the role
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/v1/roles/{}", role_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "role": {
                            "name": "Updated Role Name",
                            "permissions": ["view_project", "view_issues", "add_issues"],
                            "issues_visibility": "all",
                            "assignable": false
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

    // Verify response structure
    assert!(
        json.get("role").is_some(),
        "Response should contain 'role' object"
    );
    let role = json.get("role").unwrap();
    assert_eq!(role.get("id").unwrap().as_i64().unwrap(), role_id as i64);
    assert_eq!(
        role.get("name").unwrap().as_str().unwrap(),
        "Updated Role Name"
    );

    let permissions = role.get("permissions").unwrap().as_array().unwrap();
    assert_eq!(permissions.len(), 3, "Should have 3 permissions");

    assert_eq!(
        role.get("issues_visibility").unwrap().as_str().unwrap(),
        "all"
    );
}

/// Test: Update only the role name
#[tokio::test]
async fn update_role_name_only() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create a role
    let role_id = create_test_role(&app, &token, "Original Name").await;

    // Update only name
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/v1/roles/{}", role_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "role": {
                            "name": "New Name Only"
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

    let role = json.get("role").unwrap();
    assert_eq!(role.get("name").unwrap().as_str().unwrap(), "New Name Only");
    // Other fields should remain unchanged
    let permissions = role.get("permissions").unwrap().as_array().unwrap();
    assert_eq!(permissions.len(), 2, "Permissions should remain unchanged");
}

/// Test: Update only the permissions
#[tokio::test]
async fn update_role_permissions_only() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create a role
    let role_id = create_test_role(&app, &token, "Permission Test Role").await;

    // Update only permissions
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/v1/roles/{}", role_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "role": {
                            "permissions": ["view_project", "view_issues", "add_issues", "edit_issues", "delete_issues"]
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

    let role = json.get("role").unwrap();
    let permissions = role.get("permissions").unwrap().as_array().unwrap();
    assert_eq!(permissions.len(), 5, "Should have 5 permissions");
    // Name should remain unchanged
    assert_eq!(
        role.get("name").unwrap().as_str().unwrap(),
        "Permission Test Role"
    );
}

/// Test: Update issues_visibility only
#[tokio::test]
async fn update_role_visibility_only() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create a role
    let role_id = create_test_role(&app, &token, "Visibility Test Role").await;

    // Update only visibility
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/v1/roles/{}", role_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "role": {
                            "issues_visibility": "own"
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

    let role = json.get("role").unwrap();
    assert_eq!(
        role.get("issues_visibility").unwrap().as_str().unwrap(),
        "own"
    );
}

/// Test: Update assignable only
#[tokio::test]
async fn update_role_assignable_only() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create a role with assignable: true
    let role_id = create_test_role(&app, &token, "Assignable Test Role").await;

    // Update only assignable to false
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/v1/roles/{}", role_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "role": {
                            "assignable": false
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
}

/// Test: Non-admin cannot update role
#[tokio::test]
async fn non_admin_cannot_update_role() {
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
    let db = sea_orm::Database::connect("sqlite::memory:")
        .await
        .expect("Failed to connect to in-memory database");

    Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations");

    let password_hash = "$argon2id$v=19$m=19456,t=2,p=1$0e7xorW/878dteNtdDEBxw$pvkgDu1C4/cHqz0VfN+w/FZqRu+rl0eDUGJqwS3DlaE";

    // Create admin user
    db.execute_unprepared(&format!(
        r#"
        INSERT OR IGNORE INTO users (login, hashed_password, firstname, lastname, admin, status, created_on, updated_on, mail_notification, salt, must_change_passwd, twofa_required)
        VALUES ('admin', '{}', 'Admin', 'User', 1, 1, datetime('now'), datetime('now'), 'all', '', 0, 0)
        "#,
        password_hash
    ))
    .await
    .expect("Failed to insert admin user");

    db.execute_unprepared(
        r#"
        INSERT OR IGNORE INTO email_addresses (user_id, address, is_default, notify, created_on, updated_on)
        VALUES (1, 'admin@example.com', 1, 1, datetime('now'), datetime('now'))
        "#,
    )
    .await
    .expect("Failed to insert admin email");

    // Create regular user
    db.execute_unprepared(&format!(
        r#"
        INSERT OR IGNORE INTO users (login, hashed_password, firstname, lastname, admin, status, created_on, updated_on, mail_notification, salt, must_change_passwd, twofa_required)
        VALUES ('regularuser', '{}', 'Regular', 'User', 0, 1, datetime('now'), datetime('now'), 'all', '', 0, 0)
        "#,
        password_hash
    ))
    .await
    .expect("Failed to insert regular user");

    db.execute_unprepared(
        r#"
        INSERT OR IGNORE INTO email_addresses (user_id, address, is_default, notify, created_on, updated_on)
        VALUES (2, 'regular@example.com', 1, 1, datetime('now'), datetime('now'))
        "#,
    )
    .await
    .expect("Failed to insert regular user email");

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

    // Try to update a role as non-admin
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/api/v1/roles/1")
                .header("Authorization", format!("Bearer {}", user_token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "role": {
                            "name": "Hacked Role"
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

/// Test: Updating role with duplicate name fails
#[tokio::test]
async fn update_role_duplicate_name_fails() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create two roles
    let _role1_id = create_test_role(&app, &token, "First Role").await;
    let role2_id = create_test_role(&app, &token, "Second Role").await;

    // Try to update role2 with the same name as role1
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/v1/roles/{}", role2_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "role": {
                            "name": "First Role"
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

/// Test: Updating role with empty name fails
#[tokio::test]
async fn update_role_empty_name_fails() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create a role
    let role_id = create_test_role(&app, &token, "Role With Name").await;

    // Try to update with empty name
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/v1/roles/{}", role_id))
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

/// Test: Updating role with invalid issues_visibility fails
#[tokio::test]
async fn update_role_invalid_issues_visibility_fails() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create a role
    let role_id = create_test_role(&app, &token, "Visibility Validation Role").await;

    // Try to update with invalid visibility
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/v1/roles/{}", role_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "role": {
                            "issues_visibility": "invalid_visibility"
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

/// Test: Updating non-existent role returns 404
#[tokio::test]
async fn update_non_existent_role_returns_404() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/api/v1/roles/99999")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "role": {
                            "name": "Non-existent Role"
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
        "Expected 404 for non-existent role"
    );
}

/// Test: Unauthenticated user cannot update role
#[tokio::test]
async fn unauthenticated_cannot_update_role() {
    let app = create_test_app().await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/api/v1/roles/1")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "role": {
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

/// Test: Update role with all valid issues_visibility values
#[tokio::test]
async fn update_role_with_all_visibility_options() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create a role
    let role_id = create_test_role(&app, &token, "Visibility Options Test").await;

    // Test "all" visibility
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/v1/roles/{}", role_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "role": {
                            "issues_visibility": "all"
                        }
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Test "default" visibility
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/v1/roles/{}", role_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "role": {
                            "issues_visibility": "default"
                        }
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Test "own" visibility
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/v1/roles/{}", role_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "role": {
                            "issues_visibility": "own"
                        }
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

/// Test: Clear permissions by providing empty array
#[tokio::test]
async fn update_role_clear_permissions() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create a role with permissions
    let role_id = create_test_role(&app, &token, "Clear Permissions Test").await;

    // Clear permissions
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/v1/roles/{}", role_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "role": {
                            "permissions": []
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

    let role = json.get("role").unwrap();
    let permissions = role.get("permissions").unwrap().as_array().unwrap();
    assert_eq!(permissions.len(), 0, "Permissions should be empty");
}

/// Test: Role can keep its own name when updating other fields
#[tokio::test]
async fn update_role_keep_same_name() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create a role
    let role_id = create_test_role(&app, &token, "Same Name Role").await;

    // Update other fields but keep the same name
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/v1/roles/{}", role_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "role": {
                            "name": "Same Name Role",
                            "issues_visibility": "all"
                        }
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let status = response.status();
    assert_eq!(status, StatusCode::OK, "Should allow keeping same name");
}
