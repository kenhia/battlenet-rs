# Usage

## Adding battlenet-rs as a Dependency

Add to your `Cargo.toml`:

```toml
[dependencies]
battlenet-rs = { path = "../battlenet-rs" }
dotenvy = "0.15"
tokio = { version = "1", features = ["full"] }
```

> Once published to crates.io, replace the `path` with a version specifier.

## Creating a Client

### From Environment Variables

```rust
use battlenet_rs::client::BattleNetClient;

let _ = dotenvy::from_filename(".env");
let client = BattleNetClient::new_from_environment();
```

Required env vars: `BATTLENET_CLIENT_ID`, `BATTLENET_CLIENT_SECRET`.
Optional: `BATTLENET_REGION` (default: `US`), `BATTLENET_LOCALE` (default: `en_US`), `BATTLENET_API_TIMEOUT` (default: `5` seconds).

### From Explicit Values

```rust
use battlenet_rs::client::BattleNetClient;
use battlenet_rs::region::BattleNetRegion;

let client = BattleNetClient::new(
    BattleNetRegion::US,
    "en_US",
    "your_client_id",
    "your_client_secret",
);
```

## Making API Calls

All API models and types are available through the prelude:

```rust
use battlenet_rs::wow_models::prelude::*;
```

### Fetching Deserialized Data

Use `client.get_data::<T>()` to fetch and deserialize into a Rust struct:

```rust
use battlenet_rs::client::BattleNetClient;
use battlenet_rs::wow_models::prelude::*;

#[tokio::main]
async fn main() {
    let _ = dotenvy::from_filename(".env");
    let client = BattleNetClient::new_from_environment();

    // Fetch WoW Token price (no URL args needed)
    let result: WowTokenIndexResult = client.get_data(&UrlArgs::None).await;
    match result {
        Ok(token) => {
            let price_in_gold = token.price / 10000;
            println!("WoW Token: {} gold", price_in_gold);
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
```

### Fetching Raw JSON

Use `client.get_json::<T>()` to get the raw JSON string:

```rust
let json: WowTokenIndexJsonResult = client.get_json::<WowTokenIndex>(&UrlArgs::None).await;
match json {
    Ok(text) => println!("{text}"),
    Err(e) => eprintln!("Error: {:?}", e),
}
```

### URL Arguments

Different endpoints require different arguments:

```rust
// No arguments (index endpoints)
let result: AchievementsIndexResult = client.get_data(&UrlArgs::None).await;

// ID-based lookup
let result: AchievementResult = client.get_data(&UrlArgs::Id { id: 8 }).await;

// Character-based lookup
let args = UrlArgs::Player {
    realm_slug: "trollbane".to_string(),
    name: "belarsa".to_string(),
};
let result: CharacterProfileResult = client.get_data(&args).await;
```

## Error Handling

All API calls return `Result<T, BattleNetClientError>`:

```rust
use battlenet_rs::errors::BattleNetClientError;

let result: WowTokenIndexResult = client.get_data(&UrlArgs::None).await;
match result {
    Ok(data) => { /* use data */ }
    Err(BattleNetClientError::ClientRequestFailed(e)) => {
        eprintln!("HTTP error: {e}");
    }
    Err(BattleNetClientError::SerdeJsonError(e)) => {
        eprintln!("JSON parse error: {e}");
    }
    Err(BattleNetClientError::ClientTokenNotAvailable) => {
        eprintln!("Token not available — check credentials");
    }
    Err(e) => eprintln!("Other error: {e}"),
}
```

## Understanding Namespaces and Regions

BattleNet API data is scoped by **region** and **namespace**:

- **Region** determines which geographic API server to query (US, EU, KR, TW, CN)
- **Namespace** determines the type of data:
  - `static` — Game data that rarely changes (achievements, items, classes)
  - `dynamic` — Frequently updated data (auction prices, realm status, token prices)
  - `profile` — Character and account-specific data

