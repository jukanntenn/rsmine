use crate::domain::entities::Project;
use crate::domain::repositories::{ProjectQueryParams, ProjectRepository, RepositoryError};
use crate::infrastructure::persistence::entities::members;
use crate::infrastructure::persistence::entities::prelude::{Members, Projects, ProjectsTrackers};
use crate::infrastructure::persistence::entities::projects;
use crate::infrastructure::persistence::entities::projects_trackers;
use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Select,
};

/// SeaORM-based implementation of ProjectRepository
pub struct ProjectRepositoryImpl {
    db: DatabaseConnection,
}

impl ProjectRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Convert SeaORM model to domain entity
    fn model_to_entity(model: projects::Model) -> Project {
        Project {
            id: model.id,
            name: model.name,
            description: model.description,
            homepage: model.homepage,
            is_public: model.is_public,
            parent_id: model.parent_id,
            created_on: model
                .created_on
                .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, Utc)),
            updated_on: model
                .updated_on
                .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, Utc)),
            identifier: model.identifier,
            status: model.status,
            lft: model.lft,
            rgt: model.rgt,
            inherit_members: model.inherit_members,
            default_version_id: model.default_version_id,
            default_assigned_to_id: model.default_assigned_to_id,
        }
    }

    /// Apply query filters to a Select query
    fn apply_filters(query: Select<Projects>, params: &ProjectQueryParams) -> Select<Projects> {
        let mut condition = Condition::all();

        // Filter by status
        if let Some(status) = params.status {
            condition = condition.add(projects::Column::Status.eq(status));
        }

        // Filter by name (fuzzy search)
        if let Some(ref name) = params.name {
            let name_pattern = format!("%{}%", name);
            condition = condition.add(
                Condition::any()
                    .add(projects::Column::Name.like(&name_pattern))
                    .add(projects::Column::Identifier.like(&name_pattern)),
            );
        }

        // Filter by parent_id
        if let Some(parent_id) = params.parent_id {
            condition = condition.add(projects::Column::ParentId.eq(parent_id));
        }

        // Filter by is_public
        if let Some(is_public) = params.is_public {
            condition = condition.add(projects::Column::IsPublic.eq(is_public));
        }

        query.filter(condition)
    }
}

#[async_trait]
impl ProjectRepository for ProjectRepositoryImpl {
    async fn find_all(&self, params: ProjectQueryParams) -> Result<Vec<Project>, RepositoryError> {
        let query = Projects::find();
        let query = Self::apply_filters(query, &params);

        // Order by id
        let query = query.order_by_asc(projects::Column::Id);

        // Apply pagination
        let query = query
            .offset(params.offset as u64)
            .limit(params.limit as u64);

        let projects = query
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(projects.into_iter().map(Self::model_to_entity).collect())
    }

