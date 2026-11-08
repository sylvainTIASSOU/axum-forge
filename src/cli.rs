use camino::Utf8PathBuf;
use clap::{Args, Parser, Subcommand};

/// Principal structure corresponding to the command line.
#[derive(Debug, Parser)]
#[command(
    name = "cargo",
    bin_name = "cargo",
    version,
    about = "Generate a structured Axum application",
    long_about = "\
Generate a structured Axum application with a single command.

Axum Forge creates a ready-to-build project using the standard profile,
including routing, configuration, OpenAPI documentation, JSON request logging,
and functional tests.

The generated project uses standard Rust and Axum, without requiring Axum Forge
at runtime.",
    after_help = "\
EXAMPLES:
    cargo axum-forge new mon-api
    cargo axum-forge new mon-api --path ./projects

Use 'cargo axum-forge new --help' for more information about project creation."
)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(
        name = "axum-forge",
        version,
        about = "Generate and manage Axum projects",
        long_about = "\
Generate and manage Axum projects from reusable templates.

The current version supports creating a new project with the standard profile."
    )]
    AxumForge(GeneratorCli),
}

#[derive(Debug, Args)]
pub struct GeneratorCli {
    #[command(subcommand)]
    pub command: GeneratorCommand,
}

#[derive(Debug, Subcommand)]
pub enum GeneratorCommand {
    #[command(
        name = "new",
        about = "Create a new Axum project using the standard profile",
        long_about = "\
Create a new Axum project using the standard profile.

The command validates the project name and checks the destination directory
before generating any files. An existing non-empty directory is never
overwritten.

The generated application includes a basic HTTP API, OpenAPI documentation,
Swagger UI, JSON request logging, and functional tests.",
        after_help = "\
EXAMPLES:
    Create a project in the current directory:
        cargo axum-forge new mon-api

    Create a project inside another parent directory:
        cargo axum-forge new mon-api --path ./projects

    The second example creates the project at:
        ./projects/mon-api"
    )]
    New(NewArgs),
}

#[derive(Debug, Args)]
pub struct NewArgs {
    ///  Project name and Cargo package name.
    #[arg(
        value_name = "NAME",
        help = "Name of the project and its Cargo package",
        long_help = "\
Name of the project and its Cargo package.

This name is also used as the project directory name. It must start with a
lowercase ASCII letter and may contain lowercase letters, digits, hyphens,
and underscores.

Examples of valid names: mon-api, mon_api, api2."
    )]
    pub name: String,

    /// Parent directory in which to create the project.
    #[arg(
        long,
        value_name = "DIRECTORY",
        default_value = ".",
        help = "Parent directory in which to create the project",
        long_help = "\
Parent directory in which to create the project.

The final destination will be DIRECTORY/NAME. The destination may either be
missing or be an existing empty directory. If it contains any files, the
command fails without modifying its contents.

By default, the project is created in the current directory."
    )]
    pub path: Utf8PathBuf,
}
