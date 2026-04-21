use crate::domain::services::PasswordService;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

/// Argon2-based password service implementation
pub struct Argon2PasswordService {
    argon2: Argon2<'static>,
}

impl Argon2PasswordService {
    pub fn new() -> Self {
        Self {
            argon2: Argon2::default(),
        }
    }
}

impl Default for Argon2PasswordService {
    fn default() -> Self {
        Self::new()
    }
}

impl PasswordService for Argon2PasswordService {
    fn hash_password(&self, password: &str, salt: &str) -> String {
        let salt = SaltString::encode_b64(salt.as_bytes()).expect("Failed to create salt string");

        self.argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .expect("Failed to hash password")
    }

    fn verify_password(&self, password: &str, hash: &str, _salt: &str) -> bool {
        let parsed_hash = match PasswordHash::new(hash) {
            Ok(h) => h,
            Err(_) => return false,
        };

        self.argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }

    fn generate_salt(&self) -> String {
        let salt = SaltString::generate(&mut OsRng);
        salt.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing_and_verification() {
        let service = Argon2PasswordService::new();
        let salt = service.generate_salt();
        let password = "test_password";

        let hash = service.hash_password(password, &salt);

        // Verify with correct password
        assert!(service.verify_password(password, &hash, &salt));

        // Verify with wrong password
        assert!(!service.verify_password("wrong_password", &hash, &salt));
    }

    #[test]
    fn test_generate_salt() {
        let service = Argon2PasswordService::new();
        let salt1 = service.generate_salt();
        let salt2 = service.generate_salt();

        // Each salt should be unique
        assert_ne!(salt1, salt2);
    }
}
