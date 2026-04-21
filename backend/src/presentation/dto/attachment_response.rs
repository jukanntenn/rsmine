use crate::application::use_cases::{
    project::NamedId, AttachmentDetailResponse, GetAttachmentResponse,
    UpdateAttachmentDetailResponse, UpdateAttachmentResponse,
};
use serde::Serialize;

/// JSON response for attachment metadata endpoint
#[derive(Debug, Serialize)]
pub struct AttachmentMetadataJsonResponse {
    pub attachment: AttachmentDetailJsonResponse,
}

/// JSON representation of attachment detail
#[derive(Debug, Serialize)]
pub struct AttachmentDetailJsonResponse {
    pub id: i32,
    pub filename: String,
    pub filesize: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub content_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    pub author: NamedIdJsonResponse,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,
}

/// JSON representation of a named ID (for author, etc.)
#[derive(Debug, Serialize)]
pub struct NamedIdJsonResponse {
    pub id: i32,
    pub name: String,
}

/// JSON response for update attachment endpoint
#[derive(Debug, Serialize)]
pub struct UpdateAttachmentJsonResponse {
    pub attachment: UpdateAttachmentDetailJsonResponse,
}

/// JSON representation of attachment detail for update response
#[derive(Debug, Serialize)]
pub struct UpdateAttachmentDetailJsonResponse {
    pub id: i32,
    pub filename: String,
    pub filesize: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub content_url: String,
    pub author: NamedIdJsonResponse,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,
}

impl From<NamedId> for NamedIdJsonResponse {
    fn from(named_id: NamedId) -> Self {
        Self {
            id: named_id.id,
            name: named_id.name,
        }
    }
}

impl From<GetAttachmentResponse> for AttachmentMetadataJsonResponse {
    fn from(response: GetAttachmentResponse) -> Self {
        Self {
            attachment: AttachmentDetailJsonResponse::from(response.attachment),
        }
    }
}

impl From<AttachmentDetailResponse> for AttachmentDetailJsonResponse {
    fn from(detail: AttachmentDetailResponse) -> Self {
        Self {
            id: detail.id,
            filename: detail.filename,
            filesize: detail.filesize,
            content_type: detail.content_type,
            description: detail.description,
            content_url: detail.content_url,
            thumbnail_url: detail.thumbnail_url,
            author: NamedIdJsonResponse::from(detail.author),
            created_on: detail.created_on,
        }
    }
}

impl From<UpdateAttachmentResponse> for UpdateAttachmentJsonResponse {
    fn from(response: UpdateAttachmentResponse) -> Self {
        Self {
            attachment: UpdateAttachmentDetailJsonResponse::from(response.attachment),
        }
    }
}

impl From<UpdateAttachmentDetailResponse> for UpdateAttachmentDetailJsonResponse {
    fn from(detail: UpdateAttachmentDetailResponse) -> Self {
        Self {
            id: detail.id,
            filename: detail.filename,
            filesize: detail.filesize,
            content_type: detail.content_type,
            description: detail.description,
            content_url: detail.content_url,
            author: NamedIdJsonResponse::from(detail.author),
            created_on: detail.created_on,
        }
    }
}
