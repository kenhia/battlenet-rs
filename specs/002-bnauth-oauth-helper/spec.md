# Feature Specification: bnauth — Battle.net User OAuth Helper

**Feature Branch**: `002-bnauth-oauth-helper`  
**Created**: 2026-04-08  
**Status**: Draft  
**Spec Path**: `specs/002-bnauth-oauth-helper/spec.md` *(SDD: required before implementation)*  
**Input**: User description: "bnauth — Battle.net User OAuth Helper"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Authorize and Store User Token (Priority: P1)

As a developer working on `kubs0` (headless Linux), I need to obtain a Battle.net user-scoped OAuth token so that I can access profile endpoints (character list, collections, protected profiles) from `battlenet-rs`. I open the `bnauth` web app on `cleo` (Windows, has a browser), click a button, log in on Battle.net, and the resulting access token is stored in Redis where `kubs0` can read it.

**Why this priority**: This is the core value of the entire feature — without token acquisition and storage, nothing else works.

**Independent Test**: Start the `bnauth` Flask app on `cleo`, click "Get Battle.net Auth," log in on Battle.net, verify that the token keys (`bnauth:access_token`, `bnauth:expires_at`, etc.) appear in Redis on `rpi53` with correct TTLs.

**Acceptance Scenarios**:

1. **Given** the `bnauth` Flask app is running on `cleo` with valid environment variables, **When** the user clicks the "Get Battle.net Auth" button and completes login on Battle.net, **Then** the app exchanges the authorization code for an access token and stores it in Redis with the `bnauth:` key prefix.
2. **Given** the token exchange succeeds, **When** the app stores the token in Redis, **Then** all five keys (`bnauth:access_token`, `bnauth:token_type`, `bnauth:expires_at`, `bnauth:scope`, `bnauth:obtained_at`) are set with a TTL matching the `expires_in` value from the token response.
3. **Given** the token exchange succeeds, **When** the app finishes storing the token, **Then** the user sees a success page showing the token expiry time and granted scopes.
4. **Given** the authorization code exchange fails (e.g., expired code, network error), **When** the error is returned from Battle.net, **Then** the user sees an error page with a clear message and a link to retry.

---

### User Story 2 — Read User Token from Redis in battlenet-rs (Priority: P2)

As a developer on `kubs0`, I need `battlenet-rs` to read the user access token from Redis so that I can call user-scoped Battle.net API endpoints (e.g., account profile, character list) without needing a browser.

**Why this priority**: This completes the end-to-end flow — the Rust library consumes the token that the Python app stored. Without this, the token in Redis is unused.

**Independent Test**: After a token has been stored in Redis by `bnauth`, call the Rust method to read the token, verify it returns the correct access token string and expiry information.

**Acceptance Scenarios**:

1. **Given** a valid `bnauth:access_token` key exists in Redis, **When** the Rust code calls the Redis token reader, **Then** it returns the access token string and metadata (expiry, scope).
2. **Given** no `bnauth:access_token` key exists in Redis (token expired or never set), **When** the Rust code calls the Redis token reader, **Then** it returns an appropriate error indicating no user token is available.
3. **Given** the Redis server is unreachable, **When** the Rust code attempts to read the token, **Then** it returns an appropriate connection error.

---

### User Story 3 — Re-authorize After Token Expiry (Priority: P3)

As a developer, when my 24-hour token expires, I need to re-authorize quickly by revisiting the `bnauth` app and clicking the button again.

**Why this priority**: Tokens expire daily. The re-auth flow must be frictionless so the developer spends minimal time on it.

**Independent Test**: Wait for a token to expire (or delete the Redis keys), visit the `bnauth` app, complete the auth flow, verify new keys are stored in Redis with fresh TTLs.

**Acceptance Scenarios**:

1. **Given** the user's previous token has expired and Redis keys have been removed by TTL, **When** the user visits `bnauth` and clicks "Get Battle.net Auth" again, **Then** a new token is obtained and stored in Redis, replacing the expired one.
2. **Given** a valid token already exists in Redis, **When** the user re-authorizes, **Then** the existing keys are overwritten with the new token data and fresh TTLs.

---

### Edge Cases

- What happens when the user denies authorization on Battle.net? The app receives an error callback and displays a clear error message.
- What happens when the `state` parameter in the callback does not match the one sent in the authorize request? The app rejects the callback and displays a CSRF error.
- What happens when Redis is unreachable during token storage? The app displays the token exchange succeeded but storage failed, and suggests checking the Redis connection.
- What happens when environment variables are missing at startup? The app fails fast with a clear message listing the missing variables.
- What happens when the OAuth redirect URI doesn't match what's registered in the Blizzard Developer Portal? Battle.net returns an error; the user sees guidance to check the portal configuration.

## Requirements *(mandatory)*

### Functional Requirements

#### bnauth Flask App (Deliverable 1)

