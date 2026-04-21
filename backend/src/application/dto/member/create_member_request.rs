use serde::Deserialize;

/// Request wrapper for creating a member
#[derive(Debug, Deserialize)]
pub struct CreateMemberRequest {
    pub membership: CreateMemberDto,
}

/// Data transfer object for creating a new member
#[derive(Debug, Deserialize, Clone)]
pub struct CreateMemberDto {
    /// User ID to add as member
    pub user_id: i32,
    /// Role IDs to assign to the member
    pub role_ids: Vec<i32>,
}

impl CreateMemberDto {
    /// Validate the DTO
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.user_id <= 0 {
            errors.push("User ID must be a positive integer".to_string());
        }

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
        let dto = CreateMemberDto {
            user_id: 1,
            role_ids: vec![1, 2],
        };
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn test_validate_empty_roles() {
        let dto = CreateMemberDto {
            user_id: 1,
            role_ids: vec![],
        };
        assert!(dto.validate().is_err());
        let errors = dto.validate().unwrap_err();
        assert!(errors.iter().any(|e| e.contains("At least one role")));
    }

    #[test]
    fn test_validate_invalid_user_id() {
        let dto = CreateMemberDto {
            user_id: 0,
            role_ids: vec![1],
        };
        assert!(dto.validate().is_err());
        let errors = dto.validate().unwrap_err();
        assert!(errors.iter().any(|e| e.contains("User ID")));
    }

    #[test]
    fn test_validate_invalid_role_id() {
        let dto = CreateMemberDto {
            user_id: 1,
            role_ids: vec![0, -1],
        };
        assert!(dto.validate().is_err());
        let errors = dto.validate().unwrap_err();
        assert!(errors.iter().any(|e| e.contains("role IDs")));
    }
}