The client handles namespace and region strings automatically based on each
endpoint model's configuration. You only need to set the region and locale
when creating the client.

## Available Endpoints

See [ModelImplementProgress.md](../ModelImplementProgress.md) for the full list
of implemented API endpoints.

**Game Data APIs** (feature `wow`): ~130 endpoints across 33 modules — achievements,
auction house, creatures, items, mounts, mythic keystone, professions, PvP, quests,
realms, talents, and more.

**Profile APIs** (feature `wow,user`): ~37 endpoints across 17 modules — account
profile, character achievements, collections, encounters, equipment, professions,
PvP, quests, reputations, and more.

## Game Data Example

Enable with `--features wow`:

```rust
use battlenet_rs::client::BattleNetClient;
use battlenet_rs::wow_models::prelude::*;

#[tokio::main]
async fn main() {
    let _ = dotenvy::from_filename(".env");
    let client = BattleNetClient::new_from_environment();

    // Fetch all mounts
    let result: MountsIndexResult = client.get_data(&UrlArgs::None).await;

    // Fetch a specific item by ID
    let result: ItemResult = client.get_data(&UrlArgs::Id { id: 19019 }).await;
}
```

## Profile API Example

Enable with `--features wow,user,redis`:

```rust
use battlenet_rs::client::BattleNetClient;
use battlenet_rs::user_token::read_user_token;
use battlenet_rs::wow_models::prelude::*;

#[tokio::main]
async fn main() {
    let _ = dotenvy::from_filename(".env");
    let client = BattleNetClient::new_from_environment();
    let token = read_user_token().expect("Run bnauth first");

    let args = UrlArgs::Player {
        realm_slug: "trollbane".to_string(),
        name: "belarsa".to_string(),
    };
    let result: CharacterEquipmentSummaryResult =
        client.get_data_with_token(&args, &token.access_token).await;
}
```

### Feature Flag Reference

| Flag | Enables | Dependency |
|------|---------|------------|
| `wow` | Game Data API models (33 modules, ~130 endpoints) | — |
| `user` | Profile API models (17 modules, ~37 endpoints) | `wow` |
| `redis` | Redis user token reader | — |
| `db-sqlite` | SQLite-backed API response cache | — |
| `db-postgres` | PostgreSQL-backed API response cache | — |

**Note**: `db-sqlite` and `db-postgres` are mutually exclusive.

## Rate Limiting

Attach a rate limiter to any `BattleNetClient` to stay within Blizzard's API limits:

```rust
use battlenet_rs::client::BattleNetClient;
use battlenet_rs::rate_limiter::RateLimiterConfig;

let client = BattleNetClient::new_from_environment()
    .with_rate_limiter(RateLimiterConfig::default()); // 100/s, 36k/hr

// Enable "nice" mode for shared environments (50/s)
let config = RateLimiterConfig {
    nice_mode: true,
    ..Default::default()
};
let client = BattleNetClient::new_from_environment()
    .with_rate_limiter(config);
```

## CachedClient (Database-Backed Caching)

Enable with `--features db-sqlite` (or `db-postgres`). `CachedClient` wraps
`BattleNetClient` and caches responses based on namespace policy:

```rust
use battlenet_rs::client::BattleNetClient;
use battlenet_rs::cache::cached_client::CachedClient;
use battlenet_rs::cache::sqlite::SqliteCacheStore;
use battlenet_rs::wow_models::prelude::*;

#[tokio::main]
async fn main() {
    let client = BattleNetClient::new_from_environment();
    let store = SqliteCacheStore::new("sqlite:cache.db").await.unwrap();
    let cached = CachedClient::new(client, store).await.unwrap();

    // First call fetches from API and caches
    let mounts: MountsIndex = cached.get_data(&UrlArgs::None).await.unwrap();

    // Second call returns from cache instantly (static namespace)
    let mounts: MountsIndex = cached.get_data(&UrlArgs::None).await.unwrap();

    // Force refresh bypasses cache
    let mounts: MountsIndex = cached.get_data_force(&UrlArgs::None).await.unwrap();
}
```

