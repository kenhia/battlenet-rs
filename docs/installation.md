# Installation

## Prerequisites

- **Rust toolchain**: 1.94.0+ (stable). Install via [rustup](https://rustup.rs/):
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **BattleNet API credentials**: Register at [Blizzard Developer Portal](https://develop.battle.net/) to get a Client ID and Client Secret.
- **SQLite** (optional): For `db-sqlite` feature. Usually pre-installed on Linux/macOS.
- **PostgreSQL** (optional): For `db-postgres` feature. Requires a running PostgreSQL server.

## Clone & Setup

```sh
git clone https://github.com/Blizzard/battlenet-rs.git
cd battlenet-rs
```

### Configure API Credentials

Copy the example environment file and fill in your credentials:

```sh
cp .env-EXAMPLE .env
```

Edit `.env` with your values:

```env
BATTLENET_CLIENT_ID="your_client_id_here"
BATTLENET_CLIENT_SECRET="your_client_secret_here"
BATTLENET_REGION="US"
BATTLENET_LOCALE="en_US"
BATTLENET_API_TIMEOUT=5
```

**Supported regions**: `US`, `EU`, `KR`, `TW`, `CN`

**Supported locales by region**:
| Region | Locales |
|--------|---------|
| US | `en_US`, `es_MX`, `pt_BR` |
| EU | `en_GB`, `es_ES`, `fr_FR`, `ru_RU`, `de_DE`, `pt_PT`, `it_IT` |
| KR | `ko_KR` |
| TW | `zh_TW` |
| CN | `zh_CN` |

### Test Configuration (optional)

For running the integration test suite, also set these values in `.env`:

```env
BATTLENET_TEST_REALM_SLUG="trollbane"
BATTLENET_TEST_CHARACTER_NAME="belarsa"
BATTLENET_TEST_CHARACTER_ID=194632169
BATTLENET_TEST_WOW_TOKEN_MIN=150000
BATTLENET_TEST_WOW_TOKEN_MAX=380000
```

> Token min/max are in gold. The API returns prices in copper; tests convert automatically.

## bnauth OAuth Helper (optional)

The `bnauth/` sub-project provides browser-based OAuth for user-scoped API
access. Required only if you need user profile endpoints (character list,
collections, etc.).

### Prerequisites

- **Python 3.13+** with [uv](https://docs.astral.sh/uv/) package manager
- **Redis server** accessible from both the auth machine and the API consumer
- **Battle.net Developer Portal**: Add `http://localhost:5050/callback` as an
  allowed redirect URI in your application settings

### Setup

```sh
cd bnauth
cp .env-EXAMPLE .env
```

Edit `bnauth/.env` with your values:

```env
# Required
BATTLENET_CLIENT_ID="your_client_id_here"
BATTLENET_CLIENT_SECRET="your_client_secret_here"
FLASK_SECRET_KEY="generate_a_random_secret"
REDISCLI_AUTH="your_redis_password"

# Optional (defaults shown)
BATTLENET_REGION="us"
BNAUTH_REDIS_HOST="rpi53"
BNAUTH_REDIS_PORT="6379"
BNAUTH_FLASK_PORT="5050"
```

Install dependencies:

```sh
cd bnauth
uv sync
```

### Rust Redis Feature (optional)

To read user tokens from Redis in Rust code:

```sh
cargo build --features redis
```

Required env vars for the Rust reader: `BNAUTH_REDIS_HOST`, `BNAUTH_REDIS_PORT`, `REDISCLI_AUTH`.

## Build

Default build (core only — client, auth, region):

```sh
cargo build
```

### Feature Flags

Enable WoW Game Data APIs:

```sh
cargo build --features wow
```

Enable WoW Game Data + Profile APIs:

```sh
cargo build --features wow,user
```

If adding as a dependency:

```toml
[dependencies]
battlenet-rs = { path = "../battlenet-rs", features = ["wow"] }        # Game Data
battlenet-rs = { path = "../battlenet-rs", features = ["wow", "user"] } # + Profile
```

| Flag | Purpose |
|------|---------|
| `wow` | WoW Game Data API models (~130 endpoints) |
| `user` | WoW Profile API models (~37 endpoints; requires `wow`) |
| `redis` | Redis-based user token reader |
| `db-sqlite` | SQLite-backed API response cache (mutually exclusive with `db-postgres`) |
| `db-postgres` | PostgreSQL-backed API response cache (mutually exclusive with `db-sqlite`) |

## Run Tests

Tests hit the live BattleNet API, so valid credentials in `.env` are required:

```sh
cargo test
```

## Run an Example

```sh
cargo run --example get-client-token
```

## Verify Code Quality

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

## ktoons Desktop App

### Additional Prerequisites

- **Node.js**: v20+ (v24 recommended)
- **pnpm**: v10+ (`npm install -g pnpm`)
- **System libraries** (Linux/Debian):
  ```sh
  sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file \
    libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
  ```

### Setup & Build

```sh
cd ktoons
pnpm install
pnpm tauri build    # Production build
# or
pnpm tauri dev      # Development mode with hot reload
```

### Environment Variables

ktoons reads the same `.env` file as the library:

```env
BATTLE_NET_CLIENT_ID=your_client_id
BATTLE_NET_CLIENT_SECRET=your_client_secret
BATTLE_NET_REGION=us
```
