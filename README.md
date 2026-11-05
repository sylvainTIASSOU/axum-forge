# Axum Forge

[![CI](https://github.com/sylvainTIASSOU/axum-forge/actions/workflows/ci.yml/badge.svg)](https://github.com/sylvainTIASSOU/axum-forge/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)

Axum Forge is a Cargo subcommand designed to generate structured Axum
applications without manually repeating the same project setup.

 **Work in progress:** the current template generates a minimal Rust project.
&gt; The complete Axum application, OpenAPI documentation, JSON request logging,
&gt; and functional tests are still under development.

## Goals

The project aims to provide a single command that creates a project that is:

- immediately buildable and runnable;
- cleanly structured using standard Rust and Axum;
- documented with OpenAPI and Swagger UI;
- equipped with structured JSON HTTP logs;
- covered by functional tests;
- independent from Axum Forge at runtime.

## Installation

Axum Forge is not published on crates.io yet. You can install it from the
source repository:

```bash
git clone https://github.com/sylvainTIASSOU/axum-forge.git
cd axum-forge
cargo install --path .
```

Verify the installation:

```bash
cargo axum-forge --help
```

## Usage

Create a project in the current directory:

```bash
cargo axum-forge new mon-api
```

Create a project inside another parent directory:

```bash
cargo axum-forge new mon-api --path ./projects
```

The second command creates the project at:

```text
./projects/mon-api
```

After generation, start the project with:

```bash
cd mon-api
cargo run
```

## Project names

Project names must:

- start with a lowercase ASCII letter;
- contain only lowercase letters, digits, hyphens, or underscores.

Valid examples:

```text
mon-api
mon_api
api2
```

Invalid examples:

```text
Mon-API
2api
mon api
../mon-api
```

Hyphens are preserved in the project directory and Cargo package name. They
are converted to underscores when a Rust crate name is required:

```text
mon-api -> mon_api
```

## Destination directories

The destination directory may either:

- not exist yet;
- exist and be empty.

Axum Forge refuses to generate a project into a non-empty directory or over
an existing file. Existing content is never intentionally overwritten.

## Current generated project

The current development template produces the following structure:

```text
mon-api/
├── src/
│   └── main.rs
├── .gitignore
├── Cargo.toml
└── README.md
```

You can verify the generated project with:

```bash
cd mon-api
cargo check
cargo run
```

## Planned standard profile

The complete `standard` profile will provide:

- an Axum HTTP server;
- `/hello` and `/health` endpoints;
- centralized JSON error handling;
- application configuration through environment variables;
- OpenAPI documentation and Swagger UI;
- request identifiers and structured JSON HTTP logs;
- functional tests.

The MVP intentionally excludes:

- PostgreSQL and SQLx;
- authentication;
- multiple profiles;
- Docker;
- Redis;
- OpenTelemetry;
- additional controller generation.

These features may be considered after the MVP is complete and validated.

## Development

Clone the repository:

```bash
git clone https://github.com/sylvainTIASSOU/axum-forge.git
cd axum-forge
```

Run the generator without installing it:

```bash
cargo run -- axum-forge new mon-api --path /tmp
```

On Windows, you can use another existing parent directory:

```powershell
cargo run -- axum-forge new mon-api --path .\sandbox
```

Run the project checks:

```bash
cargo fmt --all -- --check
cargo check --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

Verify the contents and construction of the Cargo package:

```bash
cargo package --list
cargo package
```

## Repository structure

```text
axum-forge/
├── src/
│   ├── main.rs          # Process entry point
│   ├── lib.rs           # Library modules
│   ├── app.rs           # Command dispatch
│   ├── cli.rs           # Clap command-line definitions
│   ├── project.rs       # Project name and destination validation
│   ├── templates.rs     # Template loading and rendering
│   └── commands/
│       ├── mod.rs
│       └── new.rs       # New-project orchestration
├── templates/
│   └── standard/        # Embedded standard template
├── tests/
│   └── cli_new.rs       # Command-line integration tests
├── .github/
│   └── workflows/
│       └── ci.yml       # Continuous integration
├── Cargo.toml
├── Cargo.lock
├── LICENSE-MIT
└── README.md
```

## How the generator works

The `new` command follows these steps:

1. Validate the project name.
2. Compute the destination path.
3. Validate the destination directory.
4. Load the embedded `standard` profile.
5. Render the templates with MiniJinja.
6. Write the generated files.
7. Display the project location and next commands.

Templates are embedded in the executable at compile time. The installed
generator therefore does not require access to the source repository at
runtime.

Files ending in `.j2` are rendered as templates, and the `.j2` extension is
removed from their output paths.

## Contributing

Contributions, bug reports, and suggestions are welcome.

Before submitting a pull request:

1. Keep the change focused.
2. Add or update tests when behavior changes.
3. Run the formatting, Clippy, and test commands listed above.
4. Update the documentation when necessary.

By contributing to this project, you agree that your contributions will be
licensed under the MIT License.

## License

This project is licensed under the MIT License. See [LICENSE-MIT](LICENSE-MIT)
for the full license text.