# Public API Contract: battlenet-rs

**Branch**: `003-lib-wow-examples` | **Date**: 2026-04-09

This document defines the public API surface of the `battlenet-rs` library crate after sprint 003.

## Cargo Features

```toml
[features]
default = []
wow = []
user = []
redis = ["dep:redis"]
wow-classic = []
diablo = []
hearthstone = []
starcraft = []
```

## Core API (always available)

```rust
// src/client.rs
pub struct BattleNetClient { .. }

impl BattleNetClient {
    pub fn new(region, locale, client_id, client_secret) -> Self;
    pub fn new_with_timeout(region, locale, client_id, client_secret, timeout) -> Self;
    pub fn new_from_environment() -> Self;
    pub async fn get_access_token(&self) -> BattlenetClientResult<String>;
    pub async fn send_request(&self, url: String) -> BattlenetClientResult<Response>;
    pub async fn get_data<T: Deserialize + GenerateUrl>(&self, url_args: &UrlArgs) -> Result<T, Error>;
    pub async fn get_json<T: GenerateUrl>(&self, url_args: &UrlArgs) -> Result<String, Error>;

    // NEW: User-token-authenticated requests (for Profile APIs)
    pub async fn send_request_with_token(&self, url: String, token: &str) -> BattlenetClientResult<Response>;
    pub async fn get_data_with_token<T: Deserialize + GenerateUrl>(&self, url_args: &UrlArgs, token: &str) -> Result<T, Error>;
    pub async fn get_json_with_token<T: GenerateUrl>(&self, url_args: &UrlArgs, token: &str) -> Result<String, Error>;
}

pub fn json_to_struct<T: Deserialize>(json: &str) -> Result<T, Error>;

// src/region.rs
pub enum BattleNetRegion { US, EU, KR, TW, CN }

// src/namespace.rs
pub enum WowNamespace { Static, Dynamic, Profile }

// src/errors.rs
pub enum BattleNetClientError {
    ClientRequestFailed(reqwest::Error),
    SerdeJsonError(serde_json::Error),
    ClientTokenNotAvailable,
    ClientTokenMutex(String),
    UserTokenNotAvailable,
    RedisError(redis::RedisError),  // [redis feature only]
    Unknown,
}

// src/auth.rs
pub struct AccessTokenResponse { .. }
```

## WoW API (feature: `wow`)

```rust
// src/wow_models/mod.rs (wow_models.rs)
pub enum UrlArgs {
    None,
    Id { id: u64 },
    Player { realm_slug: String, name: String },
    Guild { realm_slug: String, name_slug: String },         // NEW
    TwoIds { id1: u64, id2: u64 },                          // NEW
    ThreeIds { id1: u64, id2: u64, id3: u64 },              // NEW
    PlayerExtra { realm_slug: String, name: String, extra: String }, // NEW
    TwoStrings { first: String, second: String },            // NEW
    Search { params: Vec<(String, String)> },                // NEW
}

pub trait GenerateUrl {
    fn url(client: &BattleNetClient, url_args: &UrlArgs) -> String;
}

pub mod prelude; // Re-exports all public model types

// Search response wrapper
pub struct SearchResult<T> {
    pub page: u32,
    pub page_size: u32,
    pub max_page_size: u32,
    pub page_count: u32,
    pub results: Vec<SearchResultEntry<T>>,
}
pub struct SearchResultEntry<T> {
    pub key: HrefLink,
    pub data: T,
}
```

### Per-Endpoint Contract Pattern

Every endpoint model follows this pattern:

```rust
// Example: MountsIndex
#[derive(Debug, Deserialize)]
pub struct MountsIndex { /* fields matching JSON response */ }

pub type MountsIndexResult = Result<MountsIndex, BattleNetClientError>;
pub type MountsIndexJsonResult = Result<String, BattleNetClientError>;

impl GenerateUrl for MountsIndex {
    fn url(client: &BattleNetClient, url_args: &UrlArgs) -> String { .. }
}
```

### Game Data Endpoint Modules (30+ modules)

Each module listed in `data-model.md` exports:
- One or more public model structs (deriving `Debug, Deserialize`)
- Result type aliases (`{Name}Result`, `{Name}JsonResult`)
- `GenerateUrl` implementations

All are re-exported via `wow_models::prelude`.

## Profile API (features: `wow` + `user`)

Profile API modules follow the same pattern but:
- Use `WowNamespace::Profile` for URL construction
- Require the caller to provide a user OAuth token via `get_data_with_token` / `get_json_with_token`

Profile endpoint modules are conditionally compiled:
```rust
// In wow_models/mod.rs
#[cfg(feature = "user")]
pub mod account_profile;
#[cfg(feature = "user")]
pub mod character_achievements;
// ... etc
```

## Redis Feature (unchanged from sprint 002)

```rust
// src/user_token.rs — #[cfg(feature = "redis")]
pub struct UserAccessToken { .. }
pub fn read_user_token() -> BattlenetClientResult<UserAccessToken>;
```

## Examples Convention

```toml
# In Cargo.toml
[[example]]
name = "mounts"
required-features = ["wow"]

[[example]]
name = "account-profile"
required-features = ["wow", "user"]
```

Each example:
1. Loads `.env` via `dotenvy`
2. Creates `BattleNetClient::new_from_environment()`
3. Calls one or more endpoints
4. Prints formatted output
5. Documents required features in file header comment
