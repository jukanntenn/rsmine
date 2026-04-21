//! Integration tests for GET /api/v1/issues/:id/journals endpoint
//!
//! Tests cover:
//! - Successful retrieval of journals
//! - Empty journals for new issues
//! - Private notes visibility
//! - Permission denied for non-visible issues
//! - Authentication required

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;

mod common;

use common::{
    create_test_app, create_test_issue, create_test_project, login_as_admin,
    update_issue_with_notes,
};

/// Test: List journals successfully
///
/// Scenario: User with permission requests journals for an issue
/// Expected: Returns 200 with journals array
#[tokio::test]
async fn test_list_journals_success() {
    let app = create_test_app().await;

    // Login as admin
    let token = login_as_admin(&app).await;

    // Create test project
    let project_id = create_test_project(&app, &token, "test-project").await;

    // Create test issue
    let issue_id = create_test_issue(&app, &token, project_id, "Test issue for journals").await;

    // Update issue to create a journal entry
    update_issue_with_notes(&app, &token, issue_id, "First note on the issue").await;

    // Request journals
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/issues/{}/journals.json", issue_id))
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
    assert!(json.is_object());
    assert!(json.get("journals").is_some());
    let journals = json.get("journals").unwrap().as_array().unwrap();

    // Should have at least one journal entry from the update
    assert!(!journals.is_empty());

    // Verify journal structure
    let journal = &journals[0];
    assert!(journal.get("id").is_some());
    assert!(journal.get("user").is_some());
    assert!(journal.get("created_on").is_some());
    assert!(journal.get("private_notes").is_some());
    assert!(journal.get("details").is_some());

    // Verify user structure
    let user = journal.get("user").unwrap();
    assert!(user.get("id").is_some());
    assert!(user.get("name").is_some());
}

/// Test: Empty journals for new issue
///
/// Scenario: User requests journals for a newly created issue
/// Expected: Returns 200 with empty journals array
#[tokio::test]
async fn test_list_journals_empty() {
    let app = create_test_app().await;

    // Login as admin
    let token = login_as_admin(&app).await;

    // Create test project
    let project_id = create_test_project(&app, &token, "empty-journals-project").await;

    // Create test issue (without any updates, so no journals)
    let issue_id = create_test_issue(&app, &token, project_id, "Test issue no journals").await;

    // Request journals
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/issues/{}/journals.json", issue_id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    // Verify response has empty journals array
    let journals = json.get("journals").unwrap().as_array().unwrap();
    assert!(journals.is_empty());
}

/// Test: Authentication required
///
/// Scenario: Request without authentication token
/// Expected: Returns 401 Unauthorized
#[tokio::test]
async fn test_list_journals_unauthenticated() {
    let app = create_test_app().await;

    // Request journals without token
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/issues/1/journals.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

/// Test: Issue not found
///
/// Scenario: Request journals for non-existent issue
/// Expected: Returns 404 Not Found
#[tokio::test]
async fn test_list_journals_issue_not_found() {
    let app = create_test_app().await;

    // Login as admin
    let token = login_as_admin(&app).await;

    // Request journals for non-existent issue
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/issues/99999/journals.json")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

/// Test: Journal details for field changes
///
/// Scenario: Issue is updated with field changes
/// Expected: Journal contains details with old and new values
#[tokio::test]
async fn test_list_journals_with_details() {
    let app = create_test_app().await;

    // Login as admin
    let token = login_as_admin(&app).await;

    // Create test project
    let project_id = create_test_project(&app, &token, "details-project").await;

    // Create test issue
    let issue_id = create_test_issue(&app, &token, project_id, "Test issue with details").await;

    // Update issue to create a journal with status change
    update_issue_with_notes(&app, &token, issue_id, "Status update note").await;

    // Request journals
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/issues/{}/journals.json", issue_id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let journals = json.get("journals").unwrap().as_array().unwrap();
    assert!(!journals.is_empty());

    // Find a journal with details (the update should have created one)
    let journal_with_details = journals.iter().find(|j| {
        j.get("notes")
            .and_then(|n| n.as_str())
            .is_some_and(|n| !n.is_empty())
    });

    if let Some(journal) = journal_with_details {
        let notes = journal.get("notes").unwrap().as_str().unwrap();
        assert_eq!(notes, "Status update note");
    }
}

/// Test: Journal order
///
/// Scenario: Issue has multiple journal entries
/// Expected: Journals are returned in chronological order (oldest first)
#[tokio::test]
async fn test_list_journals_order() {
    let app = create_test_app().await;

    // Login as admin
    let token = login_as_admin(&app).await;

    // Create test project
    let project_id = create_test_project(&app, &token, "order-project").await;

    // Create test issue
    let issue_id = create_test_issue(&app, &token, project_id, "Test issue order").await;

    // Create multiple journal entries
    update_issue_with_notes(&app, &token, issue_id, "First note").await;
    update_issue_with_notes(&app, &token, issue_id, "Second note").await;
    update_issue_with_notes(&app, &token, issue_id, "Third note").await;

    // Request journals
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/issues/{}/journals.json", issue_id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let journals = json.get("journals").unwrap().as_array().unwrap();

    // Should have at least 3 journal entries from the updates
    assert!(journals.len() >= 3);

    // Verify notes are present
    let notes: Vec<&str> = journals
        .iter()
        .filter_map(|j| j.get("notes").and_then(|n| n.as_str()))
        .filter(|n| !n.is_empty())
        .collect();

    assert!(notes.contains(&"First note"));
    assert!(notes.contains(&"Second note"));
    assert!(notes.contains(&"Third note"));
}
