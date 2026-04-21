#![allow(clippy::type_complexity)]

use crate::application::use_cases::{
    DeleteAttachmentUseCase, DownloadAttachmentUseCase, GetAttachmentUseCase,
    UpdateAttachmentRequest, UpdateAttachmentUseCase,
};
use crate::domain::repositories::{
    AttachmentRepository, IssueRepository, MemberRepository, ProjectRepository, UserRepository,
};
use crate::presentation::dto::{AttachmentMetadataJsonResponse, UpdateAttachmentJsonResponse};
use crate::presentation::errors::HttpError;
use crate::presentation::middleware::CurrentUser;
use axum::{
    extract::{Path, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    Extension, Json,
};
use std::sync::Arc;
use validator::Validate;

/// GET /api/v1/attachments/:id.json
/// Get attachment metadata
pub async fn get_attachment_metadata<A, I, P, U>(
    State(usecase): State<Arc<GetAttachmentUseCase<A, I, P, U>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i32>,
) -> Result<Json<AttachmentMetadataJsonResponse>, HttpError>
where
    A: AttachmentRepository,
    I: IssueRepository,
    P: ProjectRepository,
    U: UserRepository,
{
    let response = usecase.execute(id, &current_user).await?;
    Ok(Json(AttachmentMetadataJsonResponse::from(response)))
}

/// GET /api/v1/attachments/download/:id
/// GET /api/v1/attachments/download/:id/:filename
/// Download attachment content
pub async fn download_attachment<A, I, P>(
    State(usecase): State<Arc<DownloadAttachmentUseCase<A, I, P>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i32>,
) -> Result<(HeaderMap, Vec<u8>), HttpError>
where
    A: AttachmentRepository,
    I: IssueRepository,
    P: ProjectRepository,
{
    let result = usecase.execute(id, &current_user).await?;

    // Build headers
    let mut headers = HeaderMap::new();

    // Content-Disposition: attachment; filename="..."
    let disposition = format!(
        "attachment; filename=\"{}\"",
        escape_filename(&result.filename)
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&disposition)
            .unwrap_or_else(|_| HeaderValue::from_static("attachment")),
    );

    // Content-Type
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(&result.content_type)
            .unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream")),
    );

    // Content-Length
    headers.insert(
        header::CONTENT_LENGTH,
        HeaderValue::from(result.content.len()),
    );

    Ok((headers, result.content))
}

/// GET /api/v1/attachments/download/:id/:filename
/// Download attachment with filename in URL (for friendly URLs)
pub async fn download_attachment_with_filename<A, I, P>(
    State(usecase): State<Arc<DownloadAttachmentUseCase<A, I, P>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path((id, _filename)): Path<(i32, String)>,
) -> Result<(HeaderMap, Vec<u8>), HttpError>
where
    A: AttachmentRepository,
    I: IssueRepository,
    P: ProjectRepository,
{
    // Use the same handler but extract only the ID
    download_attachment(State(usecase), Extension(current_user), Path(id)).await
}

/// Escape special characters in filename for Content-Disposition header
fn escape_filename(filename: &str) -> String {
    // Replace characters that are problematic in the Content-Disposition header
    filename
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

/// DELETE /api/v1/attachments/:id.json
/// Delete an attachment
pub async fn delete_attachment<A, I, P, M>(
    State(usecase): State<Arc<DeleteAttachmentUseCase<A, I, P, M>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i32>,
) -> Result<StatusCode, HttpError>
where
    A: AttachmentRepository,
    I: IssueRepository,
    P: ProjectRepository,
    M: MemberRepository,
{
    usecase.execute(id, &current_user).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// PATCH /api/v1/attachments/:id.json
/// Update attachment description
pub async fn update_attachment<A, I, P, M, U>(
    State(usecase): State<Arc<UpdateAttachmentUseCase<A, I, P, M, U>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i32>,
    Json(request): Json<UpdateAttachmentRequest>,
) -> Result<Json<UpdateAttachmentJsonResponse>, HttpError>
where
    A: AttachmentRepository,
    I: IssueRepository,
    P: ProjectRepository,
    M: MemberRepository,
    U: UserRepository,
{
    // Validate request
    request
        .attachment
        .validate()
        .map_err(|e| HttpError::BadRequest(e.to_string()))?;

    let response = usecase
        .execute(id, request.attachment, &current_user)
        .await?;
    Ok(Json(UpdateAttachmentJsonResponse::from(response)))
}
