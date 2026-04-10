# Research: bnauth — Battle.net User OAuth Helper

**Date**: 2026-04-08
**Branch**: `002-bnauth-oauth-helper`
**Source**: Official Blizzard API docs, kpidash Redis patterns, crate ecosystem

## R1: OAuth Authorization Code Flow Mechanics

### Decision
Use Flask to handle the full authorization code flow with `state` CSRF
protection and HTTP Basic auth for the token exchange.

### Findings

**Flow**:
1. App generates random `state`, stores in Flask session
2. Redirect browser to `https://oauth.battle.net/authorize` with:
   - `response_type=code`
   - `client_id=<CLIENT_ID>`
   - `scope=wow.profile openid`
   - `state=<random>`
   - `redirect_uri=http://localhost:5050/callback`
3. User logs in on Battle.net, grants access
4. Battle.net redirects to `http://localhost:5050/callback?code=<CODE>&state=<STATE>`
5. App validates `state` matches session, then POSTs to `https://oauth.battle.net/token`:
   ```
   POST https://oauth.battle.net/token
   Authorization: Basic base64(client_id:client_secret)
   Content-Type: application/x-www-form-urlencoded

   redirect_uri=http://localhost:5050/callback
   grant_type=authorization_code
   code=<CODE>
   ```
6. Response: `{"access_token": "...", "token_type": "bearer", "expires_in": 86399, "scope": "openid"}`

**Region-specific endpoints**:
| Region | Authorize | Token |
|--------|-----------|-------|
| US/EU/KR/TW | `https://oauth.battle.net/authorize` | `https://oauth.battle.net/token` |
| CN | `https://oauth.battlenet.com.cn/authorize` | `https://oauth.battlenet.com.cn/token` |

**Scopes**: `wow.profile` + `openid`. The `wow.profile` scope unlocks:
- `GET /profile/user/wow` (account profile — character list)
- `GET /profile/user/wow/protected-character/{realmId}-{characterId}`
- `GET /profile/user/wow/collections`
- `GET /profile/user/wow/collections/pets`
- `GET /profile/user/wow/collections/mounts`

**Token lifetime**: 24 hours (86399 seconds). No refresh token available.

### Alternatives Considered
- Device code flow: Not supported by Battle.net API
- Paste-the-code CLI approach: Works but clunky (user must manually copy code from browser URL bar). Flask callback is much smoother.

---

## R2: Redis Key Schema Design

### Decision
Store token data as individual keys with `bnauth:` prefix and TTLs matching
token lifetime. This matches the kpidash pattern of flat keys with prefixes.

### Findings

**kpidash patterns observed** (from `~/src/kpidash`):
- Prefix: `kpidash:` for all keys
- TTLs set per-key to match data freshness requirements
- Auth via `REDISCLI_AUTH` env var
- Python client uses `redis.Redis(host=..., password=os.environ.get("REDISCLI_AUTH"), decode_responses=True)`

**bnauth key schema**:
| Key | Type | Value | TTL |
|-----|------|-------|-----|
| `bnauth:access_token` | STRING | Bearer token string | `expires_in` |
| `bnauth:token_type` | STRING | `"bearer"` | `expires_in` |
| `bnauth:expires_at` | STRING | Epoch timestamp (int as string) | `expires_in` |
| `bnauth:scope` | STRING | Granted scopes (e.g. `"openid"`) | `expires_in` |
| `bnauth:obtained_at` | STRING | Epoch timestamp when token was obtained | `expires_in` |

All keys share the same TTL so they expire atomically. When TTL fires, the
Rust reader will get a key-not-found, which correctly signals "token expired."

### Alternatives Considered
- Single JSON hash key: Simpler, but the Rust `redis` crate handles flat GET
  more ergonomically than HGETALL for this small number of fields. Flat keys
  also allow easily checking just `bnauth:access_token` without parsing JSON.
- Redis HASH (`HSET bnauth:token field value`): Possible, but TTL applies to
  the whole hash — same effect as flat keys with less granularity for reads.

---

## R3: Python Project Setup with uv

### Decision
Use `uv` for Python project management with `pyproject.toml`. Flask for the
web server, `redis` for Redis client, `requests` for the token exchange HTTP
call, `python-dotenv` for `.env` loading.

### Findings

**uv project init**:
```bash
cd bnauth/
uv init --name bnauth
uv add flask redis requests python-dotenv
uv add --dev pytest ruff
```

**pyproject.toml dependencies**:
```toml
[project]
name = "bnauth"
version = "0.1.0"
requires-python = ">=3.13"
dependencies = [
    "flask>=3.0",
    "redis>=5",
    "requests>=2.31",
    "python-dotenv>=1.0",
]

[tool.uv]
dev-dependencies = [
    "pytest>=8",
    "ruff>=0.4",
]
```

**Pre-commit for bnauth/**:
```bash
cd bnauth/
uv run ruff format --check
uv run ruff check
uv run pytest -q
```

Note: `ty check` from the constitution's Python supplement. `uv add --dev ty`
to include it.

---

## R4: Rust Redis Crate Selection

### Decision
Use the `redis` crate (the de facto standard Rust Redis client) behind an
optional cargo feature flag.

### Findings

**Crate**: `redis` (crates.io) — mature, actively maintained, supports sync
and async APIs.

**Cargo.toml addition**:
```toml
[features]
default = []
redis = ["dep:redis"]

[dependencies]
redis = { version = "0.27", optional = true }
```

**Usage pattern**:
```rust
use redis::Commands;

let client = redis::Client::open(format!("redis://:{password}@{host}:{port}"))?;
let mut con = client.get_connection()?;
let token: Option<String> = con.get("bnauth:access_token")?;
```

**Feature gating in lib.rs**:
```rust
#[cfg(feature = "redis")]
pub mod user_token;
```

**Error handling**: Add `RedisError` variant to `BattleNetClientError`:
```rust
#[cfg(feature = "redis")]
#[error("{0}")]
RedisError(#[from] redis::RedisError),
```

### Alternatives Considered
- `deadpool-redis` / `bb8-redis`: Connection pools — overkill for reading
  a single key on-demand.
- `fred`: More feature-rich but heavier dependency. `redis` crate is simpler
  and sufficient.
- Async redis: The sync API is fine here — we read one key. No need for async
  connection management for this use case.

---

## R5: Flask App Architecture

### Decision
Minimal Flask app with 3 routes and Jinja2 templates. Session-based `state`
storage for CSRF protection.

### Findings

**Routes**:
| Route | Method | Purpose |
|-------|--------|---------|
| `/` | GET | Landing page with "Get Battle.net Auth" button |
| `/authorize` | GET | Generate state, redirect to Battle.net |
| `/callback` | GET | Receive code, exchange for token, store in Redis |

**CSRF protection**: Flask's `session` (server-side signed cookie via
`SECRET_KEY`) stores the random `state`. On callback, compare
`request.args["state"]` against `session["oauth_state"]`.

**Templates**: Simple Jinja2 HTML — no CSS framework needed. 3 templates:
- `index.html`: Button + current token status from Redis (if exists)
- `success.html`: Token stored, shows expiry time, scopes
- `error.html`: Error message + "Try Again" link

**Token status on index page**: When loading `/`, check Redis for
`bnauth:access_token` — if present, show remaining TTL and "Re-authorize"
instead of "Get Battle.net Auth". This gives quick visual feedback.
