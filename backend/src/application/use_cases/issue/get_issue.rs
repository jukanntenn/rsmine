use super::StatusInfo;
use crate::application::errors::ApplicationError;
use crate::application::use_cases::project::NamedId;
use crate::domain::entities::{Issue, JournalDetail};
use crate::domain::repositories::{
    AttachmentRepository, EnumerationRepository, IssueRelationRepository, IssueRepository,
    IssueStatusRepository, JournalRepository, MemberRepository, ProjectRepository,
    TrackerRepository, UserRepository,
};
use crate::presentation::middleware::CurrentUser;
use std::sync::Arc;

/// Include options for the get_issue endpoint
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IncludeOption {
    Attachments,
    Journals,
    Relations,
    Children,
    Watchers,
    AllowedStatuses,
}

impl IncludeOption {
    /// Parse include string into a vector of IncludeOption
    pub fn parse(include: Option<&str>) -> Vec<IncludeOption> {
        match include {
            Some(s) if !s.is_empty() => s
                .split(',')
                .filter_map(|part| match part.trim() {
                    "attachments" => Some(IncludeOption::Attachments),
                    "journals" => Some(IncludeOption::Journals),
                    "relations" => Some(IncludeOption::Relations),
                    "children" => Some(IncludeOption::Children),
                    "watchers" => Some(IncludeOption::Watchers),
                    "allowed_statuses" => Some(IncludeOption::AllowedStatuses),
                    _ => None,
                })
                .collect(),
            _ => Vec::new(),
        }
    }
}

/// Child issue summary for response
#[derive(Debug, Clone)]
pub struct ChildIssueSummary {
    pub id: i32,
    pub tracker: NamedId,
    pub subject: String,
}

/// Journal detail response
#[derive(Debug, Clone)]
pub struct JournalDetailResponse {
    pub property: String,
    pub name: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
}

impl From<JournalDetail> for JournalDetailResponse {
    fn from(detail: JournalDetail) -> Self {
        Self {
            property: detail.property.clone(),
            name: detail.prop_key.clone(),
            old_value: detail.old_value,
            new_value: detail.value,
        }
    }
}

/// Journal response for the issue detail
#[derive(Debug, Clone)]
pub struct JournalResponse {
    pub id: i32,
    pub user: NamedId,
    pub notes: Option<String>,
    pub created_on: String,
    pub updated_on: Option<String>,
    pub private_notes: bool,
    pub details: Vec<JournalDetailResponse>,
}

/// Attachment response for the issue detail
#[derive(Debug, Clone)]
pub struct AttachmentResponse {
    pub id: i32,
    pub filename: String,
    pub filesize: i32,
    pub content_type: Option<String>,
    pub description: Option<String>,
    pub content_url: String,
    pub thumbnail_url: Option<String>,
    pub author: NamedId,
    pub created_on: Option<String>,
}

/// Relation response for the issue detail
#[derive(Debug, Clone)]
pub struct RelationResponse {
    pub id: i32,
    pub issue_id: i32,
    pub issue_to_id: i32,
    pub relation_type: String,
    pub delay: Option<i32>,
}

/// Issue detail response
#[derive(Debug, Clone)]
pub struct GetIssueResponse {
    pub id: i32,
    pub project: NamedId,
    pub tracker: NamedId,
    pub status: StatusInfo,
    pub priority: NamedId,
    pub author: NamedId,
    pub assigned_to: Option<NamedId>,
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
    pub children: Option<Vec<ChildIssueSummary>>,
    pub attachments: Option<Vec<AttachmentResponse>>,
    pub journals: Option<Vec<JournalResponse>>,
    pub relations: Option<Vec<RelationResponse>>,
}

