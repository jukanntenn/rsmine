use crate::application::use_cases::{UserListItem, UserListResponse};
use serde::Serialize;

/// JSON response for user list endpoint
#[derive(Debug, Serialize)]
pub struct UserListJsonResponse {
    pub users: Vec<UserItemJsonResponse>,
    pub total_count: u32,
    pub offset: u32,
    pub limit: u32,
}

/// JSON representation of a single user in list responses
#[derive(Debug, Serialize)]
pub struct UserItemJsonResponse {
    pub id: i32,
    pub login: String,
    pub admin: bool,
    pub firstname: String,
    pub lastname: String,
    pub mail: String,
    pub created_on: Option<String>,
    pub updated_on: Option<String>,
    pub last_login_on: Option<String>,
    pub status: i32,
}

impl From<UserListResponse> for UserListJsonResponse {
    fn from(response: UserListResponse) -> Self {
        Self {
            users: response
                .users
                .into_iter()
                .map(UserItemJsonResponse::from)
                .collect(),
            total_count: response.total_count,
            offset: response.offset,
            limit: response.limit,
        }
    }
}

impl From<UserListItem> for UserItemJsonResponse {
    fn from(item: UserListItem) -> Self {
        Self {
            id: item.id,
            login: item.login,
            admin: item.admin,
            firstname: item.firstname,
            lastname: item.lastname,
            mail: item.mail,
            created_on: item.created_on,
            updated_on: item.updated_on,
            last_login_on: item.last_login_on,
            status: item.status,
        }
    }
}
