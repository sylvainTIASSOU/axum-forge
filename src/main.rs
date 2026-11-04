use anyhow::{Context, Result};
use cargo_axum_forge::app::run;
use cargo_axum_forge::cli::Cli;
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();

    run(cli).context("Command faild retry")?;
    Ok(())
}
