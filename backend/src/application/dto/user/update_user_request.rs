use serde::Deserialize;

/// Request wrapper for updating a user
#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub user: UpdateUserDto,
}

/// Data transfer object for updating a user
/// All fields are optional - only provided fields will be updated
#[derive(Debug, Deserialize)]
pub struct UpdateUserDto {
    /// Login name (admin only)
    pub login: Option<String>,
    /// First name
    pub firstname: Option<String>,
    /// Last name
    pub lastname: Option<String>,
    /// Email address
    pub mail: Option<String>,
    /// Language preference
    pub language: Option<String>,
    /// New password
    pub password: Option<String>,
    /// Password confirmation (must match password)
    pub password_confirmation: Option<String>,
    /// Is administrator (admin only)
    pub admin: Option<bool>,
    /// User status (admin only): 1=active, 2=registered, 3=locked
    pub status: Option<i32>,
}

impl UpdateUserDto {
    /// Check if password update is requested
    pub fn has_password_update(&self) -> bool {
        self.password.is_some()
    }

    /// Validate password confirmation matches
    pub fn validate_password_confirmation(&self) -> Result<(), String> {
        match (&self.password, &self.password_confirmation) {
            (Some(password), Some(confirmation)) if password == confirmation => Ok(()),
            (Some(_), Some(_)) => Err("Password confirmation doesn't match".to_string()),
            (Some(_), None) => Err("Password confirmation is required".to_string()),
            (None, _) => Ok(()),
        }
    }
}
