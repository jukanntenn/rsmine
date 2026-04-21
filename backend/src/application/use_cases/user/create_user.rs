use crate::application::dto::CreateUserDto;
use crate::application::errors::ApplicationError;
use crate::application::use_cases::auth::UserDetail;
use crate::domain::entities::{EmailAddress, User, USER_STATUS_ACTIVE};
use crate::domain::repositories::{EmailAddressRepository, UserRepository};
use crate::domain::services::PasswordService;
use crate::presentation::middleware::CurrentUser;
use chrono::Utc;
use rand::Rng;
use std::sync::Arc;

/// Response data for create user endpoint
#[derive(Debug, Clone)]
pub struct CreateUserResponse {
    pub user: UserDetail,
}

/// Use case for creating a new user (admin only)
pub struct CreateUserUseCase<R: UserRepository, E: EmailAddressRepository, P: PasswordService> {
    user_repo: Arc<R>,
    email_repo: Arc<E>,
    password_service: Arc<P>,
}

impl<R: UserRepository, E: EmailAddressRepository, P: PasswordService> CreateUserUseCase<R, E, P> {
    pub fn new(user_repo: Arc<R>, email_repo: Arc<E>, password_service: Arc<P>) -> Self {
        Self {
            user_repo,
            email_repo,
            password_service,
        }
    }

    /// Execute the use case
    ///
    /// Permission: Admin only
    ///
    /// Steps:
    /// 1. Validate admin permission
    /// 2. Validate request data
    /// 3. Check login uniqueness
    /// 4. Check email uniqueness
    /// 5. Generate password if needed
    /// 6. Hash password
    /// 7. Create user
    /// 8. Create email address
    pub async fn execute(
        &self,
        dto: CreateUserDto,
        current_user: &CurrentUser,
    ) -> Result<CreateUserResponse, ApplicationError> {
        // 1. Check admin permission
        if !current_user.admin {
            return Err(ApplicationError::Forbidden(
                "Only administrators can create users".into(),
            ));
        }

        // 2. Validate request data
        dto.validate().map_err(ApplicationError::Validation)?;

        let login = dto.trimmed_login();
        let mail = dto.trimmed_mail();
        let firstname = dto.trimmed_firstname();
        let lastname = dto.trimmed_lastname();

        // 3. Check login uniqueness (case-insensitive)
        if self
            .user_repo
            .exists_by_login(&login)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
        {
            return Err(ApplicationError::Validation(
                "Login has already been taken".into(),
            ));
        }

        // 4. Check email uniqueness
        if self
            .email_repo
            .exists_by_address(&mail)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
        {
            return Err(ApplicationError::Validation(
                "Email has already been taken".into(),
            ));
        }

        // 5. Determine password
        let password = if dto.generate_password {
            Self::generate_random_password()
        } else {
            dto.password
                .as_ref()
                .map(|p| p.trim().to_string())
                .filter(|p| !p.is_empty())
                .unwrap_or_else(|| Self::generate_random_password())
        };

        // 6. Hash password
        let salt = self.password_service.generate_salt();
        let hashed_password = self.password_service.hash_password(&password, &salt);

        let now = Utc::now();

        // 7. Create user entity
        let user = User {
            id: 0,                       // Will be set by database
            login: login.to_lowercase(), // Store login in lowercase for case-insensitive lookup
            hashed_password: Some(hashed_password),
            firstname,
            lastname,
            admin: dto.admin,
            status: dto.status,
            last_login_on: None,
            language: Some(dto.language.clone()),
            auth_source_id: None,
            created_on: Some(now),
            updated_on: Some(now),
            r#type: None,
            mail_notification: "only_my_events".to_string(),
            salt: Some(salt),
            must_change_passwd: false,
            passwd_changed_on: Some(now),
            twofa_scheme: None,
            twofa_totp_key: None,
            twofa_totp_last_used_at: None,
            twofa_required: false,
        };

        // Create user in database
        let created_user = self
            .user_repo
            .create(user)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // 8. Create email address
        let email = EmailAddress {
            id: 0, // Will be set by database
            user_id: created_user.id,
            address: mail,
            is_default: true,
            notify: true,
            created_on: Some(now),
            updated_on: Some(now),
        };

        let created_email = self
            .email_repo
            .create(email)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // Return response (no API key for newly created users)
        Ok(CreateUserResponse {
            user: UserDetail::from_user(&created_user, Some(created_email), None),
        })
    }

    /// Generate a random password (12 characters, alphanumeric with special chars)
    fn generate_random_password() -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                0123456789\
                                !@#$%^&*";
        let mut rng = rand::thread_rng();

        (0..12)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::RepositoryError;

    // Mock implementations for testing
    struct MockUserRepository {
        users: Vec<User>,
        existing_logins: Vec<String>,
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

        async fn update(&self, _user: User) -> Result<User, RepositoryError> {
            unimplemented!()
        }

        async fn create(&self, mut user: User) -> Result<User, RepositoryError> {
            user.id = (self.users.len() + 1) as i32;
            Ok(user)
        }

