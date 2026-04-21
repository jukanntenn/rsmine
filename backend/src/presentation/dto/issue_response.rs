use crate::application::use_cases::project::NamedId;
use crate::application::use_cases::{
    AttachmentResponse, ChildIssueSummary, GetIssueResponse, IssueAttachmentItem,
    IssueAttachmentListResponse, IssueItem, IssueListResponse, JournalDetailResponse,
    JournalResponse, ListJournalsResponse, RelationResponse, StatusInfo,
};
use serde::Serialize;

/// JSON response for issue list endpoint
#[derive(Debug, Serialize)]
pub struct IssueListJsonResponse {
    pub issues: Vec<IssueItemJsonResponse>,
    pub total_count: u32,
    pub offset: u32,
    pub limit: u32,
}

/// JSON representation of a single issue in list responses
#[derive(Debug, Serialize)]
pub struct IssueItemJsonResponse {
    pub id: i32,
    pub project: NamedIdJsonResponse,
    pub tracker: NamedIdJsonResponse,
    pub status: StatusInfoJsonResponse,
    pub priority: NamedIdJsonResponse,
    pub author: NamedIdJsonResponse,
    pub assigned_to: Option<NamedIdJsonResponse>,
    pub subject: String,
    pub description: String,
    pub start_date: Option<String>,
    pub due_date: Option<String>,
    pub done_ratio: i32,
    pub is_private: bool,
    pub estimated_hours: Option<f64>,
    pub total_estimated_hours: Option<f64>,
    pub spent_hours: f64,
    pub total_spent_hours: f64,
    pub created_on: Option<String>,
    pub updated_on: Option<String>,
    pub closed_on: Option<String>,
}

/// JSON representation of a named ID (e.g., project, tracker, priority)
#[derive(Debug, Serialize)]
pub struct NamedIdJsonResponse {
    pub id: i32,
    pub name: String,
}

/// JSON representation of status info with is_closed flag
#[derive(Debug, Serialize)]
pub struct StatusInfoJsonResponse {
    pub id: i32,
    pub name: String,
    pub is_closed: bool,
}

impl From<IssueListResponse> for IssueListJsonResponse {
    fn from(response: IssueListResponse) -> Self {
        Self {
            issues: response
                .issues
                .into_iter()
                .map(IssueItemJsonResponse::from)
                .collect(),
            total_count: response.total_count,
            offset: response.offset,
            limit: response.limit,
        }
    }
}

impl From<IssueItem> for IssueItemJsonResponse {
    fn from(item: IssueItem) -> Self {
        Self {
            id: item.id,
            project: NamedIdJsonResponse::from(item.project),
            tracker: NamedIdJsonResponse::from(item.tracker),
            status: StatusInfoJsonResponse::from(item.status),
            priority: NamedIdJsonResponse::from(item.priority),
            author: NamedIdJsonResponse::from(item.author),
            assigned_to: item.assigned_to.map(NamedIdJsonResponse::from),
            subject: item.subject,
            description: item.description,
            start_date: item.start_date,
            due_date: item.due_date,
            done_ratio: item.done_ratio,
            is_private: item.is_private,
            estimated_hours: item.estimated_hours,
            total_estimated_hours: item.total_estimated_hours,
            spent_hours: item.spent_hours,
            total_spent_hours: item.total_spent_hours,
            created_on: item.created_on,
            updated_on: item.updated_on,
            closed_on: item.closed_on,
        }
    }
}

impl From<NamedId> for NamedIdJsonResponse {
    fn from(named_id: NamedId) -> Self {
        Self {
            id: named_id.id,
            name: named_id.name,
        }
    }
}

impl From<StatusInfo> for StatusInfoJsonResponse {
    fn from(status: StatusInfo) -> Self {
        Self {
            id: status.id,
            name: status.name,
            is_closed: status.is_closed,
        }
    }
}

// ==================== Get Issue Response DTOs ====================

/// JSON response wrapper for single issue endpoint
#[derive(Debug, Serialize)]
pub struct GetIssueJsonResponse {
    pub issue: IssueDetailJsonResponse,
}

