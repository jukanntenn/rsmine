use crate::application::errors::ApplicationError;
use crate::domain::entities::IssueCategory;
use crate::domain::repositories::{
    IssueCategoryRepository, MemberRepository, ProjectRepository, UserRepository,
};
use std::sync::Arc;

/// Named ID reference for API responses
#[derive(Debug, Clone)]
pub struct NamedId {
    pub id: i32,
    pub name: String,
}

/// Category item in list response
#[derive(Debug, Clone)]
pub struct CategoryItem {
    pub id: i32,
    pub name: String,
    pub project: NamedId,
    pub assigned_to: Option<NamedId>,
}

impl CategoryItem {
    pub fn from_category(
        category: &IssueCategory,
        project_name: &str,
        assignee: Option<NamedId>,
    ) -> Self {
        Self {
            id: category.id,
            name: category.name.clone(),
            project: NamedId {
                id: category.project_id,
                name: project_name.to_string(),
            },
            assigned_to: assignee,
        }
    }
}

/// Response for list categories endpoint
#[derive(Debug, Clone)]
pub struct CategoryListResponse {
    pub issue_categories: Vec<CategoryItem>,
    pub total_count: u32,
}

/// Use case for listing issue categories for a project
pub struct ListCategoriesUseCase<
    C: IssueCategoryRepository,
    P: ProjectRepository,
    U: UserRepository,
    M: MemberRepository,
> {
    category_repo: Arc<C>,
    project_repo: Arc<P>,
    user_repo: Arc<U>,
    member_repo: Arc<M>,
}

impl<C, P, U, M> ListCategoriesUseCase<C, P, U, M>
where
    C: IssueCategoryRepository,
    P: ProjectRepository,
    U: UserRepository,
    M: MemberRepository,
{
    pub fn new(
        category_repo: Arc<C>,
        project_repo: Arc<P>,
        user_repo: Arc<U>,
        member_repo: Arc<M>,
    ) -> Self {
        Self {
            category_repo,
            project_repo,
            user_repo,
            member_repo,
        }
    }

    /// Execute the use case
    ///
    /// Returns all issue categories for a project.
    /// Requires view_project permission.
    pub async fn execute(
        &self,
        project_id: i32,
        current_user_id: i32,
        is_admin: bool,
    ) -> Result<CategoryListResponse, ApplicationError> {
        // 1. Check project exists
        let project = self
            .project_repo
            .find_by_id(project_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound("Project not found".into()))?;

        // 2. Check permission - user must be a member or project must be public (for non-admins)
        if !is_admin {
            let is_member = self
                .member_repo
                .find_by_project_and_user(project_id, current_user_id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?
                .is_some();

            if !is_member && !project.is_public {
                return Err(ApplicationError::Forbidden(
                    "You don't have permission to view this project".into(),
                ));
            }
        }

        // 3. Get categories for the project
        let categories = self
            .category_repo
            .find_by_project(project_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // 4. Build response with assignee info
        let mut category_items = Vec::new();
        for category in categories {
            let assigned_to = if let Some(user_id) = category.assigned_to_id {
                self.user_repo
                    .find_by_id(user_id)
                    .await
                    .map_err(|e| ApplicationError::Internal(e.to_string()))?
                    .map(|u| NamedId {
                        id: u.id,
                        name: format!("{} {}", u.firstname, u.lastname),
                    })
            } else {
                None
            };

            category_items.push(CategoryItem::from_category(
                &category,
                &project.name,
                assigned_to,
            ));
        }

        let total_count = category_items.len() as u32;

        Ok(CategoryListResponse {
            issue_categories: category_items,
            total_count,
        })
    }
}
