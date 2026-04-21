use super::AppState;
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use chrono::Utc;
use sea_orm::ConnectionTrait;
use serde::Serialize;

/// Health check response
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Health check routes - no /api prefix for Kubernetes probes
pub fn health_routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(health_check))
        .route("/ready", get(readiness_check))
}

/// Health check - service is running
/// Returns 200 OK if the service is up
async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        timestamp: Utc::now().to_rfc3339(),
        error: None,
    })
}

/// Readiness check - service is ready to accept requests
/// Returns 200 OK if database is connected, 503 if not
async fn readiness_check(
    State(state): State<AppState>,
) -> Result<Json<HealthResponse>, (StatusCode, Json<HealthResponse>)> {
    // Get the database backend type
    let backend = state.db.get_database_backend();

    // Build the appropriate ping query for the backend
    let ping_sql = match backend {
        sea_orm::DatabaseBackend::Sqlite => "SELECT 1",
        sea_orm::DatabaseBackend::Postgres => "SELECT 1",
        sea_orm::DatabaseBackend::MySql => "SELECT 1",
        _ => "SELECT 1", // Default for any future backend types
    };

    // Check database connectivity by executing a simple query
    match state.db.execute_unprepared(ping_sql).await {
        Ok(_) => Ok(Json(HealthResponse {
            status: "ok".to_string(),
            timestamp: Utc::now().to_rfc3339(),
            error: None,
        })),
        Err(e) => Err((
            StatusCode::SERVICE_UNAVAILABLE,
            Json(HealthResponse {
                status: "error".to_string(),
                timestamp: Utc::now().to_rfc3339(),
                error: Some(format!("Database connection failed: {}", e)),
            }),
        )),
    }
}
