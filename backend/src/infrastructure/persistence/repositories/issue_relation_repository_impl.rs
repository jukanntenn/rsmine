use crate::domain::entities::IssueRelation;
use crate::domain::repositories::{IssueRelationRepository, NewIssueRelation, RepositoryError};
use crate::infrastructure::persistence::entities::issue_relations;
use crate::infrastructure::persistence::entities::prelude::IssueRelations;
use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter, Set,
};

/// SeaORM-based implementation of IssueRelationRepository
pub struct IssueRelationRepositoryImpl {
    db: DatabaseConnection,
}

impl IssueRelationRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Convert SeaORM model to domain entity
    fn model_to_entity(model: issue_relations::Model) -> IssueRelation {
        IssueRelation {
            id: model.id,
            issue_from_id: model.issue_from_id,
            issue_to_id: model.issue_to_id,
            relation_type: model.relation_type,
            delay: model.delay,
        }
    }
}

#[async_trait]
impl IssueRelationRepository for IssueRelationRepositoryImpl {
    async fn find_by_issue(&self, issue_id: i32) -> Result<Vec<IssueRelation>, RepositoryError> {
        let relations = IssueRelations::find()
            .filter(
                Condition::any()
                    .add(issue_relations::Column::IssueFromId.eq(issue_id))
                    .add(issue_relations::Column::IssueToId.eq(issue_id)),
            )
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(relations.into_iter().map(Self::model_to_entity).collect())
    }

    async fn delete_by_issue(&self, issue_id: i32) -> Result<(), RepositoryError> {
        IssueRelations::delete_many()
            .filter(
                Condition::any()
                    .add(issue_relations::Column::IssueFromId.eq(issue_id))
                    .add(issue_relations::Column::IssueToId.eq(issue_id)),
            )
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }

    async fn create(&self, relation: NewIssueRelation) -> Result<IssueRelation, RepositoryError> {
        let active_model = issue_relations::ActiveModel {
            id: ActiveValue::NotSet,
            issue_from_id: Set(relation.issue_from_id),
            issue_to_id: Set(relation.issue_to_id),
            relation_type: Set(relation.relation_type),
            delay: Set(relation.delay),
        };

        let model = active_model
            .insert(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(Self::model_to_entity(model))
    }

    async fn exists_relation(
        &self,
        issue_from_id: i32,
        issue_to_id: i32,
        relation_type: &str,
    ) -> Result<bool, RepositoryError> {
        let relation = self
            .find_relation(issue_from_id, issue_to_id, relation_type)
            .await?;
        Ok(relation.is_some())
    }

    async fn find_relation(
        &self,
        issue_from_id: i32,
        issue_to_id: i32,
        relation_type: &str,
    ) -> Result<Option<IssueRelation>, RepositoryError> {
        let relation = IssueRelations::find()
            .filter(issue_relations::Column::IssueFromId.eq(issue_from_id))
            .filter(issue_relations::Column::IssueToId.eq(issue_to_id))
            .filter(issue_relations::Column::RelationType.eq(relation_type))
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(relation.map(Self::model_to_entity))
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<IssueRelation>, RepositoryError> {
        let relation = IssueRelations::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(relation.map(Self::model_to_entity))
    }

    async fn delete(&self, id: i32) -> Result<(), RepositoryError> {
        IssueRelations::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }

    async fn delete_by_issues_and_type(
        &self,
        issue_from_id: i32,
        issue_to_id: i32,
        relation_type: &str,
    ) -> Result<(), RepositoryError> {
        IssueRelations::delete_many()
            .filter(issue_relations::Column::IssueFromId.eq(issue_from_id))
            .filter(issue_relations::Column::IssueToId.eq(issue_to_id))
            .filter(issue_relations::Column::RelationType.eq(relation_type))
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }
}
