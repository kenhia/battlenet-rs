# Feature Specification: Database Cache Layer & Rate Limiting

**Feature Branch**: `004-db-cache`  
**Created**: 2026-04-10  
**Status**: Draft  
**Spec Path**: `specs/004-db-cache/spec.md` *(SDD: required before implementation)*  
**Input**: User description: "Introduce a database-backed cache layer and API rate limiter for the battlenet-rs library"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Cache Static Endpoint Responses (Priority: P1)

A library consumer fetches WoW game data (achievements, items, mounts, spells) using `CachedClient`. On first call, data is retrieved from the Battle.net API and stored in the database cache. On subsequent calls, the cached data is returned instantly without an API call. The consumer can force a refresh when they know data has been updated (e.g., after a game patch).

**Why this priority**: Static namespace data changes infrequently (game patches only) and represents the majority of API calls. Caching this data eliminates redundant network requests, dramatically improving performance and reducing API quota usage.

**Independent Test**: Can be fully tested by calling a static endpoint twice — first call hits the API, second call returns from cache without network activity. Delivers immediate value by reducing API calls for the most common use case.

**Acceptance Scenarios**:

1. **Given** an empty cache, **When** a consumer requests a static endpoint (e.g., achievement categories index), **Then** the library fetches from the API, stores the response in the cache, and returns the data.
2. **Given** a cached static endpoint response exists, **When** the consumer requests the same endpoint again, **Then** the library returns the cached data without making an API call.
3. **Given** a cached static endpoint response exists, **When** the consumer requests the same endpoint with force-refresh enabled, **Then** the library fetches fresh data from the API, updates the cache, and returns the new data.
4. **Given** a cached entry with a known namespace type, **When** the entry is retrieved, **Then** the stored namespace type (static/dynamic/profile) is available alongside the data.

---

### User Story 2 - API Rate Limiting (Priority: P1)

A library consumer makes many API calls in rapid succession (e.g., fetching all auction house data across connected realms). The rate limiter automatically throttles outbound requests to stay within Battle.net's published limits (100/second, 36,000/hour) without the consumer needing to manage timing. An optional "nice" mode further reduces throughput for background/batch operations.

**Why this priority**: Without rate limiting, consumers risk getting temporarily banned from the API. This is a hard requirement for any production use of the library and protects both the consumer and Blizzard's infrastructure.

**Independent Test**: Can be tested by issuing a burst of requests and verifying that the rate limiter spaces them to stay within configured limits. Delivers value by making the library safe for production batch operations.

**Acceptance Scenarios**:

1. **Given** the rate limiter is configured with default limits, **When** a consumer issues 150 requests as fast as possible, **Then** the first 100 are sent within the first second and the remaining 50 are queued and sent in the next second.
2. **Given** the rate limiter is in "nice" mode, **When** a consumer issues requests, **Then** the per-second rate is reduced to the configured nice-mode limit (e.g., 50/second).
3. **Given** 36,000 requests have been sent in the current hour window, **When** a consumer issues another request, **Then** the request is queued until the hourly window resets.
4. **Given** a `CachedClient` with rate limiting enabled, **When** a cache miss triggers an API call, **Then** that API call goes through the rate limiter.
5. **Given** the rate limiter is active, **When** a consumer makes a direct `BattleNetClient` call (bypassing cache), **Then** that call also goes through the rate limiter.

---

### User Story 3 - Cache Dynamic/Profile Endpoint Responses (Priority: P2)

A library consumer fetches dynamic/profile data (auctions, character profiles). The library always calls the API to get fresh data but also caches the response. This cached data serves as a local record and is used by the 30-day TTL enforcement for character data.

**Why this priority**: Dynamic data changes frequently so always-fetch behavior is correct, but caching enables the 30-day TTL enforcement and provides a local data store for consumers doing analytics or batch processing.

**Independent Test**: Can be tested by fetching a dynamic endpoint, verifying the API is called, and confirming the response is stored in the cache with the correct namespace type and timestamp.

