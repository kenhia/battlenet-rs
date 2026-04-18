# Research: ktoons — WoW Character Viewer

**Created**: 2026-04-17
**Feature**: [spec.md](spec.md) | [plan.md](plan.md)

## R1: Tauri 2 + Svelte Integration

**Decision**: Use Tauri 2 with Svelte 5 (via SvelteKit or Vite) and TypeScript.

**Rationale**:
- Tauri 2 has first-class Svelte support via `create-tauri-app` scaffolding.
- Tauri 2 uses the OS webview (WebView2 on Windows, webkit2gtk on Linux) —
  no bundled Chromium, small binary size.
- The Tauri backend (Rust) communicates with the Svelte frontend via commands
  (IPC), which are type-safe, async-capable, and support `Result` error handling.
- Svelte 5's reactivity model (runes) is well-suited for rendering API data.

**Alternatives considered**:
- Electron: Heavier runtime, bundles Chromium (~150MB). Overkill for this app.
- Plain Vite + Svelte (no SvelteKit): Viable, but SvelteKit gives us
  routing and layout conventions for free. Either works for MVP.

## R2: Tauri Command Pattern for battlenet-rs Integration

**Decision**: Expose ~5 async Tauri commands that wrap `battlenet-rs` API calls.

**Rationale**:
- Tauri commands are the primary IPC mechanism. Each command is an `async fn`
  annotated with `#[tauri::command]` that the frontend calls via `invoke()`.
- The `CachedClient` and `BattleNetClient` are held in Tauri managed state
  (via `app.manage()`). Commands access them via `State<'_, AppState>`.
- Since `CachedClient` requires `&self` (not `&mut self`) for reads, and uses
  internal `sqlx::Pool` for concurrency, `std::sync::Mutex` is not needed
  for the client itself. Only the mutable `UserToken` needs a `Mutex`.
- Commands return `Result<T, String>` (or a custom serializable error type).
  The frontend receives JSON-serialized data.

**Key commands**:

| Command | Input | Output | Notes |
|---------|-------|--------|-------|
| `get_realms` | (none) | `Vec<{name, slug}>` | Fetched once, cached |
| `lookup_character` | `realm_slug, name` | `FullCharacter` as JSON | Client token only |
| `login` | (none) | `Vec<AccountCharacter>` | Starts OAuth, returns char list |
| `get_character` | `realm_slug, name` | `FullCharacter` as JSON | Uses user token if available |
| `refresh_character` | `realm_slug, name` | `FullCharacter` as JSON | Force-refresh (bypass cache) |

## R3: OAuth Flow via tauri-plugin-oauth

**Decision**: Use `tauri-plugin-oauth` 2.x to handle Blizzard OAuth entirely
in-app, with the user token held in Tauri managed state.

**Rationale**:
- The plugin spawns a temporary localhost HTTP server on a configured port
  (e.g., 5055) to capture the OAuth redirect.
- The flow: Rust backend constructs the Blizzard authorize URL → opens default
  browser via `shell.open()` → user authenticates → Blizzard redirects to
  `http://127.0.0.1:5055/callback?code=...&state=...` → plugin captures URL
  and passes it to a Rust callback → backend exchanges code for token via
  POST to Blizzard's token endpoint → token stored in `Mutex<Option<UserToken>>`.
- Fixed port (5055) must be registered as a redirect URI in the Blizzard
  Developer Portal: `http://127.0.0.1:5055/callback`.
- CSRF protection via `state` parameter (random token generated before auth,
  verified on callback — same pattern as `bnauth`).

**Alternatives considered**:
- Keep `bnauth` as external service: Adds Redis dependency and requires a
  separate Python process. Overkill for a desktop app.
- Tauri deep linking: Blizzard OAuth does not support custom URI schemes as
  redirect URLs, only `http://` localhost.

## R4: Tauri Managed State Design

**Decision**: Use a struct `AppState` managed by Tauri, containing the
`CachedClient`, `BattleNetClient`, and `Option<UserToken>`.

