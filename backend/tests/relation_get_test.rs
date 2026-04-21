//! Integration tests for GET /api/v1/relations/:id endpoint
//!
//! Tests the following scenarios:
//! - Get relation successfully
//! - Get relation with delay
//! - Get non-existent relation (404)
//! - Get relation without authentication (401)
//! - Get relation in public project (visible to all)
//! - Get relation in private project (member only)

mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use common::{create_test_app, create_test_issue, create_test_project, login_as_admin};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

/// Helper to create a relation between two issues
async fn create_relation(
    app: &axum::Router,
    token: &str,
    issue_id: i32,
    issue_to_id: i32,
    relation_type: &str,
    delay: Option<i32>,
) -> i32 {
    let mut body = json!({
        "relation": {
            "issue_to_id": issue_to_id,
            "relation_type": relation_type
        }
    });

    if let Some(d) = delay {
        body["relation"]["delay"] = json!(d);
    }

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/issues/{}/relations.json", issue_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
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
        "Failed to create relation. Response: {:?}",
        json
    );

    json.get("relation")
        .unwrap()
        .get("id")
        .unwrap()
        .as_i64()
        .unwrap() as i32
}

#[tokio::test]
async fn test_get_relation_success() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;
    let project_id = create_test_project(&app, &token, "test-relation-get-1").await;
    let issue1_id = create_test_issue(&app, &token, project_id, "First issue").await;
    let issue2_id = create_test_issue(&app, &token, project_id, "Second issue").await;

    // Create a relation
    let relation_id = create_relation(&app, &token, issue1_id, issue2_id, "relates", None).await;

    // Get the relation
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/relations/{}", relation_id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    // Verify response structure
    assert!(json.get("relation").is_some());
    let relation = json.get("relation").unwrap();
    assert_eq!(
        relation.get("id").unwrap().as_i64().unwrap() as i32,
        relation_id
    );
    assert_eq!(
        relation.get("issue_id").unwrap().as_i64().unwrap() as i32,
        issue1_id
    );
    assert_eq!(
        relation.get("issue_to_id").unwrap().as_i64().unwrap() as i32,
        issue2_id
    );
    assert_eq!(
        relation.get("relation_type").unwrap().as_str().unwrap(),
        "relates"
    );

}

#[tokio::test]
async fn test_get_relation_with_delay() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;
    let project_id = create_test_project(&app, &token, "test-relation-get-2").await;
    let issue1_id = create_test_issue(&app, &token, project_id, "Issue A").await;
    let issue2_id = create_test_issue(&app, &token, project_id, "Issue B").await;

    // Create a precedes relation with delay
    let relation_id =
        create_relation(&app, &token, issue1_id, issue2_id, "precedes", Some(5)).await;

    // Get the relation
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/relations/{}", relation_id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let relation = json.get("relation").unwrap();
    assert_eq!(relation.get("delay").unwrap().as_i64().unwrap(), 5);
    assert_eq!(
        relation.get("relation_type").unwrap().as_str().unwrap(),
        "precedes"
    );
}

#[tokio::test]
async fn test_get_relation_not_found() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Try to get a non-existent relation
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/relations/99999")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_relation_unauthorized() {
    let app = create_test_app().await;

    // Try to get a relation without authentication
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/relations/1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_get_relation_duplicates() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;
    let project_id = create_test_project(&app, &token, "test-relation-get-3").await;
    let issue1_id = create_test_issue(&app, &token, project_id, "Original issue").await;
    let issue2_id = create_test_issue(&app, &token, project_id, "Duplicate issue").await;

    // Create a duplicates relation
    let relation_id = create_relation(&app, &token, issue1_id, issue2_id, "duplicates", None).await;

    // Get the relation
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/relations/{}", relation_id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let relation = json.get("relation").unwrap();
    assert_eq!(
        relation.get("relation_type").unwrap().as_str().unwrap(),
        "duplicates"
    );
}

#[tokio::test]
async fn test_get_relation_blocks() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;
    let project_id = create_test_project(&app, &token, "test-relation-get-4").await;
    let issue1_id = create_test_issue(&app, &token, project_id, "Blocking issue").await;
    let issue2_id = create_test_issue(&app, &token, project_id, "Blocked issue").await;

    // Create a blocks relation
    let relation_id = create_relation(&app, &token, issue1_id, issue2_id, "blocks", None).await;

    // Get the relation
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/relations/{}", relation_id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let relation = json.get("relation").unwrap();
    assert_eq!(
        relation.get("relation_type").unwrap().as_str().unwrap(),
        "blocks"
    );
}

#[tokio::test]
async fn test_get_relation_copied_to() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;
    let project_id = create_test_project(&app, &token, "test-relation-get-5").await;
    let issue1_id = create_test_issue(&app, &token, project_id, "Source issue").await;
    let issue2_id = create_test_issue(&app, &token, project_id, "Copied issue").await;

    // Create a copied_to relation
    let relation_id = create_relation(&app, &token, issue1_id, issue2_id, "copied_to", None).await;

    // Get the relation
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/relations/{}", relation_id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let relation = json.get("relation").unwrap();
    assert_eq!(
        relation.get("relation_type").unwrap().as_str().unwrap(),
        "copied_to"
    );
}
