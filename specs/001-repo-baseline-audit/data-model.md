# Data Model: Repo Baseline Audit

**Date**: 2026-04-07
**Branch**: `001-repo-baseline-audit`

## Overview

This spec does not introduce new data models. This document inventories the
existing entities in the codebase for reference by future specs.

## Core Shared Types (`src/wow_models/core_structs.rs`)

| Struct | Purpose | Key Fields |
|--------|---------|------------|
| `HrefLink` | Generic API link | `href` |
| `LinksRef` | Response `_links.self` | `self_: HrefLink` |
| `TypeAndName` | Classifier with name | `type_`, `name` |
| `TypeAndValue` | Numeric modifier | `type_: u64`, `value: u64` |
| `NameAndId` | Named+linked entity | `key`, `name`, `id` |
| `KeyAndId` | Linked entity (no name) | `key`, `id` |
| `Realm` | Realm short ref | `name`, `id`, `slug` |
| `RealmLong` | Full realm details | `id`, `region`, `name`, `category`, `locale`, `timezone`, `type_`, `is_tournament`, `slug` |
| `CharacterGuild` | Guild reference | `name`, `id`, `realm`, `faction` |
| `CharacterTitle` | Active title | `name`, `id`, `display_string` |
| `CharacterCovenantProgress` | Covenant status | `chosen_covenant`, `renown_level`, `soulbinds` |
| `Asset` | Media asset | `key`, `value`, `file_data_id` |
| `Aggregates` | Faction totals | `quantity: i32`, `points: i32` |
| `AggregatesByFaction` | Pair of faction totals | `alliance`, `horde` |
| `AchievementCriteria` | Achievement tree node | `id`, `description`, `amount`, `operator`, `child_criteria` |
| `AchievementCriteriaOperator` | Criteria operator | `type_`, `name` |

## Endpoint Models

### Achievement (`src/wow_models/achievement.rs`)

| Model | Endpoint | Namespace |
|-------|----------|-----------|
| `AchievementCategoriesIndex` | `data/wow/achievement-category/index` | static |
| `AchievementCategory` | `data/wow/achievement-category/{id}` | static |
| `AchievementsIndex` | `data/wow/achievement/index` | static |
| `Achievement` | `data/wow/achievement/{id}` | static |
| `AchievementMedia` | `data/wow/media/achievement/{id}` | static |

### Character Profile (`src/wow_models/character_profile.rs`)

| Model | Endpoint | Namespace |
|-------|----------|-----------|
| `CharacterProfileStatus` | `profile/wow/character/{realm}/{name}/status` | profile |
| `CharacterProfile` | `profile/wow/character/{realm}/{name}` | profile |

### Connected Realm (`src/wow_models/connected_realm.rs`)

| Model | Endpoint | Namespace |
|-------|----------|-----------|
| `ConnectedRealmsIndex` | `data/wow/connected-realm/index` | dynamic |
| `ConnectedRealm` | `data/wow/connected-realm/{id}` | dynamic |

### WoW Token (`src/wow_models/wow_token.rs`)

| Model | Endpoint | Namespace |
|-------|----------|-----------|
| `WowTokenIndex` | `data/wow/token/index` | dynamic |

### Auction House (`src/wow_models/auction_house.rs`) — Models Only

| Struct/Enum | Purpose |
|-------------|---------|
| `PetItem` | Pet auction metadata |
| `AuctionItem` (enum) | Untagged: `Pet` or `Item` variant |
| `Auction` | Single auction listing |
| `Auctions` | Collection with connected realm ID |

## Infrastructure Entities

### `BattleNetClient` (`src/client.rs`)
Core HTTP client. Holds region, locale, credentials, cached OAuth token.

### `BattleNetRegion` (`src/region.rs`)
Enum: US, EU, KR, TW, CN. Provides base URLs and locale lists.

### `WowNamespace` (`src/namespace.rs`)
Enum: Static, Dynamic, Profile. Produces region-scoped namespace strings.

### `UrlArgs` (`src/wow_models.rs`)
Enum for endpoint URL parameters: None, Player, Id.

### `GenerateUrl` trait (`src/wow_models.rs`)
Implemented by each endpoint model to produce full API URLs.

## Relationships

```
BattleNetClient
  └─ get_data<T: GenerateUrl>(url_args) → T
       ├─ T::url(client, url_args)  → constructs URL
       ├─ client.send_request(url)  → HTTP GET with auth
       └─ serde_json::from_str()    → deserializes into T
```

No database or persistent storage — all data is fetched live from the API.
