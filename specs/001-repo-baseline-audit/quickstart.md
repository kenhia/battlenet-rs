# Quickstart: Repo Baseline Audit Verification

**Branch**: `001-repo-baseline-audit`

## Prerequisites

- Rust stable toolchain (1.94.0+): `rustup update stable`
- BattleNet API credentials (client ID + secret) from
  [develop.battle.net](https://develop.battle.net)

## Setup

```bash
# Clone and enter repo
git clone <repo-url> && cd battlenet-rs
git checkout 001-repo-baseline-audit

# Configure credentials
cp .env-EXAMPLE .env
# Edit .env with your client ID, secret, region, locale
```

## Verify Baseline

```bash
# 1. Check compilation
cargo check

# 2. Format check (should pass after fixes applied)
cargo fmt --check

# 3. Lint check (should pass after fixes applied)
cargo clippy --all-targets --all-features -- -D warnings

# 4. Run all tests (requires live API access)
cargo test

# 5. Full CI gate (all-in-one)
cargo fmt --check && \
cargo clippy --all-targets --all-features -- -D warnings && \
cargo test
```

## Run Examples

```bash
# WoW Token price + character profile status
cargo run --example get-client-token

# Character profile queries
cargo run --example char-profile

# Auction data parsing (offline, uses data/ files)
cargo run --example auction-play
```

## Expected Results

- `cargo test`: 10 tests pass (5 achievement, 2 connected realm, 1 wow token, 1 unit, 1 doc test)
- `cargo fmt --check`: exit 0 (no diffs)
- `cargo clippy -- -D warnings`: exit 0 (no warnings)
