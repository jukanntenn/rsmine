use serde::Deserialize;

/// Request wrapper for updating a project
#[derive(Debug, Deserialize)]
pub struct UpdateProjectRequest {
    pub project: UpdateProjectDto,
}

/// DTO for updating an existing project
#[derive(Debug, Deserialize)]
pub struct UpdateProjectDto {
    /// Project name
    pub name: Option<String>,

    /// Project identifier (admin only)
    pub identifier: Option<String>,

    /// Project description
    pub description: Option<String>,

    /// Project homepage URL
    pub homepage: Option<String>,

    /// Is the project public (requires select_project_publicity permission)
    pub is_public: Option<bool>,

    /// Parent project ID for subprojects
    pub parent_id: Option<i32>,

    /// Inherit members from parent project
    pub inherit_members: Option<bool>,

    /// Project status (admin only): 1=active, 5=closed, 9=archived
    pub status: Option<i32>,

    /// Enabled tracker IDs (replaces existing trackers)
    pub tracker_ids: Option<Vec<i32>>,
}

impl UpdateProjectDto {
    /// Validate the project identifier format
    ///
    /// Rules:
    /// - Must not be empty
    /// - Maximum 100 characters
    /// - Only lowercase letters, digits, hyphens, and underscores
    /// - Cannot start with a digit or hyphen
    pub fn validate_identifier(identifier: &str) -> Result<(), String> {
        let identifier = identifier.trim();

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
    pub fn validate_name(name: &str) -> Result<(), String> {
        let name = name.trim();

        if name.is_empty() {
            return Err("Name cannot be blank".to_string());
        }

        if name.len() > 255 {
            return Err("Name is too long (max 255 characters)".to_string());
        }

        Ok(())
    }
}
