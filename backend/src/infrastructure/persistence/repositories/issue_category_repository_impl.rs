use crate::domain::entities::IssueCategory;
use crate::domain::repositories::{
    IssueCategoryRepository, IssueCategoryUpdate, NewIssueCategory, RepositoryError,
};
use crate::infrastructure::persistence::entities::issue_categories;
use crate::infrastructure::persistence::entities::issues;
use crate::infrastructure::persistence::entities::prelude::IssueCategories;
use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    PaginatorTrait, QueryFilter, Set,
};

/// SeaORM-based implementation of IssueCategoryRepository
pub struct IssueCategoryRepositoryImpl {
    db: DatabaseConnection,
}

impl IssueCategoryRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Convert SeaORM model to domain entity
    fn model_to_entity(model: issue_categories::Model) -> IssueCategory {
        IssueCategory {
            id: model.id,
            project_id: model.project_id,
            name: model.name,
            assigned_to_id: model.assigned_to_id,
        }
    }
}

#[async_trait]
impl IssueCategoryRepository for IssueCategoryRepositoryImpl {
    async fn find_by_project(
        &self,
        project_id: i32,
    ) -> Result<Vec<IssueCategory>, RepositoryError> {
        let categories = IssueCategories::find()
            .filter(issue_categories::Column::ProjectId.eq(project_id))
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(categories.into_iter().map(Self::model_to_entity).collect())
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<IssueCategory>, RepositoryError> {
        let category = IssueCategories::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(category.map(Self::model_to_entity))
    }

    async fn create(&self, category: &NewIssueCategory) -> Result<IssueCategory, RepositoryError> {
        let active_model = issue_categories::ActiveModel {
            id: ActiveValue::NotSet,
            project_id: Set(category.project_id),
            name: Set(category.name.clone()),
            assigned_to_id: Set(category.assigned_to_id),
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
        category: &IssueCategoryUpdate,
    ) -> Result<IssueCategory, RepositoryError> {
        // First, find the existing category
        let existing = IssueCategories::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?
            .ok_or_else(|| {
                RepositoryError::NotFound(format!("Issue category with id {} not found", id))
            })?;

        // Build active model with updates
        let mut active_model: issue_categories::ActiveModel = existing.into();

        if let Some(ref name) = category.name {
            active_model.name = Set(name.clone());
        }

        if let Some(assigned_to_id) = category.assigned_to_id {
            active_model.assigned_to_id = Set(assigned_to_id);
        }

        let result = active_model
            .update(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(Self::model_to_entity(result))
    }

    async fn delete(&self, id: i32) -> Result<(), RepositoryError> {
        IssueCategories::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }

    async fn delete_by_project(&self, project_id: i32) -> Result<(), RepositoryError> {
        IssueCategories::delete_many()
            .filter(issue_categories::Column::ProjectId.eq(project_id))
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }

    async fn count_issues(&self, category_id: i32) -> Result<u32, RepositoryError> {
        use crate::infrastructure::persistence::entities::prelude::Issues;

        let count = Issues::find()
            .filter(issues::Column::CategoryId.eq(Some(category_id)))
            .count(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(count as u32)
    }

    async fn reassign_issues(
        &self,
        from_category_id: i32,
        to_category_id: i32,
    ) -> Result<(), RepositoryError> {
        // Get all issues in the from_category and update them
        let issues_to_update = issues::Entity::find()
            .filter(issues::Column::CategoryId.eq(Some(from_category_id)))
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        for issue in issues_to_update {
            let mut active_model: issues::ActiveModel = issue.into();
            active_model.category_id = Set(Some(to_category_id));
            active_model
                .update(&self.db)
                .await
                .map_err(|e| RepositoryError::Database(e.to_string()))?;
        }

        Ok(())
    }

    async fn clear_issues(&self, category_id: i32) -> Result<(), RepositoryError> {
        // Get all issues in the category and clear their category
        let issues_to_update = issues::Entity::find()
            .filter(issues::Column::CategoryId.eq(Some(category_id)))
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        for issue in issues_to_update {
            let mut active_model: issues::ActiveModel = issue.into();
            active_model.category_id = Set(None);
            active_model
                .update(&self.db)
                .await
                .map_err(|e| RepositoryError::Database(e.to_string()))?;
        }

        Ok(())
    }

    async fn exists_by_name(
        &self,
        project_id: i32,
        name: &str,
        exclude_id: Option<i32>,
    ) -> Result<bool, RepositoryError> {
        let mut condition = Condition::all()
            .add(issue_categories::Column::ProjectId.eq(project_id))
            .add(issue_categories::Column::Name.eq(name));

        if let Some(id) = exclude_id {
            condition = condition.add(issue_categories::Column::Id.ne(id));
        }

        let count = IssueCategories::find()
            .filter(condition)
            .count(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(count > 0)
    }
}