        async fn exists_by_login(&self, login: &str) -> Result<bool, RepositoryError> {
            Ok(self
                .existing_logins
                .iter()
                .any(|l| l.to_lowercase() == login.to_lowercase()))
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

    struct MockEmailAddressRepository {
        existing_addresses: Vec<String>,
    }

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

        async fn create(&self, mut email: EmailAddress) -> Result<EmailAddress, RepositoryError> {
            email.id = 1;
            Ok(email)
        }

        async fn exists_by_address(&self, address: &str) -> Result<bool, RepositoryError> {
            Ok(self.existing_addresses.iter().any(|a| a == address))
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

    struct MockPasswordService;

    impl PasswordService for MockPasswordService {
        fn hash_password(&self, password: &str, _salt: &str) -> String {
            format!("hashed_{}", password)
        }

        fn verify_password(&self, _password: &str, _hash: &str, _salt: &str) -> bool {
            true
        }

        fn generate_salt(&self) -> String {
            "mock_salt".to_string()
        }
    }

    #[tokio::test]
    async fn test_create_user_as_admin() {
        let user_repo = Arc::new(MockUserRepository {
            users: vec![],
            existing_logins: vec![],
        });
        let email_repo = Arc::new(MockEmailAddressRepository {
            existing_addresses: vec![],
        });
        let password_service = Arc::new(MockPasswordService);

        let usecase = CreateUserUseCase::new(user_repo, email_repo, password_service);
        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let dto = CreateUserDto {
            login: "newuser".to_string(),
            firstname: "New".to_string(),
            lastname: "User".to_string(),
            mail: "newuser@example.com".to_string(),
            password: Some("password123".to_string()),
            password_confirmation: Some("password123".to_string()),
            admin: false,
            status: USER_STATUS_ACTIVE,
            language: "en".to_string(),
            generate_password: false,
            send_information: false,
        };

        let result = usecase.execute(dto, &current_user).await.unwrap();
        assert_eq!(result.user.login, "newuser");
        assert_eq!(result.user.firstname, "New");
        assert_eq!(result.user.lastname, "User");
        assert_eq!(result.user.mail, "newuser@example.com");
        assert!(!result.user.admin);
        assert_eq!(result.user.status, USER_STATUS_ACTIVE);
    }

    #[tokio::test]
    async fn test_create_user_as_non_admin() {
        let user_repo = Arc::new(MockUserRepository {
            users: vec![],
            existing_logins: vec![],
        });
        let email_repo = Arc::new(MockEmailAddressRepository {
            existing_addresses: vec![],
        });
        let password_service = Arc::new(MockPasswordService);

        let usecase = CreateUserUseCase::new(user_repo, email_repo, password_service);
        let current_user = CurrentUser {
            id: 1,
            login: "user".to_string(),
            admin: false,
        };

        let dto = CreateUserDto {
            login: "newuser".to_string(),
            firstname: "New".to_string(),
            lastname: "User".to_string(),
            mail: "newuser@example.com".to_string(),
            password: Some("password123".to_string()),
            password_confirmation: Some("password123".to_string()),
            admin: false,
            status: USER_STATUS_ACTIVE,
            language: "en".to_string(),
            generate_password: false,
            send_information: false,
        };

        let result = usecase.execute(dto, &current_user).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::Forbidden(_)
        ));
    }

    #[tokio::test]
    async fn test_create_user_duplicate_login() {
        let user_repo = Arc::new(MockUserRepository {
            users: vec![],
            existing_logins: vec!["existinguser".to_string()],
        });
        let email_repo = Arc::new(MockEmailAddressRepository {
            existing_addresses: vec![],
        });
        let password_service = Arc::new(MockPasswordService);

        let usecase = CreateUserUseCase::new(user_repo, email_repo, password_service);
        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let dto = CreateUserDto {
            login: "existinguser".to_string(),
            firstname: "New".to_string(),
            lastname: "User".to_string(),
            mail: "newuser@example.com".to_string(),
            password: Some("password123".to_string()),
            password_confirmation: Some("password123".to_string()),
            admin: false,
            status: USER_STATUS_ACTIVE,
            language: "en".to_string(),
            generate_password: false,
            send_information: false,
        };

        let result = usecase.execute(dto, &current_user).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::Validation(_)
        ));
    }

    #[tokio::test]
    async fn test_create_user_duplicate_email() {
        let user_repo = Arc::new(MockUserRepository {
            users: vec![],
            existing_logins: vec![],
        });
        let email_repo = Arc::new(MockEmailAddressRepository {
            existing_addresses: vec!["existing@example.com".to_string()],
        });
        let password_service = Arc::new(MockPasswordService);

        let usecase = CreateUserUseCase::new(user_repo, email_repo, password_service);
        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let dto = CreateUserDto {
            login: "newuser".to_string(),
            firstname: "New".to_string(),
            lastname: "User".to_string(),
            mail: "existing@example.com".to_string(),
            password: Some("password123".to_string()),
            password_confirmation: Some("password123".to_string()),
            admin: false,
            status: USER_STATUS_ACTIVE,
            language: "en".to_string(),
            generate_password: false,
            send_information: false,
        };

        let result = usecase.execute(dto, &current_user).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::Validation(_)
        ));
    }

    #[tokio::test]
    async fn test_create_user_with_generated_password() {
        let user_repo = Arc::new(MockUserRepository {
            users: vec![],
            existing_logins: vec![],
        });
        let email_repo = Arc::new(MockEmailAddressRepository {
            existing_addresses: vec![],
        });
        let password_service = Arc::new(MockPasswordService);

        let usecase = CreateUserUseCase::new(user_repo, email_repo, password_service);
        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let dto = CreateUserDto {
            login: "newuser".to_string(),
            firstname: "New".to_string(),
            lastname: "User".to_string(),
            mail: "newuser@example.com".to_string(),
            password: None,
            password_confirmation: None,
            admin: false,
            status: USER_STATUS_ACTIVE,
            language: "en".to_string(),
            generate_password: true,
            send_information: false,
        };

        let result = usecase.execute(dto, &current_user).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_random_password() {
        let password = CreateUserUseCase::<
            MockUserRepository,
            MockEmailAddressRepository,
            MockPasswordService,
        >::generate_random_password();
        assert_eq!(password.len(), 12);
    }
}
