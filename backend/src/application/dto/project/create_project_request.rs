use serde::Deserialize;

/// Request wrapper for creating a project
#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub project: CreateProjectDto,
}

/// DTO for creating a new project
#[derive(Debug, Deserialize)]
pub struct CreateProjectDto {
    /// Project name (required)
    pub name: String,

    /// Project identifier (required, must be URL-safe)
    pub identifier: String,

    /// Project description
    pub description: Option<String>,

    /// Project homepage URL
    pub homepage: Option<String>,

    /// Is the project public (default: true)
    #[serde(default = "default_is_public")]
    pub is_public: bool,

    /// Parent project ID for subprojects
    pub parent_id: Option<i32>,

    /// Inherit members from parent project
    #[serde(default)]
    pub inherit_members: bool,

    /// Enabled tracker IDs
    pub tracker_ids: Option<Vec<i32>>,
}

fn default_is_public() -> bool {
    true
}

impl CreateProjectDto {
    /// Validate the project identifier format
    ///
    /// Rules:
    /// - Must not be empty
    /// - Maximum 100 characters
    /// - Only lowercase letters, digits, hyphens, and underscores
    /// - Cannot start with a digit or hyphen
    pub fn validate_identifier(&self) -> Result<(), String> {
        let identifier = self.identifier.trim();

        if identifier.is_empty() {
            return Err("Identifier cannot be blank".to_string());
        }

        if identifier.len() > 100 {
            return Err("Identifier is too long (max 100 characters)".to_string());
        }

        // Must contain only lowercase letters, digits, hyphens, and underscores
        if !identifier
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
        {
            return Err(
                "Identifier must contain only lowercase letters, digits, hyphens and underscores"
                    .to_string(),
            );
        }

        // Cannot start with digit or hyphen
        if identifier.starts_with(|c: char| c.is_ascii_digit() || c == '-') {
            return Err("Identifier cannot start with a digit or hyphen".to_string());
        }

        Ok(())
    }

    /// Validate the project name
    pub fn validate_name(&self) -> Result<(), String> {
        let name = self.name.trim();

        if name.is_empty() {
            return Err("Name cannot be blank".to_string());
        }

        if name.len() > 255 {
            return Err("Name is too long (max 255 characters)".to_string());
        }

        Ok(())
    }

    /// Validate all fields
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if let Err(e) = self.validate_name() {
            errors.push(e);
        }

        if let Err(e) = self.validate_identifier() {
            errors.push(e);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
