use crate::application::dto::UpdateUserDto;
use crate::application::errors::ApplicationError;
use crate::application::use_cases::auth::UserDetail;
use crate::domain::entities::EmailAddress;
use crate::domain::repositories::{EmailAddressRepository, UserRepository};
use crate::domain::services::PasswordService;
use crate::presentation::middleware::CurrentUser;
use chrono::Utc;
use std::sync::Arc;

/// Response data for update user endpoint
#[derive(Debug, Clone)]
pub struct UpdateUserResponse {
    pub user: UserDetail,
}

/// Use case for updating a user
pub struct UpdateUserUseCase<R: UserRepository, E: EmailAddressRepository, P: PasswordService> {
    user_repo: Arc<R>,
    email_repo: Arc<E>,
    password_service: Arc<P>,
}

impl<R: UserRepository, E: EmailAddressRepository, P: PasswordService> UpdateUserUseCase<R, E, P> {
    pub fn new(user_repo: Arc<R>, email_repo: Arc<E>, password_service: Arc<P>) -> Self {
        Self {
            user_repo,
            email_repo,
            password_service,
        }
    }

    /// Execute the use case
    ///
    /// Permission rules:
    /// - Admin: can update any user, can modify all fields
    /// - Non-admin: can only update own profile, can only modify firstname, lastname, mail, language, password
    pub async fn execute(
        &self,
        user_id: i32,
        dto: UpdateUserDto,
        current_user: &CurrentUser,
    ) -> Result<UpdateUserResponse, ApplicationError> {
        // 1. Get existing user
        let mut user = self
            .user_repo
            .find_by_id(user_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound("User not found".into()))?;

        // 2. Check permission
        let is_admin = current_user.admin;
        let is_self = current_user.id == user_id;

        if !is_admin && !is_self {
            return Err(ApplicationError::Forbidden(
                "You don't have permission to update this user".into(),
            ));
        }

        // 3. Validate password confirmation if password is provided
        if let Err(e) = dto.validate_password_confirmation() {
            return Err(ApplicationError::Validation(e));
        }

        // 4. Update fields based on permission
        if is_admin {
            // Admin can update all fields
            if let Some(ref login) = dto.login {
                let trimmed_login = login.trim();
                if !trimmed_login.is_empty() {
                    // Check if login is already taken by another user
                    if self
                        .user_repo
                        .exists_by_login_excluding(trimmed_login, user_id)
                        .await
                        .map_err(|e| ApplicationError::Internal(e.to_string()))?
                    {
                        return Err(ApplicationError::Validation(
                            "Login has already been taken".into(),
                        ));
                    }
                    user.login = trimmed_login.to_string();
                }
            }
            if let Some(admin) = dto.admin {
                user.admin = admin;
            }
            if let Some(status) = dto.status {
                user.status = status;
            }
        }

        // Both admin and self can update these fields
        if let Some(ref firstname) = dto.firstname {
            let trimmed = firstname.trim();
            if !trimmed.is_empty() {
                user.firstname = trimmed.to_string();
            }
        }
        if let Some(ref lastname) = dto.lastname {
            let trimmed = lastname.trim();
            if !trimmed.is_empty() {
                user.lastname = trimmed.to_string();
            }
        }
        if let Some(ref language) = dto.language {
            user.language = Some(language.clone());
        }

        // 5. Handle email update
        if let Some(ref mail) = dto.mail {
            let trimmed_mail = mail.trim();
            if !trimmed_mail.is_empty() {
                let current_email = self
                    .email_repo
                    .find_default_by_user_id(user_id)
                    .await
                    .map_err(|e| ApplicationError::Internal(e.to_string()))?;

                let current_address = current_email.as_ref().map(|e| e.address.as_str());

                if current_address != Some(trimmed_mail) {
                    // Check if email is already taken by another user
                    if self
                        .email_repo
                        .exists_by_address_excluding_user(trimmed_mail, user_id)
                        .await
                        .map_err(|e| ApplicationError::Internal(e.to_string()))?
                    {
                        return Err(ApplicationError::Validation(
                            "Email has already been taken".into(),
                        ));
                    }

                    if let Some(email) = current_email {
                        self.email_repo
                            .update_address(email.id, trimmed_mail)
                            .await
                            .map_err(|e| ApplicationError::Internal(e.to_string()))?;
                    } else {
                        self.email_repo
                            .create(EmailAddress {
                                id: 0, // Will be set by database
                                user_id,
                                address: trimmed_mail.to_string(),
                                is_default: true,
                                notify: true,
                                created_on: Some(Utc::now()),
                                updated_on: Some(Utc::now()),
                            })
                            .await
                            .map_err(|e| ApplicationError::Internal(e.to_string()))?;
                    }
                }
            }
        }

        // 6. Handle password update
        if let Some(ref password) = dto.password {
            let password = password.trim();
            if !password.is_empty() {
                // Update password
                let salt = user
                    .salt
                    .clone()
                    .unwrap_or_else(|| self.password_service.generate_salt());
                user.hashed_password = Some(self.password_service.hash_password(password, &salt));
                user.salt = Some(salt);
                user.passwd_changed_on = Some(Utc::now());

                // Note: In a full implementation, we would invalidate other sessions here
                // when updating own password (keeping current session valid)
            }
        }

        // 7. Save user
        user.updated_on = Some(Utc::now());
        let updated_user = self
            .user_repo
            .update(user)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // 8. Get updated email
        let email = self
            .email_repo
            .find_default_by_user_id(user_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // 9. Return response
        Ok(UpdateUserResponse {
            user: UserDetail::from_user(&updated_user, email, None),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::{EmailAddress, User, USER_STATUS_ACTIVE};
    use crate::domain::repositories::RepositoryError;

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

    struct MockEmailAddressRepository {
        email: Option<EmailAddress>,
    }

    #[async_trait::async_trait]
    impl EmailAddressRepository for MockEmailAddressRepository {
        async fn find_default_by_user_id(
            &self,
            _user_id: i32,
        ) -> Result<Option<EmailAddress>, RepositoryError> {
            Ok(self.email.clone())
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
    async fn test_update_own_profile() {
        let users = vec![create_test_user(1, "user1", false, USER_STATUS_ACTIVE)];
        let user_repo = Arc::new(MockUserRepository { users });
        let email_repo = Arc::new(MockEmailAddressRepository {
            email: Some(EmailAddress {
                id: 1,
                user_id: 1,
                address: "test@example.com".to_string(),
                is_default: true,
                notify: true,
                created_on: Some(Utc::now()),
                updated_on: Some(Utc::now()),
            }),
        });
        let password_service = Arc::new(MockPasswordService);

        let usecase = UpdateUserUseCase::new(user_repo, email_repo, password_service);
        let current_user = CurrentUser {
            id: 1,
            login: "user1".to_string(),
            admin: false,
        };

        let dto = UpdateUserDto {
            login: None,
            firstname: Some("Updated".to_string()),
            lastname: Some("Name".to_string()),
            mail: None,
            language: Some("zh".to_string()),
            password: None,
            password_confirmation: None,
            admin: None,
            status: None,
        };

        let result = usecase.execute(1, dto, &current_user).await.unwrap();
        assert_eq!(result.user.firstname, "Updated");
        assert_eq!(result.user.lastname, "Name");
    }

    #[tokio::test]
    async fn test_admin_can_update_other_user() {
        let users = vec![create_test_user(2, "user2", false, USER_STATUS_ACTIVE)];
        let user_repo = Arc::new(MockUserRepository { users });
        let email_repo = Arc::new(MockEmailAddressRepository { email: None });
        let password_service = Arc::new(MockPasswordService);

        let usecase = UpdateUserUseCase::new(user_repo, email_repo, password_service);
        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let dto = UpdateUserDto {
            login: Some("newlogin".to_string()),
            firstname: Some("Updated".to_string()),
            lastname: Some("Name".to_string()),
            mail: None,
            language: None,
            password: None,
            password_confirmation: None,
            admin: Some(true),
            status: None,
        };

        let result = usecase.execute(2, dto, &current_user).await.unwrap();
        assert_eq!(result.user.login, "newlogin");
        assert!(result.user.admin);
    }

    #[tokio::test]
    async fn test_non_admin_cannot_update_other_user() {
        let users = vec![create_test_user(2, "user2", false, USER_STATUS_ACTIVE)];
        let user_repo = Arc::new(MockUserRepository { users });
        let email_repo = Arc::new(MockEmailAddressRepository { email: None });
        let password_service = Arc::new(MockPasswordService);

        let usecase = UpdateUserUseCase::new(user_repo, email_repo, password_service);
        let current_user = CurrentUser {
            id: 1,
            login: "user1".to_string(),
            admin: false,
        };

        let dto = UpdateUserDto {
            login: None,
            firstname: Some("Updated".to_string()),
            lastname: None,
            mail: None,
            language: None,
            password: None,
            password_confirmation: None,
            admin: None,
            status: None,
        };

        let result = usecase.execute(2, dto, &current_user).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::Forbidden(_)
        ));
    }

    #[tokio::test]
    async fn test_password_mismatch() {
        let users = vec![create_test_user(1, "user1", false, USER_STATUS_ACTIVE)];
        let user_repo = Arc::new(MockUserRepository { users });
        let email_repo = Arc::new(MockEmailAddressRepository { email: None });
        let password_service = Arc::new(MockPasswordService);

        let usecase = UpdateUserUseCase::new(user_repo, email_repo, password_service);
        let current_user = CurrentUser {
            id: 1,
            login: "user1".to_string(),
            admin: false,
        };

        let dto = UpdateUserDto {
            login: None,
            firstname: None,
            lastname: None,
            mail: None,
            language: None,
            password: Some("newpassword".to_string()),
            password_confirmation: Some("differentpassword".to_string()),
            admin: None,
            status: None,
        };

        let result = usecase.execute(1, dto, &current_user).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::Validation(_)
        ));
    }

    #[tokio::test]
    async fn test_user_not_found() {
        let user_repo = Arc::new(MockUserRepository { users: vec![] });
        let email_repo = Arc::new(MockEmailAddressRepository { email: None });
        let password_service = Arc::new(MockPasswordService);

        let usecase = UpdateUserUseCase::new(user_repo, email_repo, password_service);
        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let dto = UpdateUserDto {
            login: None,
            firstname: Some("Updated".to_string()),
            lastname: None,
            mail: None,
            language: None,
            password: None,
            password_confirmation: None,
            admin: None,
            status: None,
        };

        let result = usecase.execute(999, dto, &current_user).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApplicationError::NotFound(_)));
    }
}
