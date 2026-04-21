use crate::domain::repositories::IssueCategoryUpdate;
use serde::Deserialize;
use validator::Validate;

/// Request wrapper for updating an issue category
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCategoryRequest {
    pub issue_category: UpdateCategoryDto,
}

/// DTO for updating an issue category
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCategoryDto {
    /// Name of the category (optional, 1-60 characters if provided)
    #[validate(length(
        min = 1,
        max = 60,
        message = "Name must be between 1 and 60 characters"
    ))]
    pub name: Option<String>,

    /// Optional default assignee for issues in this category (None to clear)
    pub assigned_to_id: Option<Option<i32>>,
}

impl UpdateCategoryDto {
    /// Convert to domain IssueCategoryUpdate
    pub fn into_update(self) -> IssueCategoryUpdate {
        IssueCategoryUpdate {
            name: self.name,
            assigned_to_id: self.assigned_to_id,
        }
    }
}
