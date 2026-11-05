use anyhow::{Context, Result};

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
