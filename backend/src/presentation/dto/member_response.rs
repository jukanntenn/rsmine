use crate::application::use_cases::{
    MemberListResponse, MemberNamedId, MemberRoleItem, MembershipItem, MembershipResponse,
};
use serde::Serialize;

/// JSON response for member list endpoint
#[derive(Debug, Serialize)]
pub struct MemberListJsonResponse {
    pub memberships: Vec<MembershipJsonItem>,
    pub total_count: u32,
    pub offset: u32,
    pub limit: u32,
}

/// JSON representation of a single membership in list responses
#[derive(Debug, Serialize)]
pub struct MembershipJsonItem {
    pub id: i32,
    pub project: ProjectJsonInfo,
    pub user: UserJsonInfo,
    pub roles: Vec<RoleJsonInfo>,
}

/// JSON representation of project info in membership
#[derive(Debug, Serialize)]
pub struct ProjectJsonInfo {
    pub id: i32,
    pub name: String,
}

/// JSON representation of user info in membership
#[derive(Debug, Serialize)]
pub struct UserJsonInfo {
    pub id: i32,
    pub name: String,
}

/// JSON representation of role info in membership
#[derive(Debug, Serialize)]
pub struct RoleJsonInfo {
    pub id: i32,
    pub name: String,
}

/// JSON response for create member endpoint
#[derive(Debug, Serialize)]
pub struct CreateMemberJsonResponse {
    pub membership: MembershipDetailJson,
}

/// JSON response for get member endpoint
#[derive(Debug, Serialize)]
pub struct GetMemberJsonResponse {
    pub membership: MembershipDetailJson,
}

/// JSON response for update member endpoint
#[derive(Debug, Serialize)]
pub struct UpdateMemberJsonResponse {
    pub membership: MembershipDetailJson,
}

/// JSON representation of a single membership detail
#[derive(Debug, Serialize)]
pub struct MembershipDetailJson {
    pub id: i32,
    pub project: NamedIdJson,
    pub user: NamedIdJson,
    pub roles: Vec<NamedIdJson>,
}

/// JSON representation of a named ID
#[derive(Debug, Serialize)]
pub struct NamedIdJson {
    pub id: i32,
    pub name: String,
}

impl From<MemberListResponse> for MemberListJsonResponse {
    fn from(response: MemberListResponse) -> Self {
        Self {
            memberships: response
                .memberships
                .into_iter()
                .map(MembershipJsonItem::from)
                .collect(),
            total_count: response.total_count,
            offset: response.offset,
            limit: response.limit,
        }
    }
}

impl From<MembershipItem> for MembershipJsonItem {
    fn from(item: MembershipItem) -> Self {
        Self {
            id: item.id,
            project: ProjectJsonInfo {
                id: item.project.id,
                name: item.project.name,
            },
            user: UserJsonInfo {
                id: item.user.id,
                name: item.user.name,
            },
            roles: item.roles.into_iter().map(RoleJsonInfo::from).collect(),
        }
    }
}

impl From<MemberRoleItem> for RoleJsonInfo {
    fn from(item: MemberRoleItem) -> Self {
        Self {
            id: item.id,
            name: item.name,
        }
    }
}

impl From<MembershipResponse> for CreateMemberJsonResponse {
    fn from(response: MembershipResponse) -> Self {
        Self {
            membership: MembershipDetailJson::from(response),
        }
    }
}

impl From<MembershipResponse> for GetMemberJsonResponse {
    fn from(response: MembershipResponse) -> Self {
        Self {
            membership: MembershipDetailJson::from(response),
        }
    }
}

impl From<MembershipResponse> for UpdateMemberJsonResponse {
    fn from(response: MembershipResponse) -> Self {
        Self {
            membership: MembershipDetailJson::from(response),
        }
    }
}

impl From<MembershipResponse> for MembershipDetailJson {
    fn from(response: MembershipResponse) -> Self {
        Self {
            id: response.id,
            project: NamedIdJson::from(response.project),
            user: NamedIdJson::from(response.user),
            roles: response.roles.into_iter().map(NamedIdJson::from).collect(),
        }
    }
}

impl From<MemberNamedId> for NamedIdJson {
    fn from(item: MemberNamedId) -> Self {
        Self {
            id: item.id,
            name: item.name,
        }
    }
}
