# Architecture: battlenet-rs

**Last Updated**: 2026-04-13
**Rust Edition**: 2021 | **MSRV**: 1.94.0+

## Module Layout

```text
src/
├── lib.rs               # Crate root — declares all public modules
├── auth.rs              # OAuth token response struct
├── client.rs            # BattleNetClient — HTTP client + token management
├── errors.rs            # Error types (BattleNetClientError enum)
├── namespace.rs         # WowNamespace enum (Static, Dynamic, Profile)
├── rate_limiter.rs      # RateLimiter — dual-window token bucket (unconditional)
├── region.rs            # BattleNetRegion enum (US, EU, KR, TW, CN)
├── user_token.rs        # [redis feature] UserAccessToken + Redis reader
├── cache/               # [db-sqlite | db-postgres feature] Response caching
│   ├── mod.rs           # CacheStore trait, CacheEntry, CacheError
│   ├── sqlite.rs        # SqliteCacheStore (WAL mode, in-memory or file)
│   ├── postgres.rs      # PostgresCacheStore (connection pool)
│   └── cached_client.rs # CachedClient<S> — cache-aware BattleNetClient wrapper
└── wow_models/          # [wow feature] All WoW Game Data + Profile models
    ├── mod.rs (wow_models.rs)  # Module root — UrlArgs, GenerateUrl trait, prelude
    ├── core_structs.rs         # Shared serde structs (HrefLink, NameAndId, Realm, etc.)
    ├── core_other.rs           # Re-exports (currently minimal)
    │
    │  ── Game Data modules (33, gated by `wow` feature) ──
    ├── achievement.rs          # Achievement endpoints (5)
    ├── auction_house.rs        # Auction House endpoints (2)
    ├── azerite_essence.rs      # Azerite Essence endpoints (4)
    ├── connected_realm.rs      # Connected Realm endpoints (3)
    ├── covenant.rs             # Covenant endpoints (7)
    ├── creature.rs             # Creature endpoints (7)
    ├── guild.rs                # Guild endpoints (4)
    ├── heirloom.rs             # Heirloom endpoints (2)
    ├── item.rs                 # Item endpoints (8)
    ├── journal.rs              # Journal endpoints (8)
    ├── media_search.rs         # Media Search endpoints (1)
    ├── modified_crafting.rs    # Modified Crafting endpoints (5)
    ├── mount.rs                # Mount endpoints (3)
    ├── mythic_keystone.rs      # Mythic Keystone endpoints (12)
    ├── pet.rs                  # Pet endpoints (6)
    ├── playable_class.rs       # Playable Class endpoints (4)
    ├── playable_race.rs        # Playable Race endpoints (2)
    ├── playable_spec.rs        # Playable Specialization endpoints (3)
    ├── power_type.rs           # Power Type endpoints (2)
    ├── profession.rs           # Profession endpoints (6)
    ├── pvp.rs                  # PvP Season + Tier endpoints (8)
    ├── quest.rs                # Quest endpoints (8)
    ├── realm.rs                # Realm endpoints (3)
    ├── region_api.rs           # Region endpoints (2)
    ├── reputation.rs           # Reputation endpoints (4)
    ├── spell.rs                # Spell endpoints (3)
    ├── talent.rs               # Talent endpoints (7)
    ├── title.rs                # Title endpoints (2)
    ├── toy.rs                  # Toy endpoints (2)
    ├── wow_token.rs            # WoW Token endpoints (1)
    │
    │  ── Profile modules (17, gated by `user` feature) ──
    ├── account_profile.rs      # Account Profile endpoints (9)
    ├── character_achievements.rs
    ├── character_appearance.rs
    ├── character_collections.rs
    ├── character_encounters.rs
    ├── character_equipment.rs
    ├── character_hunter_pets.rs
    ├── character_media.rs
    ├── character_mythic_keystone.rs
    ├── character_professions.rs
    ├── character_profile.rs
    ├── character_pvp.rs
    ├── character_quests.rs
    ├── character_reputations.rs
    ├── character_soulbinds.rs
    ├── character_specializations.rs
    ├── character_statistics.rs
    ├── character_titles.rs
    └── full_character.rs       # Composite download — FullCharacter, CharacterFetcher trait

bnauth/                          # Python Flask app — OAuth user token helper
├── pyproject.toml               # uv-managed project metadata + deps
├── bnauth/
│   ├── __init__.py
│   └── app.py                   # Flask routes (/  /authorize  /callback)
├── templates/
│   ├── index.html               # Landing page with auth button
│   ├── success.html             # Token stored confirmation
│   └── error.html               # Error display with retry link
└── tests/
    ├── test_app.py              # Unit tests (10 tests)
    └── test_e2e.py              # End-to-end Redis + API test

model-macro/
└── src/
    ├── lib.rs            # bendpoint proc macro implementation
    └── input.rs          # Macro attribute parser

pygen/
├── gen_models.py         # YAML → Rust code generator
├── sort_model_yaml.py    # YAML field sorter utility
└── models/               # YAML model definitions (input to gen_models.py)
```

