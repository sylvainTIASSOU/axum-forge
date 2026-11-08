# Changelog

All notable changes to Axum Forge are documented in this file.

## 0.1.1

### Fixed

- Reject project names that are reserved Rust keywords.
- Prevent reserved project names from creating a destination directory.
- Render only `.j2` files with MiniJinja.
- Copy non-template files without interpreting their contents.
- Improve CLI help, error messages, and source documentation.

## 0.1.0

### Added

- Initial release of the `standard` Axum template.
- `/hello` and `/health` endpoints.
- OpenAPI documentation and Swagger UI.
- Structured HTTP logs and request ID propagation.
- Centralized JSON errors.
- Unit and functional tests.
- Verification of a real generated project.