//! Integration tests for PATCH /api/v1/attachments/:id endpoint
//!
//! Tests the attachment description update endpoint

mod common;

use axum::{
    body::Body,
    http::{header, Request, StatusCode},
    Router,
};
use common::{create_test_app, create_test_issue, create_test_project, login_as_admin};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

/// Helper to upload an attachment to an issue and return the attachment ID
async fn upload_test_attachment(
    app: &Router,
    token: &str,
    issue_id: i32,
    filename: &str,
    content_type: &str,
    content: &[u8],
    description: Option<&str>,
) -> i32 {
    let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";

    let multipart_body = if let Some(desc) = description {
        format!(
            "--{}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"{}\"\r\nContent-Type: {}\r\n\r\n{}\r\n--{}\r\nContent-Disposition: form-data; name=\"attachment[description]\"\r\n\r\n{}\r\n--{}--\r\n",
            boundary,
            filename,
            content_type,
            std::str::from_utf8(content).unwrap(),
            boundary,
            desc,
            boundary
        )
    } else {
        format!(
            "--{}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"{}\"\r\nContent-Type: {}\r\n\r\n{}\r\n--{}--\r\n",
            boundary,
            filename,
            content_type,
            std::str::from_utf8(content).unwrap(),
            boundary
        )
    };

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/issues/{}/attachments.json", issue_id))
                .header("Authorization", format!("Bearer {}", token))
                .header(
                    header::CONTENT_TYPE,
                    format!("multipart/form-data; boundary={}", boundary),
                )
                .body(Body::from(multipart_body))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    json.get("attachment")
        .unwrap()
        .get("id")
        .unwrap()
        .as_i64()
        .unwrap() as i32
}

#[tokio::test]
async fn test_update_attachment_description_success() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;
    let project_id = create_test_project(&app, &token, "test-update-attachment").await;
    let issue_id =
        create_test_issue(&app, &token, project_id, "Test Issue for Update Attachment").await;

    // Upload an attachment first
    let file_content = b"Test file content for update";
    let attachment_id = upload_test_attachment(
        &app,
        &token,
        issue_id,
        "test_document.txt",
        "text/plain",
        file_content,
        Some("Original description"),
    )
    .await;

    // Update attachment description
    let new_description = "Updated description text";
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(format!("/api/v1/attachments/{}", attachment_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "attachment": {
                            "description": new_description
                        }
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    // Verify response structure
    assert!(json.get("attachment").is_some());
    let attachment = json.get("attachment").unwrap();

    assert_eq!(
        attachment.get("id").unwrap().as_i64().unwrap() as i32,
        attachment_id
    );
    assert_eq!(
        attachment.get("filename").unwrap().as_str().unwrap(),
        "test_document.txt"
    );
    assert_eq!(
        attachment.get("description").unwrap().as_str().unwrap(),
        new_description
    );
}

#[tokio::test]
async fn test_update_attachment_clear_description() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;
    let project_id = create_test_project(&app, &token, "test-clear-desc").await;
    let issue_id =
        create_test_issue(&app, &token, project_id, "Test Issue for Clear Description").await;

    // Upload an attachment with description
    let file_content = b"Test file content";
    let attachment_id = upload_test_attachment(
        &app,
        &token,
        issue_id,
        "test_doc.txt",
        "text/plain",
        file_content,
        Some("Will be cleared"),
    )
    .await;

    // Clear the description by sending null
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(format!("/api/v1/attachments/{}", attachment_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "attachment": {
                            "description": null
                        }
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let attachment = json.get("attachment").unwrap();
    // When description is None, it's either null or not present in the response
    // Both cases indicate the description was cleared
    let desc = attachment.get("description");
    assert!(
        desc.is_none() || desc.unwrap().is_null(),
        "Description should be null or absent, but got: {:?}",
        desc
    );
}

#[tokio::test]
async fn test_update_attachment_not_found() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Try to update non-existent attachment
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri("/api/v1/attachments/99999")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "attachment": {
                            "description": "New description"
                        }
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_update_attachment_unauthorized() {
    let app = create_test_app().await;

    // Try to update attachment without authentication
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri("/api/v1/attachments/1")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "attachment": {
                            "description": "New description"
                        }
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_update_attachment_description_too_long() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;
    let project_id = create_test_project(&app, &token, "test-desc-length").await;
    let issue_id = create_test_issue(&app, &token, project_id, "Test Description Length").await;

    // Upload an attachment
    let file_content = b"Test file content";
    let attachment_id = upload_test_attachment(
        &app,
        &token,
        issue_id,
        "test_doc.txt",
        "text/plain",
        file_content,
        None,
    )
    .await;

    // Try to update with description over 255 characters
    let long_description = "a".repeat(256);
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(format!("/api/v1/attachments/{}", attachment_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "attachment": {
                            "description": long_description
                        }
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_update_attachment_empty_description() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;
    let project_id = create_test_project(&app, &token, "test-empty-desc").await;
    let issue_id = create_test_issue(&app, &token, project_id, "Test Empty Description").await;

    // Upload an attachment with initial description
    let file_content = b"Test file content";
    let attachment_id = upload_test_attachment(
        &app,
        &token,
        issue_id,
        "test_doc.txt",
        "text/plain",
        file_content,
        Some("Original description"),
    )
    .await;

    // Update with empty string description
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(format!("/api/v1/attachments/{}", attachment_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "attachment": {
                            "description": ""
                        }
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let attachment = json.get("attachment").unwrap();
    assert_eq!(attachment.get("description").unwrap().as_str().unwrap(), "");
}

#[tokio::test]
async fn test_update_attachment_preserves_other_fields() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;
    let project_id = create_test_project(&app, &token, "test-preserve-fields").await;
    let issue_id = create_test_issue(&app, &token, project_id, "Test Preserve Fields").await;

    // Upload an attachment
    let file_content = b"Test file content for preservation check";
    let attachment_id = upload_test_attachment(
        &app,
        &token,
        issue_id,
        "preserve_test.txt",
        "text/plain",
        file_content,
        Some("Original"),
    )
    .await;

    // Update description
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(format!("/api/v1/attachments/{}", attachment_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "attachment": {
                            "description": "Modified description"
                        }
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let attachment = json.get("attachment").unwrap();

    // Verify that other fields are preserved
    assert_eq!(
        attachment.get("id").unwrap().as_i64().unwrap() as i32,
        attachment_id
    );
    assert_eq!(
        attachment.get("filename").unwrap().as_str().unwrap(),
        "preserve_test.txt"
    );
    assert_eq!(
        attachment.get("filesize").unwrap().as_i64().unwrap() as usize,
        file_content.len()
    );
    assert_eq!(
        attachment.get("content_type").unwrap().as_str().unwrap(),
        "text/plain"
    );
    assert!(attachment.get("content_url").is_some());
    assert!(attachment.get("author").is_some());
    assert!(attachment.get("created_on").is_some());
}