    async fn count(&self, params: &ProjectQueryParams) -> Result<u32, RepositoryError> {
        let query = Projects::find();
        let query = Self::apply_filters(query, params);

        let count = query
            .count(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(count as u32)
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Project>, RepositoryError> {
        let project = Projects::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(project.map(Self::model_to_entity))
    }

    async fn find_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<Option<Project>, RepositoryError> {
        let project = Projects::find()
            .filter(projects::Column::Identifier.eq(identifier))
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(project.map(Self::model_to_entity))
    }

    async fn find_visible_for_user(
        &self,
        user_id: i32,
        params: ProjectQueryParams,
    ) -> Result<Vec<Project>, RepositoryError> {
        // Get project IDs the user is a member of
        let member_project_ids = self.find_project_ids_by_user_membership(user_id).await?;

        // Build visibility condition: public projects OR member projects
        let mut visibility_condition = Condition::any().add(projects::Column::IsPublic.eq(true));

        if !member_project_ids.is_empty() {
            visibility_condition =
                visibility_condition.add(projects::Column::Id.is_in(member_project_ids));
        }

        let query = Projects::find().filter(visibility_condition);
        let query = Self::apply_filters(query, &params);

        // Order by id
        let query = query.order_by_asc(projects::Column::Id);

        // Apply pagination
        let query = query
            .offset(params.offset as u64)
            .limit(params.limit as u64);

        let projects = query
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(projects.into_iter().map(Self::model_to_entity).collect())
    }

    async fn count_visible_for_user(
        &self,
        user_id: i32,
        params: &ProjectQueryParams,
    ) -> Result<u32, RepositoryError> {
        // Get project IDs the user is a member of
        let member_project_ids = self.find_project_ids_by_user_membership(user_id).await?;

        // Build visibility condition: public projects OR member projects
        let mut visibility_condition = Condition::any().add(projects::Column::IsPublic.eq(true));

        if !member_project_ids.is_empty() {
            visibility_condition =
                visibility_condition.add(projects::Column::Id.is_in(member_project_ids));
        }

        let query = Projects::find().filter(visibility_condition);
        let query = Self::apply_filters(query, params);

        let count = query
            .count(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(count as u32)
    }

    async fn find_project_ids_by_user_membership(
        &self,
        user_id: i32,
    ) -> Result<Vec<i32>, RepositoryError> {
        let members = Members::find()
            .filter(members::Column::UserId.eq(user_id))
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        let project_ids: Vec<i32> = members.into_iter().map(|m| m.project_id).collect();
        Ok(project_ids)
    }

    async fn create(&self, project: Project) -> Result<Project, RepositoryError> {
        let created_on = project.created_on.map(|dt| dt.naive_utc());
        let updated_on = project.updated_on.map(|dt| dt.naive_utc());

        let new_project = projects::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            name: sea_orm::ActiveValue::Set(project.name),
            description: sea_orm::ActiveValue::Set(project.description),
            homepage: sea_orm::ActiveValue::Set(project.homepage),
            is_public: sea_orm::ActiveValue::Set(project.is_public),
            parent_id: sea_orm::ActiveValue::Set(project.parent_id),
            created_on: sea_orm::ActiveValue::Set(created_on),
            updated_on: sea_orm::ActiveValue::Set(updated_on),
            identifier: sea_orm::ActiveValue::Set(project.identifier),
            status: sea_orm::ActiveValue::Set(project.status),
            lft: sea_orm::ActiveValue::Set(project.lft),
            rgt: sea_orm::ActiveValue::Set(project.rgt),
            inherit_members: sea_orm::ActiveValue::Set(project.inherit_members),
            default_version_id: sea_orm::ActiveValue::Set(project.default_version_id),
            default_assigned_to_id: sea_orm::ActiveValue::Set(project.default_assigned_to_id),
            default_issue_query_id: sea_orm::ActiveValue::Set(None),
        };

        let inserted = new_project
            .insert(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(Self::model_to_entity(inserted))
    }

    async fn exists_by_identifier(&self, identifier: &str) -> Result<bool, RepositoryError> {
        let count = Projects::find()
            .filter(projects::Column::Identifier.eq(identifier))
            .count(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(count > 0)
    }

    async fn get_max_rgt(&self) -> Result<Option<i32>, RepositoryError> {
        let result = Projects::find()
            .order_by_desc(projects::Column::Rgt)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(result.and_then(|p| p.rgt))
    }

    async fn add_tracker(&self, project_id: i32, tracker_id: i32) -> Result<(), RepositoryError> {
        let new_tracker = projects_trackers::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            project_id: sea_orm::ActiveValue::Set(project_id),
            tracker_id: sea_orm::ActiveValue::Set(tracker_id),
        };

        new_tracker
            .insert(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }

    async fn update_nested_set_for_insert(&self, lft: i32) -> Result<(), RepositoryError> {
        // Update all projects with lft >= new_lft to shift them by 2
        // This makes room for the new project node
        use sea_orm::ConnectionTrait;

        let sql = format!("UPDATE projects SET lft = lft + 2 WHERE lft >= {}", lft);
        self.db
            .execute_unprepared(&sql)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        let sql = format!("UPDATE projects SET rgt = rgt + 2 WHERE rgt >= {}", lft);
        self.db
            .execute_unprepared(&sql)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }

    async fn update(&self, project: Project) -> Result<Project, RepositoryError> {
        let updated_on = project.updated_on.map(|dt| dt.naive_utc());

        // Find existing project
        let existing = Projects::find_by_id(project.id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?
            .ok_or_else(|| RepositoryError::NotFound("Project not found".to_string()))?;

        // Update the active model
        let mut active_model: projects::ActiveModel = existing.into();
        active_model.name = sea_orm::ActiveValue::Set(project.name);
        active_model.description = sea_orm::ActiveValue::Set(project.description);
        active_model.homepage = sea_orm::ActiveValue::Set(project.homepage);
        active_model.is_public = sea_orm::ActiveValue::Set(project.is_public);
        active_model.parent_id = sea_orm::ActiveValue::Set(project.parent_id);
        active_model.updated_on = sea_orm::ActiveValue::Set(updated_on);
        active_model.identifier = sea_orm::ActiveValue::Set(project.identifier);
        active_model.status = sea_orm::ActiveValue::Set(project.status);
        active_model.inherit_members = sea_orm::ActiveValue::Set(project.inherit_members);
        active_model.default_version_id = sea_orm::ActiveValue::Set(project.default_version_id);
        active_model.default_assigned_to_id =
            sea_orm::ActiveValue::Set(project.default_assigned_to_id);

        let updated = active_model
            .update(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(Self::model_to_entity(updated))
    }

    async fn set_trackers(
        &self,
        project_id: i32,
        tracker_ids: &[i32],
    ) -> Result<(), RepositoryError> {
        // Delete existing trackers for this project
        ProjectsTrackers::delete_many()
            .filter(projects_trackers::Column::ProjectId.eq(project_id))
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        // Add new trackers
        for tracker_id in tracker_ids {
            self.add_tracker(project_id, *tracker_id).await?;
        }

        Ok(())
    }

    async fn exists_by_identifier_excluding(
        &self,
        identifier: &str,
        exclude_project_id: i32,
    ) -> Result<bool, RepositoryError> {
        let count = Projects::find()
            .filter(projects::Column::Identifier.eq(identifier))
            .filter(projects::Column::Id.ne(exclude_project_id))
            .count(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(count > 0)
    }

    async fn find_children(&self, project_id: i32) -> Result<Vec<Project>, RepositoryError> {
        let children = Projects::find()
            .filter(projects::Column::ParentId.eq(project_id))
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(children.into_iter().map(Self::model_to_entity).collect())
    }

    async fn delete(&self, project_id: i32) -> Result<(), RepositoryError> {
        let project = Projects::find_by_id(project_id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?
            .ok_or_else(|| RepositoryError::NotFound("Project not found".to_string()))?;

        project
            .delete(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }

    async fn clear_trackers(&self, project_id: i32) -> Result<(), RepositoryError> {
        ProjectsTrackers::delete_many()
            .filter(projects_trackers::Column::ProjectId.eq(project_id))
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }
}
