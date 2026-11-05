use crate::config::config::Config;
use crate::controllers::{hello, health};
use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;

pub struct App;

impl App {
    pub async fn run() {
        dotenvy::dotenv().ok();

        let config = Config::from_env().unwrap_or_else(|err| {
            eprintln!("Configurations error: {}", err);
            std::process::exit(1);
        });

        let app = Router::new()
            .route("/health", get(health::health_handler))
            .route("/hello", get(hello::hello_handler));

        let listener = TcpListener::bind(format!("{}:{}", config.host, config.port))
            .await
            .unwrap();

        println!("==========================Axum server start ==========================");
        println!(
            "          Server is listening on: http://{}",
            listener.local_addr().unwrap()
        );
        println!("=======================================================================");

        axum::serve(listener, app).await.unwrap();
    }
}
