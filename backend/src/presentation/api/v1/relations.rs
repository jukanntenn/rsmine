use crate::application::dto::CreateRelationRequest;
use crate::application::use_cases::{
    CreateRelationUseCase, DeleteRelationUseCase, GetRelationUseCase, ListRelationsUseCase,
};
use crate::domain::repositories::{
    IssueRelationRepository, IssueRepository, MemberRepository, ProjectRepository,
};
use crate::presentation::dto::RelationListJsonResponse;
use crate::presentation::errors::HttpError;
use crate::presentation::middleware::CurrentUser;
use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    Json,
};
use serde::Serialize;
use std::sync::Arc;

/// List relations for an issue endpoint handler
/// GET /api/v1/issues/:id/relations.json
pub async fn list_relations<I, P, R>(
    State(usecase): State<Arc<ListRelationsUseCase<I, P, R>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i32>,
) -> Result<Json<RelationListJsonResponse>, HttpError>
where
    I: IssueRepository,
    P: ProjectRepository,
    R: IssueRelationRepository,
{
    let response = usecase.execute(id, &current_user).await?;

    Ok(Json(RelationListJsonResponse::from(response)))
}

/// Create relation for an issue endpoint handler
/// POST /api/v1/issues/:id/relations.json
pub async fn create_relation<I, P, M, R>(
    State(usecase): State<Arc<CreateRelationUseCase<I, P, M, R>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i32>,
    Json(request): Json<CreateRelationRequest>,
) -> Result<(StatusCode, Json<RelationCreateJsonResponse>), HttpError>
where
    I: IssueRepository,
    P: ProjectRepository,
    M: MemberRepository,
    R: IssueRelationRepository,
{
    let response = usecase.execute(id, request.relation, &current_user).await?;

    Ok((
        StatusCode::CREATED,
        Json(RelationCreateJsonResponse::from(response)),
    ))
}

/// JSON response wrapper for creating a relation
#[derive(Debug, serde::Serialize)]
pub struct RelationCreateJsonResponse {
    pub relation: RelationItemJsonResponse,
}

impl From<crate::application::use_cases::CreateRelationResponse> for RelationCreateJsonResponse {
    fn from(response: crate::application::use_cases::CreateRelationResponse) -> Self {
        Self {
            relation: RelationItemJsonResponse {
                id: response.id,
                issue_id: response.issue_id,
                issue_to_id: response.issue_to_id,
                relation_type: response.relation_type,
                delay: response.delay,
            },
        }
    }
}

/// JSON representation of a single issue relation (for create response)
#[derive(Debug, serde::Serialize)]
pub struct RelationItemJsonResponse {
    pub id: i32,
    pub issue_id: i32,
    pub issue_to_id: i32,
    pub relation_type: String,
    pub delay: Option<i32>,
}

/// Delete relation endpoint handler
/// DELETE /api/v1/relations/:id.json
pub async fn delete_relation<I, P, M, R>(
    State(usecase): State<Arc<DeleteRelationUseCase<I, P, M, R>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i32>,
) -> Result<StatusCode, HttpError>
where
    I: IssueRepository,
    P: ProjectRepository,
    M: MemberRepository,
    R: IssueRelationRepository,
{
    usecase.execute(id, &current_user).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// Get relation details endpoint handler
/// GET /api/v1/relations/:id
pub async fn get_relation<I, P, R>(
    State(usecase): State<Arc<GetRelationUseCase<I, P, R>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i32>,
) -> Result<Json<RelationDetailJsonResponse>, HttpError>
where
    I: IssueRepository,
    P: ProjectRepository,
    R: IssueRelationRepository,
{
    let response = usecase.execute(id, &current_user).await?;

    Ok(Json(RelationDetailJsonResponse::from(response)))
}

/// JSON response wrapper for relation detail
#[derive(Debug, Serialize)]
pub struct RelationDetailJsonResponse {
    pub relation: RelationDetailItem,
}

/// JSON representation of a relation detail
#[derive(Debug, Serialize)]
pub struct RelationDetailItem {
    pub id: i32,
    pub issue_id: i32,
    pub issue_to_id: i32,
    pub relation_type: String,
    pub delay: Option<i32>,
}

impl From<crate::application::use_cases::GetRelationResponse> for RelationDetailJsonResponse {
    fn from(response: crate::application::use_cases::GetRelationResponse) -> Self {
        Self {
            relation: RelationDetailItem {
                id: response.relation.id,
                issue_id: response.relation.issue_from_id,
                issue_to_id: response.relation.issue_to_id,
                relation_type: response.relation.relation_type,
                delay: response.relation.delay,
            },
        }
    }
}
