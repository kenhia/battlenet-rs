# Installation

## Prerequisites

- **Rust toolchain**: 1.94.0+ (stable). Install via [rustup](https://rustup.rs/):
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **BattleNet API credentials**: Register at [Blizzard Developer Portal](https://develop.battle.net/) to get a Client ID and Client Secret.

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

## Build

```sh
cargo build
```

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
