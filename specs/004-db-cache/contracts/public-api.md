# Public API Contracts: Database Cache Layer & Rate Limiting

**Branch**: `004-db-cache` | **Date**: 2026-04-10

## New Public Types

### `src/cache/mod.rs`

```rust
use async_trait::async_trait;
use chrono::{DateTime, Utc};

/// A cached API response entry.
#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub cache_key: String,
    pub namespace: String,           // "static", "dynamic", "profile"
    pub response: String,            // JSON
    pub fetched_at: DateTime<Utc>,
    pub character_id: Option<u64>,
    pub realm_slug: Option<String>,
    pub char_name: Option<String>,
}

/// Errors from cache operations.
#[derive(Debug, thiserror::Error)]
pub enum CacheError {
    #[error("database error: {0}")]
    DatabaseError(String),
    #[error("serialization error: {0}")]
    SerializationError(String),
    #[error("schema initialization error: {0}")]
    SchemaInitError(String),
}

/// Abstract cache storage backend.
#[async_trait]
pub trait CacheStore: Send + Sync {
    async fn initialize(&self) -> Result<(), CacheError>;
    async fn get(&self, key: &str) -> Result<Option<CacheEntry>, CacheError>;
    async fn put(&self, entry: &CacheEntry) -> Result<(), CacheError>;
    async fn delete(&self, key: &str) -> Result<(), CacheError>;
    async fn delete_character(&self, realm_slug: &str, char_name: &str) -> Result<(), CacheError>;
    async fn refresh_character_timestamp(
        &self, realm_slug: &str, char_name: &str,
    ) -> Result<(), CacheError>;
}
```

### `src/cache/cached_client.rs`

```rust
/// Cache-aware wrapper around BattleNetClient.
pub struct CachedClient<S: CacheStore> {
    pub client: BattleNetClient,
    store: S,
    ttl_days: u32,
}

impl<S: CacheStore> CachedClient<S> {
    /// Create a new CachedClient wrapping an existing client and cache store.
    pub async fn new(client: BattleNetClient, store: S) -> Result<Self, CacheError>;

    /// Create with custom TTL (default: 30 days).
    pub async fn new_with_ttl(client: BattleNetClient, store: S, ttl_days: u32) -> Result<Self, CacheError>;

    /// Get typed data, using cache based on namespace policy.
    pub async fn get_data<T>(&self, url_args: &UrlArgs) -> Result<T, BattleNetClientError>
    where T: for<'a> Deserialize<'a> + Serialize + GenerateUrl;

    /// Get typed data, bypassing cache (force refresh).
    pub async fn get_data_force<T>(&self, url_args: &UrlArgs) -> Result<T, BattleNetClientError>
    where T: for<'a> Deserialize<'a> + Serialize + GenerateUrl;

    /// Get raw JSON, using cache based on namespace policy.
    pub async fn get_json<T>(&self, url_args: &UrlArgs) -> Result<String, BattleNetClientError>
    where T: Serialize + GenerateUrl;

    /// Get typed data with user token, using cache.
    pub async fn get_data_with_token<T>(
        &self, url_args: &UrlArgs, token: &str,
    ) -> Result<T, BattleNetClientError>
    where T: for<'a> Deserialize<'a> + Serialize + GenerateUrl;

    /// Get typed data with user token, bypassing cache (force refresh).
    pub async fn get_data_with_token_force<T>(
        &self, url_args: &UrlArgs, token: &str,
    ) -> Result<T, BattleNetClientError>
    where T: for<'a> Deserialize<'a> + Serialize + GenerateUrl;
}
```

### `src/rate_limiter.rs`

```rust
/// Configuration for the API rate limiter.
#[derive(Debug, Clone)]
pub struct RateLimiterConfig {
    pub per_second: u32,        // default: 100
    pub per_hour: u32,          // default: 36_000
    pub nice_mode: bool,        // default: false
    pub nice_per_second: u32,   // default: 50
}

impl Default for RateLimiterConfig { /* 100/s, 36000/h, nice=false, nice=50/s */ }

/// Dual-window rate limiter.
pub struct RateLimiter { /* internal state */ }

impl RateLimiter {
    pub fn new(config: RateLimiterConfig) -> Self;

    /// Wait until a request slot is available, then consume it.
    pub async fn acquire(&self);

    /// Check current remaining tokens (for diagnostics).
    pub fn remaining_per_second(&self) -> u32;
    pub fn remaining_per_hour(&self) -> u32;
}
```

## Modified Public Types

### `src/errors.rs` — new variant

```rust
pub enum BattleNetClientError {
    // ... existing variants ...

    /// Cache layer error.
    #[error("cache error: {0}")]
    CacheError(#[from] CacheError),
}
```

### `src/wow_models.rs` — `GenerateUrl` trait extension

```rust
pub trait GenerateUrl {
    fn url(client: &BattleNetClient, url_args: &UrlArgs) -> String;

    /// Returns the namespace for cache behavior. Default: Static.
    fn cache_namespace() -> WowNamespace {
        WowNamespace::Static
    }
}
```

### `src/client.rs` — rate limiter integration

```rust
impl BattleNetClient {
    // Existing constructors remain unchanged.

    /// Attach a rate limiter to this client.
    pub fn with_rate_limiter(self, config: RateLimiterConfig) -> Self;

    // send_request and send_request_with_token internally call
    // self.rate_limiter.acquire().await before making HTTP requests.
}
```

### `model-macro/src/lib.rs` — bendpoint changes

- Emits `#[derive(Debug, Serialize, Deserialize)]` instead of `#[derive(Debug, Deserialize)]`
- Emits `fn cache_namespace() -> WowNamespace` based on the `namespace` attribute

## New Feature Flags

```toml
[features]
db-sqlite = ["dep:sqlx", "sqlx/sqlite"]
db-postgres = ["dep:sqlx", "sqlx/postgres", "sqlx/tls-native-tls"]
```

Mutually exclusive — enabling both produces a compile error.

## New Dependencies

```toml
[dependencies]
sqlx = { version = "0.8", optional = true, features = ["runtime-tokio", "chrono", "json"] }
async-trait = "0.1"
log = "0.4"
```
