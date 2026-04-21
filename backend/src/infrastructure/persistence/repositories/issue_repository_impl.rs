use crate::domain::entities::Issue;
use crate::domain::repositories::{IssueRepository, NewIssue, RepositoryError};
use crate::domain::value_objects::IssueQueryParams;
use crate::infrastructure::persistence::entities::issue_statuses;
use crate::infrastructure::persistence::entities::issues;
use crate::infrastructure::persistence::entities::prelude::{IssueStatuses, Issues};
use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, Set,
};

/// SeaORM-based implementation of IssueRepository
pub struct IssueRepositoryImpl {
    db: DatabaseConnection,
}

impl IssueRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Convert SeaORM model to domain entity
    fn model_to_entity(model: issues::Model) -> Issue {
        Issue {
            id: model.id,
            tracker_id: model.tracker_id,
            project_id: model.project_id,
            subject: model.subject,
            description: model.description,
            due_date: model.due_date,
            category_id: model.category_id,
            status_id: model.status_id,
            assigned_to_id: model.assigned_to_id,
            priority_id: model.priority_id,
            fixed_version_id: model.fixed_version_id,
            author_id: model.author_id,
            lock_version: model.lock_version,
            created_on: model
                .created_on
                .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, Utc)),
            updated_on: model
                .updated_on
                .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, Utc)),
            start_date: model.start_date,
            done_ratio: model.done_ratio,
            estimated_hours: model.estimated_hours,
            parent_id: model.parent_id,
            root_id: model.root_id,
            lft: model.lft,
            rgt: model.rgt,
            is_private: model.is_private,
            closed_on: model
                .closed_on
                .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, Utc)),
        }
    }

    /// Get status IDs that are closed
    async fn get_closed_status_ids(&self) -> Result<Vec<i32>, RepositoryError> {
        let statuses = IssueStatuses::find()
            .filter(issue_statuses::Column::IsClosed.eq(true))
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(statuses.into_iter().map(|s| s.id).collect())
    }

    /// Get status IDs that are open (not closed)
    async fn get_open_status_ids(&self) -> Result<Vec<i32>, RepositoryError> {
        let statuses = IssueStatuses::find()
            .filter(issue_statuses::Column::IsClosed.eq(false))
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(statuses.into_iter().map(|s| s.id).collect())
    }
}

