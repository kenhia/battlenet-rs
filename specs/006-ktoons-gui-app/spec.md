# Feature Specification: ktoons — WoW Character Viewer

**Feature Branch**: `006-ktoons-gui-app`
**Created**: 2026-04-17
**Status**: Draft
**Spec Path**: `specs/006-ktoons-gui-app/spec.md` *(SDD: required before implementation)*
**Input**: User description: "Tauri-Svelte desktop GUI app (ktoons) for viewing WoW character data via battlenet-rs, with OAuth login, quick-lookup, cached data, and character summary display"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Quick Lookup a Character (Priority: P1)

A user wants to look up any WoW character by name and realm without logging
in. They launch the app, enter a character name, select a realm from a
dropdown showing friendly names, and click "Lookup." The main panel displays
a summary of the character: name, level, race, class, faction, guild, realm,
achievement points, item level, equipped gear, stats, specializations, and
character portrait.

**Why this priority**: This is the core value proposition — viewing character
data. It works without OAuth, making it the simplest end-to-end path and a
standalone MVP. Every other story builds on top of this display.

**Independent Test**: Launch the app, enter a known character name and realm
(e.g., "Belarsa" on "Trollbane"), click Lookup. Verify the character summary
appears with all MVP sections populated.

**Acceptance Scenarios**:

1. **Given** the app is launched, **When** the user enters a character name
   and selects a realm and clicks "Lookup," **Then** the main panel displays
   the character summary with header info (name, level, race, class, faction,
   guild, realm), achievement points, item level, equipped gear list, stats,
   specializations, and character portrait.
2. **Given** a character name that does not exist on the selected realm,
   **When** the user clicks "Lookup," **Then** the app displays a clear error
   message and the user remains on the launch/lookup screen.
3. **Given** the user has already looked up a character, **When** they look up
   a second character, **Then** both characters appear in the left navigation
   and the user can click between them.

---

### User Story 2 — Login with Battle.net and View All Characters (Priority: P2)

A user wants to see all characters on their Battle.net account. They click
"Login with Battle.net," authenticate in their browser, and return to the app.
The left navigation populates with all their characters grouped by realm. They
click any character to view its summary.

**Why this priority**: This unlocks the full character list and enables
user-token-scoped data (account-level endpoints). It is the expected primary
workflow for regular use, but depends on US1's display and data plumbing.

**Independent Test**: Click "Login with Battle.net," complete OAuth in the
browser. Verify the left nav shows the user's characters grouped by realm.
Click a character and verify the summary displays.

**Acceptance Scenarios**:

1. **Given** the app is on the launch screen, **When** the user clicks "Login
   with Battle.net" and completes OAuth, **Then** the left navigation shows
   all account characters grouped by realm.
2. **Given** the user is logged in and sees their character list, **When** they
   click a character in the left nav, **Then** the main panel displays that
   character's summary.
3. **Given** OAuth authentication fails (user cancels, network error, or
   Blizzard error), **When** the error occurs, **Then** the app displays an
   error message and remains on the launch screen so the user can retry or use
   quick lookup instead.

---

### User Story 3 — Refresh Character Data (Priority: P2)

A user is viewing a character's summary and wants to see the latest data from
the API (e.g., after playing and earning new gear). They click the "Refresh"
button, and the app re-fetches all data from the Blizzard API, bypassing the
cache.

**Why this priority**: Without refresh, the app shows potentially stale data
forever. Critical for a useful tool, but only meaningful after US1 is working.

**Independent Test**: View a character, click "Refresh." Verify the data is
re-fetched (the fetched timestamp updates, and any changes from the API are
reflected).

**Acceptance Scenarios**:

1. **Given** a character is displayed, **When** the user clicks "Refresh,"
   **Then** all character data is re-fetched from the API (cache bypassed),
   and the display updates with fresh data.
2. **Given** a refresh is in progress, **When** the user views the screen,
   **Then** a loading indicator is visible until the refresh completes.
3. **Given** a refresh fails (API error, network issue), **When** the error
   occurs, **Then** the previously displayed data remains visible, and the
   error is shown as a warning so the user can retry.

---

### User Story 4 — Cached Data for Fast Repeat Views (Priority: P3)

A user views a character they have looked up before. The app loads the
character data instantly from the local cache instead of re-fetching from the
API. Data is served from cache if it is within the cache TTL.

**Why this priority**: Improves responsiveness and reduces API load for
repeated access. The caching layer already exists in `battlenet-rs`; this
story ensures it is wired up correctly in the desktop app context.

**Independent Test**: Look up a character, note the load time. Close and
relaunch the app, look up the same character. Verify it loads significantly
faster (cache hit).

**Acceptance Scenarios**:

1. **Given** a character was previously fetched and cached, **When** the user
   views that character again, **Then** data loads from the local cache
   without API calls (within TTL).
2. **Given** cached data has expired (beyond TTL), **When** the user views
   that character, **Then** the app fetches fresh data from the API and updates
   the cache.

---

### User Story 5 — Partial Failure Handling (Priority: P3)

A user views a character, but one or more data sections fail to load (e.g.,
the achievements endpoint returns an error while equipment succeeds). The app
displays all available data and shows a warning listing which sections could
not be loaded.

**Why this priority**: Graceful partial failure is already built into
`FullCharacter.errors` in the library. This story ensures the UI surfaces
that behavior clearly.

**Independent Test**: View a character where at least one endpoint fails
(e.g., hunter pets for a non-hunter). Verify available sections display and
a warning lists the unavailable sections.

**Acceptance Scenarios**:

1. **Given** one or more character data sections fail to load, **When** the
   character summary is displayed, **Then** the available sections are shown
   normally and a warning message lists which sections failed and why.
