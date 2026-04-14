# Feature Specification: Full Character Download

**Feature Branch**: `005-full-toon`  
**Created**: 2026-04-11  
**Status**: Draft  
**Spec Path**: `specs/005-full-toon/spec.md` *(SDD: required before implementation)*  
**Input**: User description: "Full Character Download — composite download of all character profile endpoints with graceful degradation, cache integration, and JSON/struct output"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Full Toon Download as Struct (Priority: P1)

A library consumer wants to retrieve all available profile data for a specific
WoW character in a single call, receiving a strongly-typed composite struct with
each endpoint's data in its own field. Endpoints that return errors (e.g.,
hunter pets for a non-hunter) gracefully degrade to `None` with an optional
error detail rather than failing the entire download.

**Why this priority**: This is the core deliverable — without the composite
struct, there is no "full character download." Everything else builds on this.

**Independent Test**: Call `full_character()` with a known character
(realm/name). Verify the returned struct contains populated data for
common endpoints (profile, equipment, achievements) and `None` for
class-specific endpoints the character doesn't have (e.g., hunter pets on
a mage). Verify the `has_profile_data` flag is `true` when a user token
is provided.

**Acceptance Scenarios**:

1. **Given** a valid character name and realm slug, **When** `full_character()`
   is called with a user token, **Then** all 28 profile endpoint fields are
   populated (or `None` with error detail for class-specific misses) and
   `has_profile_data` is `true`.
2. **Given** a valid character name and realm slug, **When** `full_character()`
   is called without a user token, **Then** only public profile endpoints are
   populated, token-required endpoints are `None`, and `has_profile_data` is
   `false`.
3. **Given** a character that does not exist, **When** `full_character()` is
   called, **Then** the call fails with a clear error (not a partial struct
   with everything `None`).

---

### User Story 2 — Full Toon Download as JSON (Priority: P1)

A library consumer wants the same full character data as a JSON string for
serialization, storage, or forwarding to a front-end application. The JSON
function calls the struct version internally and serializes the result.

**Why this priority**: Equally critical as US1 — the Tauri/Svelte front-end
use case requires JSON output. Trivial to implement once US1 exists.

**Independent Test**: Call `full_character_json()` and verify the output is
valid JSON that round-trips back to the `FullCharacter` struct without data
loss.

**Acceptance Scenarios**:

1. **Given** a valid character, **When** `full_character_json()` is called,
   **Then** the result is a valid JSON string containing all the same data
   as the struct version.
2. **Given** the JSON output, **When** it is deserialized back to
   `FullCharacter`, **Then** all fields match the original struct
   (verified via `serde_json::to_value` equality).

---

### User Story 3 — Cache-Aware Full Download (Priority: P2)

A library consumer wants the full character download to leverage the existing
cache layer (from sprint 004) when available. This means the download checks
the cache first for each endpoint, only calling the API for cache misses, and
stores new responses in the cache. A force-refresh option bypasses the cache
for all endpoints.

**Why this priority**: The cache layer already exists. Integrating it into the
full download dramatically reduces API calls for repeated character lookups
and enables the multi-reader architecture (one process updates, another reads).

**Independent Test**: Call `full_character()` for a character twice. Verify the
second call completes significantly faster (cache hits). Call with force-refresh
and verify all endpoints are re-fetched from the API.

**Acceptance Scenarios**:

1. **Given** a `CachedClient` with a populated cache for a character, **When**
   `full_character()` is called, **Then** cached endpoints return from the
   database without API calls.
2. **Given** a `CachedClient`, **When** `full_character_force()` is called,
   **Then** all endpoints are fetched from the API regardless of cache state,
   and the cache is updated.
3. **Given** a plain `BattleNetClient` (no cache), **When** `full_character()`
   is called, **Then** all endpoints are fetched from the API and the download
   completes normally without errors.

---

### User Story 4 — Full Toon Example (Priority: P3)

A new user of the library wants a working example that demonstrates the full
character download with both struct and JSON output, showing how it handles
characters with and without a user token.

**Why this priority**: Examples are the library's primary onboarding mechanism.
Lower priority because it's documentation, not functionality.

**Independent Test**: Run the example with `cargo run --example full-toon`.
Verify it prints character data to stdout.

**Acceptance Scenarios**:

1. **Given** valid API credentials and a character name, **When** the example
   is run, **Then** it prints a summary of the downloaded character data.

---

### Edge Cases

- What happens when one or more profile endpoints return HTTP 404 (e.g., hunter
  pets for a non-hunter, soulbinds for a character that never played
  Shadowlands)? The corresponding field is `None` and the error detail from
  the API response is captured.
- What happens when the API returns HTTP 429 (rate limited)? The existing rate
  limiter in `BattleNetClient` prevents this scenario. If it occurs despite
  rate limiting, the endpoint field is `None` with the rate-limit error
  captured.
- What happens when the character's base profile (`CharacterProfile`)
  fails? The entire download fails — a character that doesn't exist cannot
  return partial data.
