use super::super::api::AppState;
use crate::domain::entities::{TOKEN_ACTION_API, TOKEN_ACTION_BLACKLIST, USER_STATUS_ACTIVE};
use crate::domain::repositories::{TokenRepository, UserRepository};
use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};

/// Current user information extracted from the JWT token
#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub id: i32,
    pub login: String,
    pub admin: bool,
}

/// Authentication middleware that validates JWT tokens or API Keys and extracts the current user
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract token from Authorization header
    let token = extract_token_from_request(&req)?;

    // Try JWT validation first
    let (user_id, is_jwt) = match state.jwt_service.validate_token(&token) {
        Ok(claims) => (claims.sub, true),
        Err(_) => {
            // JWT validation failed, try API Key lookup
            let token_record = state
                .token_repository
                .find_by_value(&token)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                .ok_or(StatusCode::UNAUTHORIZED)?;

            // Verify it's an API token
            if token_record.action != TOKEN_ACTION_API {
                return Err(StatusCode::UNAUTHORIZED);
            }

            (token_record.user_id, false)
        }
    };

    // For JWT tokens, check if it's blacklisted
    if is_jwt {
        // Use the token value as a unique identifier for blacklist check
        // We check if there's a blacklist entry with this token value
        if let Ok(Some(blacklisted)) = state.token_repository.find_by_value(&token).await {
            if blacklisted.action == TOKEN_ACTION_BLACKLIST {
                return Err(StatusCode::UNAUTHORIZED);
            }
        }
    }

    // Get user from database
    let user = state
        .user_repository
        .find_by_id(user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Check user status
    if user.status != USER_STATUS_ACTIVE {
        return Err(StatusCode::FORBIDDEN);
    }

    // Add current user to request extensions
    req.extensions_mut().insert(CurrentUser {
        id: user.id,
        login: user.login,
        admin: user.admin,
    });

    Ok(next.run(req).await)
}

/// Extract JWT token from the request
/// Tries Authorization header first (Bearer token), then falls back to X-Api-Key header
fn extract_token_from_request(req: &Request) -> Result<String, StatusCode> {
    // Try Authorization header (Bearer token)
    if let Some(auth_header) = req.headers().get(header::AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(stripped) = auth_str.strip_prefix("Bearer ") {
                return Ok(stripped.to_string());
            }
        }
    }

    // Try X-Api-Key header
    if let Some(api_key_header) = req.headers().get("X-Api-Key") {
        if let Ok(api_key) = api_key_header.to_str() {
            return Ok(api_key.to_string());
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}
