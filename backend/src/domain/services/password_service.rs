/// Password service trait for hashing and verifying passwords
pub trait PasswordService: Send + Sync {
    /// Hash a password with the given salt
    fn hash_password(&self, password: &str, salt: &str) -> String;

    /// Verify a password against a hash and salt
    fn verify_password(&self, password: &str, hash: &str, salt: &str) -> bool;

    /// Generate a new random salt
    fn generate_salt(&self) -> String;
}
