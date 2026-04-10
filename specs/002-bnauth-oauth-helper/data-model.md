# Data Model: bnauth — Battle.net User OAuth Helper

**Date**: 2026-04-08
**Branch**: `002-bnauth-oauth-helper`

## Overview

This feature introduces a Redis-based token store and a Rust struct to
represent the user access token. No database tables or persistent models
are created — Redis keys with TTLs are the sole storage mechanism.

## Redis Key Schema

All keys use the `bnauth:` prefix. TTL on all keys equals the `expires_in`
value from the Battle.net token response (typically 86399 seconds / ~24h).

| Key | Type | Value | Example |
|-----|------|-------|---------|
| `bnauth:access_token` | STRING | Bearer token string | `"USVb1nGO9kwQlhNR..."` |
| `bnauth:token_type` | STRING | Token type | `"bearer"` |
| `bnauth:expires_at` | STRING | Epoch timestamp (seconds) | `"1744243200"` |
| `bnauth:scope` | STRING | Granted OAuth scopes | `"openid"` |
| `bnauth:obtained_at` | STRING | Epoch timestamp when token was acquired | `"1744156800"` |

### Key Lifecycle

```text
bnauth app stores token
  │
  ├─ SET bnauth:access_token <token> EX <expires_in>
  ├─ SET bnauth:token_type "bearer" EX <expires_in>
  ├─ SET bnauth:expires_at <now + expires_in> EX <expires_in>
  ├─ SET bnauth:scope <scope> EX <expires_in>
  └─ SET bnauth:obtained_at <now> EX <expires_in>

24 hours later → all keys auto-expire via Redis TTL

battlenet-rs reads token
  │
  ├─ GET bnauth:access_token → Some(token) or None
  ├─ GET bnauth:expires_at → Some(epoch) or None
  └─ Uses token as Bearer header for user-scoped API calls
```

## Rust Entities

### UserAccessToken (new, in `src/user_token.rs`)

Represents a user-scoped OAuth token read from Redis.

| Field | Type | Description |
|-------|------|-------------|
| `access_token` | `String` | Bearer token for API calls |
| `token_type` | `String` | Usually `"bearer"` |
| `expires_at` | `i64` | Epoch timestamp when token expires |
| `scope` | `String` | Granted OAuth scopes |
| `obtained_at` | `i64` | Epoch timestamp when token was obtained |

### BattleNetClientError (extended)

New variant added (feature-gated behind `redis`):

| Variant | Description |
|---------|-------------|
| `RedisError(redis::RedisError)` | Redis connection or command failure |
| `UserTokenNotAvailable` | `bnauth:access_token` key not found (expired or never set) |

## Python Entities

### OAuth State (Flask session)

| Field | Type | Description |
|-------|------|-------------|
| `oauth_state` | `str` | Random string for CSRF validation |

### Token Response (from Battle.net)

| Field | Type | Description |
|-------|------|-------------|
| `access_token` | `str` | Bearer token |
| `token_type` | `str` | `"bearer"` |
| `expires_in` | `int` | Seconds until expiry (typically 86399) |
| `scope` | `str` | Granted scopes |

## Relationships

```text
bnauth (Python/Flask on cleo)
  │
  ├─ Authenticates via → Battle.net OAuth (authorize + token endpoints)
  │
  └─ Stores token in → Redis (rpi53)
                           │
                           └─ Read by → battlenet-rs (Rust on kubs0)
                                          │
                                          └─ Calls → Battle.net user-scoped APIs
                                                     (profile, collections, etc.)
```

The user access token is **separate** from the client credentials token.
`BattleNetClient` continues to manage its own client token for Game Data APIs.
The user token is only needed for the Profile APIs that require authorization
code flow.