/// JSON representation of issue detail
#[derive(Debug, Serialize)]
pub struct IssueDetailJsonResponse {
    pub id: i32,
    pub project: NamedIdJsonResponse,
    pub tracker: NamedIdJsonResponse,
    pub status: StatusInfoJsonResponse,
    pub priority: NamedIdJsonResponse,
    pub author: NamedIdJsonResponse,
    pub assigned_to: Option<NamedIdJsonResponse>,
    pub subject: String,
    pub description: Option<String>,
    pub start_date: Option<String>,
    pub due_date: Option<String>,
    pub done_ratio: i32,
    pub is_private: bool,
    pub estimated_hours: Option<f64>,
    pub total_estimated_hours: Option<f64>,
    pub spent_hours: f64,
    pub total_spent_hours: f64,
    pub created_on: Option<String>,
    pub updated_on: Option<String>,
    pub closed_on: Option<String>,

    // Optional includes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<ChildIssueJsonResponse>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AttachmentJsonResponse>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journals: Option<Vec<JournalJsonResponse>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relations: Option<Vec<RelationJsonResponse>>,
}

/// JSON representation of child issue summary
#[derive(Debug, Serialize)]
pub struct ChildIssueJsonResponse {
    pub id: i32,
    pub tracker: NamedIdJsonResponse,
    pub subject: String,
}

/// JSON representation of journal detail
#[derive(Debug, Serialize)]
pub struct JournalDetailJsonResponse {
    pub property: String,
    pub name: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
}

/// JSON representation of journal entry
#[derive(Debug, Serialize)]
pub struct JournalJsonResponse {
    pub id: i32,
    pub user: NamedIdJsonResponse,
    pub notes: Option<String>,
    pub created_on: String,
    pub updated_on: Option<String>,
    pub private_notes: bool,
    pub details: Vec<JournalDetailJsonResponse>,
}

/// JSON representation of attachment
#[derive(Debug, Serialize)]
pub struct AttachmentJsonResponse {
    pub id: i32,
    pub filename: String,
    pub filesize: i32,
    pub content_type: Option<String>,
    pub description: Option<String>,
    pub content_url: String,
    pub thumbnail_url: Option<String>,
    pub author: NamedIdJsonResponse,
    pub created_on: Option<String>,
}

/// JSON representation of issue relation
#[derive(Debug, Serialize)]
pub struct RelationJsonResponse {
    pub id: i32,
    pub issue_id: i32,
    pub issue_to_id: i32,
    pub relation_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<i32>,
}

impl From<GetIssueResponse> for GetIssueJsonResponse {
    fn from(response: GetIssueResponse) -> Self {
        Self {
            issue: IssueDetailJsonResponse::from(response),
        }
    }
}

impl From<GetIssueResponse> for IssueDetailJsonResponse {
    fn from(response: GetIssueResponse) -> Self {
        Self {
            id: response.id,
            project: NamedIdJsonResponse::from(response.project),
            tracker: NamedIdJsonResponse::from(response.tracker),
            status: StatusInfoJsonResponse::from(response.status),
            priority: NamedIdJsonResponse::from(response.priority),
            author: NamedIdJsonResponse::from(response.author),
            assigned_to: response.assigned_to.map(NamedIdJsonResponse::from),
            subject: response.subject,
            description: response.description,
            start_date: response.start_date,
            due_date: response.due_date,
            done_ratio: response.done_ratio,
            is_private: response.is_private,
            estimated_hours: response.estimated_hours,
            total_estimated_hours: response.total_estimated_hours,
            spent_hours: response.spent_hours,
            total_spent_hours: response.total_spent_hours,
            created_on: response.created_on,
            updated_on: response.updated_on,
            closed_on: response.closed_on,
            children: response
                .children
                .map(|c| c.into_iter().map(ChildIssueJsonResponse::from).collect()),
            attachments: response
                .attachments
                .map(|a| a.into_iter().map(AttachmentJsonResponse::from).collect()),
            journals: response
                .journals
                .map(|j| j.into_iter().map(JournalJsonResponse::from).collect()),
            relations: response
                .relations
                .map(|r| r.into_iter().map(RelationJsonResponse::from).collect()),
        }
    }
}

impl From<ChildIssueSummary> for ChildIssueJsonResponse {
    fn from(summary: ChildIssueSummary) -> Self {
        Self {
            id: summary.id,
            tracker: NamedIdJsonResponse::from(summary.tracker),
            subject: summary.subject,
        }
    }
}

