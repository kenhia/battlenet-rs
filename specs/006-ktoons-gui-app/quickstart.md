# Quickstart: ktoons — WoW Character Viewer

**Created**: 2026-04-17

## Prerequisites

### System dependencies

**Linux (kubs0)**:
```bash
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file \
  libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

**Windows (cleo)**:
- WebView2 runtime (bundled with Windows 10/11)
- No additional system dependencies

### Toolchain

- **Rust**: 1.94.0+ (already installed)
- **Node.js**: 18+ (for Svelte/Vite frontend)
- **pnpm**: Preferred package manager (or npm/yarn)

### Environment variables

```bash
export BATTLENET_CLIENT_ID="your_client_id"
export BATTLENET_CLIENT_SECRET="your_client_secret"
export BATTLENET_REGION="us"  # optional, defaults to "us"
```

### Blizzard Developer Portal

Register a new redirect URI for OAuth:
```
http://127.0.0.1:5055/callback
```

## Project Setup

### 1. Scaffold the Tauri + Svelte project

```bash
cd /path/to/battlenet-rs
# Option A: create-tauri-app (interactive)
pnpm create tauri-app ktoons
# Select: TypeScript/JavaScript → pnpm → Svelte → TypeScript

# Option B: manual setup
mkdir ktoons && cd ktoons
pnpm create vite . --template svelte-ts
pnpm add -D @tauri-apps/cli@latest
pnpm tauri init
```

### 2. Add workspace member

In the root `Cargo.toml`, add:
```toml
[workspace]
members = ["model-macro", "ktoons/src-tauri"]
```

### 3. Configure Tauri backend dependencies

In `ktoons/src-tauri/Cargo.toml`:
```toml
[dependencies]
battlenet-rs = { path = "../..", features = ["wow", "user", "db-sqlite"] }
tauri = { version = "2", features = [] }
tauri-plugin-oauth = "2"
tauri-plugin-shell = "2"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
reqwest = { version = "0.12", features = ["json"] }
```

### 4. Install frontend dependencies

```bash
cd ktoons
pnpm install
pnpm add @tauri-apps/api @fabianlars/tauri-plugin-oauth
```

## Running

### Development mode

```bash
cd ktoons
pnpm tauri dev
```

This compiles the Rust backend and starts the Vite dev server with hot reload
for the Svelte frontend.

### Quick test (Rust backend only)

```bash
cd ktoons/src-tauri
cargo check
cargo test
```

## Architecture at a Glance

```
User ──► Svelte UI ──invoke()──► Tauri Commands (Rust)
                                       │
                                 ┌─────┴──────┐
                                 │ CachedClient│
                                 │  (SQLite)   │
                                 └─────┬──────┘
                                       │
                                 Blizzard API
```

**Flow**: Svelte calls Tauri commands → Rust handlers use `CachedClient` →
cache hit returns instantly, cache miss fetches from API → JSON response
sent back to Svelte → components render data.
