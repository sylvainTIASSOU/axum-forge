use crate::config::config::Config;
use crate::controllers::{health, hello};
use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;
use anyhow::{Context, Result};



pub fn build_router() -> Router {
    Router::new()
        .route("/health", get(health::health_handler))
        .route("/hello", get(hello::hello_handler))
}

pub struct App;

impl App {
    pub async fn run() -> Result<()> {
        dotenvy::dotenv().ok();

        // L'erreur de configuration est remontée à main.
        let config = Config::from_env()?;
        let address = format!("{}:{}", config.host, config.port);

        let listener = TcpListener::bind(&address)
            .await
            .with_context(|| format!("Failed to bind server to {address}"))?;

        let local_address = listener
            .local_addr()
            .context("Failed to read the server local address")?;

        println!("Server is listening on http://{local_address}");

        axum::serve(listener, build_router())
            .await
            .context("Axum server failed")?;

        Ok(())
    }
}
