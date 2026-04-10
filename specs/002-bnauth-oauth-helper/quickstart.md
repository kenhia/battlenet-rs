# Quickstart: bnauth — Battle.net User OAuth Helper

**Branch**: `002-bnauth-oauth-helper`

## Prerequisites

- Python 3.13+ and `uv` installed on `cleo` (or any machine with a browser)
- Rust stable toolchain (1.94.0+) on `kubs0`
- Redis server running on `rpi53` (accessible from both machines)
- BattleNet API credentials (client ID + secret) from
  [develop.battle.net](https://develop.battle.net)
- Redirect URI `http://localhost:5050/callback` registered in the Blizzard
  Developer Portal for your application

## Setup: bnauth Flask App

```bash
# From the repo root
cd bnauth/

# Create .env from example
cp .env-EXAMPLE .env
# Edit .env with your credentials, Redis host, etc.

# Install dependencies
uv sync

# Run the app
uv run flask --app bnauth.app run --port 5050
```

## Get a User Token

1. Open `http://localhost:5050` in your browser
2. Click "Get Battle.net Auth"
3. Log in on Battle.net, authorize the app
4. You'll be redirected back — success page shows token expiry

## Verify Token in Redis

```bash
# From any machine with redis-cli
redis-cli -h rpi53 -a $REDISCLI_AUTH GET bnauth:access_token
redis-cli -h rpi53 -a $REDISCLI_AUTH TTL bnauth:access_token
```

## Use Token from Rust

```bash
# Build with redis feature
cargo build --features redis

# Run integration test (requires token in Redis)
cargo test --features redis user_token
```

## Verify Rust CI Suite

```bash
# Without redis feature (default — no Redis dependency)
cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test

# With redis feature
cargo fmt --check && \
cargo clippy --all-targets --all-features -- -D warnings && \
cargo test --features redis
```

## Verify Python CI Suite

```bash
cd bnauth/
uv run ruff format --check
uv run ruff check
uv run pytest -q
```

## Expected Results

- `http://localhost:5050` shows "Get Battle.net Auth" button (or token status)
- After auth flow, Redis has `bnauth:access_token` with ~24h TTL
- `cargo test --features redis` passes (with live Redis + token)
- `cargo test` passes (without redis feature — existing tests unaffected)