## BattleNetClient Data Flow

```text
User Code
  │
  ▼
client.get_data::<T>(&url_args)          // T: Deserialize + GenerateUrl
  │
  ├─ T::url(&client, &url_args)         // Build full API URL
  │    ├─ endpoint path (e.g. "data/wow/token/index")
  │    ├─ namespace (e.g. "dynamic-us")
  │    ├─ region base URL (e.g. "https://us.api.blizzard.com")
  │    └─ locale (e.g. "en_US")
  │
  ├─ client.send_request(url)           // HTTP GET with bearer token
  │    ├─ client.get_access_token()     // Lazy token fetch + cache
  │    │    └─ POST /oauth/token (if expired/missing)
  │    └─ reqwest GET with Authorization header
  │
  └─ serde_json::from_str::<T>(body)    // Deserialize JSON → struct
       └─ returns Result<T, BattleNetClientError>
```

There is also `client.get_json::<T>(&url_args)` which returns the raw JSON
string instead of a deserialized struct.

## Profile API Token Flow

Profile API endpoints (character data, account collections) require a user
OAuth token instead of client credentials. The client provides parallel methods:

```text
client.get_data_with_token::<T>(&url_args, &user_token)
client.get_json_with_token::<T>(&url_args, &user_token)
```

These use the same URL construction as `get_data` / `get_json` but substitute
the user access token (from Redis via `read_user_token()`) in the Authorization
header instead of the client credentials token.

## Feature-Gated Module Tree

Modules are conditionally compiled via cargo feature flags:

| Feature | Gates | Depends On |
|---------|-------|------------|
| (none) | Core: `client`, `auth`, `errors`, `region`, `namespace`, `rate_limiter` | — |
| `wow` | `wow_models` module + all Game Data sub-modules (33) | — |
| `user` | Profile sub-modules within `wow_models` (17) | `wow` |
| `redis` | `user_token` module (Redis reader) | — |
| `db-sqlite` | `cache` module with SQLite backend (`SqliteCacheStore`) | — |
| `db-postgres` | `cache` module with PostgreSQL backend (`PostgresCacheStore`) | — |

**Note**: `db-sqlite` and `db-postgres` are mutually exclusive — enabling both
produces a compile error.

In `src/wow_models.rs`, the prelude re-exports are gated:
- `#[cfg(feature = "wow")]` — all Game Data types
- `#[cfg(feature = "user")]` — all Profile types

## OAuth Token Lifecycle

1. **First API call**: `get_access_token()` detects no cached token
2. **Token request**: POST to region's OAuth endpoint with client credentials
   - US/EU/KR/TW: `https://oauth.battle.net/token`
   - CN: `https://www.battlenet.com.cn/oauth/token`
3. **Token cached**: Access token and expiry stored in `Mutex`-protected fields
4. **Subsequent calls**: Token reused if not expired (checked via `expires_at`)
5. **Expiry**: When token expires, next API call triggers a fresh token request

The client uses `client_credentials` grant type (no user authorization needed
for Game Data and Profile endpoints).

