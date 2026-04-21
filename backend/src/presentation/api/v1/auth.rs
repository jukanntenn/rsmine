use crate::application::dto::LoginRequest;
use crate::application::use_cases::{GetCurrentUserUseCase, LoginUseCase, LogoutUseCase};
use crate::domain::repositories::{EmailAddressRepository, TokenRepository, UserRepository};
use crate::domain::services::PasswordService;
use crate::presentation::api::AppState;
use crate::presentation::dto::{CurrentUserJsonResponse, LoginJsonResponse};
use crate::presentation::errors::HttpError;
use crate::presentation::middleware::CurrentUser;
use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode},
    Extension, Json,
};
use serde::Deserialize;
use std::sync::Arc;
use validator::Validate;

/// JSON body for login endpoint
#[derive(Debug, Deserialize, Validate)]
pub struct LoginJson {
    #[validate(length(min = 1, message = "Username is required"))]
    pub username: String,

    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

/// Login endpoint handler
/// POST /api/v1/auth/login
pub async fn login<R: UserRepository, P: PasswordService>(
    State(usecase): State<Arc<LoginUseCase<R, P>>>,
    Json(body): Json<LoginJson>,
) -> Result<Json<LoginJsonResponse>, HttpError> {
    // Validate input
    body.validate()
        .map_err(|e| HttpError::BadRequest(e.to_string()))?;

    // Create request DTO
    let request = LoginRequest {
        username: body.username,
        password: body.password,
    };

    // Execute use case
    let response = usecase.execute(request).await?;

    // Convert to JSON response
    Ok(Json(LoginJsonResponse::from(response)))
}

/// Get current user endpoint handler
/// GET /api/v1/auth/me
pub async fn get_current_user<R: UserRepository, E: EmailAddressRepository, T: TokenRepository>(
    State(usecase): State<Arc<GetCurrentUserUseCase<R, E, T>>>,
    Extension(current_user): Extension<CurrentUser>,
) -> Result<Json<CurrentUserJsonResponse>, HttpError> {
    let response = usecase.execute(current_user.id).await?;
    Ok(Json(CurrentUserJsonResponse::from(response)))
}

/// Logout endpoint handler using AppState directly
/// POST /api/v1/auth/logout
/// This version extracts the logout use case from AppState
pub async fn logout_with_app_state(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
    headers: HeaderMap,
) -> Result<StatusCode, HttpError> {
    // Extract token from Authorization header
    let token = extract_bearer_token(&headers)?;

    // Create the logout use case from state
    let logout_usecase = LogoutUseCase::new(
        state.token_repository.clone(),
        state.jwt_service.clone(),
        state.config.jwt.expiration,
    );

    // Execute logout use case
    logout_usecase.execute(&token, current_user.id).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// Extract Bearer token from Authorization header
fn extract_bearer_token(headers: &HeaderMap) -> Result<String, HttpError> {
    let auth_header = headers
        .get(header::AUTHORIZATION)
        .ok_or_else(|| HttpError::Unauthorized("Missing Authorization header".into()))?;

    let auth_str = auth_header
        .to_str()
        .map_err(|_| HttpError::Unauthorized("Invalid Authorization header".into()))?;

    if !auth_str.starts_with("Bearer ") {
        return Err(HttpError::Unauthorized(
            "Invalid Authorization header format".into(),
        ));
    }

    Ok(auth_str[7..].to_string())
}