#[async_trait]
impl IssueRepository for IssueRepositoryImpl {
    async fn find_all(&self, params: IssueQueryParams) -> Result<Vec<Issue>, RepositoryError> {
        let mut query = Issues::find();

        // Filter by project_id
        if let Some(project_id) = params.project_id {
            query = query.filter(issues::Column::ProjectId.eq(project_id));
        }

        // Filter by status_id (can be ID, "open", or "closed")
        if let Some(status_filter) = &params.status_id {
            match status_filter.as_str() {
                "open" => {
                    let open_status_ids = self.get_open_status_ids().await?;
                    if !open_status_ids.is_empty() {
                        query = query.filter(issues::Column::StatusId.is_in(open_status_ids));
                    }
                }
                "closed" => {
                    let closed_status_ids = self.get_closed_status_ids().await?;
                    if !closed_status_ids.is_empty() {
                        query = query.filter(issues::Column::StatusId.is_in(closed_status_ids));
                    }
                }
                id_str => {
                    if let Ok(id) = id_str.parse::<i32>() {
                        query = query.filter(issues::Column::StatusId.eq(id));
                    }
                }
            }
        }

        // Filter by tracker_id
        if let Some(tracker_id) = params.tracker_id {
            query = query.filter(issues::Column::TrackerId.eq(tracker_id));
        }

        // Filter by priority_id
        if let Some(priority_id) = params.priority_id {
            query = query.filter(issues::Column::PriorityId.eq(priority_id));
        }

        // Filter by category_id
        if let Some(category_id) = params.category_id {
            query = query.filter(issues::Column::CategoryId.eq(category_id));
        }

        // Filter by assigned_to_id (can be ID or "me")
        // Note: "me" is handled at the use case level by converting to the current user's ID
        if let Some(assigned_to_id) = &params.assigned_to_id {
            if assigned_to_id != "me" {
                if let Ok(id) = assigned_to_id.parse::<i32>() {
                    query = query.filter(issues::Column::AssignedToId.eq(id));
                }
            }
        }

        // Filter by author_id
        if let Some(author_id) = params.author_id {
            query = query.filter(issues::Column::AuthorId.eq(author_id));
        }

        // Filter by subject (fuzzy search)
        if let Some(subject) = &params.subject {
            query = query.filter(issues::Column::Subject.contains(subject));
        }

        // Filter by parent_id
        if let Some(parent_id) = params.parent_id {
            query = query.filter(issues::Column::ParentId.eq(parent_id));
        }

        // Filter by created_on (>=YYYY-MM-DD)
        if let Some(created_on) = &params.created_on {
            if let Ok(date) = chrono::NaiveDate::parse_from_str(created_on, "%Y-%m-%d") {
                query = query.filter(issues::Column::CreatedOn.gte(date));
            }
        }

        // Filter by updated_on (>=YYYY-MM-DD)
        if let Some(updated_on) = &params.updated_on {
            if let Ok(date) = chrono::NaiveDate::parse_from_str(updated_on, "%Y-%m-%d") {
                query = query.filter(issues::Column::UpdatedOn.gte(date));
            }
        }

        // Apply sorting
        if let Some(sort) = &params.sort {
            let parts: Vec<&str> = sort.split(':').collect();
            if !parts.is_empty() {
                let field = parts[0];
                let direction = if parts.len() > 1 { parts[1] } else { "asc" };

                let order_fn = if direction == "desc" {
                    sea_orm::Order::Desc
                } else {
                    sea_orm::Order::Asc
                };

                query = match field {
                    "created_on" => query.order_by(issues::Column::CreatedOn, order_fn),
                    "updated_on" => query.order_by(issues::Column::UpdatedOn, order_fn),
                    "id" => query.order_by(issues::Column::Id, order_fn),
                    "subject" => query.order_by(issues::Column::Subject, order_fn),
                    "priority" | "priority_id" => {
                        query.order_by(issues::Column::PriorityId, order_fn)
                    }
                    "status" | "status_id" => query.order_by(issues::Column::StatusId, order_fn),
                    "tracker" | "tracker_id" => query.order_by(issues::Column::TrackerId, order_fn),
                    "project" | "project_id" => query.order_by(issues::Column::ProjectId, order_fn),
                    "author" | "author_id" => query.order_by(issues::Column::AuthorId, order_fn),
                    "assigned_to" | "assigned_to_id" => {
                        query.order_by(issues::Column::AssignedToId, order_fn)
                    }
                    "due_date" => query.order_by(issues::Column::DueDate, order_fn),
                    "start_date" => query.order_by(issues::Column::StartDate, order_fn),
                    "done_ratio" => query.order_by(issues::Column::DoneRatio, order_fn),
                    _ => query.order_by(issues::Column::Id, sea_orm::Order::Desc),
                };
            }
        } else {
            // Default sort by id descending
            query = query.order_by(issues::Column::Id, sea_orm::Order::Desc);
        }

        // Apply pagination
        let issues = query
            .offset(params.offset as u64)
            .limit(params.limit as u64)
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(issues.into_iter().map(Self::model_to_entity).collect())
    }