## User Token Flow (bnauth + Redis)

User-scoped API endpoints (character profile, collections, etc.) require a
separate OAuth authorization code flow that involves a browser login. This
is handled by the `bnauth` Flask app and consumed by `battlenet-rs` through
the optional `redis` cargo feature.

```text
Developer on cleo (browser)
  │
  ▼
bnauth Flask app (http://localhost:5050)
  │
  ├─ GET /authorize → redirect to Battle.net OAuth
  │     scope: wow.profile openid
  │     state: random CSRF token (Flask session)
  │
  ├─ GET /callback?code=...&state=...
  │     ├─ Validate state (CSRF check)
  │     ├─ POST https://oauth.battle.net/token (HTTP Basic auth)
  │     └─ Store 5 keys in Redis with TTL:
  │          bnauth:access_token, bnauth:token_type,
  │          bnauth:expires_at, bnauth:scope, bnauth:obtained_at
  │
  └─ Redis on rpi53 ←── read by ──→ battlenet-rs on kubs0
                                      │
                                      └─ user_token::read_user_token()
                                           ├─ GET bnauth:access_token
                                           └─ Returns UserAccessToken or error
```

**Token separation**: The client credentials token (`src/auth.rs`) and user
access token (`src/user_token.rs`) are distinct types with no runtime mixing.
The client token is used for Game Data endpoints; the user token is used for
Profile endpoints that require user authorization.

**Redis key lifecycle**: All 5 keys share the same TTL (~24h). When they
expire, `read_user_token()` returns `UserTokenNotAvailable`. The user
re-authorizes by visiting bnauth again; `SET ... EX` inherently overwrites.

## `bendpoint` Proc Macro

The `model-macro` crate provides `#[bendpoint]` — an attribute macro that
generates boilerplate for API endpoint structs.

**Input** (user writes):
```rust
#[bendpoint(
    endpoint = "profile/wow/character/{realm_slug}/{name}/status"
    url_args = "Player"
    namespace = "profile"
)]
struct CharacterProfileStatus {
    id: u64,
    is_valid: bool,
}
```

**Generated** (macro expands to):
- `#[derive(Debug, Serialize, Deserialize)]` on the struct
- `pub` visibility on struct and fields
- `pub type CharacterProfileStatusResult = Result<CharacterProfileStatus, BattleNetClientError>;`
- `pub type CharacterProfileStatusJsonResult = Result<String, BattleNetClientError>;`
- `impl GenerateUrl for CharacterProfileStatus { ... }` with URL construction and `cache_namespace()`

**Attributes**:
| Attribute | Required | Description |
|-----------|----------|-------------|
| `endpoint` | Yes | API path template (e.g. `"data/wow/token/index"`) |
| `url_args` | No | `"None"` (default), `"Player"`, or `"Id"` |
| `namespace` | Yes | `"static"`, `"dynamic"`, or `"profile"` |

## `pygen` Code Generator

A Python-based tool that reads YAML model definitions and generates Rust source
files. Located in `pygen/`.

**Flow**: `pygen/models/*.yaml` → `gen_models.py` → `src/wow_models/*.rs`

The generated files follow the same pattern as hand-written models: struct
definition, Result type aliases, and `GenerateUrl` implementation. This tool
is used for bulk-generating endpoint models from the API catalog.

## URL Construction Scheme

Every endpoint model implements the `GenerateUrl` trait:

```rust
pub trait GenerateUrl {
    fn url(client: &BattleNetClient, url_args: &UrlArgs) -> String;
}
```

URL format: `{base_url}/{endpoint}?namespace={namespace}&locale={locale}`

**UrlArgs variants**:
| Variant | Fields | Used For |
|---------|--------|----------|
| `None` | — | Index endpoints (no parameters) |
| `Id { id }` | `u64` | Entity-by-ID lookups |
| `Player { realm_slug, name }` | 2× `String` | Character-scoped endpoints |
| `Guild { realm_slug, name_slug }` | 2× `String` | Guild-scoped endpoints |
| `TwoIds { id1, id2 }` | 2× `u64` | Two-ID endpoints (e.g. skill tier) |
| `ThreeIds { id1, id2, id3 }` | 3× `u64` | Three-ID endpoints (e.g. leaderboard) |
| `PlayerExtra { realm_slug, name, extra }` | 3× `String` | Character + sub-resource (e.g. PvP bracket) |
| `TwoStrings { first, second }` | 2× `String` | Two-string paths (e.g. raid/faction) |
| `Search { params }` | `Vec<(String, String)>` | Search endpoints with query params |

