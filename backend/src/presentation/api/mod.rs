pub mod routes;
pub mod v1;

use crate::application::use_cases::{
    AddMemberUseCase, CreateCategoryUseCase, CreateIssueStatusUseCase, CreateIssueUseCase,
    CreateProjectUseCase, CreateRelationUseCase, CreateRoleUseCase, CreateTrackerUseCase,
    CreateUserUseCase, DeleteAttachmentUseCase, DeleteCategoryUseCase, DeleteIssueStatusUseCase,
    DeleteIssueUseCase, DeleteProjectUseCase, DeleteRelationUseCase, DeleteRoleUseCase,
    DeleteTrackerUseCase, DeleteUserUseCase, DownloadAttachmentUseCase, GetAttachmentUseCase,
    GetCategoryUseCase, GetCurrentUserUseCase, GetIssueStatusUseCase, GetIssueUseCase,
    GetMemberUseCase, GetProjectTrackersUseCase, GetProjectUseCase, GetRelationUseCase,
    GetRoleUseCase, GetTrackerUseCase, GetUserUseCase, ListCategoriesUseCase,
    ListIssueAttachmentsUseCase, ListIssueStatusesUseCase, ListIssuesUseCase, ListJournalsUseCase,
    ListMembersUseCase, ListPrioritiesUseCase, ListProjectsUseCase, ListRelationsUseCase,
    ListRolesUseCase, ListTrackersUseCase, ListUsersUseCase, LoginUseCase, LogoutUseCase,
    RemoveMemberUseCase, UpdateAttachmentUseCase, UpdateCategoryUseCase, UpdateIssueStatusUseCase,
    UpdateIssueUseCase, UpdateMemberUseCase, UpdateProjectUseCase, UpdateRoleUseCase,
    UpdateTrackerUseCase, UpdateUserUseCase, UploadFileUseCase, UploadIssueAttachmentUseCase,
};
use crate::config::AppConfig;
use crate::infrastructure::auth::{Argon2PasswordService, JwtService};
use crate::infrastructure::persistence::repositories::{
    AttachmentRepositoryImpl, EmailAddressRepositoryImpl, EnumerationRepositoryImpl,
    InMemoryTempAttachmentStore, IssueCategoryRepositoryImpl, IssueRelationRepositoryImpl,
    IssueRepositoryImpl, IssueStatusRepositoryImpl, JournalRepositoryImpl, MemberRepositoryImpl,
    ProjectRepositoryImpl, RoleRepositoryImpl, TokenRepositoryImpl, TrackerRepositoryImpl,
    UserRepositoryImpl,
};
use crate::infrastructure::storage::LocalFileStorage;
use crate::presentation::middleware::auth_middleware;
use axum::{
    middleware,
    routing::{delete, get, patch, post, put},
    Router,
};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

/// Application state for routes
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub config: AppConfig,
    pub jwt_service: Arc<JwtService>,
    pub user_repository: Arc<UserRepositoryImpl>,
    pub email_repository: Arc<EmailAddressRepositoryImpl>,
    pub token_repository: Arc<TokenRepositoryImpl>,
    pub member_repository: Arc<MemberRepositoryImpl>,
    pub project_repository: Arc<ProjectRepositoryImpl>,
    pub role_repository: Arc<RoleRepositoryImpl>,
    pub issue_repository: Arc<IssueRepositoryImpl>,
    pub issue_status_repository: Arc<IssueStatusRepositoryImpl>,
    pub tracker_repository: Arc<TrackerRepositoryImpl>,
    pub attachment_repository: Arc<AttachmentRepositoryImpl>,
    pub category_repository: Arc<IssueCategoryRepositoryImpl>,
    pub journal_repository: Arc<JournalRepositoryImpl>,
    pub relation_repository: Arc<IssueRelationRepositoryImpl>,
    pub enumeration_repository: Arc<EnumerationRepositoryImpl>,
    pub password_service: Arc<Argon2PasswordService>,
    pub temp_attachment_store: Arc<InMemoryTempAttachmentStore>,
    pub file_storage: Arc<LocalFileStorage>,
}

