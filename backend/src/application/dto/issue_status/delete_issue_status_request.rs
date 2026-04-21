use serde::{Deserialize, Serialize};

/// Request for deleting an issue status with optional reassignment
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteIssueStatusRequest {
    /// ID of the status to reassign issues to (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reassign_to_id: Option<i32>,
}