- **FR-001**: The app MUST display a single-page UI with a "Get Battle.net Auth" button.
- **FR-002**: Clicking the button MUST redirect the browser to the Battle.net OAuth authorize endpoint with `response_type=code`, `client_id`, `scope=wow.profile openid`, a random `state` value, and the configured `redirect_uri`.
- **FR-003**: The app MUST validate the `state` parameter on the OAuth callback to prevent CSRF attacks.
- **FR-004**: The app MUST exchange the authorization code for an access token by POSTing to the Battle.net token endpoint with HTTP Basic authentication (client_id:client_secret).
- **FR-005**: The app MUST store the following keys in Redis with the `bnauth:` prefix: `access_token`, `token_type`, `expires_at` (calculated epoch), `scope` (the *granted* scope from the token response, which may differ from the requested scope), and `obtained_at` (current epoch).
- **FR-006**: All Redis keys MUST be set with a TTL equal to the `expires_in` value from the token response.
- **FR-007**: The app MUST display a success page showing token expiry information after successful authorization.
- **FR-008**: The app MUST display an error page with a descriptive message and retry link when authorization or token exchange fails.
- **FR-009**: The app MUST read configuration from environment variables: `BATTLENET_CLIENT_ID`, `BATTLENET_CLIENT_SECRET`, `BATTLENET_REGION` (default: `us`), `REDISCLI_AUTH`, `BNAUTH_REDIS_HOST` (default: `rpi53`), `BNAUTH_REDIS_PORT` (default: `6379`), `BNAUTH_FLASK_PORT` (default: `5050`), `FLASK_SECRET_KEY`.
- **FR-010**: The app MUST fail fast at startup if required environment variables (`BATTLENET_CLIENT_ID`, `BATTLENET_CLIENT_SECRET`, `FLASK_SECRET_KEY`, `REDISCLI_AUTH`) are missing.
- **FR-011**: The app MUST use `http://localhost:<BNAUTH_FLASK_PORT>/callback` as the OAuth redirect URI.

#### Redis Token Reader in battlenet-rs (Deliverable 2)

- **FR-012**: The `battlenet-rs` library MUST provide a Redis-based token reader gated behind an optional `redis` cargo feature.
- **FR-013**: The token reader MUST connect to Redis using `BNAUTH_REDIS_HOST`, `BNAUTH_REDIS_PORT`, and `REDISCLI_AUTH` environment variables.
- **FR-014**: The token reader MUST read the `bnauth:access_token` key and return the user access token.
- **FR-015**: The token reader MUST return an error when the token key does not exist (expired or never set).
- **FR-016**: The token reader MUST return a connection error when Redis is unreachable.
- **FR-017**: The user token MUST remain separate from the existing client credentials token used for Game Data APIs.

### Key Entities

- **User Access Token**: A bearer token obtained via OAuth authorization code flow, granting access to user-scoped Battle.net API endpoints. Attributes: token string, token type, expiry timestamp, granted scopes, acquisition timestamp.
- **OAuth Authorization Code**: A short-lived code received from Battle.net after user login, exchanged once for an access token. Attributes: code string, state parameter for CSRF validation.
- **Redis Token Store**: A set of key-value pairs under the `bnauth:` prefix in Redis, representing the most recent user access token and its metadata, with TTLs matching the token's lifetime.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A user can complete the full authorization flow (click button → log in → token stored) in under 60 seconds.
- **SC-002**: After authorization, the Rust library can successfully read the token from Redis and use it to call at least one user-scoped Battle.net endpoint (e.g., account profile).
- **SC-003**: Token re-authorization (repeating the flow after expiry) takes under 30 seconds.
- **SC-004**: 100% of authorization failures (denied access, expired code, network error) result in a user-visible error message with a retry option.
- **SC-005**: The token stored in Redis is automatically removed after it expires, preventing use of stale tokens.

## Assumptions

- The developer has a valid Battle.net developer account with an application registered in the Blizzard Developer Portal.
- The registered application's redirect URI is set to `http://localhost:5050/callback` (when using default `BNAUTH_FLASK_PORT=5050`).
- The Redis server on `rpi53` is accessible from both `cleo` (for writing) and `kubs0` (for reading).
- The developer runs `bnauth` on-demand when a new token is needed — it is not a persistent background service.
- The `wow.profile` and `openid` scopes are sufficient for the desired user-scoped API endpoints.
- CN region (`oauth.battlenet.com.cn`) is explicitly excluded from `bnauth` scope; support may be added in a later sprint.
- No refresh token is available from Battle.net; re-authorization is required every 24 hours.
- The `bnauth` Flask app lives in the `bnauth/` directory within the `battlenet-rs` repository.
- Python project uses modern tooling (pyproject.toml, uv for package management).
- `http://localhost` is an accepted redirect URI for development/testing by Battle.net OAuth.

## Polish Phase Checklist *(SDD/TDD — mandatory)*

The following MUST be completed before the feature branch is merged:

- [x] `docs/specification.md` updated with changes from this spec
- [x] `docs/architecture.md` updated to reflect any structural changes
- [x] `docs/installation.md` updated if setup steps changed
- [x] `docs/usage.md` updated with new usage examples
- [x] All tests written first (TDD) and passing
- [x] Pre-commit suite passes clean (CI variant)
