use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
#[derive(Deserialize, Serialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct HealthResponse {
    pub is_healthy: bool,
    pub message: String,
    pub status: u64,
}
