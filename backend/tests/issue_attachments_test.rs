//! Integration tests for issue attachments endpoints
//!
//! Tests the following endpoints:
//! - GET /api/v1/issues/:id/attachments.json - List attachments for an issue
//! - POST /api/v1/issues/:id/attachments.json - Upload attachment to an issue

mod common;

use axum::{
    body::Body,
    http::{header, Request, StatusCode},
};
use common::{create_test_app, create_test_issue, create_test_project, login_as_admin};
use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;

#[tokio::test]
async fn test_list_issue_attachments_empty() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;
    let project_id = create_test_project(&app, &token, "test-attachments-1").await;
    let issue_id = create_test_issue(&app, &token, project_id, "Test Issue for Attachments").await;

    // List attachments for the issue (should be empty)
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/issues/{}/attachments.json", issue_id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json.get("attachments").is_some());
    let attachments = json.get("attachments").unwrap().as_array().unwrap();
    assert_eq!(attachments.len(), 0);
}

#[tokio::test]
async fn test_upload_and_list_issue_attachment() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;
    let project_id = create_test_project(&app, &token, "test-attachments-2").await;
    let issue_id = create_test_issue(&app, &token, project_id, "Test Issue for Upload").await;

    // Create multipart form data
    let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
    let file_content = b"Hello, this is a test file for attachment upload!";
    let filename = "test_document.txt";

    let multipart_body = format!(
        "--{}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"{}\"\r\nContent-Type: text/plain\r\n\r\n{}\r\n--{}--\r\n",
        boundary,
        filename,
        std::str::from_utf8(file_content).unwrap(),
        boundary
    );

    // Upload attachment
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

    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(
        status,
        StatusCode::CREATED,
        "Upload should succeed. Response: {:?}",
        json
    );

    // Verify response structure
    assert!(json.get("attachment").is_some());
    let attachment = json.get("attachment").unwrap();
    assert!(attachment.get("id").is_some());
    assert_eq!(
        attachment.get("filename").unwrap().as_str().unwrap(),
        filename
    );
    assert!(attachment.get("filesize").is_some());

    // List attachments and verify the uploaded file
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/issues/{}/attachments.json", issue_id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let attachments = json.get("attachments").unwrap().as_array().unwrap();
    assert_eq!(attachments.len(), 1);

    let attachment = &attachments[0];
    assert_eq!(
        attachment.get("filename").unwrap().as_str().unwrap(),
        filename
    );
    assert_eq!(
        attachment.get("filesize").unwrap().as_i64().unwrap() as usize,
        file_content.len()
    );
    assert!(attachment.get("content_type").is_some());
    assert!(attachment.get("author").is_some());
    assert!(attachment.get("created_on").is_some());
}

#[tokio::test]
async fn test_upload_attachment_with_description() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;
    let project_id = create_test_project(&app, &token, "test-attachments-3").await;
    let issue_id = create_test_issue(&app, &token, project_id, "Test Issue with Description").await;

    // Create multipart form data with description
    let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
    let file_content = b"Test file with description field";
    let filename = "document_with_desc.pdf";
    let description = "Important documentation file";

    let multipart_body = format!(
        "--{}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"{}\"\r\nContent-Type: application/pdf\r\n\r\n{}\r\n--{}\r\nContent-Disposition: form-data; name=\"attachment[description]\"\r\n\r\n{}\r\n--{}--\r\n",
        boundary,
        filename,
        std::str::from_utf8(file_content).unwrap(),
        boundary,
        description,
        boundary
    );

    // Upload attachment
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

    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(
        status,
        StatusCode::CREATED,
        "Upload with description should succeed. Response: {:?}",
        json
    );

    // Verify the attachment was uploaded
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/issues/{}/attachments.json", issue_id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let attachments = json.get("attachments").unwrap().as_array().unwrap();
    assert_eq!(attachments.len(), 1);

    let attachment = &attachments[0];
    assert_eq!(
        attachment.get("description").unwrap().as_str().unwrap(),
        description
    );
}

#[tokio::test]
async fn test_list_attachments_nonexistent_issue() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Try to list attachments for a non-existent issue
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/issues/99999/attachments.json")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_upload_attachment_nonexistent_issue() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;

    // Create multipart form data
    let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
    let multipart_body = format!(
        "--{}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"test.txt\"\r\n\r\ntest\r\n--{}--\r\n",
        boundary,
        boundary
    );

    // Try to upload attachment to a non-existent issue
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/issues/99999/attachments.json")
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

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_list_attachments_unauthorized() {
    let app = create_test_app().await;

    // Try to list attachments without authentication
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/issues/1/attachments.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_upload_attachment_unauthorized() {
    let app = create_test_app().await;

    // Create multipart form data
    let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
    let multipart_body = format!(
        "--{}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"test.txt\"\r\n\r\ntest\r\n--{}--\r\n",
        boundary,
        boundary
    );

    // Try to upload attachment without authentication
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/issues/1/attachments.json")
                .header(
                    header::CONTENT_TYPE,
                    format!("multipart/form-data; boundary={}", boundary),
                )
                .body(Body::from(multipart_body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_multiple_attachments() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;
    let project_id = create_test_project(&app, &token, "test-attachments-multi").await;
    let issue_id = create_test_issue(&app, &token, project_id, "Test Multiple Attachments").await;

    // Upload first attachment
    let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
    let multipart_body1 = format!(
        "--{}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"file1.txt\"\r\n\r\nContent 1\r\n--{}--\r\n",
        boundary,
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
                .body(Body::from(multipart_body1))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    // Upload second attachment
    let multipart_body2 = format!(
        "--{}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"file2.txt\"\r\n\r\nContent 2\r\n--{}--\r\n",
        boundary,
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
                .body(Body::from(multipart_body2))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    // List attachments and verify we have two
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/issues/{}/attachments.json", issue_id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let attachments = json.get("attachments").unwrap().as_array().unwrap();
    assert_eq!(attachments.len(), 2);

    // Verify filenames are correct
    let filenames: Vec<&str> = attachments
        .iter()
        .map(|a| a.get("filename").unwrap().as_str().unwrap())
        .collect();

    assert!(filenames.contains(&"file1.txt"));
    assert!(filenames.contains(&"file2.txt"));
}

#[tokio::test]
async fn test_get_issue_include_attachments_contains_content_and_thumbnail_urls() {
    let app = create_test_app().await;
    let token = login_as_admin(&app).await;
    let project_id = create_test_project(&app, &token, "test-issue-include-attachments").await;
    let issue_id = create_test_issue(&app, &token, project_id, "Issue Include Attachments").await;

    let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
    let multipart_body = format!(
        "--{}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"diagram.png\"\r\nContent-Type: image/png\r\n\r\nfake-image-content\r\n--{}--\r\n",
        boundary,
        boundary
    );

    let upload_response = app
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

    assert_eq!(upload_response.status(), StatusCode::CREATED);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/issues/{}?include=attachments", issue_id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    let attachments = json
        .get("issue")
        .and_then(|v| v.get("attachments"))
        .and_then(|v| v.as_array())
        .unwrap();
    assert_eq!(attachments.len(), 1);

    let attachment = &attachments[0];
    let content_url = attachment
        .get("content_url")
        .and_then(|v| v.as_str())
        .unwrap();
    let thumbnail_url = attachment
        .get("thumbnail_url")
        .and_then(|v| v.as_str())
        .unwrap();
    assert!(content_url.contains("/attachments/download/"));
    assert!(content_url.contains("/diagram.png"));
    assert!(thumbnail_url.contains("/attachments/thumbnail/"));
}
