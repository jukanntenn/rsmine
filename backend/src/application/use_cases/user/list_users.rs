use crate::application::errors::ApplicationError;
use crate::domain::entities::{EmailAddress, User};
use crate::domain::repositories::{EmailAddressRepository, UserQueryParams, UserRepository};
use std::sync::Arc;

/// User summary for list responses
#[derive(Debug, Clone)]
pub struct UserListItem {
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

impl UserListItem {
    pub fn from_user(user: &User, email: Option<EmailAddress>) -> Self {
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
            status: user.status,
        }
    }
}

/// Response for user list endpoint
#[derive(Debug, Clone)]
pub struct UserListResponse {
    pub users: Vec<UserListItem>,
    pub total_count: u32,
    pub offset: u32,
    pub limit: u32,
}

/// Use case for listing users
pub struct ListUsersUseCase<R: UserRepository, E: EmailAddressRepository> {
    user_repo: Arc<R>,
    email_repo: Arc<E>,
}

impl<R: UserRepository, E: EmailAddressRepository> ListUsersUseCase<R, E> {
    pub fn new(user_repo: Arc<R>, email_repo: Arc<E>) -> Self {
        Self {
            user_repo,
            email_repo,
        }
    }

    /// Execute the use case
    ///
    /// Note: For MVP, all authenticated users can list users.
    /// In future iterations, we may add permission checks for non-admin users.
    pub async fn execute(
        &self,
        params: UserQueryParams,
    ) -> Result<UserListResponse, ApplicationError> {
        // Get total count
        let total_count = self
            .user_repo
            .count(&params)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // Get users
        let users = self
            .user_repo
            .find_all(params.clone())
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // Get email addresses for each user
        let mut user_items = Vec::with_capacity(users.len());
        for user in users {
            let email = self
                .email_repo
                .find_default_by_user_id(user.id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;

            user_items.push(UserListItem::from_user(&user, email));
        }

        Ok(UserListResponse {
            users: user_items,
            total_count,
            offset: params.offset,
            limit: params.limit,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::USER_STATUS_ACTIVE;
    use crate::domain::repositories::RepositoryError;
    use chrono::Utc;

    // Mock implementations for testing
    struct MockUserRepository {
        users: Vec<User>,
    }

    #[async_trait::async_trait]
    impl UserRepository for MockUserRepository {
        async fn find_by_login(&self, _login: &str) -> Result<Option<User>, RepositoryError> {
            Ok(None)
        }

        async fn find_by_id(&self, _id: i32) -> Result<Option<User>, RepositoryError> {
            Ok(None)
        }

        async fn update_last_login(&self, _user_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn find_all(&self, params: UserQueryParams) -> Result<Vec<User>, RepositoryError> {
            let mut users: Vec<User> = self
                .users
                .iter()
                .filter(|u| {
                    if let Some(status) = params.status {
                        if u.status != status {
                            return false;
                        }
                    }
                    true
                })
                .cloned()
                .collect();

            users.truncate(params.limit as usize);
            Ok(users)
        }

        async fn count(&self, params: &UserQueryParams) -> Result<u32, RepositoryError> {
            let count = self
                .users
                .iter()
                .filter(|u| {
                    if let Some(status) = params.status {
                        if u.status != status {
                            return false;
                        }
                    }
                    true
                })
                .count() as u32;
            Ok(count)
        }

        async fn update(&self, user: User) -> Result<User, RepositoryError> {
            Ok(user)
        }

        async fn create(&self, _user: User) -> Result<User, RepositoryError> {
            unimplemented!()
        }

        async fn exists_by_login(&self, _login: &str) -> Result<bool, RepositoryError> {
            Ok(false)
        }

        async fn exists_by_login_excluding(
            &self,
            _login: &str,
            _exclude_user_id: i32,
        ) -> Result<bool, RepositoryError> {
            Ok(false)
        }

        async fn find_all_admins(&self) -> Result<Vec<User>, RepositoryError> {
            Ok(self.users.iter().filter(|u| u.admin).cloned().collect())
        }

        async fn delete(&self, _user_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }
    }

    struct MockEmailAddressRepository;

    #[async_trait::async_trait]
    impl EmailAddressRepository for MockEmailAddressRepository {
        async fn find_default_by_user_id(
            &self,
            _user_id: i32,
        ) -> Result<Option<EmailAddress>, RepositoryError> {
            Ok(None)
        }

        async fn find_by_user_id(
            &self,
            _user_id: i32,
        ) -> Result<Vec<EmailAddress>, RepositoryError> {
            Ok(vec![])
        }

        async fn update_address(
            &self,
            _email_id: i32,
            _address: &str,
        ) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn create(&self, email: EmailAddress) -> Result<EmailAddress, RepositoryError> {
            Ok(email)
        }

        async fn exists_by_address(&self, _address: &str) -> Result<bool, RepositoryError> {
            Ok(false)
        }

        async fn exists_by_address_excluding_user(
            &self,
            _address: &str,
            _exclude_user_id: i32,
        ) -> Result<bool, RepositoryError> {
            Ok(false)
        }

        async fn delete_by_user_id(&self, _user_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }
    }

    fn create_test_user(id: i32, login: &str, admin: bool) -> User {
        User {
            id,
            login: login.to_string(),
            hashed_password: Some("hash".to_string()),
            firstname: "Test".to_string(),
            lastname: "User".to_string(),
            admin,
            status: USER_STATUS_ACTIVE,
            last_login_on: None,
            language: Some("en".to_string()),
            auth_source_id: None,
            created_on: Some(Utc::now()),
            updated_on: Some(Utc::now()),
            r#type: None,
            mail_notification: "only_my_events".to_string(),
            salt: Some("salt".to_string()),
            must_change_passwd: false,
            passwd_changed_on: None,
            twofa_scheme: None,
            twofa_totp_key: None,
            twofa_totp_last_used_at: None,
            twofa_required: false,
        }
    }

    #[tokio::test]
    async fn test_list_users_empty() {
        let user_repo = Arc::new(MockUserRepository { users: vec![] });
        let email_repo = Arc::new(MockEmailAddressRepository);

        let usecase = ListUsersUseCase::new(user_repo, email_repo);
        let params = UserQueryParams::new(None, None, 0, 25);

        let result = usecase.execute(params).await.unwrap();
        assert_eq!(result.users.len(), 0);
        assert_eq!(result.total_count, 0);
    }

    #[tokio::test]
    async fn test_list_users_with_data() {
        let users = vec![
            create_test_user(1, "admin", true),
            create_test_user(2, "user1", false),
        ];
        let user_repo = Arc::new(MockUserRepository { users });
        let email_repo = Arc::new(MockEmailAddressRepository);

        let usecase = ListUsersUseCase::new(user_repo, email_repo);
        let params = UserQueryParams::new(None, None, 0, 25);

        let result = usecase.execute(params).await.unwrap();
        assert_eq!(result.users.len(), 2);
        assert_eq!(result.total_count, 2);
        assert_eq!(result.users[0].login, "admin");
        assert_eq!(result.users[1].login, "user1");
    }

    #[tokio::test]
    async fn test_list_users_with_status_filter() {
        let users = vec![
            create_test_user(1, "admin", true),
            create_test_user(2, "user1", false),
        ];
        let user_repo = Arc::new(MockUserRepository { users });
        let email_repo = Arc::new(MockEmailAddressRepository);

        let usecase = ListUsersUseCase::new(user_repo, email_repo);
        let params = UserQueryParams::new(Some(USER_STATUS_ACTIVE), None, 0, 25);

        let result = usecase.execute(params).await.unwrap();
        assert_eq!(result.users.len(), 2);
        assert_eq!(result.total_count, 2);
    }
}
