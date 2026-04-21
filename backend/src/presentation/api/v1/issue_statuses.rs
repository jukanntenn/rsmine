use crate::application::dto::{
    CreateIssueStatusRequest, DeleteIssueStatusRequest, UpdateIssueStatusRequest,
};
use crate::application::use_cases::{
    CreateIssueStatusUseCase, DeleteIssueStatusUseCase, GetIssueStatusUseCase,
    ListIssueStatusesUseCase, UpdateIssueStatusUseCase,
};
use crate::domain::repositories::IssueStatusRepository;
use crate::presentation::dto::{
    CreateIssueStatusJsonResponse, GetIssueStatusJsonResponse, IssueStatusListJsonResponse,
    UpdateIssueStatusJsonResponse,
};
use crate::presentation::errors::HttpError;
use crate::presentation::middleware::CurrentUser;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use std::sync::Arc;

/// List issue statuses endpoint handler
/// GET /api/v1/issue_statuses.json
pub async fn list_issue_statuses<T: IssueStatusRepository>(
    State(usecase): State<Arc<ListIssueStatusesUseCase<T>>>,
    Extension(_current_user): Extension<CurrentUser>,
) -> Result<Json<IssueStatusListJsonResponse>, HttpError> {
    let response = usecase.execute().await?;
    Ok(Json(IssueStatusListJsonResponse::from(response)))
}

/// Get issue status endpoint handler
/// GET /api/v1/issue_statuses/:id.json
pub async fn get_issue_status<T: IssueStatusRepository>(
    State(usecase): State<Arc<GetIssueStatusUseCase<T>>>,
    Extension(_current_user): Extension<CurrentUser>,
    Path(id): Path<i32>,
) -> Result<Json<GetIssueStatusJsonResponse>, HttpError> {
    let response = usecase.execute(id).await?;
    Ok(Json(GetIssueStatusJsonResponse::from(response)))
}

/// Create issue status endpoint handler
/// POST /api/v1/issue_statuses.json
pub async fn create_issue_status<T: IssueStatusRepository>(
    State(usecase): State<Arc<CreateIssueStatusUseCase<T>>>,
    Extension(current_user): Extension<CurrentUser>,
    Json(request): Json<CreateIssueStatusRequest>,
) -> Result<(StatusCode, Json<CreateIssueStatusJsonResponse>), HttpError> {
    // Only admins can create issue statuses
    if !current_user.admin {
        return Err(HttpError::Forbidden(
            "Only administrators can create issue statuses".into(),
        ));
    }

    let response = usecase.execute(request.issue_status.into()).await?;
    Ok((
        StatusCode::CREATED,
        Json(CreateIssueStatusJsonResponse::from(response)),
    ))
}

/// Update issue status endpoint handler
/// PUT /api/v1/issue_statuses/:id.json
pub async fn update_issue_status<T: IssueStatusRepository>(
    State(usecase): State<Arc<UpdateIssueStatusUseCase<T>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i32>,
    Json(request): Json<UpdateIssueStatusRequest>,
) -> Result<Json<UpdateIssueStatusJsonResponse>, HttpError> {
    // Only admins can update issue statuses
    if !current_user.admin {
        return Err(HttpError::Forbidden(
            "Only administrators can update issue statuses".into(),
        ));
    }

    let update_request = crate::application::use_cases::UpdateIssueStatusRequest {
        name: request.issue_status.name,
        is_closed: request.issue_status.is_closed,
        is_default: request.issue_status.is_default,
        default_done_ratio: request.issue_status.default_done_ratio,
    };

    let response = usecase.execute(id, update_request).await?;
    Ok(Json(UpdateIssueStatusJsonResponse::from(response)))
}

/// Delete issue status endpoint handler
/// DELETE /api/v1/issue_statuses/:id.json
pub async fn delete_issue_status<T: IssueStatusRepository>(
    State(usecase): State<Arc<DeleteIssueStatusUseCase<T>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i32>,
    request: Option<Json<DeleteIssueStatusRequest>>,
) -> Result<StatusCode, HttpError> {
    // Only admins can delete issue statuses
    if !current_user.admin {
        return Err(HttpError::Forbidden(
            "Only administrators can delete issue statuses".into(),
        ));
    }

    let reassign_to_id = request.and_then(|r| r.reassign_to_id);

    usecase.execute(id, reassign_to_id, &current_user).await?;
    Ok(StatusCode::NO_CONTENT)
}
