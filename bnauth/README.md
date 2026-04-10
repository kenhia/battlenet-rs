# bnauth — Battle.net User OAuth Helper

`bnauth` is a minimal Flask web app that performs the Battle.net OAuth
authorization code flow via a browser and stores the resulting user access
token in Redis. It enables headless machines (like a Linux dev box without a
browser) to use user-scoped Battle.net API endpoints by running the auth step
on any machine that has a browser.

## Overview

```
Developer (browser on cleo)          Redis on rpi53        battlenet-rs on kubs0
         │                                  │                        │
  visit http://127.0.0.1:5051/             │                        │
         │                                  │                        │
  click "Get Battle.net Auth"               │                        │
         │                                  │                        │
  Battle.net login → callback               │                        │
         │                                  │                        │
         └─── stores 5 keys ──────────────► │ ◄────── read_user_token()
              bnauth:access_token           │
              bnauth:token_type             │
              bnauth:expires_at             │
              bnauth:scope                  │
              bnauth:obtained_at            │
```

Tokens expire after ~24 hours. Re-authorize by visiting the app again.

## Prerequisites

- Python 3.13+ and [uv](https://docs.astral.sh/uv/)
- Redis server accessible from both the auth machine and the API consumer
- A Blizzard Developer Portal application with a registered redirect URI
  (see Setup below)

## Setup

```sh
cd bnauth
cp .env-EXAMPLE .env
```

Edit `.env` with your values:

```env
# Required
BATTLENET_CLIENT_ID=your_client_id
BATTLENET_CLIENT_SECRET=your_client_secret
FLASK_SECRET_KEY=any_random_string
REDISCLI_AUTH=your_redis_password

# Optional (defaults shown)
BATTLENET_REGION=us
BNAUTH_REDIS_HOST=rpi53
BNAUTH_REDIS_PORT=6379
BNAUTH_FLASK_PORT=5051
BNAUTH_CALLBACK_HOST=127.0.0.1
```

Install dependencies:

```sh
uv sync
```

Register the redirect URI in the
[Blizzard Developer Portal](https://develop.battle.net/access/clients):

```
http://127.0.0.1:5051/callback
```

## Running

```sh
cd bnauth
uv run python -m bnauth.app
```

Navigate to `http://127.0.0.1:5051/` (use the IP, not `localhost` — see
[port-troubleshoot.md](port-troubleshoot.md)), click **Get Battle.net Auth**,
log in on Battle.net, and the token is saved to Redis automatically.

## Port Selection

If using **VS Code Remote SSH**, VS Code may auto-forward Flask's port, which
blocks the OAuth callback. Use any port that VS Code has not forwarded. Check
the VS Code **Ports** panel (`Ctrl+Shift+P` → *Ports: Focus on Ports View*).

See [port-troubleshoot.md](port-troubleshoot.md) for the full diagnosis and
workarounds.

## Running Tests

```sh
cd bnauth
uv run pytest -q
```

Full CI suite:

```sh
uv run ruff format --check && uv run ruff check && uv run ty check && uv run pytest -q
```
