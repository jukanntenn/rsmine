use crate::domain::entities::Tracker;
use crate::domain::repositories::{NewTracker, RepositoryError, TrackerRepository};
use crate::infrastructure::persistence::entities::prelude::{ProjectsTrackers, Trackers};
use crate::infrastructure::persistence::entities::projects_trackers;
use crate::infrastructure::persistence::entities::trackers;
use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};

/// SeaORM-based implementation of TrackerRepository
pub struct TrackerRepositoryImpl {
    db: DatabaseConnection,
}

impl TrackerRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Convert database model to domain entity
    fn model_to_entity(model: trackers::Model) -> Tracker {
        Tracker {
            id: model.id,
            name: model.name,
            position: model.position,
            is_in_roadmap: model.is_in_roadmap,
            fields_bits: model.fields_bits,
            default_status_id: model.default_status_id,
        }
    }
}

#[async_trait]
impl TrackerRepository for TrackerRepositoryImpl {
    async fn find_all(&self) -> Result<Vec<Tracker>, RepositoryError> {
        let trackers = Trackers::find()
            .order_by_asc(trackers::Column::Position)
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(trackers.into_iter().map(Self::model_to_entity).collect())
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Tracker>, RepositoryError> {
        let tracker = Trackers::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(tracker.map(Self::model_to_entity))
    }

    async fn find_by_project(&self, project_id: i32) -> Result<Vec<Tracker>, RepositoryError> {
        // Find tracker IDs associated with the project
        let project_trackers = ProjectsTrackers::find()
            .filter(projects_trackers::Column::ProjectId.eq(project_id))
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        let tracker_ids: Vec<i32> = project_trackers.iter().map(|pt| pt.tracker_id).collect();

        if tracker_ids.is_empty() {
            return Ok(Vec::new());
        }

        // Find trackers by IDs
        let trackers = Trackers::find()
            .filter(trackers::Column::Id.is_in(tracker_ids))
            .order_by_asc(trackers::Column::Position)
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(trackers.into_iter().map(Self::model_to_entity).collect())
    }

    async fn create(&self, tracker: &NewTracker) -> Result<Tracker, RepositoryError> {
        // Get max position for ordering
        let max_position = Trackers::find()
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?
            .map(|t| t.position.unwrap_or(0))
            .unwrap_or(0);

        let active_model = trackers::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            name: Set(tracker.name.clone()),
            position: Set(tracker.position.or(Some(max_position + 1))),
            is_in_roadmap: Set(tracker.is_in_roadmap),
            fields_bits: Set(tracker.fields_bits),
            default_status_id: Set(tracker.default_status_id),
        };

        let result = active_model
            .insert(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(Self::model_to_entity(result))
    }

    async fn update(&self, tracker: &Tracker) -> Result<Tracker, RepositoryError> {
        let existing = Trackers::find_by_id(tracker.id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?
            .ok_or_else(|| {
                RepositoryError::NotFound(format!("Tracker with id {} not found", tracker.id))
            })?;

        let mut active_model: trackers::ActiveModel = existing.into();
        active_model.name = Set(tracker.name.clone());
        active_model.position = Set(tracker.position);
        active_model.is_in_roadmap = Set(tracker.is_in_roadmap);
        active_model.fields_bits = Set(tracker.fields_bits);
        active_model.default_status_id = Set(tracker.default_status_id);

        let result = active_model
            .update(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(Self::model_to_entity(result))
    }

    async fn delete(&self, id: i32) -> Result<(), RepositoryError> {
        let tracker = Trackers::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?
            .ok_or_else(|| {
                RepositoryError::NotFound(format!("Tracker with id {} not found", id))
            })?;

        let active_model: trackers::ActiveModel = tracker.into();
        active_model
            .delete(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        // Also delete project-tracker associations
        ProjectsTrackers::delete_many()
            .filter(projects_trackers::Column::TrackerId.eq(id))
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }

    async fn exists_by_name(&self, name: &str) -> Result<bool, RepositoryError> {
        let count = Trackers::find()
            .filter(trackers::Column::Name.eq(name))
            .count(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(count > 0)
    }

    async fn exists_by_name_excluding(
        &self,
        name: &str,
        exclude_id: i32,
    ) -> Result<bool, RepositoryError> {
        let count = Trackers::find()
            .filter(trackers::Column::Name.eq(name))
            .filter(trackers::Column::Id.ne(exclude_id))
            .count(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(count > 0)
    }

    async fn set_projects(
        &self,
        tracker_id: i32,
        project_ids: &[i32],
    ) -> Result<(), RepositoryError> {
        // Delete all existing associations for this tracker
        ProjectsTrackers::delete_many()
            .filter(projects_trackers::Column::TrackerId.eq(tracker_id))
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        // Create new associations
        for project_id in project_ids {
            let active_model = projects_trackers::ActiveModel {
                id: sea_orm::ActiveValue::NotSet,
                project_id: Set(*project_id),
                tracker_id: Set(tracker_id),
            };
            active_model
                .insert(&self.db)
                .await
                .map_err(|e| RepositoryError::Database(e.to_string()))?;
        }

        Ok(())
    }
}
