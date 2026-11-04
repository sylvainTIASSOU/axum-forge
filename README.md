# Axum Forge

[![CI](https://github.com/sylvainTIASSOU/axum-forge/actions/workflows/ci.yml/badge.svg)](https://github.com/sylvainTIASSOU/axum-forge/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)

Axum Forge is a Cargo subcommand designed to generate structured Axum
applications without manually repeating the same project setup.

&gt; **Work in progress:** the current template generates a minimal Rust project.
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
