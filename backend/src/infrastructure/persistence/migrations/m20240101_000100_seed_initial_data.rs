use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Insert roles
        // Role builtin values: 0=custom, 1=non-member, 2=anonymous
        // issues_visibility: all, default, own
        db.execute_unprepared(
            "INSERT INTO roles (name, position, builtin, issues_visibility, assignable, users_visibility, time_entries_visibility, all_roles_managed) VALUES 
            ('Manager', 1, 0, 'all', 1, 'all', 'all', 1),
            ('Developer', 2, 0, 'all', 1, 'members_of_visible_projects', 'all', 1),
            ('Reporter', 3, 0, 'default', 1, 'members_of_visible_projects', 'all', 1),
            ('Non-Member', 4, 1, 'default', 1, 'members_of_visible_projects', 'all', 1),
            ('Anonymous', 5, 2, 'default', 0, '-', '-', 0)"
        ).await?;

        // Insert trackers
        db.execute_unprepared(
            "INSERT INTO trackers (name, position, is_in_roadmap, default_status_id) VALUES 
            ('Bug', 1, 1, 1),
            ('Feature', 2, 1, 1),
            ('Support', 3, 0, 1)",
        )
        .await?;

        // Insert issue statuses
        db.execute_unprepared(
            "INSERT INTO issue_statuses (name, position, is_closed, is_default, default_done_ratio) VALUES 
            ('New', 1, 0, 1, 0),
            ('In Progress', 2, 0, 0, NULL),
            ('Resolved', 3, 0, 0, 100),
            ('Feedback', 4, 0, 0, NULL),
            ('Closed', 5, 1, 0, 100),
            ('Rejected', 6, 1, 0, 100)"
        ).await?;

        // Insert priorities (enumerations)
        db.execute_unprepared(
            "INSERT INTO enumerations (name, position, is_default, type, active) VALUES 
            ('Low', 1, 0, 'IssuePriority', 1),
            ('Normal', 2, 1, 'IssuePriority', 1),
            ('High', 3, 0, 'IssuePriority', 1),
            ('Urgent', 4, 0, 'IssuePriority', 1),
            ('Immediate', 5, 0, 'IssuePriority', 1)",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared("DELETE FROM enumerations WHERE type = 'IssuePriority'")
            .await?;
        db.execute_unprepared("DELETE FROM issue_statuses").await?;
        db.execute_unprepared("DELETE FROM trackers").await?;
        db.execute_unprepared("DELETE FROM roles").await?;

        Ok(())
    }
}
