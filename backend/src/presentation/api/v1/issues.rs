#![allow(clippy::type_complexity)]

use crate::application::dto::{CreateIssueRequest, UpdateIssueRequest};
use crate::application::use_cases::{
    CreateIssueUseCase, DeleteIssueUseCase, GetIssueUseCase, IncludeOption,
    ListIssueAttachmentsUseCase, ListIssuesUseCase, ListJournalsUseCase, UpdateIssueUseCase,
    UploadIssueAttachmentUseCase,
};
use crate::domain::repositories::{
    AttachmentRepository, EnumerationRepository, IssueQueryParams, IssueRelationRepository,
    IssueRepository, IssueStatusRepository, JournalRepository, MemberRepository, ProjectRepository,
    TrackerRepository, UserRepository,
};
use crate::presentation::dto::{
    CreateIssueJsonResponse, GetIssueJsonResponse, IssueAttachmentListJsonResponse,
    IssueListJsonResponse, ListJournalsJsonResponse, UpdateIssueJsonResponse,
};
use crate::presentation::errors::HttpError;
use crate::presentation::middleware::CurrentUser;
use axum::{
    extract::{Extension, Multipart, Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use std::sync::Arc;

/// Query parameters for the issue list endpoint
#[derive(Debug, Deserialize)]
pub struct IssueListQuery {
    /// Filter by project ID
    pub project_id: Option<i32>,

    /// Filter by status ID or "open"/"closed"
    pub status_id: Option<String>,

    /// Filter by tracker ID
    pub tracker_id: Option<i32>,

    /// Filter by priority ID
    pub priority_id: Option<i32>,

    /// Filter by category ID
    pub category_id: Option<i32>,

    /// Filter by assignee ID or "me"
    pub assigned_to_id: Option<String>,

    /// Filter by author ID
    pub author_id: Option<i32>,

    /// Filter by subject (fuzzy search)
    pub subject: Option<String>,

    /// Filter by parent issue ID
    pub parent_id: Option<i32>,

    /// Filter by created_on (>=YYYY-MM-DD)
    pub created_on: Option<String>,

    /// Filter by updated_on (>=YYYY-MM-DD)
    pub updated_on: Option<String>,

    /// Offset for pagination (default: 0)
    #[serde(default)]
    pub offset: Option<u32>,

    /// Limit for pagination (default: 25, max: 100)
    #[serde(default = "default_limit")]
    pub limit: Option<u32>,

    /// Sort field and direction (e.g., "created_on:desc")
    pub sort: Option<String>,
}

fn default_limit() -> Option<u32> {
    Some(25)
}

/// List issues endpoint handler
/// GET /api/v1/issues.json
pub async fn list_issues<I, P, U, T, S, E>(
    State(usecase): State<Arc<ListIssuesUseCase<I, P, U, T, S, E>>>,
    Extension(current_user): Extension<CurrentUser>,
    Query(query): Query<IssueListQuery>,
) -> Result<Json<IssueListJsonResponse>, HttpError>
where
    I: IssueRepository,
    P: ProjectRepository,
    U: UserRepository,
    T: TrackerRepository,
    S: IssueStatusRepository,
    E: EnumerationRepository,
{
    let params = IssueQueryParams::new(
        query.project_id,
        query.status_id,
        query.tracker_id,
        query.priority_id,
        query.category_id,
        query.assigned_to_id,
        query.author_id,
        query.subject,
        query.parent_id,
        query.created_on,
        query.updated_on,
        query.offset.unwrap_or(0),
        query.limit.unwrap_or(25),
        query.sort,
    );

    let response = usecase.execute(params, &current_user).await?;

    Ok(Json(IssueListJsonResponse::from(response)))
}

/// Query parameters for the get issue endpoint
#[derive(Debug, Deserialize)]
pub struct GetIssueQuery {
    /// Comma-separated list of includes: attachments, journals, relations, children, watchers, allowed_statuses
    pub include: Option<String>,
}

/// Get single issue endpoint handler
/// GET /api/v1/issues/:id.json
pub async fn get_issue<I, P, U, T, S, E, J, A, R, M>(
    State(usecase): State<Arc<GetIssueUseCase<I, P, U, T, S, E, J, A, R, M>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i32>,
    Query(query): Query<GetIssueQuery>,
) -> Result<Json<GetIssueJsonResponse>, HttpError>
where
    I: IssueRepository,
    P: ProjectRepository,
    U: UserRepository,
    T: TrackerRepository,
    S: IssueStatusRepository,
    E: EnumerationRepository,
    J: JournalRepository,
    A: AttachmentRepository,
    R: IssueRelationRepository,
    M: MemberRepository,
{
    let include = IncludeOption::parse(query.include.as_deref());
    let response = usecase.execute(id, include, &current_user).await?;

    Ok(Json(GetIssueJsonResponse::from(response)))
}

/// Create issue endpoint handler
/// POST /api/v1/issues.json
pub async fn create_issue<I, P, U, T, S, E, M>(
    State(usecase): State<Arc<CreateIssueUseCase<I, P, U, T, S, E, M>>>,
    Extension(current_user): Extension<CurrentUser>,
    Json(request): Json<CreateIssueRequest>,
) -> Result<(StatusCode, Json<CreateIssueJsonResponse>), HttpError>
where
    I: IssueRepository,
    P: ProjectRepository,
    U: UserRepository,
    T: TrackerRepository,
    S: IssueStatusRepository,
    E: EnumerationRepository,
    M: MemberRepository,
{
    let response = usecase.execute(request.issue, &current_user).await?;

    Ok((
        StatusCode::CREATED,
        Json(CreateIssueJsonResponse::from(response)),
    ))
}

/// Update issue endpoint handler
/// PUT /api/v1/issues/:id.json
pub async fn update_issue<I, P, U, T, S, E, J, M>(
    State(usecase): State<Arc<UpdateIssueUseCase<I, P, U, T, S, E, J, M>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i32>,
    Json(request): Json<UpdateIssueRequest>,
) -> Result<Json<UpdateIssueJsonResponse>, HttpError>
where
    I: IssueRepository,
    P: ProjectRepository,
    U: UserRepository,
    T: TrackerRepository,
    S: IssueStatusRepository,
    E: EnumerationRepository,
    J: JournalRepository,
    M: MemberRepository,
{
    let response = usecase.execute(id, request.issue, &current_user).await?;

    Ok(Json(UpdateIssueJsonResponse::from(response)))
}

/// Delete issue endpoint handler
/// DELETE /api/v1/issues/:id.json
pub async fn delete_issue<I, A, J, R, M>(
    State(usecase): State<Arc<DeleteIssueUseCase<I, A, J, R, M>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i32>,
) -> Result<StatusCode, HttpError>
where
    I: IssueRepository,
    A: AttachmentRepository,
    J: JournalRepository,
    R: IssueRelationRepository,
    M: MemberRepository,
{
    usecase.execute(id, &current_user).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// List journals (change history) endpoint handler
/// GET /api/v1/issues/:id/journals.json
pub async fn list_journals<I, P, U, J>(
    State(usecase): State<Arc<ListJournalsUseCase<I, P, U, J>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i32>,
) -> Result<Json<ListJournalsJsonResponse>, HttpError>
where
    I: IssueRepository,
    P: ProjectRepository,
    U: UserRepository,
    J: JournalRepository,
{
    let response = usecase.execute(id, &current_user).await?;

    Ok(Json(ListJournalsJsonResponse::from(response)))
}

/// List attachments for an issue endpoint handler
/// GET /api/v1/issues/:id/attachments.json
pub async fn list_issue_attachments<A, I, P, U>(
    State(usecase): State<Arc<ListIssueAttachmentsUseCase<A, I, P, U>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i32>,
) -> Result<Json<IssueAttachmentListJsonResponse>, HttpError>
where
    A: AttachmentRepository,
    I: IssueRepository,
    P: ProjectRepository,
    U: UserRepository,
{
    let response = usecase.execute(id, &current_user).await?;

    Ok(Json(IssueAttachmentListJsonResponse::from(response)))
}

/// Upload attachment to an issue endpoint handler
/// POST /api/v1/issues/:id/attachments.json
pub async fn upload_issue_attachment<A, I, P, U, M>(
    State(usecase): State<Arc<UploadIssueAttachmentUseCase<A, I, P, U, M>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i32>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<UploadedAttachmentJsonResponse>), HttpError>
where
    A: AttachmentRepository,
    I: IssueRepository,
    P: ProjectRepository,
    U: UserRepository,
    M: MemberRepository,
{
    let mut filename: Option<String> = None;
    let mut content: Option<Vec<u8>> = None;
    let content_type: Option<String> = None;
    let mut description: Option<String> = None;

    // Extract fields from multipart form
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| HttpError::BadRequest(format!("Failed to read multipart field: {}", e)))?
    {
        match field.name() {
            Some("file") => {
                // Get filename from content-disposition if available
                if filename.is_none() {
                    filename = field.file_name().map(|s| s.to_string());
                }
                // Read the file content
                let bytes = field.bytes().await.map_err(|e| {
                    HttpError::BadRequest(format!("Failed to read file content: {}", e))
                })?;
                content = Some(bytes.to_vec());
            }
            Some("attachment[description]") => {
                let text = field.text().await.map_err(|e| {
                    HttpError::BadRequest(format!("Failed to read description: {}", e))
                })?;
                description = if text.is_empty() { None } else { Some(text) };
            }
            _ => {
                // Unknown field, skip
            }
        }
    }

    // Validate required fields
    let filename = filename.ok_or_else(|| HttpError::BadRequest("file is required".to_string()))?;

    let content = content.ok_or_else(|| HttpError::BadRequest("file is required".to_string()))?;

    // Detect content type from filename if not provided
    let content_type = content_type.or_else(|| detect_content_type(&filename));

    // Execute upload use case
    let response = usecase
        .execute(
            id,
            filename,
            content,
            content_type,
            description,
            &current_user,
        )
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(UploadedAttachmentJsonResponse::from(response)),
    ))
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

/// JSON response wrapper for uploaded attachment
#[derive(Debug, serde::Serialize)]
pub struct UploadedAttachmentJsonResponse {
    pub attachment: UploadedAttachmentItemJsonResponse,
}

/// JSON item for uploaded attachment response
#[derive(Debug, serde::Serialize)]
pub struct UploadedAttachmentItemJsonResponse {
    pub id: i32,
    pub filename: String,
    pub filesize: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

impl From<crate::application::use_cases::UploadIssueAttachmentResponse>
    for UploadedAttachmentJsonResponse
{
    fn from(response: crate::application::use_cases::UploadIssueAttachmentResponse) -> Self {
        Self {
            attachment: UploadedAttachmentItemJsonResponse {
                id: response.attachment.id,
                filename: response.attachment.filename,
                filesize: response.attachment.filesize,
                content_type: response.attachment.content_type,
            },
        }
    }
}
