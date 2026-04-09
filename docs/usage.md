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
of implemented and planned API endpoints.

Currently implemented:
- **Achievement API** (5 endpoints): Categories Index, Category, Index, Achievement, Media
- **Character Profile API** (2 endpoints): Summary, Status
- **Connected Realm API** (2 endpoints): Index, by ID
- **WoW Token API** (1 endpoint): Token Index
