//! Integration tests for DELETE /api/v1/roles/:id - Admin deletes role

mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use http_body_util::BodyExt;
use sea_orm::ConnectionTrait;
use sea_orm_migration::MigratorTrait;
use serde_json::{json, Value};
use tower::ServiceExt;

use common::{create_test_app, create_test_project, login_as_admin};

/// Helper to create a custom role and return its ID
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

/// Test: Admin can delete an unused custom role
#[tokio::test]
async fn admin_can_delete_unused_custom_role() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create a custom role
    let role_id = create_test_role(&app, &token, "Role To Delete").await;

    // Verify role exists by getting it
    let get_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/roles/{}", role_id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        get_response.status(),
        StatusCode::OK,
        "Role should exist before delete"
    );

    // Delete the role
    let delete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/v1/roles/{}", role_id))
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

    // Verify role no longer exists
    let get_after_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/roles/{}", role_id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        get_after_response.status(),
        StatusCode::NOT_FOUND,
        "Role should not exist after delete"
    );
}

/// Test: Cannot delete a builtin role (Non-Member, builtin=1)
#[tokio::test]
async fn cannot_delete_builtin_non_member_role() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Role ID 4 is "Non-Member" with builtin=1
    let delete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/v1/roles/4")
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

    // Validation errors return 400 BAD_REQUEST
    assert_eq!(
        status,
        StatusCode::BAD_REQUEST,
        "Expected 400 when deleting builtin Non-Member role. Response: {:?}",
        json
    );
    assert!(
        json.get("errors").is_some(),
        "Response should contain 'errors'"
    );
}

/// Test: Cannot delete a builtin role (Anonymous, builtin=2)
#[tokio::test]
async fn cannot_delete_builtin_anonymous_role() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Role ID 5 is "Anonymous" with builtin=2
    let delete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/v1/roles/5")
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

    // Validation errors return 400 BAD_REQUEST
    assert_eq!(
        status,
        StatusCode::BAD_REQUEST,
        "Expected 400 when deleting builtin Anonymous role. Response: {:?}",
        json
    );
    assert!(
        json.get("errors").is_some(),
        "Response should contain 'errors'"
    );
}

/// Test: Cannot delete a role that is in use (has members assigned)
#[tokio::test]
async fn cannot_delete_role_in_use() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create a project - this automatically adds the admin as a member with Reporter role (role_id=3)
    // Note: add_manager uses role_id=3
    let _project_id = create_test_project(&app, &token, "test-role-in-use").await;

    // Try to delete the Reporter role (role_id=3) - it's now in use
    let delete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/v1/roles/3")
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

    // Validation errors return 400 BAD_REQUEST
    assert_eq!(
        status,
        StatusCode::BAD_REQUEST,
        "Expected 400 when deleting role in use. Response: {:?}",
        json
    );
    assert!(
        json.get("errors").is_some(),
        "Response should contain 'errors'"
    );
}

/// Test: Deleting non-existent role returns 404
#[tokio::test]
async fn delete_non_existent_role_returns_404() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/v1/roles/99999")
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
        "Expected 404 for non-existent role"
    );
}

