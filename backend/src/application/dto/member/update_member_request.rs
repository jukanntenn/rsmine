use serde::Deserialize;

/// Request wrapper for updating a member
#[derive(Debug, Deserialize)]
pub struct UpdateMemberRequest {
    pub membership: UpdateMemberDto,
}

/// Data transfer object for updating member roles
#[derive(Debug, Deserialize, Clone)]
pub struct UpdateMemberDto {
    /// Role IDs to assign to the member (replaces existing roles)
    pub role_ids: Vec<i32>,
}

impl UpdateMemberDto {
    /// Validate the DTO
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.role_ids.is_empty() {
            errors.push("At least one role is required".to_string());
        }

        if self.role_ids.iter().any(|&id| id <= 0) {
            errors.push("All role IDs must be positive integers".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_valid_dto() {
        let dto = UpdateMemberDto {
            role_ids: vec![1, 2, 3],
        };
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn test_validate_single_role() {
        let dto = UpdateMemberDto { role_ids: vec![1] };
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn test_validate_empty_roles() {
        let dto = UpdateMemberDto { role_ids: vec![] };
        assert!(dto.validate().is_err());
        let errors = dto.validate().unwrap_err();
        assert!(errors.iter().any(|e| e.contains("At least one role")));
    }

    #[test]
    fn test_validate_invalid_role_id() {
        let dto = UpdateMemberDto {
            role_ids: vec![0, -1],
        };
        assert!(dto.validate().is_err());
        let errors = dto.validate().unwrap_err();
        assert!(errors.iter().any(|e| e.contains("role IDs")));
    }
}
