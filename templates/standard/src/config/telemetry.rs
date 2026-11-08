use std::{net::SocketAddr, time::Duration};

use axum::{
    body::Body,
    extract::{ConnectInfo, MatchedPath},
    http::{HeaderName, Request, Response, StatusCode},
};
use tower_http::classify::ServerErrorsFailureClass;
use tracing::{Level, Span, field};

pub fn make_span_with(request: &Request<Body>) -> Span {
    let route_key = request
        .extensions()
        .get::<MatchedPath>()
        .map_or("unmatched", MatchedPath::as_str);

    let request_id = request
        .headers()
        .get(HeaderName::from_static("x-request-id"))
        .and_then(|value| value.to_str().ok())
        .unwrap_or("unknown");

    // This is the IP address of the directly connected peer.
    // X-Forwarded-For is intentionally not trusted.
    let client_ip = request
        .extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|info| info.0.ip().to_string())
        .unwrap_or_else(|| "unknown".to_owned());

    tracing::info_span!(
        "http_request",
        request_id = %request_id,
        method = %request.method(),
        routeKey = route_key,
        client_ip = %client_ip,
        status = field::Empty,
        latency_ms = field::Empty,
        error = field::Empty,
    )
}

pub fn on_response(response: &Response<Body>, latency: Duration, span: &Span) {
    let status = response.status();

    span.record("status", u64::from(status.as_u16()));
    span.record("latency_ms", latency_to_ms(latency));

    match level_for_status(status) {
        Level::ERROR => {
            tracing::error!("HTTP request completed");
        }
        Level::WARN => {
            tracing::warn!("HTTP request completed");
        }
        _ => {
            tracing::info!("HTTP request completed");
        }
    }
}

fn level_for_status(status: StatusCode) -> Level {
    if status.is_server_error() {
        Level::ERROR
    } else if status.is_client_error() {
        Level::WARN
    } else {
        Level::INFO
    }
}

pub fn on_failure(error: ServerErrorsFailureClass, latency: Duration, span: &Span) {
    span.record("latency_ms", latency_to_ms(latency));
    span.record("error", field::display(error));

    tracing::error!("HTTP request failed");
}

fn latency_to_ms(latency: Duration) -> u64 {
    u64::try_from(latency.as_millis()).unwrap_or(u64::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn successful_responses_are_logged_as_info() {
        assert_eq!(level_for_status(StatusCode::OK), Level::INFO);
        assert_eq!(level_for_status(StatusCode::CREATED), Level::INFO);
        assert_eq!(level_for_status(StatusCode::NOT_MODIFIED), Level::INFO);
    }

    #[test]
    fn client_errors_are_logged_as_warnings() {
        assert_eq!(level_for_status(StatusCode::BAD_REQUEST), Level::WARN);
        assert_eq!(level_for_status(StatusCode::NOT_FOUND), Level::WARN);
    }

    #[test]
    fn server_errors_are_logged_as_errors() {
        assert_eq!(
            level_for_status(StatusCode::INTERNAL_SERVER_ERROR),
            Level::ERROR
        );
        assert_eq!(level_for_status(StatusCode::BAD_GATEWAY), Level::ERROR);
    }
}
