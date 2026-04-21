use crate::application::dto::LoginRequest;
use crate::application::errors::ApplicationError;
use crate::domain::entities::{User, USER_STATUS_ACTIVE, USER_STATUS_LOCKED};
use crate::domain::repositories::UserRepository;
use crate::domain::services::PasswordService;
use crate::infrastructure::auth::JwtService;
use std::sync::Arc;

/// Response data for successful login
#[derive(Debug, Clone)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserSummary,
}

/// Summary of user data returned after login
#[derive(Debug, Clone)]
pub struct UserSummary {
    pub id: i32,
    pub login: String,
    pub firstname: String,
    pub lastname: String,
    pub admin: bool,
}

impl From<User> for UserSummary {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            login: user.login,
            firstname: user.firstname,
            lastname: user.lastname,
            admin: user.admin,
        }
    }
}

/// Use case for user login
pub struct LoginUseCase<R: UserRepository + 'static, P: PasswordService + 'static> {
    user_repo: Arc<R>,
    password_service: Arc<P>,
    jwt_service: Arc<JwtService>,
}

impl<R: UserRepository + 'static, P: PasswordService + 'static> LoginUseCase<R, P> {
    pub fn new(user_repo: Arc<R>, password_service: Arc<P>, jwt_service: Arc<JwtService>) -> Self {
        Self {
            user_repo,
            password_service,
            jwt_service,
        }
    }

    /// Execute the login use case
    pub async fn execute(&self, dto: LoginRequest) -> Result<LoginResponse, ApplicationError> {
        // Strip whitespace from username (Redmine compatibility)
        let username = dto.username.trim();

        // 1. Find user by login (case-insensitive)
        let user = self
            .user_repo
            .find_by_login(username)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| ApplicationError::Unauthorized("Invalid username or password".into()))?;

        // 2. Check user status
        if user.status == USER_STATUS_LOCKED {
            return Err(ApplicationError::Forbidden("Account is locked".into()));
        }

        if user.status != USER_STATUS_ACTIVE {
            return Err(ApplicationError::Forbidden("Account is not active".into()));
        }

        // 3. Verify password
        let salt = user.salt.as_deref().unwrap_or("");
        let hash = user.hashed_password.as_deref().unwrap_or("");

        if !self
            .password_service
            .verify_password(&dto.password, hash, salt)
        {
            return Err(ApplicationError::Unauthorized(
                "Invalid username or password".into(),
            ));
        }

        // 4. Generate JWT token
        let token = self
            .jwt_service
            .generate_token(user.id)
            .map_err(|_| ApplicationError::Internal("Failed to generate token".into()))?;

        // 5. Update last login time (non-blocking, ignore errors)
        let user_id = user.id;
        let user_repo = self.user_repo.clone();
        tokio::spawn(async move {
            let _ = user_repo.update_last_login(user_id).await;
        });

        // 6. Return response
        Ok(LoginResponse {
            token,
            user: UserSummary::from(user),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::User;
    use crate::domain::repositories::{RepositoryError, UserQueryParams};
    use chrono::Utc;

    // Mock implementations for testing
    struct MockUserRepository {
        users: Vec<User>,
    }

    #[async_trait::async_trait]
    impl UserRepository for MockUserRepository {
        async fn find_by_login(&self, login: &str) -> Result<Option<User>, RepositoryError> {
            Ok(self
                .users
                .iter()
                .find(|u| u.login.eq_ignore_ascii_case(login))
                .cloned())
        }

        async fn find_by_id(&self, id: i32) -> Result<Option<User>, RepositoryError> {
            Ok(self.users.iter().find(|u| u.id == id).cloned())
        }

        async fn update_last_login(&self, _user_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn find_all(&self, _params: UserQueryParams) -> Result<Vec<User>, RepositoryError> {
            Ok(self.users.clone())
        }

        async fn count(&self, _params: &UserQueryParams) -> Result<u32, RepositoryError> {
            Ok(self.users.len() as u32)
        }

        async fn update(&self, user: User) -> Result<User, RepositoryError> {
            Ok(user)
        }

        async fn create(&self, user: User) -> Result<User, RepositoryError> {
            Ok(user)
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

    struct MockPasswordService;

    impl PasswordService for MockPasswordService {
        fn hash_password(&self, password: &str, _salt: &str) -> String {
            password.to_string()
        }

        fn verify_password(&self, password: &str, hash: &str, _salt: &str) -> bool {
            password == hash
        }

        fn generate_salt(&self) -> String {
            "mock_salt".to_string()
        }
    }

    fn create_test_user() -> User {
        User {
            id: 1,
            login: "admin".to_string(),
            hashed_password: Some("password123".to_string()),
            firstname: "Admin".to_string(),
            lastname: "User".to_string(),
            admin: true,
            status: USER_STATUS_ACTIVE,
            last_login_on: None,
            language: Some("en".to_string()),
            auth_source_id: None,
            created_on: Some(Utc::now()),
            updated_on: Some(Utc::now()),
            r#type: None,
            mail_notification: "only_my_events".to_string(),
            salt: Some("mock_salt".to_string()),
            must_change_passwd: false,
            passwd_changed_on: None,
            twofa_scheme: None,
            twofa_totp_key: None,
            twofa_totp_last_used_at: None,
            twofa_required: false,
        }
    }

    #[tokio::test]
    async fn test_login_with_good_credentials() {
        let user = create_test_user();
        let user_repo = Arc::new(MockUserRepository { users: vec![user] });
        let password_service = Arc::new(MockPasswordService);
        let jwt_service = Arc::new(JwtService::new("test-secret".to_string(), 3600));

        let usecase = LoginUseCase::new(user_repo, password_service, jwt_service);

        let request = LoginRequest {
            username: "admin".to_string(),
            password: "password123".to_string(),
        };

        let result = usecase.execute(request).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.user.login, "admin");
        assert!(response.user.admin);
    }

    #[tokio::test]
    async fn test_login_with_wrong_password() {
        let user = create_test_user();
        let user_repo = Arc::new(MockUserRepository { users: vec![user] });
        let password_service = Arc::new(MockPasswordService);
        let jwt_service = Arc::new(JwtService::new("test-secret".to_string(), 3600));

        let usecase = LoginUseCase::new(user_repo, password_service, jwt_service);

        let request = LoginRequest {
            username: "admin".to_string(),
            password: "wrong_password".to_string(),
        };

        let result = usecase.execute(request).await;

        assert!(result.is_err());
        match result {
            Err(ApplicationError::Unauthorized(msg)) => {
                assert_eq!(msg, "Invalid username or password");
            }
            _ => panic!("Expected Unauthorized error"),
        }
    }

    #[tokio::test]
    async fn test_login_with_locked_account() {
        let mut user = create_test_user();
        user.status = USER_STATUS_LOCKED;

        let user_repo = Arc::new(MockUserRepository { users: vec![user] });
        let password_service = Arc::new(MockPasswordService);
        let jwt_service = Arc::new(JwtService::new("test-secret".to_string(), 3600));

        let usecase = LoginUseCase::new(user_repo, password_service, jwt_service);

        let request = LoginRequest {
            username: "admin".to_string(),
            password: "password123".to_string(),
        };

        let result = usecase.execute(request).await;

        assert!(result.is_err());
        match result {
            Err(ApplicationError::Forbidden(msg)) => {
                assert_eq!(msg, "Account is locked");
            }
            _ => panic!("Expected Forbidden error"),
        }
    }
}
