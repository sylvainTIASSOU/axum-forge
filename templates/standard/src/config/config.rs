use anyhow::{Context, Result};
use console::style;
#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub host: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let port = std::env::var("PORT").unwrap_or_else(|_| "3030".to_string());

        let port = port
            .parse::<u16>()
            .with_context(|| format!("PORT must be a valid u16, got `{port}`"))?;

        let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

        Ok(Self { port, host })
    }
}

use console::style;

pub fn print_startup_banner(base_url: &str) {
    let swagger_url = format!("{base_url}/swagger-ui/");
    let openapi_url = format!("{base_url}/api-docs/openapi.json");

    println!();

    println!(
        "{}",
        style(
            r#"                ⚒️
       ╔═══════════════╗
       ║  AXUM FORGE   ║
       ╚═══════════════╝"#
        )
        .color256(208)
        .bold()
    );

    println!();

    println!(
        "           {}",
        style("●  DEVELOPMENT SERVER READY").green().bold()
    );

    println!("           {}", style("Forge your next API.").dim());

    println!();
    println!(
        "  {}",
        style("────────────────────────────────────────────────────────────").dim()
    );

    println!(
        "  {}  {}",
        style("SERVER  ").color256(208).bold(),
        style(format!("{base_url} ↗")).cyan().underlined()
    );

    println!(
        "  {}  {}",
        style("SWAGGER ").color256(208).bold(),
        style(format!("{swagger_url} ↗")).cyan().underlined()
    );

    println!(
        "  {}  {}",
        style("OPENAPI ").color256(208).bold(),
        style(format!("{openapi_url} ↗")).cyan().underlined()
    );

    println!(
        "  {}",
        style("────────────────────────────────────────────────────────────").dim()
    );

    println!();

    println!(
        "  {}  {} to stop the server\n",
        style("●").green(),
        style("Ctrl+C").bold()
    );
}