**Acceptance Scenarios**:

1. **Given** a consumer requests a dynamic endpoint (e.g., auctions for connected realm 1175), **When** the request is made, **Then** the library always calls the API regardless of cache state, and stores the response in the cache.
2. **Given** a consumer requests a profile endpoint (e.g., character profile for "Belarsa"), **When** the request is made, **Then** the library calls the API, caches the response with a `fetched_at` timestamp, and returns the data.
3. **Given** cached profile data exists, **When** the same profile endpoint is requested again, **Then** the library calls the API again (not returning stale cached data) and updates the cache entry.

---

### User Story 4 - 30-Day TTL Enforcement for Character Data (Priority: P2)

A library consumer retrieves cached character data. If the cached data is older than 30 days, the library automatically validates it by calling the `CharacterProfileStatus` endpoint. Stale, invalid, or changed characters are purged; valid characters get their timestamp refreshed. This enforcement is automatic and not opt-in, ensuring compliance with Blizzard's Terms of Use (Section 2.R).

**Why this priority**: This is a legal/ToS compliance requirement. It depends on the cache layer (P1) being functional first, but must be implemented before the library is used in production.

**Independent Test**: Can be tested by inserting a cache entry with a `fetched_at` timestamp > 30 days ago and then requesting it — verifying the validation call is made and the correct purge/refresh behavior occurs.

**Acceptance Scenarios**:

1. **Given** cached character data with a `fetched_at` timestamp older than 30 days, **When** the data is retrieved from cache, **Then** the library calls `CharacterProfileStatus` to validate.
2. **Given** the validation call returns `is_valid == true` and the character `id` matches the stored `id`, **When** validation completes, **Then** the cached entry's timestamp is refreshed (resetting the 30-day clock) and the cached data is returned.
3. **Given** the validation call returns `is_valid == false` or HTTP 404, **When** validation completes, **Then** the cached character data is purged from the database.
4. **Given** the validation call returns `is_valid == true` but the character `id` does not match the stored `id`, **When** validation completes, **Then** the cached data is purged and a fresh download of the character is triggered.
5. **Given** cached character data with a `fetched_at` timestamp less than 30 days old, **When** the data is retrieved from cache, **Then** no validation call is made and the cached data is returned directly.

---

### User Story 5 - Model Serialization Round-Trip (Priority: P1)

All endpoint model structs (both `bendpoint`-annotated and manually implemented) can be serialized to storage and deserialized back without data loss. This is a prerequisite for the cache layer to function.

**Why this priority**: Without `Serialize` on all model structs, data cannot be written to the cache. This is a blocking dependency for all cache functionality.

**Independent Test**: Can be tested by serializing any endpoint model struct to JSON, deserializing it back, and verifying equality. No database or API access needed.

**Acceptance Scenarios**:

1. **Given** any `bendpoint`-annotated struct, **When** it is serialized to JSON and deserialized back, **Then** the resulting struct is identical to the original.
2. **Given** a manually implemented struct (SearchResult impls, CharacterProfile, core_structs types), **When** it is serialized to JSON and deserialized back, **Then** the resulting struct is identical to the original.
3. **Given** the `bendpoint` proc macro, **When** it generates a struct, **Then** the struct has both `Serialize` and `Deserialize` derives.

---

### User Story 6 - Multi-Reader Database Access (Priority: P3)

Multiple processes can access the cache database concurrently — one writing updated data while others read. This supports architectures where a background job refreshes the cache while a web service reads from it.

**Why this priority**: Important for production deployments but the library works correctly in single-process mode without this. Addressed after core functionality is solid.

**Independent Test**: Can be tested by spawning two processes — one writing to the cache, one reading — and verifying no data corruption or locking errors occur.

**Acceptance Scenarios**:

