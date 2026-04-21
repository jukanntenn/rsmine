use crate::application::dto::{CreateUserRequest, UpdateUserRequest};
use crate::application::use_cases::{
    CreateUserUseCase, DeleteUserUseCase, GetUserUseCase, ListUsersUseCase, UpdateUserUseCase,
};
use crate::domain::repositories::{
    EmailAddressRepository, MemberRepository, TokenRepository, UserQueryParams, UserRepository,
};
use crate::domain::services::PasswordService;
use crate::presentation::dto::{UserDetailJsonResponse, UserListJsonResponse};
use crate::presentation::errors::HttpError;
use crate::presentation::middleware::CurrentUser;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension, Json,
};
use serde::Deserialize;
use std::sync::Arc;

/// Query parameters for the user list endpoint
#[derive(Debug, Deserialize)]
pub struct UserListQuery {
    /// Offset for pagination (default: 0)
    #[serde(default)]
    pub offset: Option<u32>,

    /// Limit for pagination (default: 25, max: 100)
    #[serde(default = "default_limit")]
    pub limit: Option<u32>,

    /// Filter by status (1=active, 2=registered, 3=locked)
    pub status: Option<i32>,

    /// Filter by name (fuzzy search on login, firstname, or lastname)
    pub name: Option<String>,
}

fn default_limit() -> Option<u32> {
    Some(25)
}

/// JSON response for get user endpoint
#[derive(Debug, serde::Serialize)]
pub struct GetUserJsonResponse {
    pub user: UserDetailJsonResponse,
}

/// JSON response for create user endpoint
#[derive(Debug, serde::Serialize)]
pub struct CreateUserJsonResponse {
    pub user: UserDetailJsonResponse,
}

/// JSON response for update user endpoint
#[derive(Debug, serde::Serialize)]
pub struct UpdateUserJsonResponse {
    pub user: UserDetailJsonResponse,
}

impl From<crate::application::use_cases::GetUserResponse> for GetUserJsonResponse {
    fn from(response: crate::application::use_cases::GetUserResponse) -> Self {
        Self {
            user: UserDetailJsonResponse::from(response.user),
        }
    }
}

impl From<crate::application::use_cases::CreateUserResponse> for CreateUserJsonResponse {
    fn from(response: crate::application::use_cases::CreateUserResponse) -> Self {
        Self {
            user: UserDetailJsonResponse::from(response.user),
        }
    }
}

impl From<crate::application::use_cases::UpdateUserResponse> for UpdateUserJsonResponse {
    fn from(response: crate::application::use_cases::UpdateUserResponse) -> Self {
        Self {
            user: UserDetailJsonResponse::from(response.user),
        }
    }
}

/// List users endpoint handler
/// GET /api/v1/users.json
pub async fn list_users<R: UserRepository, E: EmailAddressRepository>(
    State(usecase): State<Arc<ListUsersUseCase<R, E>>>,
    Extension(_current_user): Extension<CurrentUser>,
    Query(query): Query<UserListQuery>,
) -> Result<Json<UserListJsonResponse>, HttpError> {
    let params = UserQueryParams::new(
        query.status,
        query.name,
        query.offset.unwrap_or(0),
        query.limit.unwrap_or(25),
    );

    let response = usecase.execute(params).await?;

    Ok(Json(UserListJsonResponse::from(response)))
}

/// Create user endpoint handler
/// POST /api/v1/users.json
pub async fn create_user<R: UserRepository, E: EmailAddressRepository, P: PasswordService>(
    State(usecase): State<Arc<CreateUserUseCase<R, E, P>>>,
    Extension(current_user): Extension<CurrentUser>,
    Json(req): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<CreateUserJsonResponse>), HttpError> {
    let response = usecase.execute(req.user, &current_user).await?;
    Ok((
        StatusCode::CREATED,
        Json(CreateUserJsonResponse::from(response)),
    ))
}

/// Get user endpoint handler
/// GET /api/v1/users/:id.json
pub async fn get_user<R: UserRepository, E: EmailAddressRepository, T: TokenRepository>(
    State(usecase): State<Arc<GetUserUseCase<R, E, T>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i32>,
) -> Result<Json<GetUserJsonResponse>, HttpError> {
    let response = usecase.execute(id, &current_user).await?;
    Ok(Json(GetUserJsonResponse::from(response)))
}

/// Update user endpoint handler
/// PUT /api/v1/users/:id.json
pub async fn update_user<R: UserRepository, E: EmailAddressRepository, P: PasswordService>(
    State(usecase): State<Arc<UpdateUserUseCase<R, E, P>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<Json<UpdateUserJsonResponse>, HttpError> {
    let response = usecase.execute(id, req.user, &current_user).await?;
    Ok(Json(UpdateUserJsonResponse::from(response)))
}

/// Delete user endpoint handler
/// DELETE /api/v1/users/:id
pub async fn delete_user<
    R: UserRepository,
    E: EmailAddressRepository,
    T: TokenRepository,
    M: MemberRepository,
>(
    State(usecase): State<Arc<DeleteUserUseCase<R, E, T, M>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i32>,
) -> Result<StatusCode, HttpError> {
    usecase.execute(id, &current_user).await?;
    Ok(StatusCode::NO_CONTENT)
}
