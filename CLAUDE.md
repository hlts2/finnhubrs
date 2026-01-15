# CLAUDE.md

## Project Overview

finnhub-rs is a Rust client library for the Finnhub API.
Based on auto-generated code from OpenAPI specification, providing an idiomatic Rust API.

OpenAPI spec sourced from: https://github.com/Finnhub-Stock-API/finnhub-go/blob/master/api/openapi.yaml

## Development Philosophy

### 1. OpenAPI First

- `api/openapi.yaml` is the single source of truth
- Always update OpenAPI spec first, then regenerate code for API changes
- Avoid manually adding endpoints

### 2. Rust Idioms

- Use `Result<T, E>` for error handling
- Use `Option<T>` for nullable fields
- Async/await for asynchronous operations
- Prefer references over unnecessary `.clone()`

### 3. Minimal Dependencies

- HTTP client: `reqwest` (async)
- Serialization: `serde` / `serde_json`
- Do not add unnecessary dependencies

### 4. Code Generation Fixes

Fix known openapi-generator bugs manually:
- `models::serde_json::Value` -> `serde_json::Value`

## Build Commands

```bash
# Build
cargo build

# Test
cargo test

# Generate documentation
cargo doc --open

# Release build
cargo build --release
```

## Project Structure

```
finnhub-rs/
├── api/
│   └── openapi.yaml     # OpenAPI spec (source of truth)
├── src/
│   ├── lib.rs           # Crate root
│   ├── apis/            # API client implementation
│   └── models/          # Data models
└── docs/                # Generated documentation
```
