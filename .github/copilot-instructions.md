# battlenet-rs Development Guidelines

Auto-generated from all feature plans. Last updated: 2026-04-08

## Active Technologies
- Python 3.13+ (managed with `uv`) + Flask, redis, requests, python-dotenv (002-bnauth-oauth-helper)
- Redis on `rpi53` (host: `rpi53`, port: 6379, auth: `REDISCLI_AUTH` env var) (002-bnauth-oauth-helper)

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
- 002-bnauth-oauth-helper: Added Python 3.13+ (managed with `uv`) + Flask, redis, requests, python-dotenv

- 001-repo-baseline-audit: Added Rust 1.94.0 (stable, edition 2021) + reqwest 0.12, serde/serde_json 1.x, tokio 1.x,

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
