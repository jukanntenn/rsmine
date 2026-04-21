use crate::application::use_cases::{UploadFileUseCase, UploadResponse};
use crate::presentation::errors::HttpError;
use crate::presentation::middleware::CurrentUser;
use axum::{
    extract::{Extension, Multipart, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;

/// Upload a file and get a token
/// POST /api/v1/uploads.json
pub async fn upload_file(
    State(usecase): State<Arc<UploadFileUseCase>>,
    Extension(current_user): Extension<CurrentUser>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<UploadResponse>), HttpError> {
    let mut filename: Option<String> = None;
    let mut content: Option<Vec<u8>> = None;
    let mut content_type: Option<String> = None;

    // Extract fields from multipart form
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| HttpError::BadRequest(format!("Failed to read multipart field: {}", e)))?
    {
        match field.name() {
            Some("file") => {
                // Read the file content
                let bytes = field.bytes().await.map_err(|e| {
                    HttpError::BadRequest(format!("Failed to read file content: {}", e))
                })?;
                content = Some(bytes.to_vec());
            }
            Some("filename") => {
                let text = field.text().await.map_err(|e| {
                    HttpError::BadRequest(format!("Failed to read filename: {}", e))
                })?;
                filename = Some(text);
            }
            Some("content_type") => {
                let text = field.text().await.map_err(|e| {
                    HttpError::BadRequest(format!("Failed to read content_type: {}", e))
                })?;
                content_type = Some(text);
            }
            _ => {
                // Unknown field, skip
            }
        }
    }

    // Validate required fields
    let filename =
        filename.ok_or_else(|| HttpError::BadRequest("filename is required".to_string()))?;

    let content = content.ok_or_else(|| HttpError::BadRequest("file is required".to_string()))?;

    // If content_type is not provided, try to detect from filename
    let content_type = content_type.or_else(|| detect_content_type(&filename));

    // Execute upload use case
    let response = usecase
        .execute(filename, content, content_type, &current_user)
        .await?;

    Ok((StatusCode::CREATED, Json(response)))
}

/// Detect content type from filename extension
fn detect_content_type(filename: &str) -> Option<String> {
    let ext = std::path::Path::new(filename)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase());

    match ext.as_deref() {
        Some("pdf") => Some("application/pdf".to_string()),
        Some("doc") => Some("application/msword".to_string()),
        Some("docx") => Some(
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document".to_string(),
        ),
        Some("xls") => Some("application/vnd.ms-excel".to_string()),
        Some("xlsx") => {
            Some("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet".to_string())
        }
        Some("ppt") => Some("application/vnd.ms-powerpoint".to_string()),
        Some("pptx") => Some(
            "application/vnd.openxmlformats-officedocument.presentationml.presentation".to_string(),
        ),
        Some("txt") => Some("text/plain".to_string()),
        Some("csv") => Some("text/csv".to_string()),
        Some("json") => Some("application/json".to_string()),
        Some("xml") => Some("application/xml".to_string()),
        Some("html") | Some("htm") => Some("text/html".to_string()),
        Some("css") => Some("text/css".to_string()),
        Some("js") => Some("application/javascript".to_string()),
        Some("png") => Some("image/png".to_string()),
        Some("jpg") | Some("jpeg") => Some("image/jpeg".to_string()),
        Some("gif") => Some("image/gif".to_string()),
        Some("svg") => Some("image/svg+xml".to_string()),
        Some("webp") => Some("image/webp".to_string()),
        Some("ico") => Some("image/x-icon".to_string()),
        Some("zip") => Some("application/zip".to_string()),
        Some("gz") | Some("gzip") => Some("application/gzip".to_string()),
        Some("tar") => Some("application/x-tar".to_string()),
        Some("mp3") => Some("audio/mpeg".to_string()),
        Some("mp4") => Some("video/mp4".to_string()),
        Some("avi") => Some("video/x-msvideo".to_string()),
        Some("mov") => Some("video/quicktime".to_string()),
        Some("wav") => Some("audio/wav".to_string()),
        _ => Some("application/octet-stream".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_content_type_pdf() {
        assert_eq!(
            detect_content_type("document.pdf"),
            Some("application/pdf".to_string())
        );
    }

    #[test]
    fn test_detect_content_type_png() {
        assert_eq!(
            detect_content_type("image.png"),
            Some("image/png".to_string())
        );
    }

    #[test]
    fn test_detect_content_type_jpg() {
        assert_eq!(
            detect_content_type("image.jpg"),
            Some("image/jpeg".to_string())
        );
        assert_eq!(
            detect_content_type("image.jpeg"),
            Some("image/jpeg".to_string())
        );
    }

    #[test]
    fn test_detect_content_type_unknown() {
        assert_eq!(
            detect_content_type("file.xyz"),
            Some("application/octet-stream".to_string())
        );
    }
}
