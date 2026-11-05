use crate::models::message::HealthResponse;
use axum::{Json, debug_handler, http::StatusCode};


#[debug_handler]
pub async fn health_handler() -> (StatusCode, Json<HealthResponse>) {
    (
        StatusCode::OK,
        Json(HealthResponse {
            is_healthy: true,
            message: "The application is healthy".to_string(),
            status: 200,
        }),
    )
}
