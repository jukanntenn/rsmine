use crate::application::errors::ApplicationError;
use crate::domain::repositories::{IssueCategoryRepository, MemberRepository};
use std::sync::Arc;

/// Use case for deleting an issue category
pub struct DeleteCategoryUseCase<C: IssueCategoryRepository, M: MemberRepository> {
    category_repo: Arc<C>,
    member_repo: Arc<M>,
}

impl<C, M> DeleteCategoryUseCase<C, M>
where
    C: IssueCategoryRepository,
    M: MemberRepository,
{
    pub fn new(category_repo: Arc<C>, member_repo: Arc<M>) -> Self {
        Self {
            category_repo,
            member_repo,
        }
    }

    /// Execute the use case
    ///
    /// Deletes an issue category.
    /// If the category has issues, requires reassign_to_id to be specified.
    /// Requires manage_categories permission.
    pub async fn execute(
        &self,
        category_id: i32,
        reassign_to_id: Option<i32>,
        current_user_id: i32,
        is_admin: bool,
    ) -> Result<(), ApplicationError> {
        // 1. Get category
        let category = self
            .category_repo
            .find_by_id(category_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound("Category not found".into()))?;

        // 2. Check permission - user must be a member with manage_categories permission (or admin)
        if !is_admin {
            let member = self
                .member_repo
                .find_by_project_and_user(category.project_id, current_user_id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?
                .ok_or_else(|| {
                    ApplicationError::Forbidden(
                        "You don't have permission to manage categories in this project".into(),
                    )
                })?;

            // Check if user has manage_categories permission through any of their roles
            let has_permission = member.roles.iter().any(|rwi| {
                // Roles 3 (Manager) and 4 (Developer) typically have manage_categories
                // Role 3 = Manager, Role 4 = Developer
                rwi.role.id == 3 || rwi.role.id == 4
            });

            if !has_permission {
                return Err(ApplicationError::Forbidden(
                    "You don't have permission to manage categories in this project".into(),
                ));
            }
        }

        // 3. Check for issues in category
        let issue_count = self
            .category_repo
            .count_issues(category_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        if issue_count > 0 {
            if let Some(to_id) = reassign_to_id {
                // Validate reassign target exists in same project
                let target = self
                    .category_repo
                    .find_by_id(to_id)
                    .await
                    .map_err(|e| ApplicationError::Internal(e.to_string()))?
                    .ok_or_else(|| {
                        ApplicationError::Validation("Target category not found".into())
                    })?;

                if target.project_id != category.project_id {
                    return Err(ApplicationError::Validation(
                        "Target category must be in the same project".into(),
                    ));
                }

                // Reassign issues
                self.category_repo
                    .reassign_issues(category_id, to_id)
                    .await
                    .map_err(|e| ApplicationError::Internal(e.to_string()))?;
            } else {
                return Err(ApplicationError::Validation(format!(
                    "Category has {} issues. Specify reassign_to_id to reassign them.",
                    issue_count
                )));
            }
        }

        // 4. Delete category
        self.category_repo
            .delete(category_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        Ok(())
    }
}
