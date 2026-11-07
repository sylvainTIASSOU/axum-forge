# Axum Forge

[![CI](https://github.com/sylvainTIASSOU/axum-forge/actions/workflows/ci.yml/badge.svg)](https://github.com/sylvainTIASSOU/axum-forge/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)

Axum Forge is a Cargo subcommand that generates structured, immediately
runnable Axum applications without repeating the same project setup.

The `standard` template includes HTTP endpoints, OpenAPI documentation,
Swagger UI, structured logs, request IDs, centralized JSON errors, and
functional tests.

&gt; **Work in progress:** Axum Forge is under active development and has not
&gt; been published on crates.io yet.

## Features

The generated `standard` project provides:

- a Rust 2024 and Axum application;
- `GET /hello` and `GET /health` endpoints;
- OpenAPI documentation and Swagger UI;
- JSON or colored human-readable HTTP logs;
- `x-request-id` generation and propagation;
- centralized JSON error responses;
- functional and unit tests;
- environment-based configuration;
- no runtime dependency on Axum Forge.

## Installation

Install the generator from the source repository:

```bash
git clone https://github.com/sylvainTIASSOU/axum-forge.git
cd axum-forge
cargo install --path . --locked
```

Verify the installation:

```bash
cargo axum-forge --help
```

## Quick start

Generate a new application:

```bash
cargo axum-forge new mon-api
```

Start it:

```bash
cd mon-api
cargo run
```

The following endpoints are available by default:

```text
http://127.0.0.1:3030/hello
http://127.0.0.1:3030/health
http://127.0.0.1:3030/swagger-ui/
http://127.0.0.1:3030/api-docs/openapi.json
```

To create the project inside another parent directory:

```bash
cargo axum-forge new mon-api --path ./projects
```

This creates:

```text
./projects/mon-api
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

Hyphens are preserved in the directory and Cargo package name. They are
converted to underscores for the Rust crate name:

```text
mon-api -&gt; mon_api
```

## Destination safety

The destination directory may either:

- not exist yet;
- exist and be empty.

Axum Forge refuses to generate a project into a non-empty directory or over
an existing file. Existing content is never intentionally overwritten.

## Generated project structure

```text
mon-api/
├── .cargo/
│   └── config.toml
├── src/
│   ├── bin/
│   │   └── main.rs
│   ├── config/
│   │   ├── log.rs
│   │   ├── settings.rs
│   │   ├── telemetry.rs
│   │   └── mod.rs
│   ├── controllers/
│   │   ├── health.rs
│   │   ├── hello.rs
│   │   └── mod.rs
│   ├── error/
│   │   └── mod.rs
│   ├── models/
│   │   ├── message.rs
│   │   └── mod.rs
│   ├── app.rs
│   ├── lib.rs
│   └── openapi.rs
├── tests/
│   ├── api.rs
│   └── errors.rs
├── .env.example
├── .gitignore
├── Cargo.toml
└── README.md
```

## Configuration

The generated application supports the following environment variables:

| Variable | Default | Description |
|---|---|---|
| `HOST` | `0.0.0.0` | Address on which the HTTP server listens |
| `PORT` | `3030` | TCP port used by the HTTP server |
| `RUST_LOG` | `info` | Tracing filter, such as `info`, `debug`, or `trace` |
| `LOG_FORMAT` | `json` | Log output format: `json` or `pretty` |

To use the provided local development configuration:

```bash
cp .env.example .env
cargo run
```

The supplied `.env.example` selects `LOG_FORMAT=pretty` for local
development. Without a `.env` file or an explicit `LOG_FORMAT`, logs use
JSON.

## HTTP logging

Use colored, human-readable logs during development:

```bash
LOG_FORMAT=pretty RUST_LOG=info cargo run
```

Use structured JSON logs for production or log collectors:

```bash
LOG_FORMAT=json RUST_LOG=info cargo run
```

HTTP log fields include:

```text
request_id
method
routeKey
client_ip
status
latency_ms
```

Log levels are selected from the HTTP status:

- `1xx`, `2xx`, and `3xx`: `INFO`;
- `4xx`: `WARN`;
- `5xx`: `ERROR`.

`routeKey` uses the Axum matched route pattern. Unknown routes use
`"unmatched"`.

Request bodies, cookies, authorization headers, and query strings are not
logged. `client_ip` is taken from the direct peer connection;
`X-Forwarded-For` is not trusted automatically.

## Request IDs

Every request receives an `x-request-id`:

- an existing incoming value is propagated;
- otherwise, a UUID is generated;
- the value is included in the response headers, HTTP logs, and JSON errors.

Example:

```bash
curl -i \
  -H 'x-request-id: demo-123' \
  http://127.0.0.1:3030/hello
```

## JSON errors

Errors use a consistent JSON structure:

```json
{
  "error": {
    "code": "not_found",
    "message": "The requested resource was not found",
    "request_id": "demo-123"
  }
}
```

The standard error codes include:

| HTTP status | Code |
|---|---|
| `400` | `bad_request` |
| `404` | `not_found` |
| `405` | `method_not_allowed` |
| `500` | `internal_server_error` |

Internal errors expose only a generic public message. Implementation details
must remain in the server logs and must never be added to the public error
message.

## Generated project development

Run the application normally:

```bash
cargo run
```

Run it with automatic restart:

```bash
cargo dev
```

The `dev` alias requires
[watchexec](https://github.com/watchexec/watchexec):

```bash
cargo install watchexec-cli --locked
```

When a Rust source file or `Cargo.toml` changes, watchexec rebuilds and
restarts the application. This is an automatic restart, not state-preserving
hot reload.

Axum Forge is not required to build, test, or run the generated application.

## Generated project verification

From the generated project directory:

```bash
cargo fmt --all -- --check
cargo test --all-targets
cargo clippy --all-targets --all-features -- -D warnings
```

## Axum Forge development

Clone the repository:

```bash
git clone https://github.com/sylvainTIASSOU/axum-forge.git
cd axum-forge
```

Run the generator without installing it:

```bash
cargo run -- axum-forge new mon-api --path /tmp
```

Run all generator checks:

```bash
cargo fmt --all -- --check
cargo check --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo package
```

The integration tests execute the real generator binary and compile and test
a generated project.

## MVP scope

The MVP intentionally excludes:

- PostgreSQL and SQLx;
- authentication;
- multiple generation profiles;
- Docker;
- Redis;
- OpenTelemetry;
- additional controller generation;
- state-preserving hot reload.

These features may be considered after the MVP has been fully validated.

## License

Axum Forge is distributed under the MIT license. See [LICENSE-MIT](LICENSE-MIT).