- What happens when the user token is expired mid-download? Token-dependent
  endpoints fail gracefully (field is `None`), and `has_profile_data` is
  set to `false`.
- What happens with the `CharacterPvpBracketStatistics` endpoint that requires
  a bracket parameter (PlayerExtra)? Each standard PvP bracket (2v2, 3v3,
  rated battleground) is fetched as separate fields.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The library MUST provide a function that downloads all character
  profile data for a given realm slug and character name, returning a composite
  struct with each endpoint's data in its own field.
- **FR-002**: The library MUST provide a JSON variant of the full download that
  returns the same data as a serialized JSON string.
- **FR-003**: Each endpoint field in the composite struct MUST be `Option<T>` to
  support graceful degradation when individual endpoints fail.
- **FR-004**: The composite struct MUST include per-endpoint error details when
  an endpoint returns an error, capturing the endpoint name and error message
  string.
- **FR-005**: The composite struct MUST include a `has_profile_data` boolean
  indicating whether user-token-scoped data was included.
- **FR-006**: The download MUST work without a user token, populating only
  publicly accessible endpoints and setting `has_profile_data` to `false`.
- **FR-007**: The download MUST work with a user token, populating all
  endpoints (including user-scoped ones) and setting `has_profile_data` to
  `true`.
- **FR-008**: If the character's base profile (`CharacterProfile`) fails,
  the entire download MUST fail with an error rather than returning an
  empty composite.
- **FR-009**: The download MUST include a `fetched_at` timestamp recording when
  the download was initiated.
- **FR-010**: The download MUST include the character's realm slug and name in
  the composite struct for identification.
- **FR-011**: The full download MUST integrate with `CachedClient` when used
  with a cache-enabled client, respecting existing cache policies for each
  endpoint's namespace.
- **FR-012**: The download MUST support a force-refresh mode that bypasses
  the cache for all endpoints.
- **FR-013**: The download MUST work with a plain `BattleNetClient` (no cache)
  as well as a `CachedClient`.
- **FR-014**: PvP bracket statistics MUST be fetched for standard brackets
  (2v2, 3v3, rated battleground).
- **FR-015**: The composite struct and all its fields MUST implement both
  `Serialize` and `Deserialize` for JSON round-tripping.

### Key Entities

- **FullCharacter**: Composite struct containing all downloaded character data.
  Fields include: realm slug, character name, has_profile_data flag, fetched_at
  timestamp, and one `Option<T>` field per character profile endpoint (28
  endpoints including PvP bracket variants and M+ current season). Also
  includes a collection of per-endpoint error details for any failed requests.
- **EndpointError**: Lightweight record of a failed endpoint call, capturing the
  endpoint name and the API error detail string (if available).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A full character download for a character with a user token
  populates at least 20 of 28 endpoint fields (class-specific endpoints may
  be absent).
- **SC-002**: A full character download without a user token populates the
  public profile endpoint and sets `has_profile_data` to `false`.
- **SC-003**: The JSON output of a full character download is valid JSON that
  round-trips back to the composite struct without data loss.
- **SC-004**: A cached full download (second call for the same character)
  completes at least 10x faster than the initial uncached download.
- **SC-005**: A full download for a non-existent character returns a clear
  error, not a partial struct.
- **SC-006**: The example runs successfully and produces readable output.

## Assumptions

- The cache layer from sprint 004 (`CachedClient`, `CacheStore` trait,
  SQLite/Postgres backends) is available and functional.
- The rate limiter from sprint 004 is integrated into `BattleNetClient` and
  handles per-second/per-hour throttling.
- All 24 `Player` endpoints and 4 `PlayerExtra` endpoints (1 M+ season +
  3 PvP brackets) are already implemented as bendpoint structs with
  `Serialize` + `Deserialize`.
- The `CharacterProfile` endpoint is the "base" profile call — if
  this fails, the character doesn't exist or is inaccessible.
- Standard PvP brackets are 2v2, 3v3, and rbg (rated battleground). If a
  bracket has no data for the character, the field is `None`.
- The full download is a library function, not a CLI tool. Examples
  demonstrate usage.

## Out of Scope

- Bulk character downloads (downloading multiple characters in one call).
  Consumers can call `full_character()` in a loop.
- Account-level endpoints (AccountProfileSummary, ProtectedCharacterProfile,
  AccountCollectionsIndex) — these are account-scoped, not character-scoped.
- Automatic periodic refresh or background polling — the consumer controls
  when to download.

## Polish Phase Checklist *(SDD/TDD — mandatory)*

The following MUST be completed before the feature branch is merged:

- [ ] `docs/specification.md` updated with changes from this spec
- [ ] `docs/architecture.md` updated to reflect any structural changes
- [ ] `docs/installation.md` updated if setup steps changed
- [ ] `docs/usage.md` updated with new usage examples
- [ ] All tests written first (TDD) and passing
- [ ] Pre-commit suite passes clean (CI variant)
