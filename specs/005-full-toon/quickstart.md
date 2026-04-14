# Quickstart: Full Character Download

**Feature**: 005-full-toon | **Date**: 2026-04-12

## Prerequisites

- Rust 1.94.0+ (stable)
- Valid `.env` file with `BATTLENET_CLIENT_ID` and `BATTLENET_CLIENT_SECRET`
- Feature flags: `wow` (required), `user` (for profile modules), `db-sqlite` (optional, for caching)

## Build

```sh
# Core (no cache)
cargo build --features "wow,user"

# With SQLite cache
cargo build --features "wow,user,db-sqlite"

# With Redis token reader + SQLite cache
cargo build --features "wow,user,redis,db-sqlite"
```

## Test

```sh
# Unit tests
cargo test --features "wow,user,db-sqlite" --lib --tests

# Specific test file
cargo test --features "wow,user" --test full_character_test
```

## Run Example

```sh
# Basic (client credentials only)
cargo run --example full-toon --features "wow,user"

# With cache
cargo run --example full-toon --features "wow,user,db-sqlite"
```

## Verify

1. Build succeeds with `--features "wow,user"`
2. `cargo test --features "wow,user" --test full_character_test` passes
3. `cargo run --example full-toon --features "wow,user"` produces character output
4. JSON round-trip: serialize to JSON and deserialize back without data loss
5. Pre-commit suite passes: `cargo fmt --check && cargo clippy --features "wow,db-sqlite,user" --lib --tests --examples -- -D warnings`
