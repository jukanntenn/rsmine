use super::RepositoryError;
use crate::domain::entities::User;
use async_trait::async_trait;

/// Query parameters for listing users
#[derive(Debug, Clone, Default)]
pub struct UserQueryParams {
    pub status: Option<i32>,
    pub name: Option<String>,
    pub offset: u32,
    pub limit: u32,
}

impl UserQueryParams {
    pub fn new(status: Option<i32>, name: Option<String>, offset: u32, limit: u32) -> Self {
        Self {
            status,
            name,
            offset,
            limit: limit.min(100), // Max 100 items per page
        }
    }
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Find a user by login name (case-insensitive)
    async fn find_by_login(&self, login: &str) -> Result<Option<User>, RepositoryError>;

    /// Find a user by ID
    async fn find_by_id(&self, id: i32) -> Result<Option<User>, RepositoryError>;

    /// Update the last login timestamp for a user
    async fn update_last_login(&self, user_id: i32) -> Result<(), RepositoryError>;

    /// Find all users matching the query parameters
    async fn find_all(&self, params: UserQueryParams) -> Result<Vec<User>, RepositoryError>;

    /// Count users matching the query parameters
    async fn count(&self, params: &UserQueryParams) -> Result<u32, RepositoryError>;

    /// Update a user entity
    async fn update(&self, user: User) -> Result<User, RepositoryError>;

    /// Create a new user entity
    async fn create(&self, user: User) -> Result<User, RepositoryError>;

    /// Check if a user exists with the given login (case-insensitive)
    async fn exists_by_login(&self, login: &str) -> Result<bool, RepositoryError>;

    /// Check if a user exists with the given login (excluding a specific user ID)
    async fn exists_by_login_excluding(
        &self,
        login: &str,
        exclude_user_id: i32,
    ) -> Result<bool, RepositoryError>;

    /// Find all administrator users
    async fn find_all_admins(&self) -> Result<Vec<User>, RepositoryError>;

    /// Delete a user by ID
    async fn delete(&self, user_id: i32) -> Result<(), RepositoryError>;
}
