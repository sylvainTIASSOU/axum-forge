use crate::models::message::MessageResponse;
use axum::{Json, debug_handler, http::StatusCode};

#[debug_handler]
#[utoipa::path(
    get,
    path = "/hello",
    tag = "hello",
    responses(
        (
            status = 200,
            description = "Return a greeting message",
            body = MessageResponse
        )
    )
)]
pub async fn hello_handler() -> (StatusCode, Json<MessageResponse>) {
    (
        StatusCode::OK,
        Json(MessageResponse {
            message: "Hello World!".to_string(),
        }),
    )
}

