# Data Model: Library Setup, WoW Retail API Coverage, and Examples

**Branch**: `003-lib-wow-examples` | **Date**: 2026-04-09

## Core Entities

### 1. Feature Flags (Cargo.toml)

| Feature | Dependencies | Gates |
|---------|-------------|-------|
| `default` | (empty) | Core modules only |
| `wow` | (none — just cfg gates) | `src/wow_models/` module subtree |
| `user` | (none — just cfg gates) | Profile API sub-modules within `wow_models/` |
| `redis` | `dep:redis` | `src/user_token.rs` (unchanged from sprint 002) |
| `wow-classic` | (empty stub) | Nothing yet |
| `diablo` | (empty stub) | Nothing yet |
| `hearthstone` | (empty stub) | Nothing yet |
| `starcraft` | (empty stub) | Nothing yet |

### 2. UrlArgs Enum (extended)

```
UrlArgs
├── None                                    # Index endpoints, no parameters
├── Id { id: u64 }                          # Single ID parameter
├── Player { realm_slug, name }             # Character realm + name
├── Guild { realm_slug, name_slug }         # Guild realm + name slug (NEW)
├── TwoIds { id1: u64, id2: u64 }          # Nested ID endpoints (NEW)
├── ThreeIds { id1: u64, id2: u64, id3: u64 } # Triple-nested endpoints (NEW)
├── PlayerExtra { realm_slug, name, extra } # Character sub-endpoint with param (NEW)
├── TwoStrings { first, second }            # String pair (raid/faction) (NEW)
└── Search { params: Vec<(String, String)> }  # Search query parameters (NEW)
```

### 3. GenerateUrl Trait

No structural change. Each model struct implements `GenerateUrl::url()` which takes `&BattleNetClient` and `&UrlArgs` and returns the full URL string including namespace, locale, and base URL.

### 4. BattleNetClient (extended)

New methods for user-token-authenticated requests:

| Method | Parameters | Returns | Purpose |
|--------|-----------|---------|---------|
| `send_request_with_token` | `url: String, token: &str` | `BattlenetClientResult<Response>` | Send GET with provided bearer token |
| `get_data_with_token<T>` | `url_args: &UrlArgs, token: &str` | `Result<T, Error>` | Deserialize profile endpoint response |
| `get_json_with_token<T>` | `url_args: &UrlArgs, token: &str` | `Result<String, Error>` | Raw JSON from profile endpoint |

### 5. Search Response Envelope

```
SearchResult<T>
├── page: u32
├── page_size: u32
├── max_page_size: u32
├── page_count: u32
└── results: Vec<SearchResultEntry<T>>

SearchResultEntry<T>
├── key: HrefLink
└── data: T
```

### 6. WoW Namespace

No structural change. Already supports Static, Dynamic, Profile variants.

---

## Endpoint Model Categories

### Game Data API Models (feature: `wow`)

| Module | Endpoints | Namespace | UrlArgs Used | Status |
|--------|-----------|-----------|--------------|--------|
| `achievement.rs` | 5 | Static | None, Id | Existing |
| `auction_house.rs` | 2 | Dynamic | Id, None | Fix orphan |
| `azerite_essence.rs` | 4 | Static | None, Id, Search | New |
| `connected_realm.rs` | 3 | Dynamic | None, Id, Search | Extend |
| `covenant.rs` | 7 | Static | None, Id | New |
| `creature.rs` | 7 | Static | None, Id, Search | New |
| `guild.rs` | 7 | Dynamic/Static | None, Id, Guild | New |
| `heirloom.rs` | 2 | Static | None, Id | New |
| `item.rs` | 8 | Static | None, Id, TwoIds, Search | New |
| `journal.rs` | 8 | Static | None, Id, Search | New |
| `media_search.rs` | 1 | Static | Search | New |
| `modified_crafting.rs` | 5 | Static | None, Id | New |
| `mount.rs` | 3 | Static | None, Id, Search | New |
| `mythic_keystone.rs` | 10 | Dynamic | None, Id, TwoIds, ThreeIds, TwoStrings | New |
| `pet.rs` | 6 | Static | None, Id | New |
| `playable_class.rs` | 4 | Static | None, Id | New |
| `playable_race.rs` | 2 | Static | None, Id | New |
| `playable_spec.rs` | 3 | Static | None, Id | New |
| `power_type.rs` | 2 | Static | None, Id | New |
| `profession.rs` | 6 | Static | None, Id, TwoIds | New |
| `pvp.rs` | 8 | Dynamic | None, Id, TwoIds | New |
| `quest.rs` | 8 | Static | None, Id | New |
| `realm.rs` | 3 | Dynamic | None, Id, Search | New |
| `region_api.rs` | 2 | Dynamic | None, Id | New |
| `reputation.rs` | 4 | Static | None, Id | New |
| `spell.rs` | 3 | Static | None, Id, Search | New |
| `talent.rs` | 12 | Static | None, Id, TwoIds | New |
| `title.rs` | 2 | Static | None, Id | New |
| `toy.rs` | 2 | Static | None, Id | New |
| `wow_token.rs` | 1 | Dynamic | None | Existing |

