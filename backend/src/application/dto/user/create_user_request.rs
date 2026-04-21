use serde::Deserialize;

/// Request wrapper for creating a user
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub user: CreateUserDto,
}

/// Data transfer object for creating a user
#[derive(Debug, Deserialize)]
pub struct CreateUserDto {
    /// Login name (required, 1-255 characters)
    pub login: String,
    /// First name (required, 1-30 characters)
    pub firstname: String,
    /// Last name (required, 1-255 characters)
    pub lastname: String,
    /// Email address (required)
    pub mail: String,
    /// Password (optional if generate_password is true)
    pub password: Option<String>,
    /// Password confirmation (must match password if provided)
    pub password_confirmation: Option<String>,
    /// Is administrator (default: false)
    #[serde(default)]
    pub admin: bool,
    /// User status (default: 1=active): 1=active, 2=registered, 3=locked
    #[serde(default = "default_status")]
    pub status: i32,
    /// Language preference (default: "en")
    #[serde(default = "default_language")]
    pub language: String,
    /// Generate a random password (default: false)
    #[serde(default)]
    pub generate_password: bool,
    /// Send notification email to user (default: false, not implemented in MVP)
    #[serde(default)]
    #[serde(rename = "send_information")]
    pub send_information: bool,
}

fn default_status() -> i32 {
    1 // USER_STATUS_ACTIVE
}

fn default_language() -> String {
    "en".to_string()
}

impl CreateUserDto {
    /// Check if password validation is needed
    pub fn needs_password(&self) -> bool {
        !self.generate_password
    }

    /// Validate password confirmation matches
    pub fn validate_password_confirmation(&self) -> Result<(), String> {
        // If generate_password is true, no password validation needed
        if self.generate_password {
            return Ok(());
        }

        match (&self.password, &self.password_confirmation) {
            (Some(password), Some(confirmation)) if password == confirmation => Ok(()),
            (Some(_), Some(_)) => Err("Password confirmation doesn't match".to_string()),
            (Some(_), None) => Err("Password confirmation is required".to_string()),
            (None, _) => Err("Password is required when generate_password is false".to_string()),
        }
    }

    /// Validate required fields are not empty
    pub fn validate_required_fields(&self) -> Result<(), String> {
        if self.login.trim().is_empty() {
            return Err("Login cannot be blank".to_string());
        }
        if self.firstname.trim().is_empty() {
            return Err("First name cannot be blank".to_string());
        }
        if self.lastname.trim().is_empty() {
            return Err("Last name cannot be blank".to_string());
        }
        if self.mail.trim().is_empty() {
            return Err("Email cannot be blank".to_string());
        }
        Ok(())
    }

    /// Validate email format (basic check)
    pub fn validate_email(&self) -> Result<(), String> {
        let mail = self.mail.trim();
        if !mail.contains('@') || !mail.contains('.') {
            return Err("Email is invalid".to_string());
        }
        Ok(())
    }

    /// Validate status value
    pub fn validate_status(&self) -> Result<(), String> {
        match self.status {
            1..=3 => Ok(()),
            _ => Err("Status must be 1 (active), 2 (registered), or 3 (locked)".to_string()),
        }
    }

    /// Validate password length (minimum 8 characters)
    pub fn validate_password_length(&self) -> Result<(), String> {
        if self.generate_password {
            return Ok(());
        }

        if let Some(ref password) = self.password {
            if password.len() < 8 {
                return Err("Password is too short (minimum is 8 characters)".to_string());
            }
        }
        Ok(())
    }

    /// Perform all validations
    pub fn validate(&self) -> Result<(), String> {
        self.validate_required_fields()?;
        self.validate_email()?;
        self.validate_status()?;
        self.validate_password_confirmation()?;
        self.validate_password_length()?;
        Ok(())
    }

    /// Get trimmed login
    pub fn trimmed_login(&self) -> String {
        self.login.trim().to_string()
    }

    /// Get trimmed email
    pub fn trimmed_mail(&self) -> String {
        self.mail.trim().to_string()
    }

    /// Get trimmed firstname
    pub fn trimmed_firstname(&self) -> String {
        self.firstname.trim().to_string()
    }

    /// Get trimmed lastname
    pub fn trimmed_lastname(&self) -> String {
        self.lastname.trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_values() {
        let dto = CreateUserDto {
            login: "test".to_string(),
            firstname: "Test".to_string(),
            lastname: "User".to_string(),
            mail: "test@example.com".to_string(),
            password: Some("password123".to_string()),
            password_confirmation: Some("password123".to_string()),
            admin: false,
            status: default_status(),
            language: default_language(),
            generate_password: false,
            send_information: false,
        };

        assert_eq!(dto.status, 1);
        assert_eq!(dto.language, "en");
        assert!(!dto.admin);
        assert!(!dto.generate_password);
    }

    #[test]
    fn test_password_validation_with_generate_password() {
        let dto = CreateUserDto {
            login: "test".to_string(),
            firstname: "Test".to_string(),
            lastname: "User".to_string(),
            mail: "test@example.com".to_string(),
            password: None,
            password_confirmation: None,
            admin: false,
            status: 1,
            language: "en".to_string(),
            generate_password: true,
            send_information: false,
        };

        assert!(dto.validate_password_confirmation().is_ok());
    }

    #[test]
    fn test_password_validation_mismatch() {
        let dto = CreateUserDto {
            login: "test".to_string(),
            firstname: "Test".to_string(),
            lastname: "User".to_string(),
            mail: "test@example.com".to_string(),
            password: Some("password123".to_string()),
            password_confirmation: Some("different".to_string()),
            admin: false,
            status: 1,
            language: "en".to_string(),
            generate_password: false,
            send_information: false,
        };

        assert!(dto.validate_password_confirmation().is_err());
    }

    #[test]
    fn test_password_too_short() {
        let dto = CreateUserDto {
            login: "test".to_string(),
            firstname: "Test".to_string(),
            lastname: "User".to_string(),
            mail: "test@example.com".to_string(),
            password: Some("short".to_string()),
            password_confirmation: Some("short".to_string()),
            admin: false,
            status: 1,
            language: "en".to_string(),
            generate_password: false,
            send_information: false,
        };

        assert!(dto.validate_password_length().is_err());
    }

    #[test]
    fn test_email_validation() {
        let valid_dto = CreateUserDto {
            login: "test".to_string(),
            firstname: "Test".to_string(),
            lastname: "User".to_string(),
            mail: "test@example.com".to_string(),
            password: Some("password123".to_string()),
            password_confirmation: Some("password123".to_string()),
            admin: false,
            status: 1,
            language: "en".to_string(),
            generate_password: false,
            send_information: false,
        };
        assert!(valid_dto.validate_email().is_ok());

        let invalid_dto = CreateUserDto {
            login: "test".to_string(),
            firstname: "Test".to_string(),
            lastname: "User".to_string(),
            mail: "invalid-email".to_string(),
            password: Some("password123".to_string()),
            password_confirmation: Some("password123".to_string()),
            admin: false,
            status: 1,
            language: "en".to_string(),
            generate_password: false,
            send_information: false,
        };
        assert!(invalid_dto.validate_email().is_err());
    }
}
