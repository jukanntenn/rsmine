mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use common::{create_test_app, create_test_project, login_as_admin};
use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;

#[tokio::test]
async fn test_get_membership_success() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;
    let project_id = create_test_project(&app, &token, "test-member-get-1").await;

    let list_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/projects/{project_id}/memberships"))
                .header("Authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(list_response.status(), StatusCode::OK);
    let body = list_response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    let list_json: Value = serde_json::from_slice(&body).unwrap();
    let membership_id = list_json
        .get("memberships")
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.first())
        .and_then(|v| v.get("id"))
        .and_then(|v| v.as_i64())
        .unwrap() as i32;

    let get_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/memberships/{membership_id}"))
                .header("Authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(get_response.status(), StatusCode::OK);
    let body = get_response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    let membership = json.get("membership").unwrap();
    assert_eq!(
        membership.get("id").unwrap().as_i64().unwrap() as i32,
        membership_id
    );
    assert_eq!(
        membership
            .get("project")
            .unwrap()
            .get("id")
            .unwrap()
            .as_i64()
            .unwrap() as i32,
        project_id
    );
    assert!(membership.get("user").is_some());
    assert!(membership.get("roles").unwrap().as_array().is_some());
}

#[tokio::test]
async fn test_get_membership_not_found() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/memberships/99999")
                .header("Authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_membership_unauthorized() {
    let app = create_test_app().await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/memberships/1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