impl From<JournalDetailResponse> for JournalDetailJsonResponse {
    fn from(detail: JournalDetailResponse) -> Self {
        Self {
            property: detail.property,
            name: detail.name,
            old_value: detail.old_value,
            new_value: detail.new_value,
        }
    }
}

impl From<JournalResponse> for JournalJsonResponse {
    fn from(journal: JournalResponse) -> Self {
        Self {
            id: journal.id,
            user: NamedIdJsonResponse::from(journal.user),
            notes: journal.notes,
            created_on: journal.created_on,
            updated_on: journal.updated_on,
            private_notes: journal.private_notes,
            details: journal
                .details
                .into_iter()
                .map(JournalDetailJsonResponse::from)
                .collect(),
        }
    }
}

impl From<AttachmentResponse> for AttachmentJsonResponse {
    fn from(attachment: AttachmentResponse) -> Self {
        Self {
            id: attachment.id,
            filename: attachment.filename,
            filesize: attachment.filesize,
            content_type: attachment.content_type,
            description: attachment.description,
            content_url: attachment.content_url,
            thumbnail_url: attachment.thumbnail_url,
            author: NamedIdJsonResponse::from(attachment.author),
            created_on: attachment.created_on,
        }
    }
}

impl From<RelationResponse> for RelationJsonResponse {
    fn from(relation: RelationResponse) -> Self {
        Self {
            id: relation.id,
            issue_id: relation.issue_id,
            issue_to_id: relation.issue_to_id,
            relation_type: relation.relation_type,
            delay: relation.delay,
        }
    }
}

// ==================== Create Issue Response DTOs ====================

use crate::application::use_cases::CreateIssueResponse;

/// JSON response wrapper for create issue endpoint
#[derive(Debug, Serialize)]
pub struct CreateIssueJsonResponse {
    pub issue: IssueDetailJsonResponse,
}

impl From<CreateIssueResponse> for CreateIssueJsonResponse {
    fn from(response: CreateIssueResponse) -> Self {
        Self {
            issue: IssueDetailJsonResponse::from(response.issue),
        }
    }
}

// ==================== Update Issue Response DTOs ====================

use crate::application::use_cases::UpdateIssueResponse;

/// JSON response wrapper for update issue endpoint
#[derive(Debug, Serialize)]
pub struct UpdateIssueJsonResponse {
    pub issue: IssueDetailJsonResponse,
}

impl From<UpdateIssueResponse> for UpdateIssueJsonResponse {
    fn from(response: UpdateIssueResponse) -> Self {
        Self {
            issue: IssueDetailJsonResponse::from(response.issue),
        }
    }
}

// ==================== List Journals Response DTOs ====================

/// JSON response wrapper for list journals endpoint
#[derive(Debug, Serialize)]
pub struct ListJournalsJsonResponse {
    pub journals: Vec<JournalJsonResponse>,
}

impl From<ListJournalsResponse> for ListJournalsJsonResponse {
    fn from(response: ListJournalsResponse) -> Self {
        Self {
            journals: response
                .journals
                .into_iter()
                .map(JournalJsonResponse::from)
                .collect(),
        }
    }
}

// ==================== List Issue Attachments Response DTOs ====================

/// JSON response wrapper for list issue attachments endpoint
#[derive(Debug, Serialize)]
pub struct IssueAttachmentListJsonResponse {
    pub attachments: Vec<IssueAttachmentItemJsonResponse>,
}

/// JSON representation of attachment item in issue attachments list
#[derive(Debug, Serialize)]
pub struct IssueAttachmentItemJsonResponse {
    pub id: i32,
    pub filename: String,
    pub filesize: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub author: NamedIdJsonResponse,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,
}

impl From<IssueAttachmentListResponse> for IssueAttachmentListJsonResponse {
    fn from(response: IssueAttachmentListResponse) -> Self {
        Self {
            attachments: response
                .attachments
                .into_iter()
                .map(IssueAttachmentItemJsonResponse::from)
                .collect(),
        }
    }
}

impl From<IssueAttachmentItem> for IssueAttachmentItemJsonResponse {
    fn from(item: IssueAttachmentItem) -> Self {
        Self {
            id: item.id,
            filename: item.filename,
            filesize: item.filesize,
            content_type: item.content_type,
            description: item.description,
            author: NamedIdJsonResponse::from(item.author),
            created_on: item.created_on,
        }
    }
}
