//! Integration tests for GET /api/v1/attachments/:id endpoint
//!
//! Tests the attachment metadata retrieval endpoint

mod common;

use axum::{
    body::Body,
    http::{header, Request, StatusCode},
    Router,
};
use common::{create_test_app, create_test_issue, create_test_project, login_as_admin};
use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;

/// Helper to upload an attachment to an issue and return the attachment ID
async fn upload_test_attachment(
    app: &Router,
    token: &str,
    issue_id: i32,
    filename: &str,
    content_type: &str,
    content: &[u8],
) -> i32 {
    let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
    let multipart_body = format!(
        "--{}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"{}\"\r\nContent-Type: {}\r\n\r\n{}\r\n--{}--\r\n",
        boundary,
        filename,
        content_type,
        std::str::from_utf8(content).unwrap(),
        boundary
    );

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
async fn test_get_attachment_metadata_success() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;
    let project_id = create_test_project(&app, &token, "test-get-attachment").await;
    let issue_id =
        create_test_issue(&app, &token, project_id, "Test Issue for Get Attachment").await;

    // Upload an attachment first
    let file_content = b"Test file content for metadata retrieval";
    let attachment_id = upload_test_attachment(
        &app,
        &token,
        issue_id,
        "test_document.txt",
        "text/plain",
        file_content,
    )
    .await;

    // Get attachment metadata
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/attachments/{}", attachment_id))
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
        attachment.get("filesize").unwrap().as_i64().unwrap() as usize,
        file_content.len()
    );
    assert!(attachment.get("content_type").is_some());
    assert!(attachment.get("content_url").is_some());
    assert!(attachment.get("author").is_some());
    assert!(attachment.get("created_on").is_some());

    // Verify author structure
    let author = attachment.get("author").unwrap();
    assert!(author.get("id").is_some());
    assert!(author.get("name").is_some());
}

#[tokio::test]
async fn test_get_attachment_metadata_with_image() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;
    let project_id = create_test_project(&app, &token, "test-get-attachment-img").await;
    let issue_id =
        create_test_issue(&app, &token, project_id, "Test Issue for Image Attachment").await;

    // Upload an image attachment
    let file_content = b"fake png content for testing";
    let attachment_id = upload_test_attachment(
        &app,
        &token,
        issue_id,
        "test_image.png",
        "image/png",
        file_content,
    )
    .await;

    // Get attachment metadata
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/attachments/{}", attachment_id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let attachment = json.get("attachment").unwrap();

    // Image should have thumbnail_url
    assert!(attachment.get("thumbnail_url").is_some());

    // Verify content_url is correct
    let content_url = attachment.get("content_url").unwrap().as_str().unwrap();
    assert!(content_url.contains(&format!("/attachments/download/{}", attachment_id)));
}

#[tokio::test]
async fn test_get_attachment_metadata_not_found() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Try to get non-existent attachment
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/attachments/99999")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_attachment_metadata_unauthorized() {
    let app = create_test_app().await;

    // Try to get attachment without authentication
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/attachments/1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_get_attachment_metadata_content_url_format() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;
    let project_id = create_test_project(&app, &token, "test-content-url").await;
    let issue_id = create_test_issue(&app, &token, project_id, "Test Content URL Format").await;

    // Upload attachment with special characters in filename
    let file_content = b"test content";
    let attachment_id = upload_test_attachment(
        &app,
        &token,
        issue_id,
        "my document.pdf",
        "application/pdf",
        file_content,
    )
    .await;

    // Get attachment metadata
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/attachments/{}", attachment_id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let attachment = json.get("attachment").unwrap();
    let content_url = attachment.get("content_url").unwrap().as_str().unwrap();

    // Verify content_url format includes attachment ID and encoded filename
    assert!(content_url.starts_with("http://localhost:3000/attachments/download/"));
    assert!(content_url.contains(&attachment_id.to_string()));
}

#[tokio::test]
async fn test_get_attachment_with_description() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;
    let project_id = create_test_project(&app, &token, "test-attachment-desc").await;
    let issue_id = create_test_issue(&app, &token, project_id, "Test Attachment Description").await;

    // Upload attachment with description
    let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
    let description = "Important test document";
    let multipart_body = format!(
        "--{}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"doc.txt\"\r\nContent-Type: text/plain\r\n\r\ntest content\r\n--{}\r\nContent-Disposition: form-data; name=\"attachment[description]\"\r\n\r\n{}\r\n--{}--\r\n",
        boundary,
        boundary,
        description,
        boundary
    );

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
    let attachment_id = json
        .get("attachment")
        .unwrap()
        .get("id")
        .unwrap()
        .as_i64()
        .unwrap() as i32;

    // Get attachment metadata
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/attachments/{}", attachment_id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let attachment = json.get("attachment").unwrap();
    assert_eq!(
        attachment.get("description").unwrap().as_str().unwrap(),
        description
    );
}
