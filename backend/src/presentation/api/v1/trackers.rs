use crate::application::dto::{CreateTrackerRequest, UpdateTrackerRequest};
use crate::application::use_cases::{
    CreateTrackerUseCase, DeleteTrackerUseCase, GetTrackerUseCase, ListTrackersUseCase,
    UpdateTrackerUseCase,
};
use crate::domain::repositories::{IssueRepository, IssueStatusRepository, TrackerRepository};
use crate::presentation::dto::{
    CreateTrackerJsonResponse, GetTrackerJsonResponse, TrackerListJsonResponse,
    UpdateTrackerJsonResponse,
};
use crate::presentation::errors::HttpError;
use crate::presentation::middleware::CurrentUser;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use std::sync::Arc;

/// List trackers endpoint handler
/// GET /api/v1/trackers.json
pub async fn list_trackers<T: TrackerRepository, S: IssueStatusRepository>(
    State(usecase): State<Arc<ListTrackersUseCase<T, S>>>,
    Extension(_current_user): Extension<CurrentUser>,
) -> Result<Json<TrackerListJsonResponse>, HttpError> {
    let response = usecase.execute().await?;
    Ok(Json(TrackerListJsonResponse::from(response)))
}

/// Get tracker endpoint handler
/// GET /api/v1/trackers/:id.json
pub async fn get_tracker<T: TrackerRepository, S: IssueStatusRepository>(
    State(usecase): State<Arc<GetTrackerUseCase<T, S>>>,
    Extension(_current_user): Extension<CurrentUser>,
    Path(id): Path<i32>,
) -> Result<Json<GetTrackerJsonResponse>, HttpError> {
    let response = usecase.execute(id).await?;
    Ok(Json(GetTrackerJsonResponse::from(response.tracker)))
}

/// Create tracker endpoint handler
/// POST /api/v1/trackers.json
pub async fn create_tracker<T: TrackerRepository, S: IssueStatusRepository>(
    State(usecase): State<Arc<CreateTrackerUseCase<T, S>>>,
    Extension(current_user): Extension<CurrentUser>,
    Json(request): Json<CreateTrackerRequest>,
) -> Result<(StatusCode, Json<CreateTrackerJsonResponse>), HttpError> {
    // Only admins can create trackers
    if !current_user.admin {
        return Err(HttpError::Forbidden(
            "Only administrators can create trackers".into(),
        ));
    }

    let response = usecase.execute(request.tracker.into()).await?;
    Ok((
        StatusCode::CREATED,
        Json(CreateTrackerJsonResponse::from(response)),
    ))
}

/// Update tracker endpoint handler
/// PUT /api/v1/trackers/:id.json
pub async fn update_tracker<T: TrackerRepository, S: IssueStatusRepository>(
    State(usecase): State<Arc<UpdateTrackerUseCase<T, S>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i32>,
    Json(request): Json<UpdateTrackerRequest>,
) -> Result<Json<UpdateTrackerJsonResponse>, HttpError> {
    // Only admins can update trackers
    if !current_user.admin {
        return Err(HttpError::Forbidden(
            "Only administrators can update trackers".into(),
        ));
    }

    let response = usecase.execute(id, request.tracker.into()).await?;
    Ok(Json(UpdateTrackerJsonResponse::from(response)))
}

/// Delete tracker endpoint handler
/// DELETE /api/v1/trackers/:id.json
pub async fn delete_tracker<T: TrackerRepository, I: IssueRepository>(
    State(usecase): State<Arc<DeleteTrackerUseCase<T, I>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i32>,
) -> Result<StatusCode, HttpError> {
    // Only admins can delete trackers
    if !current_user.admin {
        return Err(HttpError::Forbidden(
            "Only administrators can delete trackers".into(),
        ));
    }

    usecase.execute(id, &current_user).await?;
    Ok(StatusCode::NO_CONTENT)
}
