# Quickstart: Library Setup, WoW Retail API Coverage, and Examples

**Branch**: `003-lib-wow-examples` | **Date**: 2026-04-09

## Prerequisites

- Rust 1.94.0+ (stable)
- Battle.net API credentials (Client ID + Secret) from [Blizzard Developer Portal](https://develop.battle.net/)
- `.env` file configured (see `docs/installation.md`)

## Quick Verification Steps

### 1. Feature flags compile correctly

```bash
# Core only (no game-specific code)
cargo check

# WoW Game Data APIs
cargo check --features wow

# WoW + User Profile APIs
cargo check --features wow,user

# Everything
cargo check --all-features

# Stub features (should compile with no effect)
cargo check --features wow-classic
cargo check --features diablo
```

### 2. Run tests

```bash
# All tests with all features
cargo test --all-features

# Only WoW Game Data tests
cargo test --features wow

# Only WoW + Profile tests
cargo test --features wow,user
```

### 3. Run an example (Game Data — client credentials only)

```bash
# Fetch mount index
cargo run --example mounts --features wow
```

### 4. Run a Profile API example (requires user OAuth token)

```bash
# First, ensure you have a user token (e.g., via bnauth from sprint 002)
cd bnauth && uv run python -m bnauth.app
# Complete the OAuth flow in your browser, then:
cd ..
cargo run --example account-profile --features wow,user,redis
```

### 5. Verify the full quality gate

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

## Key Patterns

### Calling a Game Data endpoint

```rust
use battlenet_rs::client::BattleNetClient;
use battlenet_rs::wow_models::prelude::*;

let client = BattleNetClient::new_from_environment();

// Index (no params)
let mounts: MountsIndexResult = client.get_data(&UrlArgs::None).await;

// By ID
let mount: MountResult = client.get_data(&UrlArgs::Id { id: 6 }).await;

// Raw JSON
let json: MountsIndexJsonResult = client.get_json::<MountsIndex>(&UrlArgs::None).await;
```

### Calling a Profile API endpoint (user token required)

```rust
use battlenet_rs::client::BattleNetClient;
use battlenet_rs::wow_models::prelude::*;

let client = BattleNetClient::new_from_environment();
let user_token = "your_user_oauth_token";

let profile: AccountProfileSummaryResult = client
    .get_data_with_token(&UrlArgs::None, user_token)
    .await;

let char_achievements: CharacterAchievementsSummaryResult = client
    .get_data_with_token(
        &UrlArgs::Player {
            realm_slug: "trollbane".to_string(),
            name: "belarsa".to_string(),
        },
        user_token,
    )
    .await;
```

### Adding a dependency with features

```toml
[dependencies]
battlenet-rs = { path = "../battlenet-rs", features = ["wow"] }

# Or for Profile APIs too:
battlenet-rs = { path = "../battlenet-rs", features = ["wow", "user"] }
```