2. **Given** the base character profile endpoint fails, **When** the lookup
   is attempted, **Then** the app displays a clear error (not a partial
   summary with everything empty).

---

### Edge Cases

- What happens when the user's internet connection drops mid-fetch? The app
  displays an error for the failed request. If cached data exists, it can
  still be displayed. The user can retry via Refresh.
- What happens when the OAuth token expires during use? The app detects the
  expired token (API returns 401), displays a message prompting the user to
  re-authenticate, and reverts to client-token-only behavior until they log
  in again.
- What happens when the realm dropdown is loading? A loading indicator is
  shown. If the realm list fails to load, the user sees an error and can
  retry (the app is not usable without realm data for quick lookup).
- What happens when a character has class-specific absent data (e.g., hunter
  pets for a mage)? The corresponding section is simply not shown — no error
  displayed for expected absences.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The app MUST provide a "Quick Lookup" path where the user enters
  a character name and selects a realm from a dropdown to view character data
  without requiring OAuth login.
- **FR-002**: The app MUST provide a "Login with Battle.net" path that performs
  OAuth authentication and retrieves the user's full account character list.
- **FR-003**: The realm dropdown MUST display friendly realm names (e.g.,
  "Trollbane," "Area 52") and default to the configured region (US if not
  specified).
- **FR-004**: The OAuth flow MUST be handled entirely within the app (no
  external helper service required). The user token MUST be held in memory
  for the duration of the session.
- **FR-005**: The character summary view MUST display: character name, faction,
  level, race, class, guild, realm, active title, character portrait,
  achievement points, equipped item level, average item level, equipped gear
  (all slots with item name, item level, and quality), character stats (health,
  power with type, strength, agility, intellect, stamina), and specializations
  (active and available).
- **FR-006**: The left navigation MUST show characters the user has accessed.
  In the logged-in path, it shows all account characters grouped by realm.
  In the quick-lookup path, it accumulates looked-up characters.
- **FR-007**: The app MUST provide a "Refresh" button that re-fetches all
  character data from the API, bypassing the cache.
- **FR-008**: The app MUST cache character data locally in a
  platform-appropriate application data directory so that repeat views
  load quickly without redundant API calls.
- **FR-009**: The app MUST display errors clearly (OAuth failures, API errors,
  network issues) and allow the user to retry.
- **FR-010**: Individual endpoint failures within a character fetch MUST NOT
  prevent other sections from displaying. Available data is shown with a
  warning listing failed sections.
- **FR-011**: If the base character profile endpoint fails, the app MUST show
  an error rather than displaying a partial/empty character view.
- **FR-012**: The app MUST obtain API credentials (client ID and secret) from
  environment variables.
- **FR-013**: The app MUST show a loading indicator while fetching character
  data or performing OAuth.
- **FR-014**: The app MUST live within the existing `battlenet-rs` repository
  as a co-located project, sharing the character data library directly.

### Key Entities

- **Character Summary**: The primary view displayed to the user. Composed of
  data from multiple API endpoints: profile (header info, achievement points,
  item level), equipment (gear list per slot), statistics (health, power,
  primary stats), specializations (active and available specs), and media
  (character portrait).
- **Character List Entry**: A lightweight representation of a character in the
  left navigation — name, realm, level, class, race, and faction. Sourced
  from the account profile (logged-in) or accumulated from individual lookups.
- **User Session**: The in-memory state of the current user session, including
  the OAuth user token (if logged in), the list of characters in the left nav,
  and the currently viewed character.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A user can view a character summary (all MVP sections populated)
  within 10 seconds of clicking "Lookup" on a first fetch (uncached).
- **SC-002**: A previously viewed character summary loads within 2 seconds
  of clicking a character in the left nav (served from local cache).
- **SC-003**: A logged-in user sees their full account character list within
  5 seconds of completing OAuth.
- **SC-004**: The user can successfully complete the OAuth login flow and view
  a character summary in under 30 seconds (including browser authentication
  time).
- **SC-005**: When individual endpoint failures occur, the user sees the
  available data sections and a clear listing of which sections failed.
- **SC-006**: The "Refresh" button fetches fresh data from the API and updates
  the display without requiring the user to re-navigate.

## Assumptions

- The user has a stable internet connection for API calls and OAuth. Offline
  mode (cache-only browsing) is not in scope.
- Blizzard API credentials (client ID and secret) are provided via environment
  variables. A settings UI for managing credentials is post-MVP.
- The realm dropdown defaults to the US region. Region selection is controlled
  by the `BATTLENET_REGION` environment variable (default `us`).
- The user is running the app in development mode (via `cargo tauri dev`).
  Packaging and installers are out of scope.
- Character portrait images are served by Blizzard CDN and loaded directly
  via URL. No local image caching is required.
- The app targets desktop only (Windows and Linux). Mobile support is out of
  scope.
- WoW-themed or polished styling is not expected for the MVP. Functional CSS
  that presents data clearly is sufficient.
- The `battlenet-rs` library's existing `full_character()` function and
  `CachedClient` will be used without modification for this sprint. Any
  library changes discovered as needed will be handled as sub-tasks.

## Polish Phase Checklist *(SDD/TDD — mandatory)*

The following MUST be completed before the feature branch is merged:

- [ ] `docs/specification.md` updated with changes from this spec
- [ ] `docs/architecture.md` updated to reflect the ktoons application
- [ ] `docs/installation.md` updated with ktoons setup steps
- [ ] `docs/usage.md` updated with ktoons usage instructions
- [ ] All tests written first (TDD) and passing
- [ ] Pre-commit suite passes clean (CI variant)