    async fn count(&self, params: &IssueQueryParams) -> Result<u32, RepositoryError> {
        let mut query = Issues::find();

        // Filter by project_id
        if let Some(project_id) = params.project_id {
            query = query.filter(issues::Column::ProjectId.eq(project_id));
        }

        // Filter by status_id (can be ID, "open", or "closed")
        if let Some(status_filter) = &params.status_id {
            match status_filter.as_str() {
                "open" => {
                    let open_status_ids = self.get_open_status_ids().await?;
                    if !open_status_ids.is_empty() {
                        query = query.filter(issues::Column::StatusId.is_in(open_status_ids));
                    }
                }
                "closed" => {
                    let closed_status_ids = self.get_closed_status_ids().await?;
                    if !closed_status_ids.is_empty() {
                        query = query.filter(issues::Column::StatusId.is_in(closed_status_ids));
                    }
                }
                id_str => {
                    if let Ok(id) = id_str.parse::<i32>() {
                        query = query.filter(issues::Column::StatusId.eq(id));
                    }
                }
            }
        }

        // Filter by tracker_id
        if let Some(tracker_id) = params.tracker_id {
            query = query.filter(issues::Column::TrackerId.eq(tracker_id));
        }

        // Filter by priority_id
        if let Some(priority_id) = params.priority_id {
            query = query.filter(issues::Column::PriorityId.eq(priority_id));
        }

        // Filter by category_id
        if let Some(category_id) = params.category_id {
            query = query.filter(issues::Column::CategoryId.eq(category_id));
        }

        // Filter by assigned_to_id (can be ID or "me")
        if let Some(assigned_to_id) = &params.assigned_to_id {
            if assigned_to_id != "me" {
                if let Ok(id) = assigned_to_id.parse::<i32>() {
                    query = query.filter(issues::Column::AssignedToId.eq(id));
                }
            }
        }

        // Filter by author_id
        if let Some(author_id) = params.author_id {
            query = query.filter(issues::Column::AuthorId.eq(author_id));
        }

        // Filter by subject (fuzzy search)
        if let Some(subject) = &params.subject {
            query = query.filter(issues::Column::Subject.contains(subject));
        }

        // Filter by parent_id
        if let Some(parent_id) = params.parent_id {
            query = query.filter(issues::Column::ParentId.eq(parent_id));
        }

        // Filter by created_on (>=YYYY-MM-DD)
        if let Some(created_on) = &params.created_on {
            if let Ok(date) = chrono::NaiveDate::parse_from_str(created_on, "%Y-%m-%d") {
                query = query.filter(issues::Column::CreatedOn.gte(date));
            }
        }

        // Filter by updated_on (>=YYYY-MM-DD)
        if let Some(updated_on) = &params.updated_on {
            if let Ok(date) = chrono::NaiveDate::parse_from_str(updated_on, "%Y-%m-%d") {
                query = query.filter(issues::Column::UpdatedOn.gte(date));
            }
        }

        let count = query
            .count(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(count as u32)
    }

    async fn find_by_project(&self, project_id: i32) -> Result<Vec<Issue>, RepositoryError> {
        let issues = Issues::find()
            .filter(issues::Column::ProjectId.eq(project_id))
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(issues.into_iter().map(Self::model_to_entity).collect())
    }

    async fn delete_by_project(&self, project_id: i32) -> Result<(), RepositoryError> {
        Issues::delete_many()
            .filter(issues::Column::ProjectId.eq(project_id))
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Issue>, RepositoryError> {
        let issue = Issues::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(issue.map(Self::model_to_entity))
    }

    async fn clear_assignee_in_project(
        &self,
        project_id: i32,
        user_id: i32,
    ) -> Result<(), RepositoryError> {
        use sea_orm::{ActiveModelTrait, ColumnTrait, QueryFilter, Set};

        // Find all issues in the project assigned to this user
        let issues_to_update = Issues::find()
            .filter(issues::Column::ProjectId.eq(project_id))
            .filter(issues::Column::AssignedToId.eq(user_id))
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        // Update each issue to clear the assignee
        for issue in issues_to_update {
            let mut active_issue: issues::ActiveModel = issue.into();
            active_issue.assigned_to_id = sea_orm::ActiveValue::Set(None);
            active_issue.updated_on =
                sea_orm::ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));
            active_issue
                .update(&self.db)
                .await
                .map_err(|e| RepositoryError::Database(e.to_string()))?;
        }

        Ok(())
    }