1. **Given** a Postgres-backed cache, **When** one process writes a cache entry while another reads a different entry, **Then** both operations complete without errors or blocking.
2. **Given** a SQLite-backed cache in WAL mode, **When** one process writes while another reads, **Then** the reader sees a consistent snapshot and is not blocked by the writer.

---

### User Story 7 - README Terms of Use Link (Priority: P3)

The project README includes a visible link to Blizzard's API Terms of Use, making it easy for contributors and consumers to find the governing terms.

**Why this priority**: Simple documentation task with low effort. Important for compliance visibility but not functionally blocking.

**Independent Test**: Can be verified by reading the README and confirming the link is present and correct.

**Acceptance Scenarios**:

1. **Given** the project README.md, **When** a user reads it, **Then** they find a link to the Blizzard Developer API Terms of Use at `https://www.blizzard.com/en-us/legal/a2989b50-5f16-43b1-abec-2ae17cc09dd6/blizzard-developer-api-terms-of-use`.

---

### Edge Cases

- What happens when the database is unreachable at startup? The library should return a clear error when constructing `CachedClient`, not silently fall back to uncached mode.
- What happens when a cache write fails mid-operation? The API response should still be returned to the caller; the cache failure should be logged but not propagated as an error to the consumer.
- What happens when the `CharacterProfileStatus` validation call itself is rate-limited or times out? The library should return the cached data as-is and retry validation on the next retrieval.
- What happens when both the per-second and per-hour rate limits are reached simultaneously? The hourly limit takes precedence — requests wait until the hourly window permits.
- What happens when a consumer enables both `db-sqlite` and `db-postgres` feature flags? This should be a compile-time error (mutually exclusive features).
- What happens when the cache database has no schema/tables? The library should provide a mechanism to initialize the schema on first use.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Library MUST provide a `CacheStore` trait defining cache read, write, delete, and schema-initialization operations.
- **FR-002**: Library MUST provide a concrete `CacheStore` implementation for SQLite, gated behind the `db-sqlite` feature flag.
- **FR-003**: Library MUST provide a concrete `CacheStore` implementation for Postgres, gated behind the `db-postgres` feature flag.
- **FR-004**: The `db-sqlite` and `db-postgres` feature flags MUST be mutually exclusive at compile time.
- **FR-005**: Library MUST provide a `CachedClient` wrapper that accepts a `BattleNetClient` and a `CacheStore` implementation.
- **FR-006**: `CachedClient` MUST return cached data for static namespace endpoints on cache hit (without calling the API).
- **FR-007**: `CachedClient` MUST always call the API for dynamic and profile namespace endpoints, caching the response afterward.
- **FR-008**: `CachedClient` MUST support a `force_refresh` parameter that bypasses cache for any endpoint.
- **FR-009**: Each cache entry MUST store: serialized response data, `fetched_at` timestamp, and namespace type (static/dynamic/profile).
- **FR-010**: Cache entries MUST be keyed by the full request URL as returned by `GenerateUrl::url()` (including locale and namespace query parameters).
- **FR-011**: The `bendpoint` proc macro MUST emit `#[derive(Debug, Serialize, Deserialize)]` for all generated structs.
- **FR-012**: All non-`bendpoint` model structs (SearchResult impls, CharacterProfile, core_structs types) MUST derive or implement `Serialize`.
- **FR-013**: Library MUST enforce a 30-day TTL on cached character (profile namespace) data by automatically validating via `CharacterProfileStatus` when cached data older than 30 days is retrieved.
- **FR-014**: When TTL validation returns `is_valid == false` or HTTP 404, the library MUST purge the cached character data.
- **FR-015**: When TTL validation returns `is_valid == true` with matching character `id`, the library MUST refresh the `fetched_at` timestamp and return cached data.
- **FR-016**: When TTL validation returns `is_valid == true` with non-matching character `id`, the library MUST purge and re-fetch the character data.
- **FR-017**: Library MUST provide a rate limiter that enforces a per-second request limit (default: 100/second).
- **FR-018**: Library MUST provide a rate limiter that enforces a per-hour request limit (default: 36,000/hour).
- **FR-019**: The rate limiter MUST support a configurable "nice" mode that reduces the per-second limit (default nice rate: 50/second).
- **FR-020**: The rate limiter MUST be configurable: max per-second, max per-hour, nice-mode on/off.
- **FR-021**: All outbound API calls (from both `BattleNetClient` and `CachedClient` on cache miss) MUST pass through the rate limiter.
- **FR-022**: SQLite-backed cache MUST use WAL mode to support concurrent readers and a single writer.
- **FR-023**: The project README MUST include a link to the Blizzard Developer API Terms of Use.
- **FR-024**: Library MUST provide a mechanism to initialize the cache database schema on first use.
- **FR-025**: When a cache write fails, the API response MUST still be returned to the caller; the failure MUST be logged via the `log` crate (`log::warn!`), not propagated as an error.

