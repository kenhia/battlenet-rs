# Architecture: battlenet-rs

**Last Updated**: 2026-04-08
**Rust Edition**: 2021 | **MSRV**: 1.94.0+

## Module Layout

```text
src/
├── lib.rs               # Crate root — declares all public modules
├── auth.rs              # OAuth token response struct
├── client.rs            # BattleNetClient — HTTP client + token management
├── errors.rs            # Error types (BattleNetClientError enum)
├── namespace.rs         # WowNamespace enum (Static, Dynamic, Profile)
├── region.rs            # BattleNetRegion enum (US, EU, KR, TW, CN)
├── user_token.rs        # [redis feature] UserAccessToken + Redis reader
└── wow_models/
    ├── mod.rs (wow_models.rs)  # Module root — UrlArgs, GenerateUrl trait, prelude
    ├── core_structs.rs         # 16 shared serde structs (HrefLink, NameAndId, Realm, etc.)
    ├── core_other.rs           # Re-exports (currently minimal)
    ├── achievement.rs          # 5 endpoint models with GenerateUrl impls
    ├── character_profile.rs    # 2 endpoint models (1 via bendpoint macro, 1 manual)
    ├── connected_realm.rs      # 2 endpoint models with GenerateUrl impls
    ├── wow_token.rs            # 1 endpoint model via bendpoint macro
    └── auction_house.rs        # Model structs only (orphaned — not declared as module)

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
- `#[derive(Debug, Deserialize)]` on the struct
- `pub` visibility on struct and fields
- `pub type CharacterProfileStatusResult = Result<CharacterProfileStatus, BattleNetClientError>;`
- `pub type CharacterProfileStatusJsonResult = Result<String, BattleNetClientError>;`
- `impl GenerateUrl for CharacterProfileStatus { ... }` with URL construction

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
- `UrlArgs::None` — Index endpoints (no parameters)
- `UrlArgs::Id { id: u64 }` — Entity-by-ID endpoints
- `UrlArgs::Player { realm_slug, name }` — Character-scoped endpoints

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
    Unknown,                               // Catch-all (dev use)
}
```

## Test Infrastructure

Tests are integration tests in `tests/` that hit the live BattleNet API:

- `tests/common.rs` — Shared setup: loads `.env`, creates `BattleNetClient`
- `tests/achievements_test.rs` — 5 tests covering all achievement endpoints
- `tests/connected_realm_test.rs` — 2 tests (index + specific realm)
- `tests/wow_token_test.rs` — 1 test (price range validation from env vars)

Plus 1 unit test in `src/lib.rs` and 1 doc test on `BattleNetClient::new_from_environment`.

**Total**: 10 tests. All require valid API credentials in `.env`.
