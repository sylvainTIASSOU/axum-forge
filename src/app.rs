use crate::cli::{Cli, Commands, GeneratorCommand};
use crate::commands;
use anyhow::Result;

pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::AxumForge(generator_cli) => match generator_cli.command {
            GeneratorCommand::New(args) => commands::new::run_new(args),
        },
    }
}
