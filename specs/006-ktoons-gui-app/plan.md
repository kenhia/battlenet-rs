# Implementation Plan: ktoons — WoW Character Viewer

**Branch**: `006-ktoons-gui-app` | **Date**: 2026-04-17 | **Spec**: [spec.md](spec.md)  
**Input**: Feature specification from `specs/006-ktoons-gui-app/spec.md`  

## Summary

Build `ktoons`, a Tauri 2 + Svelte + TypeScript desktop application that displays
WoW character data using the `battlenet-rs` library. The app provides two entry
paths: Quick Lookup (character name + realm, client token only) and Login with
Battle.net (OAuth, full account character list). Character data is fetched via
`CachedClient` with SQLite caching and displayed as a summary view (header,
achievement points, item level, equipped gear, stats, specializations, portrait).

## Technical Context

**Language/Version**: Rust 1.94.0 (stable, edition 2021) for Tauri backend; TypeScript 5.x for Svelte frontend  
**Primary Dependencies**: Tauri 2.x, Svelte 5.x, SvelteKit (or Vite), tauri-plugin-oauth 2.x, battlenet-rs (path dep with features: wow, user, db-sqlite), reqwest 0.12, serde/serde_json 1.x, tokio 1.x  
**Storage**: SQLite (WAL mode) via `battlenet-rs` CachedClient/SqliteCacheStore — platform app data dir  
**Testing**: Rust: `cargo test` (Tauri commands unit tests); Frontend: vitest (Svelte component tests); E2E: manual for MVP  
**Target Platform**: Desktop — Windows (cleo) and Linux (kubs0); uses OS webview (WebView2 on Windows, webkit2gtk on Linux)  
**Project Type**: Desktop application (Tauri)  
**Performance Goals**: <10s uncached character load, <2s cached load, <5s account profile after OAuth  
**Constraints**: Env vars for credentials (MVP), in-memory user token (no Redis), dev mode only (no packaging)  
**Scale/Scope**: Single-user desktop app, ~6 display sections, ~5 Tauri commands, ~8 Svelte components  

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

Verify each principle before implementation begins and again after design:

- [x] **I. SDD** — Feature spec exists at `specs/006-ktoons-gui-app/spec.md` ✅
- [x] **II. TDD** — Test plan documented in spec (US1–US5 acceptance scenarios); Rust Tauri commands will have unit tests; Svelte components will have vitest tests; tests written before implementation
- [x] **III. Code Quality** — Pre-commit suite confirmed runnable. Note: `cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test` applies to `ktoons/src-tauri` crate. Frontend: not yet configured but will use standard svelte-check + eslint
- [x] **IV. User Docs** — `docs/installation.md` and `docs/usage.md` updates are in the spec's Polish Phase Checklist
- [x] **V. Architecture** — `docs/architecture.md` update is in the spec's Polish Phase Checklist

**Gate result: PASS** — No violations. Proceeding to Phase 0.

**Post-design re-check (Phase 1 complete)**: All five principles still satisfied.
Data model, contracts, and quickstart are complete and traceable to spec.
No new violations introduced during design.

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
battlenet-rs/                    # Existing repo root
├── src/                         # battlenet-rs library (existing)
├── tests/                       # Library integration tests (existing)
├── examples/                    # CLI usage examples (existing)
├── model-macro/                 # Proc-macro crate (existing)
├── bnauth/                      # OAuth helper for headless (existing)
├── Cargo.toml                   # Workspace root (add ktoons member)
│
├── ktoons/                      # NEW — Tauri-Svelte desktop app
│   ├── package.json             # Node deps (svelte, vite, tauri-plugin-oauth JS)
│   ├── svelte.config.js         # Svelte config
│   ├── vite.config.ts           # Vite bundler config
│   ├── tsconfig.json            # TypeScript config
│   ├── src/                     # Svelte frontend
│   │   ├── app.html             # HTML shell
│   │   ├── app.css              # Global styles
│   │   ├── lib/                 # Shared components and utilities
│   │   │   ├── components/      # Svelte components
│   │   │   │   ├── CharacterHeader.svelte
│   │   │   │   ├── EquipmentList.svelte
│   │   │   │   ├── StatsPanel.svelte
│   │   │   │   ├── SpecializationsPanel.svelte
│   │   │   │   ├── CharacterNav.svelte
│   │   │   │   ├── LaunchScreen.svelte
│   │   │   │   ├── LoadingSpinner.svelte
│   │   │   │   └── ErrorDisplay.svelte
│   │   │   ├── types.ts         # TypeScript interfaces for Tauri command responses
│   │   │   └── commands.ts      # Tauri invoke wrappers
│   │   └── routes/              # SvelteKit routes (or App.svelte if plain Vite)
│   │       └── +page.svelte     # Main app page
│   ├── src-tauri/               # Tauri Rust backend
│   │   ├── Cargo.toml           # Tauri crate (depends on battlenet-rs path)
│   │   ├── tauri.conf.json      # Tauri config (window, app id, etc.)
│   │   ├── build.rs             # Tauri build script
│   │   └── src/
│   │       ├── main.rs          # Tauri entry point, plugin registration
│   │       ├── commands.rs      # Tauri command handlers
│   │       ├── oauth.rs         # OAuth flow (state, URL build, token exchange)
│   │       └── state.rs         # App state (CachedClient, UserToken, character list)
│   └── tests/                   # Frontend tests (vitest)
│       └── components/
└── specs/006-ktoons-gui-app/    # This sprint's specs
```

**Structure Decision**: Desktop app with frontend + backend (Option 2 variant).
The `ktoons/` directory is a self-contained Tauri project. The Tauri backend
(`src-tauri/`) depends on `battlenet-rs` as a path dependency. The Svelte
frontend (`src/`) communicates with the backend via Tauri commands (IPC).
The root `Cargo.toml` workspace adds `ktoons/src-tauri` as a member.

## Complexity Tracking

No constitution violations. No justifications needed.