**Namespace Policies**:
- **Static**: Cache-first — returns from database if present, fetches only on miss
- **Dynamic**: Always fetches from API, caches afterward for analytics
- **Profile**: Cache-first with 30-day TTL validation per Blizzard ToS 2.R

### Cached Profile Example

The `account-profile-cached` example combines all three features — user token
from Redis, CachedClient with SQLite, and force-refresh:

```sh
cargo run --example account-profile-cached --features "wow user redis db-sqlite"
```

This fetches your account profile via the API (force refresh), caches it in
SQLite, then fetches again from cache to demonstrate the speedup.

## User Token (bnauth + Redis)

For user-scoped endpoints (account profile, character list, collections) you
need a user OAuth token obtained via the `bnauth` Flask app.

### Running bnauth

```sh
cd bnauth
uv run python -m bnauth.app
```

Visit `http://localhost:5050`, click **Get Battle.net Auth**, log in, and the
token is saved to Redis automatically.

### Reading the User Token from Rust

Enable the `redis` feature:

```toml
[dependencies]
battlenet-rs = { path = "../battlenet-rs", features = ["redis"] }
```

```rust
use battlenet_rs::user_token::read_user_token;
use battlenet_rs::errors::BattleNetClientError;

fn main() {
    match read_user_token() {
        Ok(token) => {
            println!("Token: {}", token.access_token);
            println!("Expires at: {}", token.expires_at);
            println!("Scope: {}", token.scope);
        }
        Err(BattleNetClientError::UserTokenNotAvailable) => {
            eprintln!("No user token — run bnauth to authorize");
        }
        Err(BattleNetClientError::RedisError(e)) => {
            eprintln!("Redis connection failed: {e}");
        }
        Err(e) => eprintln!("Error: {e}"),
    }
}
```

The user token is separate from the client credentials token. The client token
is used automatically by `BattleNetClient` for Game Data endpoints; the user
token must be read explicitly via `read_user_token()` for user-scoped calls.

## Full Character Download

Enable with `--features "wow,user"`. The `full_character()` function downloads
all 28 character profile endpoints in a single call:

```rust
use battlenet_rs::client::BattleNetClient;
use battlenet_rs::wow_models::full_character::*;

#[tokio::main]
async fn main() {
    let _ = dotenvy::from_filename(".env");
    let client = BattleNetClient::new_from_environment();

    // Fetch all character data (client credentials — no user token needed)
    let fc = full_character(&client, "trollbane", "belarsa", None)
        .await
        .expect("character should exist");

    println!("{} @ {}", fc.character_name, fc.realm_slug);
    if let Some(ref profile) = fc.profile {
        println!("Level {} {}", profile.level, profile.race.name);
    }

    // Check which endpoints succeeded/failed
    println!("Errors: {:?}", fc.errors);
}
```

### JSON Output

```rust
let json = full_character_json(&client, "trollbane", "belarsa", None)
    .await
    .expect("should succeed");
println!("{json}");
```

### With Cache

```rust
use battlenet_rs::cache::cached_client::CachedClient;
use battlenet_rs::cache::sqlite::SqliteCacheStore;

let store = SqliteCacheStore::new("sqlite:cache.db").await.unwrap();
let cached = CachedClient::new(client, store).await.unwrap();

// First call fetches from API, second call returns from cache
let fc1 = full_character(&cached, "trollbane", "belarsa", None).await.unwrap();
let fc2 = full_character(&cached, "trollbane", "belarsa", None).await.unwrap(); // instant

// Force bypass cache
let fc3 = full_character_force(&cached, "trollbane", "belarsa", None).await.unwrap();
```

### Running the Example

```sh
cargo run --example full-toon --features "wow,user"
```
