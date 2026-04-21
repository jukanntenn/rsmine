use serde::Deserialize;

/// JSON request wrapper for creating an issue relation
#[derive(Debug, Deserialize)]
pub struct CreateRelationRequest {
    pub relation: CreateRelationDto,
}

/// DTO for creating an issue relation
#[derive(Debug, Clone, Deserialize)]
pub struct CreateRelationDto {
    /// Target issue ID (required)
    pub issue_to_id: i32,

    /// Relation type (required): relates, duplicates, blocks, precedes, follows, copied_to
    pub relation_type: String,

    /// Delay in days (optional, only for precedes/follows)
    #[serde(default)]
    pub delay: Option<i32>,
}

impl CreateRelationDto {
    /// Validate the DTO
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Validate relation type
        let valid_types = [
            "relates",
            "duplicates",
            "blocks",
            "precedes",
            "follows",
            "copied_to",
            "duplicated",
            "blocked",
            "copied_from",
        ];

        if !valid_types.contains(&self.relation_type.as_str()) {
            errors.push(format!("Invalid relation type: {}", self.relation_type));
        }

        // Validate delay is only used with precedes/follows
        if self.delay.is_some() && !matches!(self.relation_type.as_str(), "precedes" | "follows") {
            errors
                .push("Delay can only be specified for precedes or follows relations".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Get the normalized relation type
    /// Some relation types are automatically reversed to their canonical form
    pub fn normalized_relation_type(&self) -> &str {
        match self.relation_type.as_str() {
            "duplicated" => "duplicates",
            "blocked" => "blocks",
            "copied_from" => "copied_to",
            other => other,
        }
    }

    /// Get the reverse relation type for bidirectional relations
    pub fn reverse_relation_type(&self) -> Option<&'static str> {
        match self.relation_type.as_str() {
            "relates" => Some("relates"),
            "duplicates" => Some("duplicated"),
            "blocks" => Some("blocked"),
            "precedes" => Some("follows"),
            "follows" => Some("precedes"),
            _ => None,
        }
    }
}
