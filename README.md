# battlenet-rs

A Rust library wrapping the Blizzard Battle.net REST APIs for World of Warcraft.
Provides type-safe access to Game Data and Profile API endpoints with automatic
OAuth client-credentials token management.

## Features

- **~167 WoW API endpoints** across 50 categories (Game Data + Profile)
- **Automatic OAuth** — client-credentials token acquisition and caching
- **Type-safe models** — serde structs for every endpoint response
- **Feature-gated** — compile only what you need (`wow`, `user`, `redis`)
- **`bendpoint` proc macro** — define an endpoint struct in a few lines; the
  macro generates `Deserialize`, result type aliases, and URL construction
- **All regions** — US, EU, KR, TW, CN with locale support

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
battlenet-rs = { path = ".", features = ["wow"] }
```

Set environment variables (or use a `.env` file):

```bash
export BNET_CLIENT_ID="your-client-id"
export BNET_CLIENT_SECRET="your-client-secret"
```

```rust
use battlenet_rs::client::BattleNetClient;
use battlenet_rs::wow_models::mount::*;

#[tokio::main]
async fn main() {
    let client = BattleNetClient::new_from_environment().await.unwrap();
    let mounts: MountsIndex = client.get_data(&Default::default()).await.unwrap();
    println!("Total mounts: {}", mounts.mounts.len());
}
```

## Feature Flags

| Flag | Purpose |
|------|---------|
| `wow` | WoW Game Data API models (~130 endpoints, 33 modules) |
| `user` | WoW Profile API models (~37 endpoints, 17 modules; implies `wow`) |
| `redis` | Redis-backed user token reader |

Default (no features) = core client, auth, region, namespace, and error types.

## Examples

Run any example with `cargo run --example <name> --features <flags>`:

```bash
# Game Data (--features wow)
cargo run --example mounts --features wow
cargo run --example items --features wow
cargo run --example achievements --features wow
cargo run --example realms --features wow
cargo run --example spells --features wow

# Profile APIs (--features wow,user)
cargo run --example char-profile --features wow,user
cargo run --example character-collections --features wow,user
cargo run --example account-profile --features wow,user
```

See the [examples/](examples/) directory for the full list.

## Project Structure

```text
src/
├── lib.rs               # Crate root
├── client.rs            # BattleNetClient — HTTP + OAuth
├── errors.rs            # Error types
├── namespace.rs         # WowNamespace (Static, Dynamic, Profile)
├── region.rs            # BattleNetRegion (US, EU, KR, TW, CN)
└── wow_models/          # Endpoint models (feature-gated)
    ├── mod.rs           # UrlArgs, GenerateUrl trait
    ├── core_structs.rs  # Shared types (HrefLink, NameAndId, etc.)
    ├── achievement.rs   # Achievement endpoints
    ├── mount.rs         # Mount endpoints
    ├── ...              # 48 more module files
model-macro/             # bendpoint proc macro crate
examples/                # Runnable examples
docs/                    # Architecture, usage, and setup guides
specs/                   # Feature specifications and task plans
```

## Documentation

- [Architecture](docs/architecture.md) — module layout, data flow, OAuth lifecycle
- [Usage](docs/usage.md) — API patterns, examples, error handling
- [Installation](docs/installation.md) — prerequisites and setup
- [Specification](docs/specification.md) — feature overview and quality standards

## The `bendpoint` Macro

Most endpoint structs use the `#[bendpoint]` proc macro to eliminate boilerplate:

```rust
#[bendpoint(endpoint = "data/wow/mount/{id}" url_args = "Id" namespace = "static")]
struct Mount {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
}
```

This single annotation generates `#[derive(Debug, Deserialize)]`, `pub` visibility,
`MountResult` / `MountJsonResult` type aliases, and the `GenerateUrl` implementation.

## Future Direction

- **WoW Classic** API coverage (feature flag `wow-classic` is stubbed)
- **Other games** — Diablo, Hearthstone, StarCraft II (feature flags stubbed)
- **Static namespace diffing** — detect changes in index endpoints
- **Publishing to crates.io**

## Blizzard API Terms of Use

Use of Battle.net API data is subject to the
[Blizzard Developer API Terms of Use](https://www.blizzard.com/en-us/legal/a2989b50-5f16-43b1-abec-2ae17cc09dd6/blizzard-developer-api-terms-of-use).
Cached character data must be re-validated every 30 days per Section 2.R.

## License

This project is not yet licensed. See [Initial Goals](docs/Initial-Goals.md)
for the original vision.