/// Use case for getting a single issue with optional related data
pub struct GetIssueUseCase<I, P, U, T, S, E, J, A, R, M>
where
    I: IssueRepository,
    P: ProjectRepository,
    U: UserRepository,
    T: TrackerRepository,
    S: IssueStatusRepository,
    E: EnumerationRepository,
    J: JournalRepository,
    A: AttachmentRepository,
    R: IssueRelationRepository,
    M: MemberRepository,
{
    issue_repo: Arc<I>,
    project_repo: Arc<P>,
    user_repo: Arc<U>,
    tracker_repo: Arc<T>,
    status_repo: Arc<S>,
    enumeration_repo: Arc<E>,
    journal_repo: Arc<J>,
    attachment_repo: Arc<A>,
    relation_repo: Arc<R>,
    member_repo: Arc<M>,
    base_url: String,
}

impl<I, P, U, T, S, E, J, A, R, M> GetIssueUseCase<I, P, U, T, S, E, J, A, R, M>
where
    I: IssueRepository,
    P: ProjectRepository,
    U: UserRepository,
    T: TrackerRepository,
    S: IssueStatusRepository,
    E: EnumerationRepository,
    J: JournalRepository,
    A: AttachmentRepository,
    R: IssueRelationRepository,
    M: MemberRepository,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        issue_repo: Arc<I>,
        project_repo: Arc<P>,
        user_repo: Arc<U>,
        tracker_repo: Arc<T>,
        status_repo: Arc<S>,
        enumeration_repo: Arc<E>,
        journal_repo: Arc<J>,
        attachment_repo: Arc<A>,
        relation_repo: Arc<R>,
        member_repo: Arc<M>,
        base_url: String,
    ) -> Self {
        Self {
            issue_repo,
            project_repo,
            user_repo,
            tracker_repo,
            status_repo,
            enumeration_repo,
            journal_repo,
            attachment_repo,
            relation_repo,
            member_repo,
            base_url,
        }
    }

    /// Execute the use case
    ///
    /// Visibility rules:
    /// - Admin users can see all issues
    /// - Regular users can see issues in public projects
    /// - Regular users can see issues in projects they are members of
    pub async fn execute(
        &self,
        issue_id: i32,
        include: Vec<IncludeOption>,
        current_user: &CurrentUser,
    ) -> Result<GetIssueResponse, ApplicationError> {
        // Get the issue
        let issue = self
            .issue_repo
            .find_by_id(issue_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound(format!("Issue {} not found", issue_id)))?;

        // Check visibility
        self.check_issue_visibility(&issue, current_user).await?;

        // Build base response
        let mut response = self.build_base_response(&issue).await?;

        // Include related data based on options
        for option in include {
            match option {
                IncludeOption::Attachments => {
                    response.attachments = Some(self.get_attachments(issue_id).await?);
                }
                IncludeOption::Journals => {
                    response.journals = Some(self.get_journals(issue_id, current_user).await?);
                }
                IncludeOption::Relations => {
                    response.relations = Some(self.get_relations(issue_id).await?);
                }
                IncludeOption::Children => {
                    response.children = Some(self.get_children(issue_id).await?);
                }
                IncludeOption::Watchers => {
                    // MVP: Not implemented - watchers functionality not included
                }
                IncludeOption::AllowedStatuses => {
                    // MVP: Not implemented - workflow transitions not included
                }
            }
        }

        Ok(response)
    }

    /// Check if the current user can view the issue
    async fn check_issue_visibility(
        &self,
        issue: &Issue,
        current_user: &CurrentUser,
    ) -> Result<(), ApplicationError> {
        // Admin can see all issues
        if current_user.admin {
            return Ok(());
        }

        // Get the project
        let project = self
            .project_repo
            .find_by_id(issue.project_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| {
                ApplicationError::NotFound(format!("Project {} not found", issue.project_id))
            })?;

        // Public projects are visible to all logged-in users
        if project.is_public {
            return Ok(());
        }

        // Check if user is a member of the project
        let member_project_ids = self
            .project_repo
            .find_project_ids_by_user_membership(current_user.id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        if member_project_ids.contains(&issue.project_id) {
            return Ok(());
        }

        Err(ApplicationError::Forbidden(
            "You don't have permission to view this issue".into(),
        ))
    }

    /// Build the base response with issue details
    async fn build_base_response(
        &self,
        issue: &Issue,
    ) -> Result<GetIssueResponse, ApplicationError> {
        // Get project
        let project = self
            .project_repo
            .find_by_id(issue.project_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        let project_info = project
            .map(|p| NamedId {
                id: p.id,
                name: p.name,
            })
            .unwrap_or(NamedId {
                id: issue.project_id,
                name: format!("Project {}", issue.project_id),
            });

        // Get tracker
        let tracker = self
            .tracker_repo
            .find_by_id(issue.tracker_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        let tracker_info = tracker
            .map(|t| NamedId {
                id: t.id,
                name: t.name,
            })
            .unwrap_or(NamedId {
                id: issue.tracker_id,
                name: format!("Tracker {}", issue.tracker_id),
            });

        // Get status
        let status = self
            .status_repo
            .find_by_id(issue.status_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        let status_info = status
            .map(|s| StatusInfo {
                id: s.id,
                name: s.name,
                is_closed: s.is_closed,
            })
            .unwrap_or(StatusInfo {
                id: issue.status_id,
                name: format!("Status {}", issue.status_id),
                is_closed: false,
            });

        // Get priority
        let priority = self
            .enumeration_repo
            .find_by_id(issue.priority_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        let priority_info = priority
            .map(|p| NamedId {
                id: p.id,
                name: p.name,
            })
            .unwrap_or(NamedId {
                id: issue.priority_id,
                name: format!("Priority {}", issue.priority_id),
            });

        // Get author
        let author = self
            .user_repo
            .find_by_id(issue.author_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        let author_info = author
            .map(|u| NamedId {
                id: u.id,
                name: format!("{} {}", u.firstname, u.lastname),
            })
            .unwrap_or(NamedId {
                id: issue.author_id,
                name: format!("User {}", issue.author_id),
            });

        // Get assigned_to if present
        let assigned_to_info = if let Some(assigned_to_id) = issue.assigned_to_id {
            let assigned_user = self
                .user_repo
                .find_by_id(assigned_to_id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;

            assigned_user.map(|u| NamedId {
                id: u.id,
                name: format!("{} {}", u.firstname, u.lastname),
            })
        } else {
            None
        };

        Ok(GetIssueResponse {
            id: issue.id,
            project: project_info,
            tracker: tracker_info,
            status: status_info,
            priority: priority_info,
            author: author_info,
            assigned_to: assigned_to_info,
            subject: issue.subject.clone(),
            description: issue.description.clone(),
            start_date: issue.start_date.map(|d| d.to_string()),
            due_date: issue.due_date.map(|d| d.to_string()),
            done_ratio: issue.done_ratio,
            is_private: issue.is_private,
            estimated_hours: issue.estimated_hours,
            // For MVP, total_estimated_hours = estimated_hours (no subtask calculation)
            total_estimated_hours: issue.estimated_hours,
            // For MVP, spent_hours is 0 (no time tracking)
            spent_hours: 0.0,
            total_spent_hours: 0.0,
            created_on: issue.created_on.map(|d| d.to_rfc3339()),
            updated_on: issue.updated_on.map(|d| d.to_rfc3339()),
            closed_on: issue.closed_on.map(|d| d.to_rfc3339()),
            children: None,
            attachments: None,
            journals: None,
            relations: None,
        })
    }

    /// Get attachments for the issue
    async fn get_attachments(
        &self,
        issue_id: i32,
    ) -> Result<Vec<AttachmentResponse>, ApplicationError> {
        let attachments = self
            .attachment_repo
            .find_by_container(issue_id, "Issue")
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        let mut responses = Vec::with_capacity(attachments.len());
        for attachment in attachments {
            let author = self
                .user_repo
                .find_by_id(attachment.author_id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;

            let author_info = author
                .map(|u| NamedId {
                    id: u.id,
                    name: format!("{} {}", u.firstname, u.lastname),
                })
                .unwrap_or(NamedId {
                    id: attachment.author_id,
                    name: format!("User {}", attachment.author_id),
                });

            responses.push(AttachmentResponse {
                id: attachment.id,
                filename: attachment.filename.clone(),
                filesize: attachment.filesize,
                content_type: attachment.content_type.clone(),
                description: attachment.description,
                content_url: format!(
                    "{}/attachments/download/{}/{}",
                    self.base_url,
                    attachment.id,
                    urlencoding::encode(&attachment.filename)
                ),
                thumbnail_url: if is_image_content_type(attachment.content_type.as_deref()) {
                    Some(format!(
                        "{}/attachments/thumbnail/{}",
                        self.base_url, attachment.id
                    ))
                } else {
                    None
                },
                author: author_info,
                created_on: attachment.created_on.map(|d| d.to_rfc3339()),
            });
        }

        Ok(responses)
    }

    /// Get journals (change history) for the issue
    async fn get_journals(
        &self,
        issue_id: i32,
        current_user: &CurrentUser,
    ) -> Result<Vec<JournalResponse>, ApplicationError> {
        let journals = self
            .journal_repo
            .find_by_journalized(issue_id, "Issue")
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // For MVP, all journals are visible to users who can view the issue
        // In a more complex implementation, we would check view_private_notes permission
        let _can_view_private = current_user.admin;

        let mut responses = Vec::with_capacity(journals.len());
        for journal in journals {
            // Skip private notes for non-admin users (simplified for MVP)
            if journal.private_notes && !current_user.admin {
                continue;
            }

            let user = self
                .user_repo
                .find_by_id(journal.user_id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;

            let user_info = user
                .map(|u| NamedId {
                    id: u.id,
                    name: format!("{} {}", u.firstname, u.lastname),
                })
                .unwrap_or(NamedId {
                    id: journal.user_id,
                    name: format!("User {}", journal.user_id),
                });

            let details = self
                .journal_repo
                .find_details(journal.id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;

            responses.push(JournalResponse {
                id: journal.id,
                user: user_info,
                notes: journal.notes,
                created_on: journal.created_on.to_rfc3339(),
                updated_on: journal.updated_on.map(|d| d.to_rfc3339()),
                private_notes: journal.private_notes,
                details: details
                    .into_iter()
                    .map(JournalDetailResponse::from)
                    .collect(),
            });
        }

        Ok(responses)
    }

    /// Get relations for the issue
    async fn get_relations(
        &self,
        issue_id: i32,
    ) -> Result<Vec<RelationResponse>, ApplicationError> {
        let relations = self
            .relation_repo
            .find_by_issue(issue_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        Ok(relations
            .into_iter()
            .map(|r| RelationResponse {
                id: r.id,
                issue_id: r.issue_from_id,
                issue_to_id: r.issue_to_id,
                relation_type: r.relation_type,
                delay: r.delay,
            })
            .collect())
    }

    /// Get child issues (subtasks)
    async fn get_children(
        &self,
        issue_id: i32,
    ) -> Result<Vec<ChildIssueSummary>, ApplicationError> {
        use crate::domain::value_objects::IssueQueryParams;

        let params = IssueQueryParams::new(
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(issue_id), // parent_id filter
            None,
            None,
            0,
            100, // offset, limit
            None,
        );

        let children = self
            .issue_repo
            .find_all(params)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        let mut responses = Vec::with_capacity(children.len());
        for child in children {
            let tracker = self
                .tracker_repo
                .find_by_id(child.tracker_id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;

            let tracker_info = tracker
                .map(|t| NamedId {
                    id: t.id,
                    name: t.name,
                })
                .unwrap_or(NamedId {
                    id: child.tracker_id,
                    name: format!("Tracker {}", child.tracker_id),
                });

            responses.push(ChildIssueSummary {
                id: child.id,
                tracker: tracker_info,
                subject: child.subject,
            });
        }

        Ok(responses)
    }
}

fn is_image_content_type(content_type: Option<&str>) -> bool {
    match content_type {
        Some(ct) => ct.starts_with("image/") && !ct.contains("svg") && ct != "image/x-icon",
        None => false,
    }
}
