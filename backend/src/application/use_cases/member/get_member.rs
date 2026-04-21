use super::{MemberNamedId, MembershipResponse};
use crate::application::errors::ApplicationError;
use crate::domain::entities::PROJECT_STATUS_ARCHIVED;
use crate::domain::repositories::{MemberRepository, ProjectRepository};
use crate::presentation::middleware::CurrentUser;
use std::sync::Arc;

pub struct GetMemberUseCase<M: MemberRepository, P: ProjectRepository> {
    member_repo: Arc<M>,
    project_repo: Arc<P>,
}

impl<M, P> GetMemberUseCase<M, P>
where
    M: MemberRepository,
    P: ProjectRepository,
{
    pub fn new(member_repo: Arc<M>, project_repo: Arc<P>) -> Self {
        Self {
            member_repo,
            project_repo,
        }
    }

    pub async fn execute(
        &self,
        membership_id: i32,
        current_user: &CurrentUser,
    ) -> Result<MembershipResponse, ApplicationError> {
        let membership = self
            .member_repo
            .find_by_id(membership_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound("Membership not found".into()))?;

        let project = self
            .project_repo
            .find_by_id(membership.member.project_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound("Project not found".into()))?;

        if project.status == PROJECT_STATUS_ARCHIVED && !current_user.admin {
            return Err(ApplicationError::Forbidden("Project is archived".into()));
        }

        let can_view = current_user.admin
            || project.is_public
            || self
                .member_repo
                .is_member(project.id, current_user.id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        if !can_view {
            return Err(ApplicationError::Forbidden(
                "You don't have permission to view this membership".into(),
            ));
        }

        Ok(MembershipResponse {
            id: membership.member.id,
            project: MemberNamedId {
                id: project.id,
                name: project.name,
            },
            user: MemberNamedId {
                id: membership.user.id,
                name: membership.user.full_name(),
            },
            roles: membership
                .roles
                .into_iter()
                .map(|role| MemberNamedId {
                    id: role.role.id,
                    name: role.role.name,
                })
                .collect(),
        })
    }
}
