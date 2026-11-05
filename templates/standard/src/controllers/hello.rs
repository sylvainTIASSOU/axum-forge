use crate::models::message::MessageResponse;
use axum::{Json, debug_handler, http::StatusCode};

#[debug_handler]
pub async fn hello_handler() -> (StatusCode, Json<MessageResponse>) {
    (
        StatusCode::OK,
        Json(MessageResponse {
            message: "Hello World!".to_string(),
        }),
    )
}

