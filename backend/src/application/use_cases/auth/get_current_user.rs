use crate::application::errors::ApplicationError;
use crate::domain::entities::TOKEN_ACTION_API;
use crate::domain::repositories::{EmailAddressRepository, TokenRepository, UserRepository};
use std::sync::Arc;

/// Response data for current user endpoint
#[derive(Debug, Clone)]
pub struct CurrentUserResponse {
    pub user: UserDetail,
}

/// Detailed user information for the response
#[derive(Debug, Clone)]
pub struct UserDetail {
    pub id: i32,
    pub login: String,
    pub admin: bool,
    pub firstname: String,
    pub lastname: String,
    pub mail: String,
    pub created_on: Option<String>,
    pub updated_on: Option<String>,
    pub last_login_on: Option<String>,
    pub passwd_changed_on: Option<String>,
    pub twofa_scheme: Option<String>,
    pub api_key: Option<String>,
    pub status: i32,
}

impl UserDetail {
    /// Create UserDetail from User entity and related data
    pub fn from_user(
        user: &crate::domain::entities::User,
        email: Option<crate::domain::entities::EmailAddress>,
        api_key: Option<String>,
    ) -> Self {
        Self {
            id: user.id,
            login: user.login.clone(),
            admin: user.admin,
            firstname: user.firstname.clone(),
            lastname: user.lastname.clone(),
            mail: email.map(|e| e.address).unwrap_or_default(),
            created_on: user.created_on.map(|d| d.to_rfc3339()),
            updated_on: user.updated_on.map(|d| d.to_rfc3339()),
            last_login_on: user.last_login_on.map(|d| d.to_rfc3339()),
            passwd_changed_on: user.passwd_changed_on.map(|d| d.to_rfc3339()),
            twofa_scheme: user.twofa_scheme.clone(),
            api_key,
            status: user.status,
        }
    }
}

/// Use case for getting the current authenticated user's information
pub struct GetCurrentUserUseCase<R: UserRepository, E: EmailAddressRepository, T: TokenRepository> {
    user_repo: Arc<R>,
    email_repo: Arc<E>,
    token_repo: Arc<T>,
}

impl<R: UserRepository, E: EmailAddressRepository, T: TokenRepository>
    GetCurrentUserUseCase<R, E, T>
{
    pub fn new(user_repo: Arc<R>, email_repo: Arc<E>, token_repo: Arc<T>) -> Self {
        Self {
            user_repo,
            email_repo,
            token_repo,
        }
    }

    /// Execute the use case
    pub async fn execute(&self, user_id: i32) -> Result<CurrentUserResponse, ApplicationError> {
        // 1. Get user by ID
        let user = self
            .user_repo
            .find_by_id(user_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound("User not found".into()))?;

        // 2. Get default email address
        let email = self
            .email_repo
            .find_default_by_user_id(user_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // 3. Get API key (only for self or admin)
        let api_key = self
            .token_repo
            .find_by_user_and_action(user_id, TOKEN_ACTION_API)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .map(|t| t.value);

        // 4. Build response
        Ok(CurrentUserResponse {
            user: UserDetail {
                id: user.id,
                login: user.login,
                admin: user.admin,
                firstname: user.firstname,
                lastname: user.lastname,
                mail: email.map(|e| e.address).unwrap_or_default(),
                created_on: user.created_on.map(|dt| dt.to_rfc3339()),
                updated_on: user.updated_on.map(|dt| dt.to_rfc3339()),
                last_login_on: user.last_login_on.map(|dt| dt.to_rfc3339()),
                passwd_changed_on: user.passwd_changed_on.map(|dt| dt.to_rfc3339()),
                twofa_scheme: user.twofa_scheme,
                api_key,
                status: user.status,
            },
        })
    }
}