**Total Game Data endpoints**: ~140 (10 existing + 130 new)

### Profile API Models (features: `wow` + `user`)

| Module | Endpoints | UrlArgs Used | Status |
|--------|-----------|--------------|--------|
| `account_profile.rs` | 9 | None, TwoIds | New |
| `character_achievements.rs` | 2 | Player | New |
| `character_appearance.rs` | 1 | Player | New |
| `character_collections.rs` | 5 | Player | New |
| `character_encounters.rs` | 3 | Player | New |
| `character_equipment.rs` | 1 | Player | New |
| `character_hunter_pets.rs` | 1 | Player | New |
| `character_media.rs` | 1 | Player | New |
| `character_mythic_keystone.rs` | 2 | Player, PlayerExtra | New |
| `character_professions.rs` | 1 | Player | New |
| `character_profile.rs` | 2 | Player | Existing (move to user gate) |
| `character_pvp.rs` | 2 | Player, PlayerExtra | New |
| `character_quests.rs` | 2 | Player | New |
| `character_reputations.rs` | 1 | Player | New |
| `character_soulbinds.rs` | 1 | Player | New |
| `character_specializations.rs` | 1 | Player | New |
| `character_statistics.rs` | 1 | Player | New |
| `character_titles.rs` | 1 | Player | New |
| `guild.rs` (guild Profile endpoints) | 4 | Guild | New (in same module as Game Data guild) |

**Total Profile endpoints**: ~41 (2 existing + 39 new)

---

## Shared Core Structs Additions

The existing `core_structs.rs` defines shared types (`HrefLink`, `LinksRef`, `NameAndId`, `KeyAndId`, `Realm`, `RealmLong`, `TypeAndName`, `TypeAndValue`, `Asset`, etc.). New endpoint models will likely need additional shared structs:

| Struct | Fields | Used By |
|--------|--------|---------|
| `KeyHref` | `key: HrefLink` | Search results, many index responses |
| `EnumType` | `type_: String, name: String` | Many endpoints (alias of TypeAndName) |
| `Media` | `key: HrefLink, id: u64` | Media endpoints |
| `SearchResult<T>` | `page, page_size, max_page_size, page_count, results: Vec<SearchResultEntry<T>>` | All search endpoints |
| `SearchResultEntry<T>` | `key: HrefLink, data: T` | Search result wrapper |

Additional shared structs will emerge during implementation as common response shapes are discovered.

---

## Validation Rules

- All model structs must derive `Debug` and `Deserialize`
- All public fields use `pub`
- JSON fields with reserved Rust names use `#[serde(alias = "...")]` (e.g., `_links` → `links`, `type` → `type_`)
- Optional fields use `Option<T>`
- All endpoint models must have `{Name}Result` and `{Name}JsonResult` type aliases
- All endpoint models must implement `GenerateUrl`

## State Transitions

N/A — This is a stateless library. No state machines or lifecycle management.
