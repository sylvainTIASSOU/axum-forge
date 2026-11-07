use axum::{
    Json,
    body::Body,
    http::{
        HeaderValue, Request, StatusCode,
        header::{CONTENT_LENGTH, CONTENT_TYPE},
    },
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    pub error: ErrorBody,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ErrorBody {
    pub code: String,
    pub message: String,
    pub request_id: String,
}

impl ErrorResponse {
    fn new(
        code: impl Into<String>,
        message: impl Into<String>,
        request_id: impl Into<String>,
    ) -> Self {
        Self {
            error: ErrorBody {
                code: code.into(),
                message: message.into(),
                request_id: request_id.into(),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppError {
    status: StatusCode,
    code: &'static str,
    // this message must always be intended for the client.
    message: String,
}

impl AppError {
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code: "bad_request",
            message: message.into(),
        }
    }

    pub fn not_found() -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            code: "not_found",
            message: "The requested resource was not found".to_owned(),
        }
    }

    pub fn internal() -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            code: "internal_server_error",
            message: "An unexpected error occurred".to_owned(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = ErrorResponse::new(
            self.code,
            self.message.clone(),
            // the middleware will replace this value with the real request ID.
            "unknown",
        );

        let mut response = (self.status, Json(body)).into_response();
        // the middleware get these metadata without having to decode the response body.
        response.extensions_mut().insert(self);

        response
    }
}

pub async fn normalize_errors(
    request: Request<Body>,
    next: Next,
) -> Response {
    let request_id = request
        .headers()
        .get("x-request-id")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("unknown")
        .to_owned();

    let response = next.run(request).await;
    let status = response.status();

    if !status.is_client_error() && !status.is_server_error() {
        return response;
    }

    let (code, message) = response
        .extensions()
        .get::<AppError>()
        .map(|error| {
            (error.code.to_owned(), error.message.clone())
        })
        .unwrap_or_else(|| default_error(status));

    let error_response = ErrorResponse::new(code, message, request_id);

    // We keep the status and headers of the original response, notably Allow for 405 responses.
    let (mut parts, _) = response.into_parts();

    parts.headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );

    // the old body is replaced with a new JSON body.
    parts.headers.remove(CONTENT_LENGTH);

    Response::from_parts(
        parts,
        Json(error_response).into_response().into_body(),
    )
}

fn default_error(status: StatusCode) -> (String, String) {
    let (code, message) = match status {
        StatusCode::BAD_REQUEST => (
            "bad_request",
            "The request is invalid",
        ),
        StatusCode::NOT_FOUND => (
            "not_found",
            "The requested resource was not found",
        ),
        StatusCode::METHOD_NOT_ALLOWED => (
            "method_not_allowed",
            "The HTTP method is not allowed for this resource",
        ),
        _ if status.is_server_error() => (
            "internal_server_error",
            "An unexpected error occurred",
        ),
        _ => (
            "http_error",
            status.canonical_reason().unwrap_or("HTTP error"),
        ),
    };

    (code.to_owned(), message.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal_errors_do_not_expose_implementation_details() {
        let error = AppError::internal();

        assert_eq!(
            error.status,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(error.code, "internal_server_error");
        assert_eq!(error.message, "An unexpected error occurred");
    }

    #[test]
    fn unknown_routes_have_a_public_not_found_message() {
        let (code, message) = default_error(StatusCode::NOT_FOUND);

        assert_eq!(code, "not_found");
        assert_eq!(
            message,
            "The requested resource was not found"
        );
    }
}