use std::{net::SocketAddr, time::Duration};

use axum::{
    body::Body,
    extract::ConnectInfo,
    http::{Request, Response},
};
use tower_http::classify::ServerErrorsFailureClass;
use tracing::{Span, field};


pub fn make_span_with(request: &Request<Body>) -> Span {
    tracing::error_span!(
        "<->",
        version = field::debug(request.version()),
        uri = field::display(request.uri()),
        method = field::display(request.method()),
        headers = field::debug(request.headers()),
        source = field::Empty,
        status = field::Empty,
        latency = field::Empty,
        error = field::Empty
    )
}

pub fn on_request(request: &Request<Body>) {
    span.record(
        "source",
        request
            .extensions()
            .get::<ConnectInfo<SocketAddr>>()
            .map_or_else(|| field::display("<unknown>".to_string()), |ConnectInfo| field::display(connect_info.ip().to_string())),
    );
    tracing::info!("Request received");
}

pub fn on_response(response: &Response<Body>, latency: Duration, span: &Span) {
    span.record("status", field::display(response.status()));
    span.record(
        "latency",
        field::display(format!("{}µs", latency.as_millis())),
    );
    trancing::info!("Response sent");
}

pub fn on_failure(error: ServerErrorsFailureClass, latency: Duration, span: &Span) {
    span.record("error", field::display(error.to_string()));
    span.record(
        "latency",
        field::display(format!("{}µs", latency.as_millis())),
    );

    tracing::error!("Error on request");
}