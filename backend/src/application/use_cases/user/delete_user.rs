use crate::application::errors::ApplicationError;
use crate::domain::repositories::{
    EmailAddressRepository, MemberRepository, TokenRepository, UserRepository,
};
use crate::presentation::middleware::CurrentUser;
use std::sync::Arc;

/// Use case for deleting a user
pub struct DeleteUserUseCase<
    R: UserRepository,
    E: EmailAddressRepository,
    T: TokenRepository,
    M: MemberRepository,
> {
    user_repo: Arc<R>,
    email_repo: Arc<E>,
    token_repo: Arc<T>,
    member_repo: Arc<M>,
}

impl<R: UserRepository, E: EmailAddressRepository, T: TokenRepository, M: MemberRepository>
    DeleteUserUseCase<R, E, T, M>
{
    pub fn new(
        user_repo: Arc<R>,
        email_repo: Arc<E>,
        token_repo: Arc<T>,
        member_repo: Arc<M>,
    ) -> Self {
        Self {
            user_repo,
            email_repo,
            token_repo,
            member_repo,
        }
    }

    /// Execute the use case
    ///
    /// Permission rules:
    /// - Admin only
    /// - Cannot delete self
    /// - Cannot delete the last administrator
    pub async fn execute(
        &self,
        user_id: i32,
        current_user: &CurrentUser,
    ) -> Result<(), ApplicationError> {
        // 1. Check permission - admin only
        if !current_user.admin {
            return Err(ApplicationError::Forbidden(
                "Only administrators can delete users".into(),
            ));
        }

        // 2. Cannot delete self
        if current_user.id == user_id {
            return Err(ApplicationError::Validation(
                "Cannot delete yourself".into(),
            ));
        }

        // 3. Get user to delete
        let user = self
            .user_repo
            .find_by_id(user_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound("User not found".into()))?;

        // 4. Check if last admin
        if user.admin {
            let admins = self
                .user_repo
                .find_all_admins()
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;

            if admins.len() <= 1 {
                return Err(ApplicationError::Validation(
                    "Cannot delete the last administrator".into(),
                ));
            }
        }

        // 5. Delete related data in order
        // Delete tokens first
        self.token_repo
            .delete_by_user_id(user_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // Delete memberships
        self.member_repo
            .delete_by_user_id(user_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // Delete email addresses
        self.email_repo
            .delete_by_user_id(user_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // 6. Delete user
        self.user_repo
            .delete(user_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::{User, USER_STATUS_ACTIVE};
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

        async fn delete(&self, user_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }
    }

    struct MockEmailAddressRepository;

    #[async_trait::async_trait]
    impl EmailAddressRepository for MockEmailAddressRepository {
        async fn find_default_by_user_id(
            &self,
            _user_id: i32,
        ) -> Result<Option<crate::domain::entities::EmailAddress>, RepositoryError> {
            Ok(None)
        }

        async fn find_by_user_id(
            &self,
            _user_id: i32,
        ) -> Result<Vec<crate::domain::entities::EmailAddress>, RepositoryError> {
            Ok(vec![])
        }

        async fn update_address(
            &self,
            _email_id: i32,
            _address: &str,
        ) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn create(
            &self,
            _email: crate::domain::entities::EmailAddress,
        ) -> Result<crate::domain::entities::EmailAddress, RepositoryError> {
            unimplemented!()
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

    struct MockTokenRepository;

    #[async_trait::async_trait]
    impl TokenRepository for MockTokenRepository {
        async fn find_by_user_and_action(
            &self,
            _user_id: i32,
            _action: &str,
        ) -> Result<Option<crate::domain::entities::Token>, RepositoryError> {
            Ok(None)
        }

        async fn find_by_value(
            &self,
            _value: &str,
        ) -> Result<Option<crate::domain::entities::Token>, RepositoryError> {
            Ok(None)
        }

        async fn delete_by_user_id(&self, _user_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn create(
            &self,
            _dto: crate::domain::repositories::CreateTokenDto,
        ) -> Result<crate::domain::entities::Token, RepositoryError> {
            unimplemented!("Not used in delete_user tests")
        }

        async fn delete_expired(&self) -> Result<u64, RepositoryError> {
            unimplemented!("Not used in delete_user tests")
        }
    }

    struct MockMemberRepository;

    #[async_trait::async_trait]
    impl MemberRepository for MockMemberRepository {
        async fn find_by_project(
            &self,
            _project_id: i32,
        ) -> Result<Vec<crate::domain::entities::MemberWithRoles>, RepositoryError> {
            Ok(vec![])
        }

        async fn delete_by_user_id(&self, _user_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn is_member(
            &self,
            _project_id: i32,
            _user_id: i32,
        ) -> Result<bool, RepositoryError> {
            Ok(false)
        }

        async fn add_member(
            &self,
            _member: crate::domain::repositories::NewMember,
        ) -> Result<i32, RepositoryError> {
            unimplemented!("Not used in delete_user tests")
        }

        async fn add_member_role(
            &self,
            _member_id: i32,
            _role_id: i32,
            _inherited_from: Option<i32>,
        ) -> Result<(), RepositoryError> {
            unimplemented!("Not used in delete_user tests")
        }

        async fn add_manager(
            &self,
            _project_id: i32,
            _user_id: i32,
        ) -> Result<(), RepositoryError> {
            unimplemented!("Not used in delete_user tests")
        }

        async fn inherit_from_parent(
            &self,
            _project_id: i32,
            _parent_id: i32,
        ) -> Result<(), RepositoryError> {
            unimplemented!("Not used in delete_user tests")
        }

        async fn delete_by_project(&self, _project_id: i32) -> Result<(), RepositoryError> {
            unimplemented!("Not used in delete_user tests")
        }

        async fn find_by_id(
            &self,
            _member_id: i32,
        ) -> Result<Option<crate::domain::entities::MemberWithRoles>, RepositoryError> {
            unimplemented!("Not used in delete_user tests")
        }

        async fn find_by_project_and_user(
            &self,
            _project_id: i32,
            _user_id: i32,
        ) -> Result<Option<crate::domain::entities::MemberWithRoles>, RepositoryError> {
            unimplemented!("Not used in delete_user tests")
        }

        async fn clear_roles(&self, _member_id: i32) -> Result<(), RepositoryError> {
            unimplemented!("Not used in delete_user tests")
        }

        async fn delete_by_id(&self, _member_id: i32) -> Result<(), RepositoryError> {
            unimplemented!("Not used in delete_user tests")
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
            created_on: None,
            updated_on: None,
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
    async fn test_admin_can_delete_user() {
        let users = vec![
            create_test_user(1, "admin", true, USER_STATUS_ACTIVE),
            create_test_user(2, "user2", false, USER_STATUS_ACTIVE),
        ];
        let user_repo = Arc::new(MockUserRepository { users });
        let email_repo = Arc::new(MockEmailAddressRepository);
        let token_repo = Arc::new(MockTokenRepository);
        let member_repo = Arc::new(MockMemberRepository);

        let usecase = DeleteUserUseCase::new(user_repo, email_repo, token_repo, member_repo);
        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let result = usecase.execute(2, &current_user).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_non_admin_cannot_delete_user() {
        let users = vec![
            create_test_user(1, "admin", true, USER_STATUS_ACTIVE),
            create_test_user(2, "user2", false, USER_STATUS_ACTIVE),
        ];
        let user_repo = Arc::new(MockUserRepository { users });
        let email_repo = Arc::new(MockEmailAddressRepository);
        let token_repo = Arc::new(MockTokenRepository);
        let member_repo = Arc::new(MockMemberRepository);

        let usecase = DeleteUserUseCase::new(user_repo, email_repo, token_repo, member_repo);
        let current_user = CurrentUser {
            id: 2,
            login: "user2".to_string(),
            admin: false,
        };

        let result = usecase.execute(1, &current_user).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::Forbidden(_)
        ));
    }

    #[tokio::test]
    async fn test_cannot_delete_self() {
        let users = vec![create_test_user(1, "admin", true, USER_STATUS_ACTIVE)];
        let user_repo = Arc::new(MockUserRepository { users });
        let email_repo = Arc::new(MockEmailAddressRepository);
        let token_repo = Arc::new(MockTokenRepository);
        let member_repo = Arc::new(MockMemberRepository);

        let usecase = DeleteUserUseCase::new(user_repo, email_repo, token_repo, member_repo);
        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let result = usecase.execute(1, &current_user).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::Validation(_)
        ));
    }

    #[tokio::test]
    async fn test_cannot_delete_last_admin() {
        let users = vec![create_test_user(1, "admin", true, USER_STATUS_ACTIVE)];
        let user_repo = Arc::new(MockUserRepository { users });
        let email_repo = Arc::new(MockEmailAddressRepository);
        let token_repo = Arc::new(MockTokenRepository);
        let member_repo = Arc::new(MockMemberRepository);

        let usecase = DeleteUserUseCase::new(user_repo, email_repo, token_repo, member_repo);
        let current_user = CurrentUser {
            id: 2, // Another admin trying to delete the last admin
            login: "admin2".to_string(),
            admin: true,
        };

        let result = usecase.execute(1, &current_user).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::Validation(_)
        ));
    }

    #[tokio::test]
    async fn test_user_not_found() {
        let users = vec![create_test_user(1, "admin", true, USER_STATUS_ACTIVE)];
        let user_repo = Arc::new(MockUserRepository { users });
        let email_repo = Arc::new(MockEmailAddressRepository);
        let token_repo = Arc::new(MockTokenRepository);
        let member_repo = Arc::new(MockMemberRepository);

        let usecase = DeleteUserUseCase::new(user_repo, email_repo, token_repo, member_repo);
        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let result = usecase.execute(999, &current_user).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApplicationError::NotFound(_)));
    }
}
