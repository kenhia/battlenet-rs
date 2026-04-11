# Research: Library Setup, WoW Retail API Coverage, and Examples

**Branch**: `003-lib-wow-examples` | **Date**: 2026-04-09

## R1: Cargo Feature Flag Architecture

### Decision: Feature-gated module subtrees with additive composition

### Rationale

Rust's `cfg(feature)` system works at the module level. The most idiomatic pattern for library crates is:

1. **`default = []`** — Core modules always compile: `auth`, `client`, `errors`, `namespace`, `region`. No game-specific code.
2. **`wow`** — Gates the entire `wow_models` module subtree via `#[cfg(feature = "wow")] pub mod wow_models;` in `lib.rs`.
3. **`user`** — Gates Profile API sub-modules inside `wow_models` that require a user OAuth token. These modules are conditionally declared within `wow_models/mod.rs` using `#[cfg(feature = "user")]`.
4. **`redis`** — Preserved as-is. Independent of `wow` and `user`.
5. **Stubs** — `wow-classic`, `diablo`, `hearthstone`, `starcraft` declared in `[features]` with empty dependency lists. Forward-compatible; no code gated behind them yet.

### Alternatives Considered

- **Separate crates per game** (e.g., `battlenet-wow`, `battlenet-d3`): Too much overhead for a library at this scale. A single crate with features is simpler and more discoverable.
- **One feature per endpoint category** (e.g., `wow-achievements`, `wow-items`): Too granular. 30+ features would be overwhelming for consumers. The `wow` umbrella feature is the right granularity.

### Impact on Existing Code

- `wow_models` module (currently always compiled) moves behind `#[cfg(feature = "wow")]`.
- Existing consumers who depend on wow_models must add `features = ["wow"]` to their `Cargo.toml`.
- The `model-macro` crate is a proc-macro dependency — it compiles regardless of features (proc-macros are build-time only). No change needed.
- `core_structs.rs` stays inside `wow_models/` (only used by wow models).

---

## R2: UrlArgs Enum Extension

### Decision: Add 5 new variants to cover all WoW endpoint URL patterns

### Rationale

Analysis of all ~177 endpoints in `ModelImplementProgress.md` reveals these distinct URL parameter patterns:

| Pattern | Example Endpoint | Current UrlArgs | Count |
|---------|-----------------|-----------------|-------|
| No params | `data/wow/achievement/index` | `None` | ~45 |
| Single ID | `data/wow/achievement/{id}` | `Id { id }` | ~80 |
| Realm+Name (character) | `profile/wow/character/{realm}/{name}` | `Player { realm_slug, name }` | ~25 |
| Realm+Name (guild) | `data/wow/guild/{realm}/{nameSlug}` | **NEW: `Guild`** | ~4 |
| Two IDs (nested) | `data/wow/item-class/{id}/item-subclass/{subId}` | **NEW: `TwoIds`** | ~5 |
| Three params (leaderboard) | `connected-realm/{id}/mythic-leaderboard/{dungeonId}/period/{period}` | **NEW: `ThreeIds`** | ~1 |
| Realm+Name+sub (player sub with extra param) | `profile/wow/character/{realm}/{name}/pvp-bracket/{bracket}` | **NEW: `PlayerExtra`** | ~2 |
| Raid/faction | `data/wow/hall-of-fame/{raid}/{faction}` | **NEW: `TwoStrings`** | ~1 |
| RealmId-CharacterId | `profile/user/wow/protected-character/{realmId}-{characterId}` | `TwoIds` (reuse) | ~1 |
| Search | `data/wow/search/{type}` | **NEW: `Search`** | ~9 |

New `UrlArgs` variants:

```rust
pub enum UrlArgs {
    None,
    Id { id: u64 },
    Player { realm_slug: String, name: String },
    // New variants:
    Guild { realm_slug: String, name_slug: String },
    TwoIds { id1: u64, id2: u64 },
    ThreeIds { id1: u64, id2: u64, id3: u64 },
    PlayerExtra { realm_slug: String, name: String, extra: String },
    TwoStrings { first: String, second: String },
    Search { params: Vec<(String, String)> },
}
```

The `bendpoint` macro's `url_args` attribute parser must be extended to handle the new variants.

### Alternatives Considered

- **Generic HashMap for all URL args**: Loses type safety. The enum approach is already established and works well.
- **Separate trait per endpoint with custom params**: Too much boilerplate. The enum keeps the `GenerateUrl` trait uniform.
- **Builder pattern per endpoint**: Overkill. The URL construction is simple enough that the enum covers all cases.

---

## R3: Search Endpoints Design

### Decision: Use `UrlArgs::Search` with key-value pairs; return paginated wrapper structs

### Rationale

The Blizzard Search API (`/data/wow/search/{documentType}`) uses ad-hoc query parameters:
- Field filters: `?has_queue=true&realms.timezone=America/New_York`
- Pagination: `_page=N`, `_pageSize=N` (default 100, max 1000)
- Sorting: `orderby=field1:asc,field2:desc`
- Operations: OR (`||`), NOT (`!=`), RANGE (`[min,max]`)