## Namespace / Region Handling

**Regions** (`BattleNetRegion`):
| Region | Base URL | OAuth Endpoint |
|--------|----------|----------------|
| US | `https://us.api.blizzard.com` | `https://oauth.battle.net/token` |
| EU | `https://eu.api.blizzard.com` | `https://oauth.battle.net/token` |
| KR | `https://kr.api.blizzard.com` | `https://oauth.battle.net/token` |
| TW | `https://tw.api.blizzard.com` | `https://oauth.battle.net/token` |
| CN | `https://gateway.battlenet.com.cn` | `https://www.battlenet.com.cn/oauth/token` |

**Namespaces** (`WowNamespace`):
| Namespace | Prefix | Used For |
|-----------|--------|----------|
| Static | `static-{region}` | Game data that rarely changes (achievements, items) |
| Dynamic | `dynamic-{region}` | Frequently changing data (auctions, token prices, realm status) |
| Profile | `profile-{region}` | Character and account data |

The namespace string is constructed as `{prefix}-{region}` and passed as the
`namespace` query parameter in every API request.

## Error Handling

All API operations return `Result<T, BattleNetClientError>`.

```rust
pub enum BattleNetClientError {
    ClientRequestFailed(reqwest::Error),   // HTTP/network errors
    SerdeJsonError(serde_json::Error),     // JSON parse errors
    ClientTokenNotAvailable,               // Token not yet fetched
    ClientTokenMutex(String),              // Mutex lock failure
    UserTokenNotAvailable,                 // Redis user token missing
    RedisError(redis::RedisError),         // [redis feature] Redis errors
    CacheError(CacheError),               // [db-sqlite|db-postgres] Cache errors
    Unknown,                               // Catch-all (dev use)
}
```

## Rate Limiter

The `RateLimiter` implements a dual-window token bucket rate limiter to stay
within Blizzard's API limits (100 requests/sec, 36,000 requests/hr).

- Integrated into `BattleNetClient` via `with_rate_limiter(config)` builder
- `acquire()` is called before every HTTP request in `send_request()` and `send_request_with_token()`
- **Nice mode**: Reduces per-second throughput to 50 req/s for shared-server environments

## CachedClient

`CachedClient<S: CacheStore>` wraps `BattleNetClient` and adds database-backed
response caching with namespace-aware policies:

| Namespace | Cache Policy |
|-----------|-------------|
| Static | Cache-first: return from cache if present, otherwise fetch and cache |
| Dynamic | Always-fetch: call API every time, cache afterward for analytics |
| Profile | Cache-first with 30-day TTL: validate via `CharacterProfileStatus` when expired |

Cache write failures are logged and swallowed — the API response is always
returned to the caller (FR-025).

**TTL enforcement** (profile namespace):
1. If cached entry is < 30 days old → return directly
2. If cached entry is ≥ 30 days old → validate via `CharacterProfileStatus` API
3. Valid + matching ID → refresh timestamp, return cached data
4. Invalid / 404 / ID mismatch → purge character entries, re-fetch
5. Transient validation failure → return stale data

## Test Infrastructure

Tests are integration tests in `tests/` that hit the live BattleNet API:

- `tests/common.rs` — Shared setup: loads `.env`, creates `BattleNetClient`
- `tests/achievements_test.rs` — 5 tests covering all achievement endpoints
- `tests/connected_realm_test.rs` — 2 tests (index + specific realm)
- `tests/wow_token_test.rs` — 1 test (price range validation from env vars)
- `tests/full_character_test.rs` — 12 tests covering FullCharacter struct, JSON, and cache integration

