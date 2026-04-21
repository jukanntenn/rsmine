use crate::application::use_cases::ListPrioritiesUseCase;
use crate::domain::repositories::EnumerationRepository;
use crate::presentation::dto::PriorityListJsonResponse;
use crate::presentation::errors::HttpError;
use crate::presentation::middleware::CurrentUser;
use axum::{extract::State, Extension, Json};
use std::sync::Arc;

/// List issue priorities endpoint handler
/// GET /api/v1/enumerations/issue_priorities.json
pub async fn list_priorities<E: EnumerationRepository>(
    State(usecase): State<Arc<ListPrioritiesUseCase<E>>>,
    Extension(_current_user): Extension<CurrentUser>,
) -> Result<Json<PriorityListJsonResponse>, HttpError> {
    let response = usecase.execute().await?;
    Ok(Json(PriorityListJsonResponse::from(response)))
}