**Rationale**:
- Tauri wraps managed state in `Arc` automatically — no need for manual `Arc`.
- `CachedClient` is internally thread-safe (uses `sqlx::Pool`), so it can be
  shared without a `Mutex`.
- `BattleNetClient` uses `Mutex<Option<String>>` internally for the access
  token, so it is also safe to share.
- Only `UserToken` is mutable (set after OAuth, cleared on expiry), so it
  needs `tokio::sync::Mutex<Option<UserToken>>` (async mutex because token
  exchange is async).
- Character list for left nav is frontend-only state (Svelte store) — no
  need to persist it in Rust state.

**State structure**:
```rust
struct AppState {
    client: CachedClient,      // or BattleNetClient if no cache
    user_token: tokio::sync::Mutex<Option<UserToken>>,
}
```

## R5: SQLite Cache Path

**Decision**: Use `app.path().app_data_dir()` to get the platform-appropriate
directory, then create `cache.db` within it.

**Rationale**:
- `app_data_dir()` returns:
  - Linux: `~/.local/share/com.ktoons.app/`
  - Windows: `C:\Users\<user>\AppData\Roaming\com.ktoons.app\`
- The directory is created automatically by Tauri if it doesn't exist.
- The SQLite file path is passed to `SqliteCacheStore::new()` at app startup.

**Alternatives considered**:
- Hardcoded path: Not portable across platforms.
- User-configurable: Not needed for MVP.

## R6: Frontend Component Architecture

**Decision**: ~8 Svelte components with a simple layout: left nav + main panel.

**Rationale**:
- The UI is simple enough that a full component library (e.g., Carbon, Skeleton)
  is unnecessary for MVP. Plain HTML + functional CSS.
- Components map directly to MVP display sections from the spec.

**Component breakdown**:

| Component | Responsibility |
|-----------|---------------|
| `LaunchScreen.svelte` | Two-button entry: Login / Quick Lookup with realm dropdown |
| `CharacterNav.svelte` | Left sidebar with character list (grouped by realm) |
| `CharacterHeader.svelte` | Name, level, race, class, faction, guild, realm, title, portrait, ilvl, achievement pts |
| `EquipmentList.svelte` | Equipped gear per slot (name, ilvl, quality color) |
| `StatsPanel.svelte` | Health, power, primary stats |
| `SpecializationsPanel.svelte` | Active + available specs |
| `LoadingSpinner.svelte` | Loading indicator (used during fetch/OAuth) |
| `ErrorDisplay.svelte` | Error messages with retry button |

## R7: FullCharacter → Frontend Data Mapping

**Decision**: Serialize `FullCharacter` as JSON in Tauri commands and define
TypeScript interfaces in `types.ts` for the MVP-relevant fields only.

**Rationale**:
- `FullCharacter` has 28 optional endpoint fields. The MVP only displays 5
  (profile, equipment, statistics, specializations, media).
- The TypeScript interfaces only need to cover the fields the UI renders.
  Extra fields in the JSON are ignored by the frontend.
- This avoids creating a separate DTO — the same `FullCharacter` struct is
  serialized directly (it already implements `Serialize`).

**Alternatives considered**:
- Create a separate `CharacterSummary` DTO that extracts only MVP fields in
  Rust before serializing. Cleaner API surface, but more code for minimal
  benefit in a single-consumer app.

## R8: Workspace Integration

**Decision**: Add `ktoons/src-tauri` as a workspace member in the root
`Cargo.toml`. The Tauri crate depends on `battlenet-rs` via `path = "../.."`.

**Rationale**:
- The root `Cargo.toml` already has `[workspace]` with `model-macro` as a
  member. Adding another member is established pattern.
- Path dependency means library changes are immediately available.
- Tauri's system deps (webkit2gtk, WebView2) only compile when building the
  `ktoons` crate — they don't affect `cargo test` for the library.
- `cargo clippy --all-targets --all-features` from the workspace root will
  need to handle the Tauri crate. May need to exclude it or run clippy per
  crate if system deps are missing on CI.
