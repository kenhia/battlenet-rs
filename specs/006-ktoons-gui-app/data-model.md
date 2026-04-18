# Data Model: ktoons — WoW Character Viewer

**Created**: 2026-04-17
**Feature**: [spec.md](spec.md) | [plan.md](plan.md)

## Overview

ktoons is a Tauri desktop app. There is no new persistent data model — all
character data comes from the `battlenet-rs` library's existing models and
the SQLite cache managed by `CachedClient`. This document describes the
**runtime data entities** (in-memory state and IPC data shapes).

## Entity: AppState (Tauri Managed State)

The Rust backend holds application state managed by Tauri.

| Field | Type | Mutability | Description |
|-------|------|-----------|-------------|
| `client` | `CachedClient` | Immutable | Wraps `BattleNetClient` + `SqliteCacheStore`. Created at startup. Thread-safe internally (sqlx pool + internal mutex for access token). |
| `user_token` | `tokio::sync::Mutex<Option<UserToken>>` | Mutable | Set after OAuth login. Cleared on expiry. `None` if not logged in. Async mutex because token exchange involves await. |

**Lifecycle**:
- Created in `tauri::Builder::setup()` before any commands execute.
- `client` initialized with env vars (`BATTLENET_CLIENT_ID`, `BATTLENET_CLIENT_SECRET`,
  `BATTLENET_REGION`) and SQLite cache path from `app.path().app_data_dir()`.
- `user_token` starts as `None`, populated after successful OAuth.

## Entity: UserToken (In-Memory)

Represents a Blizzard OAuth user token. Reuses the existing `battlenet-rs`
`UserToken` struct from `src/user_token.rs`.

| Field | Type | Description |
|-------|------|-------------|
| `access_token` | `String` | Bearer token for user-scoped API calls |
| `token_type` | `String` | Always `"bearer"` |
| `expires_at` | `i64` | Unix timestamp when token expires |
| `scope` | `String` | OAuth scopes (e.g., `"wow.profile openid"`) |
| `obtained_at` | `i64` | Unix timestamp when token was obtained |

## Entity: CharacterListEntry (Frontend State)

Lightweight character representation for the left navigation. Exists only in
Svelte frontend state (not persisted).

| Field | Type | Source | Description |
|-------|------|--------|-------------|
| `name` | `string` | `AccountCharacter.name` or `CharacterProfile.name` | Character name |
| `realm_name` | `string` | `AccountCharacter.realm.name` or `CharacterProfile.realm.name` | Friendly realm name |
| `realm_slug` | `string` | `AccountCharacter.realm.slug` or `CharacterProfile.realm.slug` | Realm slug (used in API calls) |
| `level` | `number` | `AccountCharacter.level` or `CharacterProfile.level` | Character level |
| `class_name` | `string` | `AccountCharacter.playable_class.name` or `CharacterProfile.character_class.name` | Class name |
| `race_name` | `string` | `AccountCharacter.playable_race.name` or `CharacterProfile.race.name` | Race name |
| `faction` | `string` | `AccountCharacter.faction.name` or `CharacterProfile.faction.name` | "Alliance" or "Horde" |

**Sources**:
- **Login path**: Populated from `AccountProfileSummary.wow_accounts[*].characters`
  (`Vec<AccountCharacter>`). All characters available immediately.
- **Quick-lookup path**: Constructed from the `CharacterProfile` after a
  successful lookup. Accumulated as user looks up additional characters.

## Entity: CharacterSummary (Frontend View Model)

The primary data structure rendered in the main panel. Constructed from
`FullCharacter` JSON returned by Tauri commands.

### Header Section
| Field | Source | Type |
|-------|--------|------|
| `name` | `profile.name` | `string` |
| `level` | `profile.level` | `number` |
| `race` | `profile.race.name` | `string` |
| `class` | `profile.character_class.name` | `string` |
| `faction` | `profile.faction.name` | `string` |
| `guild` | `profile.guild?.name` | `string \| null` |
| `realm` | `profile.realm.name` | `string` |
| `active_title` | `profile.active_title?.name` | `string \| null` |
| `achievement_points` | `profile.achievement_points` | `number` |
| `equipped_item_level` | `profile.equipped_item_level` | `number` |
| `average_item_level` | `profile.average_item_level` | `number` |
| `portrait_url` | `media.assets[?key="avatar"].value` | `string \| null` |
| `render_url` | `media.assets[?key="main-raw"].value` | `string \| null` |

### Equipment Section
Array of equipped items from `equipment.equipped_items`:

| Field | Source | Type |
|-------|--------|------|
| `slot` | `equipped_items[i].slot.name` | `string` |
| `name` | `equipped_items[i].name` | `string` |
| `ilvl` | `equipped_items[i].level.value` | `number` |
| `quality` | `equipped_items[i].quality.type_` | `string` |

Quality values map to colors: POOR → gray, COMMON → white, UNCOMMON → green,
RARE → blue, EPIC → purple, LEGENDARY → orange.

### Stats Section
From `statistics`:

| Field | Source | Type |
|-------|--------|------|
| `health` | `statistics.health` | `number` |
| `power` | `statistics.power` | `number` |
| `power_type` | `statistics.power_type.name` | `string` |
| `strength` | `statistics.strength.effective` | `number` |
| `agility` | `statistics.agility.effective` | `number` |
| `intellect` | `statistics.intellect.effective` | `number` |
| `stamina` | `statistics.stamina.effective` | `number` |

### Specializations Section
From `specializations`:

| Field | Source | Type |
|-------|--------|------|
| `active_spec` | `specializations.active_specialization.name` | `string` |
| `all_specs` | `specializations.specializations[*].specialization.name` | `string[]` |

### Errors Section
From `errors`:

| Field | Source | Type |
|-------|--------|------|
| `endpoint` | `errors[i].endpoint` | `string` |
| `message` | `errors[i].message` | `string` |

## Entity: RealmEntry (Frontend, Dropdown)

For the realm dropdown on the launch screen.

| Field | Type | Source |
|-------|------|--------|
| `name` | `string` | `RealmSearchData.name.name` | Friendly name |
| `slug` | `string` | `RealmSearchData.slug` | Slug for API calls |

## State Transitions

```
App Launch
    │
    ├──► [LaunchScreen] ─── "Login" ──► OAuth flow ──► [LoggedIn + CharacterList]
    │                   └── "Lookup" ──► fetch character ──► [CharacterView]
    │
    ▼
[CharacterView] ─── "Refresh" ──► force re-fetch ──► [CharacterView updated]
                └── click nav  ──► fetch/load character ──► [CharacterView for new char]
```

## Relationships

```
AppState (1) ──has──► CachedClient (1)
AppState (1) ──has──► UserToken (0..1)

CharacterListEntry (*) ──displayed in──► CharacterNav
CharacterSummary (1) ──displayed in──► Main Panel
CharacterSummary (1) ──derived from──► FullCharacter (1)

FullCharacter (1) ──contains──► CharacterProfile (1)
FullCharacter (1) ──contains──► CharacterEquipmentSummary (0..1)
FullCharacter (1) ──contains──► CharacterStatisticsSummary (0..1)
FullCharacter (1) ──contains──► CharacterSpecializationsSummary (0..1)
FullCharacter (1) ──contains──► CharacterMediaSummary (0..1)
FullCharacter (1) ──contains──► EndpointError (0..*)
```
