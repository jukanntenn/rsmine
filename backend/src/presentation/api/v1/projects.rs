#![allow(clippy::type_complexity)]

use crate::application::dto::{CreateProjectRequest, UpdateProjectRequest};
use crate::application::use_cases::{
    CreateProjectUseCase, DeleteProjectUseCase, GetProjectTrackersUseCase, GetProjectUseCase,
    ListProjectsUseCase, UpdateProjectUseCase,
};
use crate::domain::repositories::{
    AttachmentRepository, IssueCategoryRepository, IssueRelationRepository, IssueRepository,
    IssueStatusRepository, JournalRepository, MemberRepository, ProjectQueryParams,
    ProjectRepository, TrackerRepository, UserRepository,
};
use crate::presentation::dto::{
    CreateProjectJsonResponse, GetProjectJsonResponse, ProjectListJsonResponse,
    TrackerListJsonResponse, UpdateProjectJsonResponse,
};
use crate::presentation::errors::HttpError;
use crate::presentation::middleware::CurrentUser;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension, Json,
};
use serde::Deserialize;
use std::sync::Arc;

/// Query parameters for the project list endpoint
#[derive(Debug, Deserialize)]
pub struct ProjectListQuery {
    /// Offset for pagination (default: 0)
    #[serde(default)]
    pub offset: Option<u32>,

    /// Limit for pagination (default: 25, max: 100)
    #[serde(default = "default_limit")]
    pub limit: Option<u32>,

    /// Filter by status (1=active, 5=closed, 9=archived)
    pub status: Option<i32>,

    /// Filter by name (fuzzy search on name or identifier)
    pub name: Option<String>,
}

fn default_limit() -> Option<u32> {
    Some(25)
}

/// List projects endpoint handler
/// GET /api/v1/projects.json
pub async fn list_projects<P: ProjectRepository>(
    State(usecase): State<Arc<ListProjectsUseCase<P>>>,
    Extension(current_user): Extension<CurrentUser>,
    Query(query): Query<ProjectListQuery>,
) -> Result<Json<ProjectListJsonResponse>, HttpError> {
    let params = ProjectQueryParams::new(
        query.status,
        query.name,
        query.offset.unwrap_or(0),
        query.limit.unwrap_or(25),
    );

    let response = usecase.execute(params, &current_user).await?;

    Ok(Json(ProjectListJsonResponse::from(response)))
}

/// Get project endpoint handler
/// GET /api/v1/projects/:id.json or GET /api/v1/projects/:identifier.json
pub async fn get_project<P: ProjectRepository, M: MemberRepository, U: UserRepository>(
    State(usecase): State<Arc<GetProjectUseCase<P, M, U>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<String>,
) -> Result<Json<GetProjectJsonResponse>, HttpError> {
    let response = usecase.execute(&id, &current_user).await?;
    Ok(Json(GetProjectJsonResponse::from(response)))
}

/// Create project endpoint handler
/// POST /api/v1/projects.json
pub async fn create_project<P: ProjectRepository, M: MemberRepository, T: TrackerRepository>(
    State(usecase): State<Arc<CreateProjectUseCase<P, M, T>>>,
    Extension(current_user): Extension<CurrentUser>,
    Json(req): Json<CreateProjectRequest>,
) -> Result<(StatusCode, Json<CreateProjectJsonResponse>), HttpError> {
    let response = usecase.execute(req.project, &current_user).await?;
    Ok((
        StatusCode::CREATED,
        Json(CreateProjectJsonResponse::from(response)),
    ))
}

/// Update project endpoint handler
/// PUT /api/v1/projects/:id
pub async fn update_project<P: ProjectRepository, M: MemberRepository, U: UserRepository>(
    State(usecase): State<Arc<UpdateProjectUseCase<P, M, U>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateProjectRequest>,
) -> Result<Json<UpdateProjectJsonResponse>, HttpError> {
    let response = usecase.execute(id, req.project, &current_user).await?;
    Ok(Json(UpdateProjectJsonResponse::from(response)))
}

/// Delete project endpoint handler
/// DELETE /api/v1/projects/:id
pub async fn delete_project<
    P: ProjectRepository,
    I: IssueRepository,
    M: MemberRepository,
    A: AttachmentRepository,
    C: IssueCategoryRepository,
    J: JournalRepository,
    R: IssueRelationRepository,
>(
    State(usecase): State<Arc<DeleteProjectUseCase<P, I, M, A, C, J, R>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i32>,
) -> Result<StatusCode, HttpError> {
    usecase.execute(id, &current_user).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Get project trackers endpoint handler
/// GET /api/v1/projects/:id/trackers
pub async fn get_project_trackers<
    P: ProjectRepository,
    M: MemberRepository,
    T: TrackerRepository,
    S: IssueStatusRepository,
>(
    State(usecase): State<Arc<GetProjectTrackersUseCase<P, M, T, S>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i32>,
) -> Result<Json<TrackerListJsonResponse>, HttpError> {
    let response = usecase.execute(id, &current_user).await?;
    Ok(Json(TrackerListJsonResponse::from(response)))
}