    async fn create(&self, issue: NewIssue) -> Result<Issue, RepositoryError> {
        let now = chrono::Utc::now().naive_utc();

        let active_model = issues::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            tracker_id: Set(issue.tracker_id),
            project_id: Set(issue.project_id),
            subject: Set(issue.subject),
            description: Set(issue.description),
            due_date: Set(issue.due_date),
            category_id: Set(issue.category_id),
            status_id: Set(issue.status_id),
            assigned_to_id: Set(issue.assigned_to_id),
            priority_id: Set(issue.priority_id),
            fixed_version_id: Set(None),
            author_id: Set(issue.author_id),
            lock_version: Set(0),
            created_on: Set(Some(now)),
            updated_on: Set(Some(now)),
            start_date: Set(issue.start_date),
            done_ratio: Set(0),
            estimated_hours: Set(issue.estimated_hours),
            parent_id: Set(issue.parent_id),
            root_id: Set(None),
            lft: Set(None),
            rgt: Set(None),
            is_private: Set(issue.is_private),
            closed_on: Set(None),
        };

        let result = active_model
            .insert(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(Self::model_to_entity(result))
    }

    async fn update(
        &self,
        id: i32,
        update: crate::domain::repositories::IssueUpdate,
    ) -> Result<Issue, RepositoryError> {
        use sea_orm::ActiveValue;

        // Find existing issue
        let issue = Issues::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?
            .ok_or_else(|| RepositoryError::NotFound(format!("Issue {} not found", id)))?;

        // Convert to active model for update
        let mut active_issue: issues::ActiveModel = issue.into();

        // Apply updates
        if let Some(subject) = update.subject {
            active_issue.subject = ActiveValue::Set(subject);
        }

        if let Some(description) = update.description {
            active_issue.description =
                ActiveValue::Set(Some(description).filter(|d| !d.is_empty()));
        }

        if let Some(status_id) = update.status_id {
            active_issue.status_id = ActiveValue::Set(status_id);
        }

        if let Some(priority_id) = update.priority_id {
            active_issue.priority_id = ActiveValue::Set(priority_id);
        }

        if let Some(tracker_id) = update.tracker_id {
            active_issue.tracker_id = ActiveValue::Set(tracker_id);
        }

        if let Some(assigned_to_id) = update.assigned_to_id {
            active_issue.assigned_to_id = ActiveValue::Set(assigned_to_id);
        }

        if let Some(category_id) = update.category_id {
            active_issue.category_id = ActiveValue::Set(category_id);
        }

        if let Some(parent_id) = update.parent_id {
            active_issue.parent_id = ActiveValue::Set(parent_id);
        }

        if let Some(start_date) = update.start_date {
            active_issue.start_date = ActiveValue::Set(start_date);
        }

        if let Some(due_date) = update.due_date {
            active_issue.due_date = ActiveValue::Set(due_date);
        }

        if let Some(estimated_hours) = update.estimated_hours {
            active_issue.estimated_hours = ActiveValue::Set(estimated_hours);
        }

        if let Some(done_ratio) = update.done_ratio {
            active_issue.done_ratio = ActiveValue::Set(done_ratio);
        }

        if let Some(is_private) = update.is_private {
            active_issue.is_private = ActiveValue::Set(is_private);
        }

        // Update timestamps
        if let Some(updated_on) = update.updated_on {
            active_issue.updated_on = ActiveValue::Set(Some(updated_on.naive_utc()));
        } else {
            active_issue.updated_on = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));
        }

        if let Some(closed_on) = update.closed_on {
            active_issue.closed_on = ActiveValue::Set(closed_on.map(|dt| dt.naive_utc()));
        }

        // Increment lock version for optimistic locking
        let current_lock_version = active_issue.lock_version.unwrap();
        active_issue.lock_version = ActiveValue::Set(current_lock_version + 1);

        // Save changes
        let result = active_issue
            .update(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(Self::model_to_entity(result))
    }

    async fn find_children(&self, parent_id: i32) -> Result<Vec<Issue>, RepositoryError> {
        let issues = Issues::find()
            .filter(issues::Column::ParentId.eq(parent_id))
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(issues.into_iter().map(Self::model_to_entity).collect())
    }

    async fn delete(&self, id: i32) -> Result<(), RepositoryError> {
        Issues::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }
}
