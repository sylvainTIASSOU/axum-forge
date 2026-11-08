use std::{env::VarError, io::IsTerminal};

use anyhow::{Context, Result, bail};
use tracing_subscriber::EnvFilter;

pub fn setup() -> Result<()> {
    let filter = match std::env::var("RUST_LOG") {
        Ok(value) => EnvFilter::try_new(value).context("Invalid RUST_LOG value")?,
        Err(VarError::NotPresent) => EnvFilter::new("info"),
        Err(error) => {
            bail!("Failed to read RUST_LOG: {error}");
        }
    };

    let format = std::env::var("LOG_FORMAT").unwrap_or_else(|_| "json".to_owned());

    match format.as_str() {
        "json" => {
            tracing_subscriber::fmt()
                .with_env_filter(filter)
                .json()
                .with_ansi(false)
                .with_current_span(true)
                .with_span_list(false)
                .with_writer(std::io::stdout)
                .try_init()
                .map_err(|error| {
                    anyhow::anyhow!("Failed to initialize the tracing subscriber: {error}")
                })?;
        }
        "pretty" => {
            tracing_subscriber::fmt()
                .with_env_filter(filter)
                .pretty()
                // Disable colors when output is redirected to a file
                // or another program.
                .with_ansi(std::io::stdout().is_terminal())
                .with_writer(std::io::stdout)
                .try_init()
                .map_err(|error| {
                    anyhow::anyhow!("Failed to initialize the tracing subscriber: {error}")
                })?;
        }
        other => {
            bail!("Invalid LOG_FORMAT value `{other}`: expected `json` or `pretty`");
        }
    }

    Ok(())
}