Plus unit tests in `src/` modules and 1 doc test on `BattleNetClient::new_from_environment`.

## Full Character Download (`full_character` module)

The `full_character` module provides a composite download function that fetches
all 28 character profile endpoints in a single call, assembling results into a
typed `FullCharacter` struct.

### CharacterFetcher Trait

```rust
#[async_trait]
pub trait CharacterFetcher: Send + Sync {
    async fn fetch_endpoint<T>(&self, url_args: &UrlArgs) -> Result<T, BattleNetClientError>;
    async fn fetch_endpoint_with_token<T>(&self, url_args: &UrlArgs, token: &str) -> Result<T, BattleNetClientError>;
    async fn fetch_endpoint_force<T>(&self, url_args: &UrlArgs) -> Result<T, BattleNetClientError>;
    async fn fetch_endpoint_with_token_force<T>(&self, url_args: &UrlArgs, token: &str) -> Result<T, BattleNetClientError>;
}
```

Implemented for both `BattleNetClient` (direct API) and `CachedClient<S>`
(cache-aware, feature-gated behind `db-sqlite` or `db-postgres`).

### Orchestration Flow

```text
full_character(&fetcher, realm, name, token)
  │
  ├─ Fetch CharacterProfile (fail-fast — Err if character not found)
  ├─ Fetch MythicKeystoneSeasonsIndex → get current_season.id
  └─ Sequential fetch of 27 remaining endpoints via try_fetch()
       ├─ On success: Some(data)
       └─ On failure: None + EndpointError in errors vec
```

### Graceful Degradation

- **Base profile failure** → entire download aborts with `Err`
- **Any other endpoint failure** → `None` field + `EndpointError` entry
- Class-specific endpoints (e.g. hunter_pets) are expected to fail for other classes
- PvP bracket endpoints fail with 404 for characters without PvP data

## ktoons — Desktop GUI Application

**Architecture**: Tauri 2 (Rust backend) + Svelte 5 (TypeScript frontend)

```text
ktoons/
├── src-tauri/src/
│   ├── main.rs          # Entry point — calls lib::run()
│   ├── lib.rs           # App initialization, state setup, command registration
│   ├── state.rs         # AppState (CachedClient + UserToken mutex)
│   ├── commands.rs      # Tauri IPC commands (5)
│   └── oauth.rs         # OAuth helpers (token exchange, URL building)
├── src/
│   ├── routes/
│   │   └── +page.svelte # Main app page — view routing, state management
│   └── lib/
│       ├── types.ts     # TypeScript interfaces matching Rust types
│       ├── commands.ts  # Typed invoke wrappers for Tauri commands
│       └── components/
│           ├── LaunchScreen.svelte        # Realm select + character name input
│           ├── CharacterHeader.svelte     # Name, level, race, class, portrait
│           ├── CharacterNav.svelte        # Left sidebar character list
│           ├── EquipmentPanel.svelte      # Gear list with quality colors
│           ├── StatsPanel.svelte          # Character statistics grid
│           ├── SpecializationsPanel.svelte # Spec list with active highlight
│           ├── LoadingSpinner.svelte       # Loading indicator
│           └── ErrorDisplay.svelte        # Error with retry button
└── tests/components/    # Vitest component tests
```

### Data Flow

```text
Svelte UI  ─── invoke() ───>  Tauri Command  ─── CachedClient ───>  Blizzard API
                                    │
                                AppState
                           ┌────────┴────────┐
                    CachedClient       UserToken (Mutex)
                    (SQLite cache)     (from OAuth login)
```

### Tauri Commands

| Command | Input | Output | Auth |
|---------|-------|--------|------|
| `get_realms` | — | `Vec<RealmEntry>` | Client token |
| `lookup_character` | realm_slug, name | `FullCharacter` (JSON) | Client token |
| `login` | — | `Vec<AccountCharacterEntry>` | OAuth (browser) |
| `get_character` | realm_slug, name | `FullCharacter` (JSON) | User token if available |
| `refresh_character` | realm_slug, name | `FullCharacter` (JSON) | User token if available |
