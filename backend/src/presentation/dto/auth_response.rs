use crate::application::use_cases::{CurrentUserResponse, LoginResponse, UserDetail, UserSummary};
use serde::Serialize;

/// JSON response for successful login
#[derive(Debug, Serialize)]
pub struct LoginJsonResponse {
    pub token: String,
    pub user: UserJsonResponse,
}

/// JSON representation of user data in responses
#[derive(Debug, Serialize)]
pub struct UserJsonResponse {
    pub id: i32,
    pub login: String,
    pub firstname: String,
    pub lastname: String,
    pub admin: bool,
}

/// JSON response for current user endpoint
#[derive(Debug, Serialize)]
pub struct CurrentUserJsonResponse {
    pub user: UserDetailJsonResponse,
}

/// JSON representation of detailed user data for current user endpoint
#[derive(Debug, Serialize)]
pub struct UserDetailJsonResponse {
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

impl From<LoginResponse> for LoginJsonResponse {
    fn from(response: LoginResponse) -> Self {
        Self {
            token: response.token,
            user: UserJsonResponse::from(response.user),
        }
    }
}

impl From<UserSummary> for UserJsonResponse {
    fn from(summary: UserSummary) -> Self {
        Self {
            id: summary.id,
            login: summary.login,
            firstname: summary.firstname,
            lastname: summary.lastname,
            admin: summary.admin,
        }
    }
}

impl From<CurrentUserResponse> for CurrentUserJsonResponse {
    fn from(response: CurrentUserResponse) -> Self {
        Self {
            user: UserDetailJsonResponse::from(response.user),
        }
    }
}

impl From<UserDetail> for UserDetailJsonResponse {
    fn from(detail: UserDetail) -> Self {
        Self {
            id: detail.id,
            login: detail.login,
            admin: detail.admin,
            firstname: detail.firstname,
            lastname: detail.lastname,
            mail: detail.mail,
            created_on: detail.created_on,
            updated_on: detail.updated_on,
            last_login_on: detail.last_login_on,
            passwd_changed_on: detail.passwd_changed_on,
            twofa_scheme: detail.twofa_scheme,
            api_key: detail.api_key,
            status: detail.status,
        }
    }
}