Search responses have a common envelope:

```json
{
  "page": 1,
  "pageSize": 100,
  "maxPageSize": 1000,
  "pageCount": 5,
  "results": [
    { "key": { "href": "..." }, "data": { ... } }
  ]
}
```

For this sprint, search endpoints will:
1. Accept `UrlArgs::Search { params }` for query parameters
2. Return a generic `SearchResult<T>` wrapper struct
3. Each search-specific result type is a data struct nested in the `results` array

### Alternatives Considered

- **Typed builder per search endpoint**: Too much per-endpoint work for 9 search endpoints. The generic approach is simpler.
- **Skip search endpoints entirely**: They exist in the API docs and are listed in `ModelImplementProgress.md`. Completeness requires them.

---

## R4: Testing Strategy for ~167 New Endpoints

### Decision: JSON fixture-based unit tests (no live API required for unit tests)

### Rationale

The existing test pattern uses two approaches:
1. **Unit tests**: Load JSON from `data/` fixture files, deserialize via `json_to_struct()`, assert field values. Fast, no network.
2. **Integration tests**: Call live API via `BattleNetClient`, assert response structure. Slow, requires API credentials.

For ~167 new endpoints, unit tests are the primary gate:
- Capture one representative JSON response per endpoint in `data/`
- Test deserialization into the typed struct
- Verify key field values and presence of optional fields

Integration tests will be added selectively for endpoint groups where URL construction is novel (e.g., search, multi-ID patterns). Full live-API integration tests for all 167 endpoints would be slow and fragile.

### Alternatives Considered

- **Only integration tests**: Too slow, flaky, requires API credentials. Not viable for CI.
- **Mock server**: Adds complexity for little benefit over fixture files. The library's job is deserialization, not HTTP handling (reqwest is trusted).

---

## R5: `bendpoint` Macro vs Manual Implementation vs `pygen`

### Decision: Use `bendpoint` macro for simple endpoints; `pygen` for bulk generation; manual for complex structs

### Rationale

Three code generation tools exist:

1. **`bendpoint` proc macro**: Best for endpoints with simple structs (flat fields, standard types). Generates `#[derive(Debug, Deserialize)]`, result type aliases, and `GenerateUrl` impl. Currently supports `None`, `Id`, `Player` url_args and `static`/`dynamic`/`profile` namespaces.

2. **`pygen`**: Python YAML-to-Rust generator. Takes YAML model definitions and produces complete `.rs` files. Best for bulk-generating many structurally similar endpoints from a catalog.

3. **Manual**: Required when the response struct has complex nested types, optional fields with custom logic, or serde aliases beyond `_links`.

**Strategy for this sprint**:
- Extend `bendpoint` to support `Guild`, `TwoIds`, `ThreeIds`, `PlayerExtra`, `TwoStrings`, `Search` url_args variants
- Use `bendpoint` for all new endpoints where the struct is simple enough
- Use manual implementation for endpoints with deeply nested or polymorphic response structures (e.g., talent trees, character equipment)
- Use `pygen` if a large batch of structurally identical endpoints need to be generated at once

---

## R6: Profile API Token Handling

### Decision: New `send_request_with_token` method on `BattleNetClient`; token is caller's responsibility

### Rationale

Currently, `send_request()` uses the client-credentials token (auto-managed). Profile APIs require a user OAuth token which the library does NOT manage (per spec: "the caller's responsibility for getting/managing the token").

Approach:
- Add `send_request_with_token(&self, url: String, token: &str)` method that uses the provided token instead of the internal client token.
- Add `get_data_with_token<T>(&self, url_args: &UrlArgs, token: &str)` and `get_json_with_token<T>(&self, url_args: &UrlArgs, token: &str)` convenience methods.
- Profile API endpoints use namespace `Profile` (already supported in `WowNamespace`).
- The existing `user_token::read_user_token()` from sprint 002 can provide the token, but the library doesn't couple to it.

### Alternatives Considered

- **Store user token in BattleNetClient**: Conflates two concerns. Client-credentials and user tokens have different lifecycles and scopes.
- **Separate `UserClient` struct**: Over-engineering. The same HTTP client and region settings apply; only the token differs.

---

## R7: Feature Flag Impact on Existing Examples and Tests

### Decision: Existing examples and tests get `required-features` annotations

### Rationale

- Examples that use `wow_models` (all current examples) need `required-features = ["wow"]` in `Cargo.toml`.
- Examples that also use Profile APIs need `required-features = ["wow", "user"]`.
- Existing integration tests in `tests/` that use wow_models need `#[cfg(feature = "wow")]` gates.
- The main `cargo test` command should be `cargo test --all-features` for full coverage.

This is a one-time migration cost that pays off with cleaner dependency graphs for consumers.
