use crate::config::settings::{Config, print_startup_banner};
use crate::config::{log, telemetry};
use crate::controllers::{health, hello};
use crate::error::normalize_errors;
use crate::openapi::ApiDoc;
use anyhow::{Context, Result};
use axum::middleware::from_fn;
use axum::{Router, http::HeaderName, routing::get};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn build_router() -> Router {
    let request_id_header = HeaderName::from_static("x-request-id");

    Router::new()
        .route("/health", get(health::health_handler))
        .route("/hello", get(hello::hello_handler))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(from_fn(normalize_errors))
        .layer(PropagateRequestIdLayer::new(request_id_header.clone()))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(telemetry::make_span_with)
                .on_response(telemetry::on_response)
                .on_failure(telemetry::on_failure),
        )
        .layer(SetRequestIdLayer::new(request_id_header, MakeRequestUuid))
}

pub struct App;

impl App {
    pub async fn run() -> Result<()> {
        dotenvy::dotenv().ok();

        // Configuration errors are propagated to main.
        let config = Config::from_env()?;
        log::setup()?;

        let address = format!("{}:{}", config.host, config.port);

        let listener = TcpListener::bind(&address)
            .await
            .with_context(|| format!("Failed to bind server to {address}"))?;

        let local_address = listener
            .local_addr()
            .context("Failed to read the server local address")?;

        let display_host = match config.host.as_str() {
            // 0.0.0.0 is a listening address, while 127.0.0.1 is
            // more convenient for opening the application locally.
            "0.0.0.0" => "127.0.0.1",
            host => host,
        };

        let base_url = format!("http://{}:{}", display_host, local_address.port());

        print_startup_banner(&base_url);

        axum::serve(
            listener,
            build_router().into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await
        .context("Axum server failed")?;

        Ok(())
    }
}
