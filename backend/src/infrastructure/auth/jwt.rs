use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JwtError {
    #[error("Token generation failed")]
    TokenGeneration,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Token expired")]
    TokenExpired,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: i32,
    /// Expiration time (as Unix timestamp)
    pub exp: usize,
    /// Issued at time (as Unix timestamp)
    pub iat: usize,
}

pub struct JwtService {
    secret: String,
    expiration_seconds: u64,
}

impl JwtService {
    pub fn new(secret: String, expiration_seconds: u64) -> Self {
        Self {
            secret,
            expiration_seconds,
        }
    }

    /// Generate a JWT token for a user
    pub fn generate_token(&self, user_id: i32) -> Result<String, JwtError> {
        let now = Utc::now();
        let exp = now + Duration::seconds(self.expiration_seconds as i64);

        let claims = Claims {
            sub: user_id,
            exp: exp.timestamp() as usize,
            iat: now.timestamp() as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|_| JwtError::TokenGeneration)
    }

    /// Validate a JWT token and return the claims
    pub fn validate_token(&self, token: &str) -> Result<Claims, JwtError> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|err| {
            if *err.kind() == jsonwebtoken::errors::ErrorKind::ExpiredSignature {
                JwtError::TokenExpired
            } else {
                JwtError::InvalidToken
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_validate_token() {
        let service = JwtService::new("test-secret".to_string(), 3600);
        let token = service.generate_token(1).unwrap();
        let claims = service.validate_token(&token).unwrap();

        assert_eq!(claims.sub, 1);
    }
}
