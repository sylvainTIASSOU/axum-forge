use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Deserialize, Serialize)]
pub struct HealthResponse {
    pub is_healthy: bool,
    pub message: String,
    pub status: u64,
}
