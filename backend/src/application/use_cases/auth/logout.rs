use crate::application::errors::ApplicationError;
use crate::domain::entities::TOKEN_ACTION_BLACKLIST;
use crate::domain::repositories::{CreateTokenDto, TokenRepository};
use crate::infrastructure::auth::JwtService;
use chrono::{Duration, Utc};
use std::sync::Arc;

/// Use case for user logout
/// Invalidates the current JWT token by storing it in a blacklist
pub struct LogoutUseCase<T: TokenRepository + 'static> {
    token_repo: Arc<T>,
    jwt_service: Arc<JwtService>,
    token_expiration_seconds: u64,
}

impl<T: TokenRepository + 'static> LogoutUseCase<T> {
    pub fn new(
        token_repo: Arc<T>,
        jwt_service: Arc<JwtService>,
        token_expiration_seconds: u64,
    ) -> Self {
        Self {
            token_repo,
            jwt_service,
            token_expiration_seconds,
        }
    }

    /// Execute the logout use case
    /// Stores the token in the blacklist with the same expiration as the JWT
    pub async fn execute(&self, token: &str, user_id: i32) -> Result<(), ApplicationError> {
        // Validate the token to get its expiration
        let claims = self
            .jwt_service
            .validate_token(token)
            .map_err(|_| ApplicationError::Unauthorized("Invalid token".into()))?;

        // Calculate expiration time for the blacklist entry
        // Use the token's original expiration to ensure cleanup happens naturally
        let expires_at = Utc::now() + Duration::seconds(self.token_expiration_seconds as i64);

        // Create the blacklist entry
        let dto = CreateTokenDto {
            user_id,
            action: TOKEN_ACTION_BLACKLIST.to_string(),
            value: token.to_string(),
            validity_expires_on: Some(expires_at),
        };

        self.token_repo
            .create(dto)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::Token;
    use crate::domain::repositories::RepositoryError;

    struct MockTokenRepository {
        tokens: Vec<Token>,
    }

    impl MockTokenRepository {
        fn new() -> Self {
            Self { tokens: vec![] }
        }
    }

    #[async_trait::async_trait]
    impl TokenRepository for MockTokenRepository {
        async fn find_by_user_and_action(
            &self,
            _user_id: i32,
            _action: &str,
        ) -> Result<Option<Token>, RepositoryError> {
            Ok(None)
        }

        async fn find_by_value(&self, value: &str) -> Result<Option<Token>, RepositoryError> {
            Ok(self.tokens.iter().find(|t| t.value == value).cloned())
        }

        async fn delete_by_user_id(&self, _user_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn create(&self, dto: CreateTokenDto) -> Result<Token, RepositoryError> {
            Ok(Token {
                id: 1,
                user_id: dto.user_id,
                action: dto.action,
                value: dto.value,
                validity_expires_on: dto.validity_expires_on,
                created_on: Some(Utc::now()),
                updated_on: Some(Utc::now()),
            })
        }

        async fn delete_expired(&self) -> Result<u64, RepositoryError> {
            Ok(0)
        }
    }

    #[tokio::test]
    async fn test_logout_success() {
        let token_repo = Arc::new(MockTokenRepository::new());
        let jwt_service = Arc::new(JwtService::new("test-secret".to_string(), 3600));

        // Generate a valid token
        let token = jwt_service.generate_token(1).unwrap();

        let usecase = LogoutUseCase::new(token_repo, jwt_service, 3600);

        let result = usecase.execute(&token, 1).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_logout_with_invalid_token() {
        let token_repo = Arc::new(MockTokenRepository::new());
        let jwt_service = Arc::new(JwtService::new("test-secret".to_string(), 3600));

        let usecase = LogoutUseCase::new(token_repo, jwt_service, 3600);

        let result = usecase.execute("invalid-token", 1).await;

        assert!(result.is_err());
        match result {
            Err(ApplicationError::Unauthorized(msg)) => {
                assert_eq!(msg, "Invalid token");
            }
            _ => panic!("Expected Unauthorized error"),
        }
    }
}