### Key Entities

- **CacheEntry**: A stored API response. Attributes: endpoint URL (key), serialized response data, `fetched_at` timestamp, namespace type (static/dynamic/profile).
- **CachedClient**: A wrapper around `BattleNetClient` that intercedes with cache lookups. Owns a `CacheStore` and a reference to the underlying client. Determines cache behavior based on namespace type.
- **RateLimiter**: Governs outbound API call throughput. Attributes: per-second limit, per-hour limit, nice-mode flag, current window counters.
- **CacheStore**: An abstraction over the database backend. Defines operations for reading, writing, deleting, and initializing cache entries. Concrete implementations exist for SQLite and Postgres.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Repeated requests for the same static endpoint complete without network calls after the first fetch, returning data from cache.
- **SC-002**: A burst of 200 requests completes without exceeding 100 requests in any 1-second window, as observed by request timestamps.
- **SC-003**: 36,000 requests issued within one hour are all dispatched; the 36,001st request is held until the hourly window resets.
- **SC-004**: All 158+ `bendpoint`-annotated structs and all manually implemented model structs successfully round-trip through serialization and deserialization without data loss.
- **SC-005**: Cached character data older than 30 days triggers automatic validation via `CharacterProfileStatus` — invalid characters are purged, valid characters have their timestamp refreshed.
- **SC-006**: Two concurrent processes can read from and write to the cache database without errors or data corruption.
- **SC-007**: Enabling both `db-sqlite` and `db-postgres` feature flags results in a compile-time error.
- **SC-008**: The `CachedClient` correctly distinguishes static, dynamic, and profile namespace endpoints and applies the appropriate caching strategy for each.

## Assumptions

- The existing `BattleNetClient` API surface (`get_data`, `get_json`, etc.) remains stable and is not refactored as part of this sprint.
- The `WowNamespace` enum (Static, Dynamic, Profile) is sufficient for determining cache behavior — no additional namespace types are needed.
- The existing `redis` feature flag for user token storage is unrelated to and does not conflict with the new `db-sqlite` / `db-postgres` cache features.
- Consumers provide their own database connection strings; the library does not manage database provisioning or deployment.
- The `CharacterProfileStatus` endpoint is already implemented (or will be implemented as a prerequisite within this sprint) and returns `is_valid` and character `id` fields.
- Schema migration tooling is explicitly out of scope — the library provides a one-time schema initialization, not incremental migrations.
- The "nice" mode default of 50 requests/second is a reasonable starting point; consumers can configure this value.
- A single `CachedClient` instance is used per process; cross-process coordination relies on the database's concurrency model, not in-process locks.

## Polish Phase Checklist *(SDD/TDD — mandatory)*

The following MUST be completed before the feature branch is merged:

- [ ] `docs/specification.md` updated with changes from this spec
- [ ] `docs/architecture.md` updated to reflect any structural changes
- [ ] `docs/installation.md` updated if setup steps changed
- [ ] `docs/usage.md` updated with new usage examples
- [ ] All tests written first (TDD) and passing
- [ ] Pre-commit suite passes clean (CI variant)
