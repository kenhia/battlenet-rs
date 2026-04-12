# Quickstart: Database Cache Layer & Rate Limiting

**Branch**: `004-db-cache` | **Date**: 2026-04-10

## Prerequisites

- Rust 1.94.0+ with `cargo`
- For SQLite: no external dependencies (bundled)
- For Postgres: running PostgreSQL server with connection URL
- Battle.net API credentials (`BNET_CLIENT_ID`, `BNET_CLIENT_SECRET`)

## Setup

### 1. Add feature flags to `Cargo.toml`

```toml
[dependencies]
battlenet-rs = { path = ".", features = ["wow", "db-sqlite"] }
# or for Postgres:
# battlenet-rs = { path = ".", features = ["wow", "db-postgres"] }
```

### 2. Set environment variables

```bash
export BNET_CLIENT_ID="your-client-id"
export BNET_CLIENT_SECRET="your-client-secret"

# For SQLite (file path):
export DATABASE_URL="sqlite:cache.db"

# For Postgres:
# export DATABASE_URL="postgres://user:pass@gratch:5432/battlenet_cache"
```

### 3. Use CachedClient

```rust
use battlenet_rs::client::BattleNetClient;
use battlenet_rs::cache::{CachedClient, SqliteCacheStore};  // or PostgresCacheStore
use battlenet_rs::wow_models::mount::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the base client with rate limiting
    let client = BattleNetClient::new_from_environment()
        .await?
        .with_rate_limiter(Default::default());

    // Create a cache store and wrap the client
    let store = SqliteCacheStore::new("sqlite:cache.db").await?;
    let cached = CachedClient::new(client, store).await?;

    // First call: fetches from API, caches result
    let mounts: MountsIndex = cached.get_data(&Default::default()).await?;
    println!("Mounts: {}", mounts.mounts.len());

    // Second call: returns from cache (no API call)
    let mounts2: MountsIndex = cached.get_data(&Default::default()).await?;

    // Force refresh: bypasses cache
    let fresh: MountsIndex = cached.get_data_force(&Default::default()).await?;

    Ok(())
}
```

## Build & Test

```bash
# Build with SQLite cache
cargo build --features "wow,db-sqlite"

# Build with Postgres cache
cargo build --features "wow,db-postgres"

# Run tests
cargo test --all-features

# Run specific cache tests
cargo test --features "wow,db-sqlite" cache
```

## Validation Steps

1. `cargo fmt --check` — formatting clean
2. `cargo clippy --all-features -- -D warnings` — no warnings
3. `cargo test --all-features` — all tests pass
4. `cargo build --examples --features "wow,db-sqlite"` — examples compile
