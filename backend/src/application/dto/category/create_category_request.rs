use crate::domain::repositories::NewIssueCategory;
use serde::Deserialize;
use validator::Validate;

/// Request wrapper for creating an issue category
#[derive(Debug, Deserialize, Validate)]
pub struct CreateCategoryRequest {
    pub issue_category: CreateCategoryDto,
}

/// DTO for creating an issue category
#[derive(Debug, Deserialize, Validate)]
pub struct CreateCategoryDto {
    /// Name of the category (required, 1-60 characters)
    #[validate(length(
        min = 1,
        max = 60,
        message = "Name must be between 1 and 60 characters"
    ))]
    pub name: String,

    /// Optional default assignee for issues in this category
    pub assigned_to_id: Option<i32>,
}

impl CreateCategoryDto {
    /// Convert to domain NewIssueCategory
    pub fn into_new_category(self, project_id: i32) -> NewIssueCategory {
        NewIssueCategory {
            project_id,
            name: self.name,
            assigned_to_id: self.assigned_to_id,
        }
    }
}
