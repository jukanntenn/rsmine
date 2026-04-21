use crate::application::errors::ApplicationError;
use crate::application::use_cases::auth::UserDetail;
use crate::domain::entities::USER_STATUS_ACTIVE;
use crate::domain::repositories::{EmailAddressRepository, TokenRepository, UserRepository};
use crate::presentation::middleware::CurrentUser;
use std::sync::Arc;

/// Response data for get user endpoint
#[derive(Debug, Clone)]
pub struct GetUserResponse {
    pub user: UserDetail,
}

/// Use case for getting a single user by ID
pub struct GetUserUseCase<R: UserRepository, E: EmailAddressRepository, T: TokenRepository> {
    user_repo: Arc<R>,
    email_repo: Arc<E>,
    token_repo: Arc<T>,
}

impl<R: UserRepository, E: EmailAddressRepository, T: TokenRepository> GetUserUseCase<R, E, T> {
    pub fn new(user_repo: Arc<R>, email_repo: Arc<E>, token_repo: Arc<T>) -> Self {
        Self {
            user_repo,
            email_repo,
            token_repo,
        }
    }

    /// Execute the use case
    ///
    /// Permission rules:
    /// - Admin users can see all users
    /// - Non-admin users can see their own profile
    /// - Non-admin users cannot view inactive users (returns 404)
    /// - API key is only visible for self or admin
    pub async fn execute(
        &self,
        user_id: i32,
        current_user: &CurrentUser,
    ) -> Result<GetUserResponse, ApplicationError> {
        // Check permission: admin or self
        let can_view = current_user.admin || current_user.id == user_id;

        if !can_view {
            return Err(ApplicationError::Forbidden(
                "You don't have permission to view this user".into(),
            ));
        }

        // Get user by ID
        let user = self
            .user_repo
            .find_by_id(user_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound("User not found".into()))?;

        // Non-admin cannot view inactive users
        if !current_user.admin && user.status != USER_STATUS_ACTIVE {
            return Err(ApplicationError::NotFound("User not found".into()));
        }

        // Get email address
        let email = self
            .email_repo
            .find_default_by_user_id(user_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // Get API key (only for self or admin)
        let api_key = if current_user.admin || current_user.id == user_id {
            self.token_repo
                .find_by_user_and_action(user_id, "api")
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?
                .map(|t| t.value)
        } else {
            None
        };

        Ok(GetUserResponse {
            user: UserDetail::from_user(&user, email, api_key),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::{EmailAddress, Token, User};
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

        async fn find_by_id(&self, id: i32) -> Result<Option<User>, RepositoryError> {
            Ok(self.users.iter().find(|u| u.id == id).cloned())
        }

        async fn update_last_login(&self, _user_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn find_all(
            &self,
            _params: crate::domain::repositories::UserQueryParams,
        ) -> Result<Vec<User>, RepositoryError> {
            Ok(self.users.clone())
        }

        async fn count(
            &self,
            _params: &crate::domain::repositories::UserQueryParams,
        ) -> Result<u32, RepositoryError> {
            Ok(self.users.len() as u32)
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

    struct MockTokenRepository {
        has_api_key: bool,
    }

    #[async_trait::async_trait]
    impl TokenRepository for MockTokenRepository {
        async fn find_by_user_and_action(
            &self,
            _user_id: i32,
            action: &str,
        ) -> Result<Option<Token>, RepositoryError> {
            if action == "api" && self.has_api_key {
                Ok(Some(Token {
                    id: 1,
                    user_id: 1,
                    action: "api".to_string(),
                    value: "test-api-key".to_string(),
                    validity_expires_on: None,
                    created_on: Some(Utc::now()),
                    updated_on: Some(Utc::now()),
                }))
            } else {
                Ok(None)
            }
        }

        async fn find_by_value(&self, _value: &str) -> Result<Option<Token>, RepositoryError> {
            Ok(None)
        }

        async fn delete_by_user_id(&self, _user_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn create(
            &self,
            _dto: crate::domain::repositories::CreateTokenDto,
        ) -> Result<Token, RepositoryError> {
            unimplemented!("Not used in get_user tests")
        }

        async fn delete_expired(&self) -> Result<u64, RepositoryError> {
            unimplemented!("Not used in get_user tests")
        }
    }

    fn create_test_user(id: i32, login: &str, admin: bool, status: i32) -> User {
        User {
            id,
            login: login.to_string(),
            hashed_password: Some("hash".to_string()),
            firstname: "Test".to_string(),
            lastname: "User".to_string(),
            admin,
            status,
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
    async fn test_get_user_as_admin() {
        let users = vec![create_test_user(2, "user1", false, USER_STATUS_ACTIVE)];
        let user_repo = Arc::new(MockUserRepository { users });
        let email_repo = Arc::new(MockEmailAddressRepository);
        let token_repo = Arc::new(MockTokenRepository { has_api_key: true });

        let usecase = GetUserUseCase::new(user_repo, email_repo, token_repo);
        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let result = usecase.execute(2, &current_user).await.unwrap();
        assert_eq!(result.user.id, 2);
        assert_eq!(result.user.login, "user1");
        assert_eq!(result.user.api_key, Some("test-api-key".to_string()));
    }

    #[tokio::test]
    async fn test_get_user_as_self() {
        let users = vec![create_test_user(1, "user1", false, USER_STATUS_ACTIVE)];
        let user_repo = Arc::new(MockUserRepository { users });
        let email_repo = Arc::new(MockEmailAddressRepository);
        let token_repo = Arc::new(MockTokenRepository { has_api_key: true });

        let usecase = GetUserUseCase::new(user_repo, email_repo, token_repo);
        let current_user = CurrentUser {
            id: 1,
            login: "user1".to_string(),
            admin: false,
        };

        let result = usecase.execute(1, &current_user).await.unwrap();
        assert_eq!(result.user.id, 1);
        assert_eq!(result.user.api_key, Some("test-api-key".to_string()));
    }

    #[tokio::test]
    async fn test_get_user_forbidden_for_non_admin_other_user() {
        let users = vec![create_test_user(2, "user2", false, USER_STATUS_ACTIVE)];
        let user_repo = Arc::new(MockUserRepository { users });
        let email_repo = Arc::new(MockEmailAddressRepository);
        let token_repo = Arc::new(MockTokenRepository { has_api_key: false });

        let usecase = GetUserUseCase::new(user_repo, email_repo, token_repo);
        let current_user = CurrentUser {
            id: 1,
            login: "user1".to_string(),
            admin: false,
        };

        let result = usecase.execute(2, &current_user).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::Forbidden(_)
        ));
    }

    #[tokio::test]
    async fn test_get_user_not_found() {
        let user_repo = Arc::new(MockUserRepository { users: vec![] });
        let email_repo = Arc::new(MockEmailAddressRepository);
        let token_repo = Arc::new(MockTokenRepository { has_api_key: false });

        let usecase = GetUserUseCase::new(user_repo, email_repo, token_repo);
        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let result = usecase.execute(999, &current_user).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApplicationError::NotFound(_)));
    }

    #[tokio::test]
    async fn test_get_inactive_user_as_non_admin() {
        let users = vec![create_test_user(1, "user1", false, 3)]; // locked user
        let user_repo = Arc::new(MockUserRepository { users });
        let email_repo = Arc::new(MockEmailAddressRepository);
        let token_repo = Arc::new(MockTokenRepository { has_api_key: false });

        let usecase = GetUserUseCase::new(user_repo, email_repo, token_repo);
        let current_user = CurrentUser {
            id: 1,
            login: "user1".to_string(),
            admin: false,
        };

        let result = usecase.execute(1, &current_user).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApplicationError::NotFound(_)));
    }

    #[tokio::test]
    async fn test_get_inactive_user_as_admin() {
        let users = vec![create_test_user(2, "user2", false, 3)]; // locked user
        let user_repo = Arc::new(MockUserRepository { users });
        let email_repo = Arc::new(MockEmailAddressRepository);
        let token_repo = Arc::new(MockTokenRepository { has_api_key: false });

        let usecase = GetUserUseCase::new(user_repo, email_repo, token_repo);
        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let result = usecase.execute(2, &current_user).await.unwrap();
        assert_eq!(result.user.id, 2);
        assert_eq!(result.user.status, 3);
    }
}
