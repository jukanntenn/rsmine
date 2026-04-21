use crate::application::dto::{CreateMemberRequest, UpdateMemberRequest};
use crate::application::use_cases::{
    AddMemberUseCase, GetMemberUseCase, ListMembersUseCase, RemoveMemberUseCase,
    UpdateMemberUseCase,
};
use crate::domain::repositories::{
    IssueRepository, MemberRepository, ProjectRepository, RoleRepository, UserRepository,
};
use crate::presentation::dto::{
    CreateMemberJsonResponse, GetMemberJsonResponse, MemberListJsonResponse,
    UpdateMemberJsonResponse,
};
use crate::presentation::errors::HttpError;
use crate::presentation::middleware::CurrentUser;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use std::sync::Arc;

/// List members endpoint handler
/// GET /api/v1/projects/:id/memberships.json
pub async fn list_members<P: ProjectRepository, M: MemberRepository>(
    State(usecase): State<Arc<ListMembersUseCase<P, M>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(project_id): Path<i32>,
) -> Result<Json<MemberListJsonResponse>, HttpError> {
    let response = usecase.execute(project_id, &current_user).await?;
    Ok(Json(MemberListJsonResponse::from(response)))
}

/// Add member endpoint handler
/// POST /api/v1/projects/:id/memberships.json
pub async fn add_member<
    P: ProjectRepository,
    M: MemberRepository,
    U: UserRepository,
    R: RoleRepository,
>(
    State(usecase): State<Arc<AddMemberUseCase<M, P, U, R>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(project_id): Path<i32>,
    Json(request): Json<CreateMemberRequest>,
) -> Result<Json<CreateMemberJsonResponse>, HttpError> {
    let response = usecase
        .execute(project_id, request.membership, &current_user)
        .await?;
    Ok(Json(CreateMemberJsonResponse::from(response)))
}

/// Get member endpoint handler
/// GET /api/v1/memberships/:id
pub async fn get_member<M: MemberRepository, P: ProjectRepository>(
    State(usecase): State<Arc<GetMemberUseCase<M, P>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(membership_id): Path<i32>,
) -> Result<Json<GetMemberJsonResponse>, HttpError> {
    let response = usecase.execute(membership_id, &current_user).await?;
    Ok(Json(GetMemberJsonResponse::from(response)))
}

/// Update member endpoint handler
/// PUT /api/v1/memberships/:id
pub async fn update_member<M: MemberRepository, P: ProjectRepository, R: RoleRepository>(
    State(usecase): State<Arc<UpdateMemberUseCase<M, P, R>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(membership_id): Path<i32>,
    Json(request): Json<UpdateMemberRequest>,
) -> Result<Json<UpdateMemberJsonResponse>, HttpError> {
    let response = usecase
        .execute(membership_id, request.membership, &current_user)
        .await?;
    Ok(Json(UpdateMemberJsonResponse::from(response)))
}

/// Delete member endpoint handler
/// DELETE /api/v1/memberships/:id.json
pub async fn delete_member<M: MemberRepository, I: IssueRepository>(
    State(usecase): State<Arc<RemoveMemberUseCase<M, I>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(membership_id): Path<i32>,
) -> Result<StatusCode, HttpError> {
    usecase.execute(membership_id, &current_user).await?;
    Ok(StatusCode::NO_CONTENT)
}
