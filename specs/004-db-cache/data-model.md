# Data Model: Database Cache Layer & Rate Limiting

**Branch**: `004-db-cache` | **Date**: 2026-04-10

## Entities

### CacheEntry

A stored API response in the cache database.

| Field | Type | Description |
|-------|------|-------------|
| `cache_key` | `String` (PK) | Normalized endpoint URL |
| `namespace` | `String` | `"static"`, `"dynamic"`, or `"profile"` |
| `response` | `String` | JSON-serialized API response body |
| `fetched_at` | `chrono::DateTime<Utc>` | Timestamp when data was fetched from API |
| `character_id` | `Option<u64>` | Character ID for profile-namespace entries (TTL validation) |
| `realm_slug` | `Option<String>` | Realm slug for profile entries (character grouping) |
| `char_name` | `Option<String>` | Character name for profile entries (character grouping) |

**Relationships**: None (flat key-value cache).

**Validation Rules**:
- `cache_key` must be non-empty and unique
- `namespace` must be one of `static`, `dynamic`, `profile`
- `response` must be valid JSON
- `fetched_at` must be a valid UTC timestamp
- For profile-namespace entries: `character_id`, `realm_slug`, `char_name` must all be `Some`

**State Transitions**: None (entries are created, updated, or deleted; no state machine).

---

### RateLimiterConfig

Configuration for the API rate limiter. Not persisted — runtime only.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `per_second` | `u32` | `100` | Max requests per second |
| `per_hour` | `u32` | `36_000` | Max requests per hour |
| `nice_mode` | `bool` | `false` | Enable reduced throughput mode |
| `nice_per_second` | `u32` | `50` | Per-second limit in nice mode |

---

### RateLimiter

Runtime state for the rate limiter. Not persisted.

| Field | Type | Description |
|-------|------|-------------|
| `config` | `RateLimiterConfig` | Active configuration |
| `second_tokens` | `AtomicU32` | Tokens remaining in current 1-second window |
| `second_window_start` | `Mutex<Instant>` | Start of current 1-second window |
| `hour_tokens` | `AtomicU32` | Tokens remaining in current 1-hour window |
| `hour_window_start` | `Mutex<Instant>` | Start of current 1-hour window |

---

### CacheStore (Trait)

```rust
#[async_trait]
pub trait CacheStore: Send + Sync {
    /// Initialize the cache schema (create tables if not exist).
    async fn initialize(&self) -> Result<(), CacheError>;

    /// Get a cached entry by key.
    async fn get(&self, key: &str) -> Result<Option<CacheEntry>, CacheError>;

    /// Store or update a cache entry.
    async fn put(&self, entry: &CacheEntry) -> Result<(), CacheError>;

    /// Delete a cache entry by key.
    async fn delete(&self, key: &str) -> Result<(), CacheError>;

    /// Delete all cache entries for a specific character.
    async fn delete_character(&self, realm_slug: &str, char_name: &str) -> Result<(), CacheError>;

    /// Update the `fetched_at` timestamp for all entries matching a character.
    async fn refresh_character_timestamp(
        &self,
        realm_slug: &str,
        char_name: &str,
    ) -> Result<(), CacheError>;
}
```

---

### CachedClient

```rust
pub struct CachedClient<S: CacheStore> {
    client: BattleNetClient,
    store: S,
    ttl_days: u32,  // default: 30
}
```

**Methods** mirror `BattleNetClient`:
- `get_data<T>(&self, url_args: &UrlArgs) -> Result<T, BattleNetClientError>` — cache-aware
- `get_data_force<T>(&self, url_args: &UrlArgs) -> Result<T, BattleNetClientError>` — bypass cache
- `get_json<T>(&self, url_args: &UrlArgs) -> Result<String, BattleNetClientError>` — cache-aware
- `get_data_with_token<T>(&self, url_args: &UrlArgs, token: &str) -> Result<T, BattleNetClientError>` — profile data, cache-aware
- Same `_force` variants for all

---

### CacheError

```rust
pub enum CacheError {
    /// Database connection or query error
    DatabaseError(String),
    /// Serialization/deserialization error
    SerializationError(String),
    /// Schema initialization failed
    SchemaInitError(String),
}
```

Integrated into `BattleNetClientError` as a new variant:
```rust
#[error("cache error: {0}")]
CacheError(#[from] CacheError),
```

---

### GenerateUrl Trait Extension

```rust
pub trait GenerateUrl {
    fn url(client: &BattleNetClient, url_args: &UrlArgs) -> String;

    /// Returns the namespace type for cache behavior determination.
    /// Default: Static (most common).
    fn cache_namespace() -> WowNamespace {
        WowNamespace::Static
    }
}
```

The `bendpoint` macro emits `cache_namespace()` based on its `namespace` attribute.

---

## Database Schema

### SQLite

```sql
CREATE TABLE IF NOT EXISTS cache_entries (
    cache_key    TEXT PRIMARY KEY,
    namespace    TEXT NOT NULL CHECK(namespace IN ('static', 'dynamic', 'profile')),
    response     TEXT NOT NULL,
    fetched_at   TEXT NOT NULL,        -- ISO 8601 (SQLite has no native timestamp)
    character_id INTEGER,
    realm_slug   TEXT,
    char_name    TEXT
);
CREATE INDEX IF NOT EXISTS idx_cache_namespace ON cache_entries(namespace);
CREATE INDEX IF NOT EXISTS idx_cache_character ON cache_entries(realm_slug, char_name);
```

SQLite WAL mode enabled on connection:
```sql
PRAGMA journal_mode=WAL;
```

### PostgreSQL

```sql
CREATE TABLE IF NOT EXISTS cache_entries (
    cache_key    TEXT PRIMARY KEY,
    namespace    TEXT NOT NULL CHECK(namespace IN ('static', 'dynamic', 'profile')),
    response     TEXT NOT NULL,
    fetched_at   TIMESTAMPTZ NOT NULL,
    character_id BIGINT,
    realm_slug   TEXT,
    char_name    TEXT
);
CREATE INDEX IF NOT EXISTS idx_cache_namespace ON cache_entries(namespace);
CREATE INDEX IF NOT EXISTS idx_cache_character ON cache_entries(realm_slug, char_name);
```