pub fn create_routes(db: DatabaseConnection, config: AppConfig) -> Router {
    // Create services
    let jwt_service = Arc::new(JwtService::new(
        config.jwt.secret.clone(),
        config.jwt.expiration,
    ));
    let user_repository = Arc::new(UserRepositoryImpl::new(db.clone()));
    let email_repository = Arc::new(EmailAddressRepositoryImpl::new(db.clone()));
    let token_repository = Arc::new(TokenRepositoryImpl::new(db.clone()));
    let member_repository = Arc::new(MemberRepositoryImpl::new(db.clone()));
    let project_repository = Arc::new(ProjectRepositoryImpl::new(db.clone()));
    let role_repository = Arc::new(RoleRepositoryImpl::new(db.clone()));
    let issue_repository = Arc::new(IssueRepositoryImpl::new(db.clone()));
    let issue_status_repository = Arc::new(IssueStatusRepositoryImpl::new(db.clone()));
    let tracker_repository = Arc::new(TrackerRepositoryImpl::new(db.clone()));
    let attachment_repository = Arc::new(AttachmentRepositoryImpl::new(db.clone()));
    let category_repository = Arc::new(IssueCategoryRepositoryImpl::new(db.clone()));
    let journal_repository = Arc::new(JournalRepositoryImpl::new(db.clone()));
    let relation_repository = Arc::new(IssueRelationRepositoryImpl::new(db.clone()));
    let enumeration_repository = Arc::new(EnumerationRepositoryImpl::new(db.clone()));
    let password_service = Arc::new(Argon2PasswordService::new());

    // Create file storage and temp attachment store
    let temp_attachment_store = Arc::new(InMemoryTempAttachmentStore::new());
    let file_storage = Arc::new(LocalFileStorage::new(std::path::PathBuf::from(
        &config.storage.path,
    )));

    // Create use cases
    let login_usecase = Arc::new(LoginUseCase::new(
        user_repository.clone(),
        password_service.clone(),
        jwt_service.clone(),
    ));

    let get_current_user_usecase = Arc::new(GetCurrentUserUseCase::new(
        user_repository.clone(),
        email_repository.clone(),
        token_repository.clone(),
    ));

    let logout_usecase = Arc::new(LogoutUseCase::new(
        token_repository.clone(),
        jwt_service.clone(),
        config.jwt.expiration,
    ));

    let list_users_usecase = Arc::new(ListUsersUseCase::new(
        user_repository.clone(),
        email_repository.clone(),
    ));

    let get_user_usecase = Arc::new(GetUserUseCase::new(
        user_repository.clone(),
        email_repository.clone(),
        token_repository.clone(),
    ));

    let update_user_usecase = Arc::new(UpdateUserUseCase::new(
        user_repository.clone(),
        email_repository.clone(),
        password_service.clone(),
    ));

    let delete_user_usecase = Arc::new(DeleteUserUseCase::new(
        user_repository.clone(),
        email_repository.clone(),
        token_repository.clone(),
        member_repository.clone(),
    ));

    let create_user_usecase = Arc::new(CreateUserUseCase::new(
        user_repository.clone(),
        email_repository.clone(),
        password_service.clone(),
    ));

    let list_projects_usecase = Arc::new(ListProjectsUseCase::new(project_repository.clone()));

    let get_project_usecase = Arc::new(GetProjectUseCase::new(
        project_repository.clone(),
        member_repository.clone(),
        user_repository.clone(),
    ));

    let create_project_usecase = Arc::new(CreateProjectUseCase::new(
        project_repository.clone(),
        member_repository.clone(),
    ));

    let update_project_usecase = Arc::new(UpdateProjectUseCase::new(
        project_repository.clone(),
        member_repository.clone(),
        user_repository.clone(),
    ));

    let delete_project_usecase = Arc::new(DeleteProjectUseCase::new(
        project_repository.clone(),
        issue_repository.clone(),
        member_repository.clone(),
        attachment_repository.clone(),
        category_repository.clone(),
        journal_repository.clone(),
        relation_repository.clone(),
    ));

    let list_members_usecase = Arc::new(ListMembersUseCase::new(
        project_repository.clone(),
        member_repository.clone(),
    ));

    let add_member_usecase = Arc::new(AddMemberUseCase::new(
        member_repository.clone(),
        project_repository.clone(),
        user_repository.clone(),
        role_repository.clone(),
    ));

    let update_member_usecase = Arc::new(UpdateMemberUseCase::new(
        member_repository.clone(),
        project_repository.clone(),
        role_repository.clone(),
    ));

    let get_member_usecase = Arc::new(GetMemberUseCase::new(
        member_repository.clone(),
        project_repository.clone(),
    ));

    let remove_member_usecase = Arc::new(RemoveMemberUseCase::new(
        member_repository.clone(),
        issue_repository.clone(),
    ));

    // Tracker use cases
    let list_trackers_usecase = Arc::new(ListTrackersUseCase::new(
        tracker_repository.clone(),
        issue_status_repository.clone(),
    ));

    let get_tracker_usecase = Arc::new(GetTrackerUseCase::new(
        tracker_repository.clone(),
        issue_status_repository.clone(),
    ));

    let create_tracker_usecase = Arc::new(CreateTrackerUseCase::new(
        tracker_repository.clone(),
        issue_status_repository.clone(),
    ));

    let update_tracker_usecase = Arc::new(UpdateTrackerUseCase::new(
        tracker_repository.clone(),
        issue_status_repository.clone(),
    ));

    let delete_tracker_usecase = Arc::new(DeleteTrackerUseCase::new(
        tracker_repository.clone(),
        issue_repository.clone(),
    ));

    // Issue status use cases
    let list_issue_statuses_usecase = Arc::new(ListIssueStatusesUseCase::new(
        issue_status_repository.clone(),
    ));

    let get_issue_status_usecase =
        Arc::new(GetIssueStatusUseCase::new(issue_status_repository.clone()));

    let create_issue_status_usecase = Arc::new(CreateIssueStatusUseCase::new(
        issue_status_repository.clone(),
    ));

    let update_issue_status_usecase = Arc::new(UpdateIssueStatusUseCase::new(
        issue_status_repository.clone(),
    ));

    let delete_issue_status_usecase = Arc::new(DeleteIssueStatusUseCase::new(
        issue_status_repository.clone(),
    ));

    // Priority use cases
    let list_priorities_usecase =
        Arc::new(ListPrioritiesUseCase::new(enumeration_repository.clone()));

    // Role use cases
    let list_roles_usecase = Arc::new(ListRolesUseCase::new(role_repository.clone()));

    let get_role_usecase = Arc::new(GetRoleUseCase::new(role_repository.clone()));

    let create_role_usecase = Arc::new(CreateRoleUseCase::new(role_repository.clone()));

    let update_role_usecase = Arc::new(UpdateRoleUseCase::new(role_repository.clone()));

    let delete_role_usecase = Arc::new(DeleteRoleUseCase::new(role_repository.clone()));

    // Issue use cases
    let list_issues_usecase = Arc::new(ListIssuesUseCase::new(
        issue_repository.clone(),
        project_repository.clone(),
        user_repository.clone(),
        tracker_repository.clone(),
        issue_status_repository.clone(),
        enumeration_repository.clone(),
    ));

    let get_issue_usecase = Arc::new(GetIssueUseCase::new(
        issue_repository.clone(),
        project_repository.clone(),
        user_repository.clone(),
        tracker_repository.clone(),
        issue_status_repository.clone(),
        enumeration_repository.clone(),
        journal_repository.clone(),
        attachment_repository.clone(),
        relation_repository.clone(),
        member_repository.clone(),
        config.server.base_url.clone(),
    ));

    let create_issue_usecase = Arc::new(CreateIssueUseCase::new(
        issue_repository.clone(),
        project_repository.clone(),
        user_repository.clone(),
        tracker_repository.clone(),
        issue_status_repository.clone(),
        enumeration_repository.clone(),
        member_repository.clone(),
    ));

    let update_issue_usecase = Arc::new(UpdateIssueUseCase::new(
        issue_repository.clone(),
        project_repository.clone(),
        user_repository.clone(),
        tracker_repository.clone(),
        issue_status_repository.clone(),
        enumeration_repository.clone(),
        journal_repository.clone(),
        member_repository.clone(),
    ));

    let delete_issue_usecase = Arc::new(DeleteIssueUseCase::new(
        issue_repository.clone(),
        attachment_repository.clone(),
        journal_repository.clone(),
        relation_repository.clone(),
        member_repository.clone(),
    ));

    // Journal use cases
    let list_journals_usecase = Arc::new(ListJournalsUseCase::new(
        issue_repository.clone(),
        project_repository.clone(),
        user_repository.clone(),
        journal_repository.clone(),
    ));

    // Relation use cases
    let list_relations_usecase = Arc::new(ListRelationsUseCase::new(
        issue_repository.clone(),
        project_repository.clone(),
        relation_repository.clone(),
    ));

    let create_relation_usecase = Arc::new(CreateRelationUseCase::new(
        issue_repository.clone(),
        project_repository.clone(),
        member_repository.clone(),
        relation_repository.clone(),
    ));

    let delete_relation_usecase = Arc::new(DeleteRelationUseCase::new(
        issue_repository.clone(),
        project_repository.clone(),
        member_repository.clone(),
        relation_repository.clone(),
    ));

    let get_relation_usecase = Arc::new(GetRelationUseCase::new(
        issue_repository.clone(),
        project_repository.clone(),
        relation_repository.clone(),
    ));

    // Upload use case
    let upload_file_usecase = Arc::new(UploadFileUseCase::new(
        file_storage.clone(),
        temp_attachment_store.clone(),
        config.storage.clone(),
    ));

    // Attachment use cases
    let download_attachment_usecase = Arc::new(DownloadAttachmentUseCase::new(
        attachment_repository.clone(),
        issue_repository.clone(),
        project_repository.clone(),
        file_storage.clone(),
    ));

    let get_attachment_usecase = Arc::new(GetAttachmentUseCase::new(
        attachment_repository.clone(),
        issue_repository.clone(),
        project_repository.clone(),
        user_repository.clone(),
        config.server.base_url.clone(),
    ));

    let delete_attachment_usecase = Arc::new(DeleteAttachmentUseCase::new(
        attachment_repository.clone(),
        issue_repository.clone(),
        project_repository.clone(),
        member_repository.clone(),
        file_storage.clone(),
    ));

    let update_attachment_usecase = Arc::new(UpdateAttachmentUseCase::new(
        attachment_repository.clone(),
        issue_repository.clone(),
        project_repository.clone(),
        member_repository.clone(),
        user_repository.clone(),
        config.server.base_url.clone(),
    ));

    // Issue attachment use cases
    let list_issue_attachments_usecase = Arc::new(ListIssueAttachmentsUseCase::new(
        attachment_repository.clone(),
        issue_repository.clone(),
        project_repository.clone(),
        user_repository.clone(),
    ));

    let upload_issue_attachment_usecase = Arc::new(UploadIssueAttachmentUseCase::new(
        attachment_repository.clone(),
        issue_repository.clone(),
        project_repository.clone(),
        user_repository.clone(),
        member_repository.clone(),
        file_storage.clone(),
        config.storage.clone(),
    ));

    // Category use cases
    let list_categories_usecase = Arc::new(ListCategoriesUseCase::new(
        category_repository.clone(),
        project_repository.clone(),
        user_repository.clone(),
        member_repository.clone(),
    ));

    let get_category_usecase = Arc::new(GetCategoryUseCase::new(
        category_repository.clone(),
        project_repository.clone(),
        user_repository.clone(),
        member_repository.clone(),
    ));

    let create_category_usecase = Arc::new(CreateCategoryUseCase::new(
        category_repository.clone(),
        project_repository.clone(),
        user_repository.clone(),
        member_repository.clone(),
    ));

    let update_category_usecase = Arc::new(UpdateCategoryUseCase::new(
        category_repository.clone(),
        project_repository.clone(),
        user_repository.clone(),
        member_repository.clone(),
    ));

    let delete_category_usecase = Arc::new(DeleteCategoryUseCase::new(
        category_repository.clone(),
        member_repository.clone(),
    ));

    // Project trackers use case
    let get_project_trackers_usecase = Arc::new(GetProjectTrackersUseCase::new(
        project_repository.clone(),
        member_repository.clone(),
        tracker_repository.clone(),
        issue_status_repository.clone(),
    ));

    let state = AppState {
        db,
        config,
        jwt_service: jwt_service.clone(),
        user_repository: user_repository.clone(),
        email_repository: email_repository.clone(),
        token_repository: token_repository.clone(),
        member_repository: member_repository.clone(),
        project_repository: project_repository.clone(),
        role_repository: role_repository.clone(),
        issue_repository,
        issue_status_repository,
        tracker_repository,
        attachment_repository,
        category_repository,
        journal_repository,
        relation_repository,
        enumeration_repository,
        password_service,
        temp_attachment_store,
        file_storage,
    };

    // Auth routes that don't require authentication
    let public_auth_routes = Router::new()
        .route(
            "/api/v1/auth/login",
            axum::routing::post(v1::login::<UserRepositoryImpl, Argon2PasswordService>),
        )
        .with_state(login_usecase);

    // Protected route: current user
    let protected_current_user_routes = Router::new()
        .route(
            "/api/v1/auth/me",
            get(v1::get_current_user::<
                UserRepositoryImpl,
                EmailAddressRepositoryImpl,
                TokenRepositoryImpl,
            >),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(get_current_user_usecase);

    // Protected route: logout
    let protected_logout_routes = Router::new()
        .route("/api/v1/auth/logout", post(v1::logout_with_app_state))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    // Protected route: user list
    let protected_users_routes = Router::new()
        .route(
            "/api/v1/users.json",
            get(v1::list_users::<UserRepositoryImpl, EmailAddressRepositoryImpl>),
        )
        .route(
            "/api/v1/users",
            get(v1::list_users::<UserRepositoryImpl, EmailAddressRepositoryImpl>),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(list_users_usecase);

    // Protected route: create user (admin only)
    let protected_create_user_routes = Router::new()
        .route(
            "/api/v1/users.json",
            post(
                v1::create_user::<
                    UserRepositoryImpl,
                    EmailAddressRepositoryImpl,
                    Argon2PasswordService,
                >,
            ),
        )
        .route(
            "/api/v1/users",
            post(
                v1::create_user::<
                    UserRepositoryImpl,
                    EmailAddressRepositoryImpl,
                    Argon2PasswordService,
                >,
            ),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(create_user_usecase);

    // Protected route: get user by ID
    let protected_get_user_routes = Router::new()
        .route(
            "/api/v1/users/{id}",
            get(
                v1::get_user::<UserRepositoryImpl, EmailAddressRepositoryImpl, TokenRepositoryImpl>,
            ),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(get_user_usecase);

    // Protected route: update user by ID
    let protected_update_user_routes = Router::new()
        .route(
            "/api/v1/users/{id}",
            put(v1::update_user::<
                UserRepositoryImpl,
                EmailAddressRepositoryImpl,
                Argon2PasswordService,
            >),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(update_user_usecase);

    // Protected route: delete user by ID
    let protected_delete_user_routes = Router::new()
        .route(
            "/api/v1/users/{id}",
            delete(
                v1::delete_user::<
                    UserRepositoryImpl,
                    EmailAddressRepositoryImpl,
                    TokenRepositoryImpl,
                    MemberRepositoryImpl,
                >,
            ),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(delete_user_usecase);

    // Protected route: project list
    let protected_projects_routes = Router::new()
        .route(
            "/api/v1/projects.json",
            get(v1::list_projects::<ProjectRepositoryImpl>),
        )
        .route(
            "/api/v1/projects",
            get(v1::list_projects::<ProjectRepositoryImpl>),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(list_projects_usecase);

    // Protected route: get project by ID or identifier
    let protected_get_project_routes = Router::new()
        .route(
            "/api/v1/projects/{id}",
            get(v1::get_project::<ProjectRepositoryImpl, MemberRepositoryImpl, UserRepositoryImpl>),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(get_project_usecase);

    // Protected route: create project
    let protected_create_project_routes = Router::new()
        .route(
            "/api/v1/projects.json",
            post(v1::create_project::<ProjectRepositoryImpl, MemberRepositoryImpl>),
        )
        .route(
            "/api/v1/projects",
            post(v1::create_project::<ProjectRepositoryImpl, MemberRepositoryImpl>),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(create_project_usecase);

    // Protected route: update project
    let protected_update_project_routes = Router::new()
        .route(
            "/api/v1/projects/{id}",
            put(v1::update_project::<
                ProjectRepositoryImpl,
                MemberRepositoryImpl,
                UserRepositoryImpl,
            >),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(update_project_usecase);

    // Protected route: delete project
    let protected_delete_project_routes = Router::new()
        .route(
            "/api/v1/projects/{id}",
            delete(
                v1::delete_project::<
                    ProjectRepositoryImpl,
                    IssueRepositoryImpl,
                    MemberRepositoryImpl,
                    AttachmentRepositoryImpl,
                    IssueCategoryRepositoryImpl,
                    JournalRepositoryImpl,
                    IssueRelationRepositoryImpl,
                >,
            ),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(delete_project_usecase);

    // Protected route: list project members
    let protected_members_routes = Router::new()
        .route(
            "/api/v1/projects/{id}/memberships.json",
            get(v1::list_members::<ProjectRepositoryImpl, MemberRepositoryImpl>),
        )
        .route(
            "/api/v1/projects/{id}/memberships",
            get(v1::list_members::<ProjectRepositoryImpl, MemberRepositoryImpl>),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(list_members_usecase);

    // Protected route: add member to project
    let protected_add_member_routes = Router::new()
        .route(
            "/api/v1/projects/{id}/memberships.json",
            post(
                v1::add_member::<
                    ProjectRepositoryImpl,
                    MemberRepositoryImpl,
                    UserRepositoryImpl,
                    RoleRepositoryImpl,
                >,
            ),
        )
        .route(
            "/api/v1/projects/{id}/memberships",
            post(
                v1::add_member::<
                    ProjectRepositoryImpl,
                    MemberRepositoryImpl,
                    UserRepositoryImpl,
                    RoleRepositoryImpl,
                >,
            ),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(add_member_usecase);

    // Protected route: update member roles
    let protected_update_member_routes =
        Router::new()
            .route(
                "/api/v1/memberships/{id}",
                put(v1::update_member::<
                    MemberRepositoryImpl,
                    ProjectRepositoryImpl,
                    RoleRepositoryImpl,
                >),
            )
            .route_layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            ))
            .with_state(update_member_usecase);

    // Protected route: get member details
    let protected_get_member_routes = Router::new()
        .route(
            "/api/v1/memberships/{id}",
            get(v1::get_member::<MemberRepositoryImpl, ProjectRepositoryImpl>),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(get_member_usecase);

    // Protected route: delete member
    let protected_delete_member_routes = Router::new()
        .route(
            "/api/v1/memberships/{id}",
            delete(v1::delete_member::<MemberRepositoryImpl, IssueRepositoryImpl>),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(remove_member_usecase);

    // Protected route: list trackers
    let protected_list_trackers_routes = Router::new()
        .route(
            "/api/v1/trackers.json",
            get(v1::list_trackers::<TrackerRepositoryImpl, IssueStatusRepositoryImpl>),
        )
        .route(
            "/api/v1/trackers",
            get(v1::list_trackers::<TrackerRepositoryImpl, IssueStatusRepositoryImpl>),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(list_trackers_usecase);

    // Protected route: get tracker by ID
    let protected_get_tracker_routes = Router::new()
        .route(
            "/api/v1/trackers/{id}",
            get(v1::get_tracker::<TrackerRepositoryImpl, IssueStatusRepositoryImpl>),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(get_tracker_usecase);

    // Protected route: create tracker
    let protected_create_tracker_routes = Router::new()
        .route(
            "/api/v1/trackers.json",
            post(v1::create_tracker::<TrackerRepositoryImpl, IssueStatusRepositoryImpl>),
        )
        .route(
            "/api/v1/trackers",
            post(v1::create_tracker::<TrackerRepositoryImpl, IssueStatusRepositoryImpl>),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(create_tracker_usecase);

    // Protected route: update tracker
    let protected_update_tracker_routes = Router::new()
        .route(
            "/api/v1/trackers/{id}",
            put(v1::update_tracker::<TrackerRepositoryImpl, IssueStatusRepositoryImpl>),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(update_tracker_usecase);

    // Protected route: delete tracker
    let protected_delete_tracker_routes = Router::new()
        .route(
            "/api/v1/trackers/{id}",
            delete(v1::delete_tracker::<TrackerRepositoryImpl, IssueRepositoryImpl>),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(delete_tracker_usecase);

    // Protected route: list issue statuses
    let protected_list_issue_statuses_routes = Router::new()
        .route(
            "/api/v1/issue_statuses.json",
            get(v1::list_issue_statuses::<IssueStatusRepositoryImpl>),
        )
        .route(
            "/api/v1/issue_statuses",
            get(v1::list_issue_statuses::<IssueStatusRepositoryImpl>),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(list_issue_statuses_usecase);

    // Protected route: get issue status by ID
    let protected_get_issue_status_routes = Router::new()
        .route(
            "/api/v1/issue_statuses/{id}",
            get(v1::get_issue_status::<IssueStatusRepositoryImpl>),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(get_issue_status_usecase);

    // Protected route: create issue status
    let protected_create_issue_status_routes = Router::new()
        .route(
            "/api/v1/issue_statuses.json",
            post(v1::create_issue_status::<IssueStatusRepositoryImpl>),
        )
        .route(
            "/api/v1/issue_statuses",
            post(v1::create_issue_status::<IssueStatusRepositoryImpl>),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(create_issue_status_usecase);

    // Protected route: update issue status
    let protected_update_issue_status_routes = Router::new()
        .route(
            "/api/v1/issue_statuses/{id}",
            put(v1::update_issue_status::<IssueStatusRepositoryImpl>),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(update_issue_status_usecase);

    // Protected route: delete issue status
    let protected_delete_issue_status_routes = Router::new()
        .route(
            "/api/v1/issue_statuses/{id}",
            delete(v1::delete_issue_status::<IssueStatusRepositoryImpl>),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(delete_issue_status_usecase);

    // Protected route: list priorities
    let protected_list_priorities_routes = Router::new()
        .route(
            "/api/v1/enumerations/issue_priorities.json",
            get(v1::list_priorities::<EnumerationRepositoryImpl>),
        )
        .route(
            "/api/v1/enumerations/issue_priorities",
            get(v1::list_priorities::<EnumerationRepositoryImpl>),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(list_priorities_usecase);

    // Protected route: list roles
    let protected_list_roles_routes = Router::new()
        .route(
            "/api/v1/roles.json",
            get(v1::list_roles::<RoleRepositoryImpl>),
        )
        .route("/api/v1/roles", get(v1::list_roles::<RoleRepositoryImpl>))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(list_roles_usecase);

    // Protected route: get role by ID
    let protected_get_role_routes = Router::new()
        .route(
            "/api/v1/roles/{id}",
            get(v1::get_role::<RoleRepositoryImpl>),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(get_role_usecase);

    // Protected route: create role
    let protected_create_role_routes = Router::new()
        .route(
            "/api/v1/roles.json",
            post(v1::create_role::<RoleRepositoryImpl>),
        )
        .route("/api/v1/roles", post(v1::create_role::<RoleRepositoryImpl>))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(create_role_usecase);

    // Protected route: update role
    let protected_update_role_routes = Router::new()
        .route(
            "/api/v1/roles/{id}",
            put(v1::update_role::<RoleRepositoryImpl>),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(update_role_usecase);

    // Protected route: delete role
    let protected_delete_role_routes = Router::new()
        .route(
            "/api/v1/roles/{id}",
            delete(v1::delete_role::<RoleRepositoryImpl>),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(delete_role_usecase);

    // Protected route: list issues
    let protected_list_issues_routes = Router::new()
        .route(
            "/api/v1/issues.json",
            get(v1::list_issues::<
                IssueRepositoryImpl,
                ProjectRepositoryImpl,
                UserRepositoryImpl,
                TrackerRepositoryImpl,
                IssueStatusRepositoryImpl,
                EnumerationRepositoryImpl,
            >),
        )
        .route(
            "/api/v1/issues",
            get(v1::list_issues::<
                IssueRepositoryImpl,
                ProjectRepositoryImpl,
                UserRepositoryImpl,
                TrackerRepositoryImpl,
                IssueStatusRepositoryImpl,
                EnumerationRepositoryImpl,
            >),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(list_issues_usecase);

    // Protected route: get issue by ID
    let protected_get_issue_routes = Router::new()
        .route(
            "/api/v1/issues/{id}",
            get(v1::get_issue::<
                IssueRepositoryImpl,
                ProjectRepositoryImpl,
                UserRepositoryImpl,
                TrackerRepositoryImpl,
                IssueStatusRepositoryImpl,
                EnumerationRepositoryImpl,
                JournalRepositoryImpl,
                AttachmentRepositoryImpl,
                IssueRelationRepositoryImpl,
                MemberRepositoryImpl,
            >),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(get_issue_usecase);

    // Protected route: create issue
    let protected_create_issue_routes = Router::new()
        .route(
            "/api/v1/issues.json",
            post(
                v1::create_issue::<
                    IssueRepositoryImpl,
                    ProjectRepositoryImpl,
                    UserRepositoryImpl,
                    TrackerRepositoryImpl,
                    IssueStatusRepositoryImpl,
                    EnumerationRepositoryImpl,
                    MemberRepositoryImpl,
                >,
            ),
        )
        .route(
            "/api/v1/issues",
            post(
                v1::create_issue::<
                    IssueRepositoryImpl,
                    ProjectRepositoryImpl,
                    UserRepositoryImpl,
                    TrackerRepositoryImpl,
                    IssueStatusRepositoryImpl,
                    EnumerationRepositoryImpl,
                    MemberRepositoryImpl,
                >,
            ),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(create_issue_usecase);

    // Protected route: update issue
    let protected_update_issue_routes = Router::new()
        .route(
            "/api/v1/issues/{id}",
            put(v1::update_issue::<
                IssueRepositoryImpl,
                ProjectRepositoryImpl,
                UserRepositoryImpl,
                TrackerRepositoryImpl,
                IssueStatusRepositoryImpl,
                EnumerationRepositoryImpl,
                JournalRepositoryImpl,
                MemberRepositoryImpl,
            >),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(update_issue_usecase);

    // Protected route: delete issue
    let protected_delete_issue_routes = Router::new()
        .route(
            "/api/v1/issues/{id}",
            delete(
                v1::delete_issue::<
                    IssueRepositoryImpl,
                    AttachmentRepositoryImpl,
                    JournalRepositoryImpl,
                    IssueRelationRepositoryImpl,
                    MemberRepositoryImpl,
                >,
            ),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(delete_issue_usecase);

    // Protected route: list issue journals (change history)
    let protected_list_journals_routes = Router::new()
        .route(
            "/api/v1/issues/{id}/journals.json",
            get(v1::list_journals::<
                IssueRepositoryImpl,
                ProjectRepositoryImpl,
                UserRepositoryImpl,
                JournalRepositoryImpl,
            >),
        )
        .route(
            "/api/v1/issues/{id}/journals",
            get(v1::list_journals::<
                IssueRepositoryImpl,
                ProjectRepositoryImpl,
                UserRepositoryImpl,
                JournalRepositoryImpl,
            >),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(list_journals_usecase);

    // Protected route: list issue attachments
    let protected_list_issue_attachments_routes = Router::new()
        .route(
            "/api/v1/issues/{id}/attachments.json",
            get(v1::list_issue_attachments::<
                AttachmentRepositoryImpl,
                IssueRepositoryImpl,
                ProjectRepositoryImpl,
                UserRepositoryImpl,
            >),
        )
        .route(
            "/api/v1/issues/{id}/attachments",
            get(v1::list_issue_attachments::<
                AttachmentRepositoryImpl,
                IssueRepositoryImpl,
                ProjectRepositoryImpl,
                UserRepositoryImpl,
            >),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(list_issue_attachments_usecase);

    // Protected route: upload issue attachment
    let protected_upload_issue_attachment_routes = Router::new()
        .route(
            "/api/v1/issues/{id}/attachments.json",
            post(
                v1::upload_issue_attachment::<
                    AttachmentRepositoryImpl,
                    IssueRepositoryImpl,
                    ProjectRepositoryImpl,
                    UserRepositoryImpl,
                    MemberRepositoryImpl,
                >,
            ),
        )
        .route(
            "/api/v1/issues/{id}/attachments",
            post(
                v1::upload_issue_attachment::<
                    AttachmentRepositoryImpl,
                    IssueRepositoryImpl,
                    ProjectRepositoryImpl,
                    UserRepositoryImpl,
                    MemberRepositoryImpl,
                >,
            ),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(upload_issue_attachment_usecase);

    // Protected route: list issue relations
    let protected_list_relations_routes = Router::new()
        .route(
            "/api/v1/issues/{id}/relations.json",
            get(v1::list_relations::<
                IssueRepositoryImpl,
                ProjectRepositoryImpl,
                IssueRelationRepositoryImpl,
            >),
        )
        .route(
            "/api/v1/issues/{id}/relations",
            get(v1::list_relations::<
                IssueRepositoryImpl,
                ProjectRepositoryImpl,
                IssueRelationRepositoryImpl,
            >),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(list_relations_usecase);

    // Protected route: create issue relation
    let protected_create_relation_routes = Router::new()
        .route(
            "/api/v1/issues/{id}/relations.json",
            post(
                v1::create_relation::<
                    IssueRepositoryImpl,
                    ProjectRepositoryImpl,
                    MemberRepositoryImpl,
                    IssueRelationRepositoryImpl,
                >,
            ),
        )
        .route(
            "/api/v1/issues/{id}/relations",
            post(
                v1::create_relation::<
                    IssueRepositoryImpl,
                    ProjectRepositoryImpl,
                    MemberRepositoryImpl,
                    IssueRelationRepositoryImpl,
                >,
            ),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(create_relation_usecase);

    // Protected route: delete relation
    let protected_delete_relation_routes = Router::new()
        .route(
            "/api/v1/relations/{id}",
            delete(
                v1::delete_relation::<
                    IssueRepositoryImpl,
                    ProjectRepositoryImpl,
                    MemberRepositoryImpl,
                    IssueRelationRepositoryImpl,
                >,
            ),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(delete_relation_usecase);

    // Protected route: get relation by ID
    let protected_get_relation_routes = Router::new()
        .route(
            "/api/v1/relations/{id}",
            get(v1::get_relation::<
                IssueRepositoryImpl,
                ProjectRepositoryImpl,
                IssueRelationRepositoryImpl,
            >),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(get_relation_usecase);

    // Protected route: upload file
    let protected_upload_routes = Router::new()
        .route("/api/v1/uploads.json", post(v1::upload_file))
        .route("/api/v1/uploads", post(v1::upload_file))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(upload_file_usecase);

    // Protected route: get attachment metadata
    let protected_get_attachment_routes = Router::new()
        .route(
            "/api/v1/attachments/{id}",
            get(v1::get_attachment_metadata::<
                AttachmentRepositoryImpl,
                IssueRepositoryImpl,
                ProjectRepositoryImpl,
                UserRepositoryImpl,
            >),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(get_attachment_usecase);

    // Protected route: download attachment
    let protected_download_attachment_routes = Router::new()
        .route(
            "/api/v1/attachments/download/{id}",
            get(v1::download_attachment::<
                AttachmentRepositoryImpl,
                IssueRepositoryImpl,
                ProjectRepositoryImpl,
            >),
        )
        .route(
            "/api/v1/attachments/download/{id}/{filename}",
            get(v1::download_attachment_with_filename::<
                AttachmentRepositoryImpl,
                IssueRepositoryImpl,
                ProjectRepositoryImpl,
            >),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(download_attachment_usecase);

    // Protected route: delete attachment
    let protected_delete_attachment_routes = Router::new()
        .route(
            "/api/v1/attachments/{id}",
            delete(
                v1::delete_attachment::<
                    AttachmentRepositoryImpl,
                    IssueRepositoryImpl,
                    ProjectRepositoryImpl,
                    MemberRepositoryImpl,
                >,
            ),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(delete_attachment_usecase);

    // Protected route: update attachment
    let protected_update_attachment_routes = Router::new()
        .route(
            "/api/v1/attachments/{id}",
            patch(
                v1::update_attachment::<
                    AttachmentRepositoryImpl,
                    IssueRepositoryImpl,
                    ProjectRepositoryImpl,
                    MemberRepositoryImpl,
                    UserRepositoryImpl,
                >,
            ),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(update_attachment_usecase);

    // Protected route: list project issue categories
    let protected_list_categories_routes = Router::new()
        .route(
            "/api/v1/projects/{id}/issue_categories.json",
            get(v1::list_categories::<
                IssueCategoryRepositoryImpl,
                ProjectRepositoryImpl,
                UserRepositoryImpl,
                MemberRepositoryImpl,
            >),
        )
        .route(
            "/api/v1/projects/{id}/issue_categories",
            get(v1::list_categories::<
                IssueCategoryRepositoryImpl,
                ProjectRepositoryImpl,
                UserRepositoryImpl,
                MemberRepositoryImpl,
            >),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(list_categories_usecase);

    // Protected route: create issue category
    let protected_create_category_routes = Router::new()
        .route(
            "/api/v1/projects/{id}/issue_categories.json",
            post(
                v1::create_category::<
                    IssueCategoryRepositoryImpl,
                    ProjectRepositoryImpl,
                    UserRepositoryImpl,
                    MemberRepositoryImpl,
                >,
            ),
        )
        .route(
            "/api/v1/projects/{id}/issue_categories",
            post(
                v1::create_category::<
                    IssueCategoryRepositoryImpl,
                    ProjectRepositoryImpl,
                    UserRepositoryImpl,
                    MemberRepositoryImpl,
                >,
            ),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(create_category_usecase);

    // Protected route: get issue category by ID
    let protected_get_category_routes = Router::new()
        .route(
            "/api/v1/issue_categories/{id}",
            get(v1::get_category::<
                IssueCategoryRepositoryImpl,
                ProjectRepositoryImpl,
                UserRepositoryImpl,
                MemberRepositoryImpl,
            >),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(get_category_usecase);

    // Protected route: update issue category
    let protected_update_category_routes = Router::new()
        .route(
            "/api/v1/issue_categories/{id}",
            put(v1::update_category::<
                IssueCategoryRepositoryImpl,
                ProjectRepositoryImpl,
                UserRepositoryImpl,
                MemberRepositoryImpl,
            >),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(update_category_usecase);

    // Protected route: delete issue category
    let protected_delete_category_routes = Router::new()
        .route(
            "/api/v1/issue_categories/{id}",
            delete(v1::delete_category::<IssueCategoryRepositoryImpl, MemberRepositoryImpl>),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(delete_category_usecase);

    // Protected route: get project trackers
    let protected_get_project_trackers_routes = Router::new()
        .route(
            "/api/v1/projects/{id}/trackers",
            get(v1::get_project_trackers::<
                ProjectRepositoryImpl,
                MemberRepositoryImpl,
                TrackerRepositoryImpl,
                IssueStatusRepositoryImpl,
            >),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(get_project_trackers_usecase);

    Router::new()
        .merge(routes::health_routes())
        .merge(public_auth_routes)
        .merge(protected_current_user_routes)
        .merge(protected_logout_routes)
        .merge(protected_users_routes)
        .merge(protected_create_user_routes)
        .merge(protected_get_user_routes)
        .merge(protected_update_user_routes)
        .merge(protected_delete_user_routes)
        .merge(protected_projects_routes)
        .merge(protected_get_project_routes)
        .merge(protected_create_project_routes)
        .merge(protected_update_project_routes)
        .merge(protected_delete_project_routes)
        .merge(protected_members_routes)
        .merge(protected_add_member_routes)
        .merge(protected_get_member_routes)
        .merge(protected_update_member_routes)
        .merge(protected_delete_member_routes)
        .merge(protected_list_trackers_routes)
        .merge(protected_get_tracker_routes)
        .merge(protected_create_tracker_routes)
        .merge(protected_update_tracker_routes)
        .merge(protected_delete_tracker_routes)
        .merge(protected_list_issue_statuses_routes)
        .merge(protected_get_issue_status_routes)
        .merge(protected_create_issue_status_routes)
        .merge(protected_update_issue_status_routes)
        .merge(protected_delete_issue_status_routes)
        .merge(protected_list_priorities_routes)
        .merge(protected_list_roles_routes)
        .merge(protected_get_role_routes)
        .merge(protected_create_role_routes)
        .merge(protected_update_role_routes)
        .merge(protected_delete_role_routes)
        .merge(protected_list_issues_routes)
        .merge(protected_get_issue_routes)
        .merge(protected_create_issue_routes)
        .merge(protected_update_issue_routes)
        .merge(protected_delete_issue_routes)
        .merge(protected_list_journals_routes)
        .merge(protected_list_issue_attachments_routes)
        .merge(protected_upload_issue_attachment_routes)
        .merge(protected_list_relations_routes)
        .merge(protected_create_relation_routes)
        .merge(protected_delete_relation_routes)
        .merge(protected_get_relation_routes)
        .merge(protected_upload_routes)
        .merge(protected_get_attachment_routes)
        .merge(protected_download_attachment_routes)
        .merge(protected_delete_attachment_routes)
        .merge(protected_update_attachment_routes)
        .merge(protected_list_categories_routes)
        .merge(protected_create_category_routes)
        .merge(protected_get_category_routes)
        .merge(protected_update_category_routes)
        .merge(protected_delete_category_routes)
        .merge(protected_get_project_trackers_routes)
        .with_state(state)
}
