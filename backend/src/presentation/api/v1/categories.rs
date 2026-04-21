use crate::application::dto::{CreateCategoryRequest, UpdateCategoryRequest};
use crate::application::use_cases::{
    CreateCategoryUseCase, DeleteCategoryUseCase, GetCategoryUseCase, ListCategoriesUseCase,
    UpdateCategoryUseCase,
};
use crate::domain::repositories::{
    IssueCategoryRepository, MemberRepository, ProjectRepository, UserRepository,
};
use crate::presentation::dto::{
    CategoryListJsonResponse, CreateCategoryJsonResponse, GetCategoryJsonResponse,
    UpdateCategoryJsonResponse,
};
use crate::presentation::errors::HttpError;
use crate::presentation::middleware::CurrentUser;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension, Json,
};
use serde::Deserialize;
use std::sync::Arc;
use validator::Validate;

/// Query parameters for delete category endpoint
#[derive(Debug, Deserialize)]
pub struct DeleteCategoryQuery {
    /// Category to reassign issues to when deleting a category with issues
    pub reassign_to_id: Option<i32>,
}

/// List categories endpoint handler
/// GET /api/v1/projects/:id/issue_categories.json
pub async fn list_categories<
    C: IssueCategoryRepository,
    P: ProjectRepository,
    U: UserRepository,
    M: MemberRepository,
>(
    State(usecase): State<Arc<ListCategoriesUseCase<C, P, U, M>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(project_id): Path<i32>,
) -> Result<Json<CategoryListJsonResponse>, HttpError> {
    let response = usecase
        .execute(project_id, current_user.id, current_user.admin)
        .await?;
    Ok(Json(CategoryListJsonResponse::from(response)))
}

/// Get category endpoint handler
/// GET /api/v1/issue_categories/:id.json
pub async fn get_category<
    C: IssueCategoryRepository,
    P: ProjectRepository,
    U: UserRepository,
    M: MemberRepository,
>(
    State(usecase): State<Arc<GetCategoryUseCase<C, P, U, M>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(category_id): Path<i32>,
) -> Result<Json<GetCategoryJsonResponse>, HttpError> {
    let response = usecase
        .execute(category_id, current_user.id, current_user.admin)
        .await?;
    Ok(Json(GetCategoryJsonResponse::from(response)))
}

/// Create category endpoint handler
/// POST /api/v1/projects/:id/issue_categories.json
pub async fn create_category<
    C: IssueCategoryRepository,
    P: ProjectRepository,
    U: UserRepository,
    M: MemberRepository,
>(
    State(usecase): State<Arc<CreateCategoryUseCase<C, P, U, M>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(project_id): Path<i32>,
    Json(request): Json<CreateCategoryRequest>,
) -> Result<(StatusCode, Json<CreateCategoryJsonResponse>), HttpError> {
    // Validate request
    request
        .issue_category
        .validate()
        .map_err(|e| HttpError::BadRequest(format!("Validation error: {}", e)))?;

    let response = usecase
        .execute(
            project_id,
            request.issue_category,
            current_user.id,
            current_user.admin,
        )
        .await?;
    Ok((
        StatusCode::CREATED,
        Json(CreateCategoryJsonResponse::from(response)),
    ))
}

/// Update category endpoint handler
/// PUT /api/v1/issue_categories/:id.json
pub async fn update_category<
    C: IssueCategoryRepository,
    P: ProjectRepository,
    U: UserRepository,
    M: MemberRepository,
>(
    State(usecase): State<Arc<UpdateCategoryUseCase<C, P, U, M>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(category_id): Path<i32>,
    Json(request): Json<UpdateCategoryRequest>,
) -> Result<Json<UpdateCategoryJsonResponse>, HttpError> {
    // Validate request
    if let Some(ref name) = request.issue_category.name {
        if name.is_empty() || name.len() > 60 {
            return Err(HttpError::BadRequest(
                "Name must be between 1 and 60 characters".into(),
            ));
        }
    }

    let response = usecase
        .execute(
            category_id,
            request.issue_category,
            current_user.id,
            current_user.admin,
        )
        .await?;
    Ok(Json(UpdateCategoryJsonResponse::from(response)))
}

/// Delete category endpoint handler
/// DELETE /api/v1/issue_categories/:id.json
pub async fn delete_category<C: IssueCategoryRepository, M: MemberRepository>(
    State(usecase): State<Arc<DeleteCategoryUseCase<C, M>>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(category_id): Path<i32>,
    Query(query): Query<DeleteCategoryQuery>,
) -> Result<StatusCode, HttpError> {
    usecase
        .execute(
            category_id,
            query.reassign_to_id,
            current_user.id,
            current_user.admin,
        )
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
