# battlenet-rs Development Guidelines

Auto-generated from all feature plans. Last updated: 2026-04-10

## Active Technologies
- Python 3.13+ (managed with `uv`) + Flask, redis, requests, python-dotenv (002-bnauth-oauth-helper)
- Redis on `rpi53` (host: `rpi53`, port: 6379, auth: `REDISCLI_AUTH` env var) (002-bnauth-oauth-helper)
- Rust 1.94.0 (stable, edition 2021) + reqwest 0.12, serde/serde_json 1.x, tokio 1.x, thiserror 1.x, model-macro (local proc-macro crate) (003-lib-wow-examples)
- N/A (database explicitly out of scope) (003-lib-wow-examples)
- Rust 1.94.0 (stable, edition 2021) + reqwest 0.12, serde/serde_json 1.x, tokio 1.x, thiserror 1.x, model-macro (local proc-macro), **sqlx 0.8** (new), **chrono 0.4** (existing) (004-db-cache)
- SQLite (WAL mode) via `db-sqlite` feature; PostgreSQL via `db-postgres` feature (004-db-cache)

- Rust 1.94.0 (stable, edition 2021) + reqwest 0.12, serde/serde_json 1.x, tokio 1.x, (001-repo-baseline-audit)

## Project Structure

```text
src/
tests/
```

## Commands

cargo test [ONLY COMMANDS FOR ACTIVE TECHNOLOGIES][ONLY COMMANDS FOR ACTIVE TECHNOLOGIES] cargo clippy

## Code Style

Rust 1.94.0 (stable, edition 2021): Follow standard conventions

## Recent Changes
- 004-db-cache: Added Rust 1.94.0 (stable, edition 2021) + reqwest 0.12, serde/serde_json 1.x, tokio 1.x, thiserror 1.x, model-macro (local proc-macro), **sqlx 0.8** (new), **chrono 0.4** (existing)
- 003-lib-wow-examples: Added Rust 1.94.0 (stable, edition 2021) + reqwest 0.12, serde/serde_json 1.x, tokio 1.x, thiserror 1.x, model-macro (local proc-macro crate)
- 002-bnauth-oauth-helper: Added Python 3.13+ (managed with `uv`) + Flask, redis, requests, python-dotenv


<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
