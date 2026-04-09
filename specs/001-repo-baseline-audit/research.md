# Research: Repo Baseline Audit & BattleNet API Research

**Date**: 2026-04-07
**Branch**: `001-repo-baseline-audit`
**Source**: Official Blizzard API documentation, live codebase analysis

## R1: Current Codebase Baseline Status

### Decision
The existing codebase compiles but has minor lint issues that must be fixed.

### Findings

**Compilation**: `cargo check` passes. `cargo build` succeeds.

**Formatting** (`cargo fmt --check`): 3 files with extra blank line diffs:
- `examples/auction-play.rs` — extra blank line before `fn main()`
- `src/wow_models/character_profile.rs` — extra blank lines around commented code
- `src/wow_models/wow_token.rs` — extra blank line before `#[bendpoint]`

**Clippy** (`cargo clippy --all-targets --all-features -- -D warnings`):
1 error — `struct Ack` in `character_profile.rs:67` is never constructed
(dead code, `-D dead-code` implied by `-D warnings`).

**Tests** (`cargo test`): All 8 tests pass when valid `.env` credentials are
present (confirmed via terminal output in session).

### Fix Plan
1. Run `cargo fmt` to resolve all formatting diffs
2. Remove the unused `Ack` struct from `character_profile.rs`
3. Re-verify: `cargo fmt --check && cargo clippy ... && cargo test`

---

## R2: Implemented Endpoint Inventory

### Decision
10 endpoints are fully implemented across 4 API categories. 1 has model but
no endpoint implementation (auction house).

### Current Implementation Status

| Category | Endpoint | Model File | Impl | Test |
|----------|----------|-----------|:----:|:----:|
| Achievement | Categories Index | `achievement.rs` | ✅ | ✅ |
| Achievement | Category by ID | `achievement.rs` | ✅ | ✅ |
| Achievement | Index | `achievement.rs` | ✅ | ✅ |
| Achievement | by ID | `achievement.rs` | ✅ | ✅ |
| Achievement | Media by ID | `achievement.rs` | ✅ | ✅ |
| Character Profile | Summary | `character_profile.rs` | ✅ | ❌ |
| Character Profile | Status | `character_profile.rs` | ✅ | ✅ |
| Connected Realm | Index | `connected_realm.rs` | ✅ | ✅ |
| Connected Realm | by ID | `connected_realm.rs` | ✅ | ✅ |
| WoW Token | Index | `wow_token.rs` | ✅ | ✅ |

**Partial / Models Only**:
- `auction_house.rs` — structs defined (`PetItem`, `AuctionItem` enum,
  `Auction`, `Auctions`) but no `GenerateUrl` impl or endpoint methods.

### Gap
- Character Profile Summary has implementation but missing test (❌)
- Auction House has data models but no endpoint wiring

---

## R3: BattleNet API Delta — New & Changed Endpoints

### Decision
Compare the official API documentation (April 2026) against the existing
`ModelImplementProgress.md` to identify any additions or changes.

### New Profile API Endpoints (not in progress doc)

| Endpoint | Path | Notes |
|----------|------|-------|
| Account Decor Collection Summary | `/profile/user/wow/collections/decor` | New collection type |
| Account Transmog Collection Summary | `/profile/user/wow/collections/transmogs` | New collection type |

### Existing Endpoints Confirmed Current
All Game Data API categories from the official docs match the existing progress
document. The following categories are confirmed present and unchanged in the
API documentation:

- Achievement API (5 endpoints)
- Auction House API (2 endpoints)
- Azerite Essence API (4 endpoints)
- Connected Realm API (3 endpoints, including search)
- Covenant API (7 endpoints)
- Creature API (7 endpoints)
- Guild Crest API (3 endpoints)
- Heirloom API (2 endpoints)
- Item API (8 endpoints, including search)
- Journal API (8 endpoints, including search)
- Media Search API (1 endpoint)
- Modified Crafting API (5 endpoints)
- Mount API (3 endpoints, including search)
- Mythic Keystone Affix API (3 endpoints)
- Mythic Keystone Dungeon API (7 endpoints)
- Mythic Keystone Leaderboard API (2 endpoints)
- Mythic Raid Leaderboard API (1 endpoint)
- Pet API (6 endpoints)
- Playable Class API (4 endpoints)
- Playable Race API (2 endpoints)
- Playable Specialization API (3 endpoints)
- Power Type API (2 endpoints)
- Profession API (6 endpoints)
- PvP Season API (5 endpoints)
- PvP Tier API (3 endpoints)
- Quest API (8 endpoints)
- Realm API (3 endpoints, including search)
- Region API (2 endpoints)
- Reputation API (4 endpoints)
- Spell API (3 endpoints, including search)
- Talent API (7 endpoints)
- Tech Talent API (5 endpoints)
- Title API (2 endpoints)
- Toy API (2 endpoints)
- WoW Token API (1 endpoint)

All Profile API categories confirmed:
- Account Profile API (9 endpoints — 7 original + 2 new)
- Character Achievements API (2 endpoints)
- Character Appearance API (1 endpoint)
- Character Collections API (5 endpoints)
- Character Encounters API (3 endpoints)
- Character Equipment API (1 endpoint)
- Character Hunter Pets API (1 endpoint)
- Character Media API (1 endpoint)
- Character Mythic Keystone Profile API (2 endpoints)
- Character Professions API (1 endpoint)
- Character Profile API (2 endpoints)
- Character PvP API (2 endpoints)
- Character Quests API (2 endpoints)
- Character Reputations API (1 endpoint)
- Character Soulbinds API (1 endpoint)
- Character Specializations API (1 endpoint)
- Character Statistics API (1 endpoint)
- Character Titles API (1 endpoint)
- Guild API (4 endpoints)

### Battle.net OAuth API Status
The OAuth flow is confirmed unchanged:
- `POST /token` (client credentials flow) — matches current `auth.rs` impl
- `GET /authorize` — matches current `region.rs` authorize endpoint
- `GET /userinfo` — not implemented, not needed for client-credentials flow
- `POST /check_token` / `GET /check_token` — not implemented, optional

The existing `auth.rs` and `client.rs` implementation aligns with the current
documented API. No breaking changes detected.

---

## R4: Dependency Currency

### Decision
Run `cargo update` to refresh all dependencies to latest compatible versions.

### Current Dependency Versions (from Cargo.toml)

| Crate | Pinned Version | Latest Compatible | Notes |
|-------|----------------|-------------------|-------|
| chrono | 0.4.38 | 0.4.x | Minor updates likely |
| dotenvy | 0.15.7 | 0.15.x | Stable |
| num-format | 0.4.4 | 0.4.x | Stable |
| reqwest | 0.12.4 | 0.12.x | Active development |
| serde | 1.0.201 | 1.x | Frequent patches |
| serde_json | 1.0.117 | 1.x | Frequent patches |
| thiserror | 1.0.60 | 1.x | Stable |
| time | 0.3.36 | 0.3.x | Active development |
| tokio | 1.37.0 | 1.x | Active development |

### Risk Assessment
All dependencies use semver-compatible version specifiers. `cargo update`
should be safe. If any test breaks after update, the specific crate can be
pinned while investigating.

---

## R5: Documentation Gaps

### Decision
Create three new documents per constitution requirements:

| Document | Purpose | Exists |
|----------|---------|:------:|
| `docs/architecture.md` | Module layout, data flow, proc macro role | ❌ |
| `docs/installation.md` | Rust setup, `.env` config, build & test | ❌ |
| `docs/usage.md` | Library usage examples, API call walkthrough | ❌ |
| `docs/specification.md` | Combined project specification (SDD) | ❌ |

### Content Sources
- Architecture: derived from codebase analysis in this session
- Installation: standard Rust toolchain + `.env-EXAMPLE` file already exists
- Usage: existing examples (`get-client-token.rs`, `char-profile.rs`) as basis
- Specification: synthesized from this spec + future specs
