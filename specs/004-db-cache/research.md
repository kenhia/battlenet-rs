# Research: Database Cache Layer & Rate Limiting

**Branch**: `004-db-cache` | **Date**: 2026-04-10

## R1: sqlx Database Abstraction

### Decision: Use sqlx 0.8 with compile-time feature flags (`db-sqlite`, `db-postgres`)

### Rationale

sqlx 0.8 is the dominant async SQL toolkit in Rust. It supports SQLite, Postgres, and MySQL with the same API surface (`sqlx::query`, `sqlx::Pool`, etc.). Key features for this project:

- **Async-native** with tokio support (`runtime-tokio` feature)
- **Connection pooling** built-in via `sqlx::Pool<DB>`
- **No ORM** — raw SQL queries, which keeps caching logic simple (just store/retrieve JSON blobs)
- **Compile-time checked queries** available via `sqlx::query!` macro (optional; we'll use runtime `sqlx::query` to avoid requiring `DATABASE_URL` at build time)
- **chrono support** via `chrono` feature flag for timestamp columns

**Feature flag mapping** in `Cargo.toml`:

```toml
[dependencies]
sqlx = { version = "0.8", optional = true, features = ["runtime-tokio", "chrono", "json"] }

[features]
db-sqlite = ["dep:sqlx", "sqlx/sqlite"]
db-postgres = ["dep:sqlx", "sqlx/postgres", "sqlx/tls-native-tls"]
```

**CacheStore trait**: A generic async trait that both backends implement. The trait is defined in `src/cache/mod.rs` and is database-agnostic. The concrete type used is selected at compile time via feature flags.

### Alternatives Considered

- **sqlx `any` driver** (runtime dispatch via connection URL): Simpler to use but adds overhead from dynamic dispatch and prevents compile-time optimization. The user explicitly chose compile-time feature flags.
- **diesel**: Sync-only ORM. Doesn't fit the async tokio architecture.
- **sea-orm**: Full ORM built on sqlx. Overkill for storing JSON blobs in a single table.
- **Raw sqlite3/postgres crates**: No shared abstraction. Would duplicate all query logic.

---

## R2: Cache Storage Schema

### Decision: Single `cache_entries` table storing serialized JSON with metadata

### Rationale

The cache stores entire API responses as JSON text, not decomposed relational data. This is the simplest approach because:

1. Every endpoint model already implements `Serialize` + `Deserialize` (after this sprint)
2. The cache key is the endpoint URL (or a normalized hash of it)
3. No schema changes needed when new endpoints are added
4. JSON storage works identically on SQLite and Postgres

**Schema**:

```sql
CREATE TABLE IF NOT EXISTS cache_entries (
    cache_key    TEXT PRIMARY KEY,        -- normalized endpoint URL
    namespace    TEXT NOT NULL,           -- 'static', 'dynamic', 'profile'
    response     TEXT NOT NULL,           -- JSON-serialized API response
    fetched_at   TIMESTAMP NOT NULL,      -- when the data was fetched
    character_id BIGINT,                  -- character ID for profile data (TTL validation)
    realm_slug   TEXT,                    -- realm slug for profile data (TTL grouping)
    char_name    TEXT                     -- character name for profile data (TTL grouping)
);
CREATE INDEX IF NOT EXISTS idx_cache_namespace ON cache_entries(namespace);
CREATE INDEX IF NOT EXISTS idx_cache_character ON cache_entries(realm_slug, char_name);
```

**Cache key normalization**: Use the full URL minus the `locale` parameter (since locale doesn't change the data, only the display strings — though for simplicity we'll include it in v1). This means the same endpoint called from different locales gets cached separately, which is correct.

### Alternatives Considered

- **Per-entity tables** (one table per endpoint type): Proper normalization but massive schema, impractical for 167 endpoints. Would require schema changes for every new endpoint.
- **Binary serialization** (bincode/msgpack): Faster but not human-readable. JSON is debuggable and the performance difference is negligible for a cache.

---

## R3: CachedClient Design

### Decision: Wrapper struct that owns a CacheStore and delegates to BattleNetClient

### Rationale

`CachedClient` wraps `BattleNetClient` and adds cache-aware behavior based on the namespace type:

- **Static**: Check cache first. On hit, return cached data. On miss, call API → cache → return.
- **Dynamic**: Always call API → cache → return. (Cached data is a record, not a speed optimization.)
- **Profile**: Always call API → cache with timestamp → return. On retrieval from cache (for TTL validation), check 30-day window.

The wrapper provides the same method signatures as `BattleNetClient` (`get_data<T>`, `get_json<T>`, etc.) plus a `force_refresh` option.

**Key design consideration**: `CachedClient` needs to know the namespace type for a given request. Currently, the namespace is baked into each struct's `GenerateUrl::url()` method, not exposed as metadata. Two options:

1. **Add a `namespace()` method to `GenerateUrl` trait**: Returns the `WowNamespace` for the struct. This is the cleanest approach but requires updating the trait and all impls.
2. **Parse namespace from the generated URL**: The URL contains `?namespace=static-us` — we can extract it. Fragile but requires no trait changes.
3. **Add a separate trait `CachePolicy`**: Defines caching behavior per type. Most flexible but most code.

**Decision**: Option 1 — extend `GenerateUrl` with a `fn namespace() -> WowNamespace` method. The `bendpoint` macro already knows the namespace from its attribute; we'll have it emit the method. Manual impls add it straightforwardly. This is a minimal, non-breaking addition (add with a default impl returning static to avoid breaking existing code).

### Alternatives Considered

- **Middleware/interceptor pattern**: Too complex for a library. Appropriate for web frameworks, not client libraries.
- **Caching inside `BattleNetClient`**: Couples caching to the core client, which should remain simple.

---

## R4: Rate Limiter Architecture

### Decision: Dual-window token bucket with configurable limits and "nice" mode

### Rationale

The Blizzard API enforces two independent limits:
- **Per-second**: 100 requests/second (burst limit)
- **Per-hour**: 36,000 requests/hour (sustained limit, effectively 10/sec)

A simple token-bucket algorithm handles both:

1. **Per-second bucket**: Refills 100 tokens/second. Each request consumes 1 token. If empty, wait until next second.
2. **Per-hour bucket**: Refills 36,000 tokens/hour. Each request consumes 1 token. If empty, wait until hourly window resets.
3. **Nice mode**: Overrides per-second bucket to 50 tokens/second (or user-configured value).

Implementation: `tokio::sync::Semaphore` or a custom struct with `tokio::time::Instant` tracking. A `RateLimiter` struct wraps both windows and exposes an `async fn acquire(&self)` that waits if needed before returning.

The rate limiter is shared via `Arc<RateLimiter>` between `BattleNetClient` and `CachedClient`. It's integrated at the `send_request` / `send_request_with_token` level so ALL outbound calls are governed.

**Configuration**:

```rust
pub struct RateLimiterConfig {
    pub per_second: u32,       // default: 100
    pub per_hour: u32,         // default: 36_000
    pub nice_mode: bool,       // default: false
    pub nice_per_second: u32,  // default: 50
}
```

### Alternatives Considered

- **governor crate**: Popular Rust rate limiter. Adds a dependency but is battle-tested. Good alternative if our custom impl proves insufficient — but for two simple windows, a custom approach avoids the dependency.
- **tower middleware**: Designed for server-side rate limiting. Overkill for a client library.
- **Leaky bucket**: Similar to token bucket but slightly different semantics. Token bucket is simpler for burst + sustained limits.

---

## R5: Serialize Round-Trip for Model Structs

### Decision: Add `Serialize` derive to all model structs; update `bendpoint` macro to emit both derives

### Rationale

Currently, all model structs derive only `Deserialize` (for parsing API responses). To cache them as JSON, they need `Serialize` too.

**Changes needed**:

1. **`bendpoint` macro**: Change `#[derive(Debug, Deserialize)]` → `#[derive(Debug, Serialize, Deserialize)]` in `model-macro/src/lib.rs`
2. **Manual structs**: Add `Serialize` derive to all structs in `core_structs.rs`, `core_other.rs`, `character_profile.rs`, and search data structs
3. **Import**: `use serde::{Deserialize, Serialize};` in all affected files (the macro expansion will need `Serialize` in scope)

The `bendpoint` macro emits bare `Serialize` and `Deserialize` identifiers in the derive — these resolve in the calling module's scope, where `use serde::{Deserialize, Serialize}` must be present. Currently only `use serde::Deserialize` is imported; each module needs `Serialize` added.

### Alternatives Considered

- **Store raw JSON text from API response instead of re-serializing**: Would work but prevents the cache from storing modified/enriched data. Also inconsistent — if we later want to store computed fields, we'd need Serialize anyway.
- **serde_json::Value as cache format**: Avoids needing Serialize on structs but loses type safety on reads.

---

## R6: 30-Day TTL Enforcement

### Decision: Automatic validation on cache retrieval for profile-namespace entries older than 30 days

### Rationale

Per Blizzard ToS Section 2.R, cached character data must be validated after 30 days. The enforcement flow:

1. `CachedClient` retrieves a profile-namespace cache entry
2. If `fetched_at` is ≤ 30 days old → return cached data
3. If `fetched_at` > 30 days old:
   a. Call `CharacterProfileStatus` endpoint (using realm_slug + char_name from cache entry)
   b. If `is_valid == true` and `id` matches `character_id` → update `fetched_at`, return cached data
   c. If `is_valid == false` or HTTP 404 → purge all cache entries for that character
   d. If `is_valid == true` but `id` doesn't match → purge all entries, trigger re-download

**Character grouping**: Profile cache entries for the same character (realm_slug + char_name) share a TTL — if one entry triggers validation, the result applies to all entries for that character.

**Edge case**: If the `CharacterProfileStatus` call itself fails (rate limited, timeout, network error), return the cached data as-is and retry validation on the next retrieval. Do not purge on transient failures.

### Alternatives Considered

- **Background TTL sweep**: A periodic task that validates all stale entries. More proactive but adds complexity (background task management). Can be added later.
- **Caller-driven validation**: Let the caller decide when to validate. Violates the ToS requirement that this be automatic.

---

## R7: Feature Flag Mutual Exclusivity

### Decision: Compile-time error via `compile_error!` when both `db-sqlite` and `db-postgres` are enabled

### Rationale

```rust
#[cfg(all(feature = "db-sqlite", feature = "db-postgres"))]
compile_error!("Features `db-sqlite` and `db-postgres` are mutually exclusive. Enable only one.");
```

This goes in `src/cache/mod.rs` (or `src/lib.rs`) and produces a clear error at compile time.

### Alternatives Considered

- **Runtime error**: Less safe. User discovers the problem only at runtime.
- **Default to one**: Surprising behavior. Explicit error is better.

---

## R8: Rate Limiter Integration Point

### Decision: Integrate into `BattleNetClient::send_request` and `send_request_with_token`

### Rationale

The rate limiter must govern ALL outbound API calls. The two methods that actually make HTTP requests are:
- `send_request(&self, url: String)` — client-credential calls
- `send_request_with_token(&self, url: String, token: &str)` — user-token calls

Both should call `self.rate_limiter.acquire().await` before the HTTP request. This means `BattleNetClient` gains an `Option<Arc<RateLimiter>>` field — `None` if rate limiting is not desired (backward compatible), `Some(limiter)` if configured.

**Construction**: New builder methods or a config struct:

```rust
let client = BattleNetClient::new_from_environment()
    .await?
    .with_rate_limiter(RateLimiterConfig::default());
```

Or integrate into the constructor as an optional parameter.

### Alternatives Considered

- **Rate limiter only in CachedClient**: Misses direct `BattleNetClient` calls. The spec requires all calls to be governed.
- **Global singleton rate limiter**: Thread-safe but makes testing harder and prevents per-client configuration.