/// Test: Unauthenticated user cannot delete role
#[tokio::test]
async fn unauthenticated_cannot_delete_role() {
    let app = create_test_app().await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/v1/roles/1")
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

/// Test: Non-admin user cannot delete role
#[tokio::test]
async fn non_admin_cannot_delete_role() {
    use rsmine::infrastructure::persistence::Migrator;

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

    db.execute_unprepared(
        r#"
        INSERT INTO email_addresses (user_id, address, is_default, notify, created_on, updated_on)
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

    // Try to delete role as non-admin
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/v1/roles/1")
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

/// Test: Deleted role is removed from list
#[tokio::test]
async fn deleted_role_removed_from_list() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create a new role
    let role_id = create_test_role(&app, &token, "Temporary Role").await;

    // Verify it appears in list
    let list_response_before = app
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

    let body = list_response_before
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    let roles_before = json.get("roles").unwrap().as_array().unwrap();
    let count_before = roles_before.len();

    // Delete the role
    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/v1/roles/{}", role_id))
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
                .uri("/api/v1/roles.json")
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
    let roles_after = json.get("roles").unwrap().as_array().unwrap();
    let count_after = roles_after.len();

    assert_eq!(
        count_after,
        count_before - 1,
        "Role count should decrease by 1 after delete"
    );

    // Verify the specific role is not in the list
    let found = roles_after
        .iter()
        .any(|r| r.get("id").unwrap().as_i64().unwrap() == role_id as i64);
    assert!(!found, "Deleted role should not appear in list");
}

/// Test: Can delete role after member assignment is removed
#[tokio::test]
async fn can_delete_role_after_member_removed() {
    use rsmine::infrastructure::persistence::entities::member_roles;
    use rsmine::infrastructure::persistence::Migrator;
    use sea_orm::{ActiveModelTrait, ColumnTrait, DeleteResult, EntityTrait, QueryFilter, Set};

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

    db.execute_unprepared(
        r#"
        INSERT INTO email_addresses (user_id, address, is_default, notify, created_on, updated_on)
        VALUES (1, 'admin@example.com', 1, 1, datetime('now'), datetime('now'))
        "#,
    )
    .await
    .expect("Failed to insert admin email");

    // Create the app with the database
    let app = common::create_test_app_with_db(db.clone()).await;

    // Login as admin
    let token = login_as_admin(&app).await;

    // Create a custom role
    let role_id = create_test_role(&app, &token, "Role To Test Member Removal").await;

    // Create a project - this adds admin as member with Reporter role (role_id=3)
    let _project_id = create_test_project(&app, &token, "test-role-member-removal").await;

    // Get the existing member_role for Reporter (role_id=3)
    let member_role = member_roles::Entity::find()
        .filter(member_roles::Column::RoleId.eq(3)) // Reporter role
        .one(&db)
        .await
        .expect("Failed to find member_role")
        .expect("Should have a member_role for Reporter");

    // Add the custom role to this existing member
    let new_member_role = member_roles::ActiveModel {
        member_id: Set(member_role.member_id),
        role_id: Set(role_id),
        inherited_from: Set(None),
        ..Default::default()
    };
    new_member_role
        .insert(&db)
        .await
        .expect("Failed to insert member_role");

    // Try to delete the role - should fail
    let delete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/v1/roles/{}", role_id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        delete_response.status(),
        StatusCode::BAD_REQUEST,
        "Should not be able to delete role with members"
    );

    // Remove the member_role assignment for our custom role using SeaORM
    let delete_result: DeleteResult = member_roles::Entity::delete_many()
        .filter(member_roles::Column::RoleId.eq(role_id))
        .exec(&db)
        .await
        .expect("Failed to delete member_roles");

    assert!(
        delete_result.rows_affected > 0,
        "Should have deleted at least one member_role"
    );

    // Now try to delete the role again - should succeed
    let delete_response2 = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/v1/roles/{}", role_id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        delete_response2.status(),
        StatusCode::NO_CONTENT,
        "Should be able to delete role after member assignment is removed"
    );
}

/// Test: Can delete custom role (Manager, Developer have builtin=0 and are not in use)
#[tokio::test]
async fn can_delete_unused_manager_role() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Role ID 1 is "Manager" with builtin=0 (custom)
    // Role ID 2 is "Developer" with builtin=0 (custom)
    // These are custom roles (builtin=0) that can be deleted if not in use

    // Test deleting Developer role (ID 2) - less likely to be in use initially
    // Note: add_manager uses role_id=3 (Reporter), not 1 or 2
    let delete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/v1/roles/2")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Developer role is not assigned to any member in our test setup,
    // it should be deletable
    assert_eq!(
        delete_response.status(),
        StatusCode::NO_CONTENT,
        "Expected 204 for deleting unused Developer role"
    );
}
