use crate::application::errors::ApplicationError;
use crate::domain::entities::PROJECT_STATUS_ARCHIVED;
use crate::domain::repositories::{MemberRepository, ProjectRepository};
use crate::presentation::middleware::CurrentUser;
use std::sync::Arc;

/// Role item in membership response
#[derive(Debug, Clone)]
pub struct MemberRoleItem {
    pub id: i32,
    pub name: String,
}

/// Membership item in list response
#[derive(Debug, Clone)]
pub struct MembershipItem {
    pub id: i32,
    pub project: ProjectInfo,
    pub user: UserInfo,
    pub roles: Vec<MemberRoleItem>,
}

/// Project info in membership
#[derive(Debug, Clone)]
pub struct ProjectInfo {
    pub id: i32,
    pub name: String,
}

/// User info in membership
#[derive(Debug, Clone)]
pub struct UserInfo {
    pub id: i32,
    pub name: String,
}

/// Response for member list endpoint
#[derive(Debug, Clone)]
pub struct MemberListResponse {
    pub memberships: Vec<MembershipItem>,
    pub total_count: u32,
    pub offset: u32,
    pub limit: u32,
}

/// Use case for listing members of a project
pub struct ListMembersUseCase<P: ProjectRepository, M: MemberRepository> {
    project_repo: Arc<P>,
    member_repo: Arc<M>,
}

impl<P: ProjectRepository, M: MemberRepository> ListMembersUseCase<P, M> {
    pub fn new(project_repo: Arc<P>, member_repo: Arc<M>) -> Self {
        Self {
            project_repo,
            member_repo,
        }
    }

    /// Execute the use case
    ///
    /// Visibility rules:
    /// - Admin users can see all project members
    /// - For public projects: all logged-in users can see members
    /// - For private projects: only members can see other members
    /// - Archived projects: only admins can view
    pub async fn execute(
        &self,
        project_id: i32,
        current_user: &CurrentUser,
    ) -> Result<MemberListResponse, ApplicationError> {
        // 1. Check project exists
        let project = self
            .project_repo
            .find_by_id(project_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound("Project not found".into()))?;

        // 2. Check archived status - only admins can view archived projects
        if project.status == PROJECT_STATUS_ARCHIVED && !current_user.admin {
            return Err(ApplicationError::Forbidden("Project is archived".into()));
        }

        // 3. Check visibility
        let can_view = current_user.admin
            || project.is_public
            || self
                .member_repo
                .is_member(project_id, current_user.id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        if !can_view {
            return Err(ApplicationError::Forbidden(
                "You don't have permission to view this project's members".into(),
            ));
        }

        // 4. Get members with roles
        let members = self
            .member_repo
            .find_by_project(project_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // 5. Build response
        let project_info = ProjectInfo {
            id: project.id,
            name: project.name,
        };

        let memberships: Vec<MembershipItem> = members
            .into_iter()
            .map(|m| MembershipItem {
                id: m.member.id,
                project: project_info.clone(),
                user: UserInfo {
                    id: m.user.id,
                    name: m.user.full_name(),
                },
                roles: m
                    .roles
                    .into_iter()
                    .map(|r| MemberRoleItem {
                        id: r.role.id,
                        name: r.role.name,
                    })
                    .collect(),
            })
            .collect();

        let total_count = memberships.len() as u32;

        Ok(MemberListResponse {
            memberships,
            total_count,
            offset: 0,
            limit: 25,
        })
    }
}
