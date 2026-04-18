# IPC Contract: Tauri Commands

**Created**: 2026-04-17
**Feature**: [spec.md](../spec.md)

## Overview

ktoons uses Tauri IPC commands as the interface between the Svelte frontend
and the Rust backend. All commands are async, return `Result<T, String>`, and
are invoked from the frontend via `@tauri-apps/api/core` `invoke()`.

Arguments use `camelCase` (Tauri default). Responses are JSON-serialized Rust
structs.

---

## Commands

### `get_realms`

Fetch the list of available realms for the configured region.

**Arguments**: None

**Response** (`Result<Vec<RealmEntry>, String>`):
```json
[
  { "name": "Trollbane", "slug": "trollbane" },
  { "name": "Area 52", "slug": "area-52" },
  ...
]
```

**Errors**: API unreachable, invalid credentials.

**Notes**: Called once on app startup to populate the realm dropdown. Result
is cached client-side (Svelte store).

---

### `lookup_character`

Look up a character by name and realm slug using client token only (no OAuth).

**Arguments**:
| Name | Type | Description |
|------|------|-------------|
| `realmSlug` | `string` | Realm slug (e.g., `"trollbane"`) |
| `characterName` | `string` | Character name (case-insensitive) |

**Response** (`Result<FullCharacter, String>`):
Full `FullCharacter` struct serialized as JSON. Contains all 28 endpoint fields
(each `Option<T>` — `null` in JSON if absent). Only public profile endpoints
are populated (no user-token-scoped data).

**Errors**: Character not found (404), API errors, network failure.

---

### `login`

Initiate the Blizzard OAuth flow, then fetch the account profile.

**Arguments**: None

**Response** (`Result<Vec<AccountCharacterEntry>, String>`):
```json
[
  {
    "name": "Belarsa",
    "realmName": "Trollbane",
    "realmSlug": "trollbane",
    "level": 80,
    "className": "Mage",
    "raceName": "Void Elf",
    "faction": "Alliance"
  },
  ...
]
```

**Errors**: OAuth cancelled by user, OAuth flow failed, token exchange failed,
account profile fetch failed.

**Side effects**: Sets the in-memory `UserToken` in `AppState`. Subsequent
`get_character` calls will use this token.

---

### `get_character`

Fetch a character's full data. Uses the user token if logged in (provides
user-scoped endpoint data), otherwise falls back to client token only.

**Arguments**:
| Name | Type | Description |
|------|------|-------------|
| `realmSlug` | `string` | Realm slug |
| `characterName` | `string` | Character name |

**Response** (`Result<FullCharacter, String>`):
Same as `lookup_character`, but with user-scoped endpoints populated if a
user token is available.

**Errors**: Character not found, API errors, network failure.

---

### `refresh_character`

Force re-fetch all character data from the API, bypassing the cache.

**Arguments**:
| Name | Type | Description |
|------|------|-------------|
| `realmSlug` | `string` | Realm slug |
| `characterName` | `string` | Character name |

**Response** (`Result<FullCharacter, String>`):
Same as `get_character` but all data is fresh from the API.

**Errors**: Same as `get_character`.

---

## Error Contract

All commands return errors as plain strings (`Result<T, String>`).

The frontend displays the error message directly. No structured error codes
for MVP — future iterations may introduce typed error enums.

Error examples:
- `"Character not found: belarsa on trollbane"`
- `"OAuth authentication was cancelled"`
- `"Failed to connect to Blizzard API: connection timeout"`
- `"API credentials not configured. Set BATTLENET_CLIENT_ID and BATTLENET_CLIENT_SECRET environment variables."`

## Event Contract

No Tauri events are used in the MVP. All communication is request/response
via commands. Future iterations may use events for background refresh
notifications.
