use crate::application::dto::{CreateRoleRequest, UpdateRoleRequest};
use crate::application::use_cases::{
    CreateRoleUseCase, DeleteRoleUseCase, GetRoleUseCase, ListRolesUseCase, UpdateRoleUseCase,
};
use crate::domain::repositories::RoleRepository;
use crate::presentation::dto::{
    CreateRoleJsonResponse, GetRoleJsonResponse, RoleListJsonResponse, UpdateRoleJsonResponse,
};
use crate::presentation::errors::HttpError;
use crate::presentation::middleware::CurrentUser;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use std::sync::Arc;

/// List roles endpoint handler
/// GET /api/v1/roles.json
pub async fn list_roles<R: RoleRepository>(
    State(usecase): State<Arc<ListRolesUseCase<R>>>,
    Extension(_current_user): Extension<CurrentUser>,
) -> Result<Json<RoleListJsonResponse>, HttpError> {
    let response = usecase.execute().await?;
    Ok(Json(RoleListJsonResponse::from(response)))
}

/// Get role endpoint handler
/// GET /api/v1/roles/:id.json
pub async fn get_role<R: RoleRepository>(
    State(usecase): State<Arc<GetRoleUseCase<R>>>,
    Extension(_current_user): Extension<CurrentUser>,
    Path(id): Path<i32>,
) -> Result<Json<GetRoleJsonResponse>, HttpError> {
    let response = usecase.execute(id).await?;
    Ok(Json(GetRoleJsonResponse::from(response)))
}

/// Create role endpoint handler
/// POST /api/v1/roles.json
pub async fn create_role<R: RoleRepository>(
    State(usecase): State<Arc<CreateRoleUseCase<R>>>,
    Extension(current_user): Extension<CurrentUser>,
    Json(request): Json<CreateRoleRequest>,
) -> Result<(StatusCode, Json<CreateRoleJsonResponse>), HttpError> {
    // Only admins can create roles
    if !current_user.admin {
        return Err(HttpError::Forbidden(
            "Only administrators can create roles".into(),
        ));
    }

    let response = usecase.execute(request.role.into()).await?;
    Ok((
        StatusCode::CREATED,
        Json(CreateRoleJsonResponse::from(response)),
    ))
}

/// Update role endpoint handler
/// PUT /api/v1/roles/:id.json
pub async fn update_role<R: RoleRepository>(
    State(usecase): State<Arc<UpdateRoleUseCase<R>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i32>,
    Json(request): Json<UpdateRoleRequest>,
) -> Result<Json<UpdateRoleJsonResponse>, HttpError> {
    // Only admins can update roles
    if !current_user.admin {
        return Err(HttpError::Forbidden(
            "Only administrators can update roles".into(),
        ));
    }

    let response = usecase.execute(id, request.role.into()).await?;
    Ok(Json(UpdateRoleJsonResponse::from(response)))
}

/// Delete role endpoint handler
/// DELETE /api/v1/roles/:id.json
pub async fn delete_role<R: RoleRepository>(
    State(usecase): State<Arc<DeleteRoleUseCase<R>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i32>,
) -> Result<StatusCode, HttpError> {
    // Only admins can delete roles
    if !current_user.admin {
        return Err(HttpError::Forbidden(
            "Only administrators can delete roles".into(),
        ));
    }

    usecase.execute(id, &current_user).await?;
    Ok(StatusCode::NO_CONTENT)
}
