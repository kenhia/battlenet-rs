"""bnauth — Battle.net User OAuth Helper Flask app."""

import os
import secrets
import time

import redis
import requests
from dotenv import load_dotenv
from flask import Flask, redirect, render_template, request, session

load_dotenv()

# Required env vars — fail fast if missing (FR-010)
_REQUIRED_ENV_VARS = [
    "BATTLENET_CLIENT_ID",
    "BATTLENET_CLIENT_SECRET",
    "FLASK_SECRET_KEY",
    "REDISCLI_AUTH",
]

OAUTH_AUTHORIZE_URL = "https://oauth.battle.net/authorize"
OAUTH_TOKEN_URL = "https://oauth.battle.net/token"


def _check_required_env_vars() -> None:
    missing = [v for v in _REQUIRED_ENV_VARS if not os.environ.get(v)]
    if missing:
        raise RuntimeError(
            f"Missing required environment variables: {', '.join(missing)}"
        )


def _get_redis_client() -> redis.Redis:
    return redis.Redis(
        host=os.environ.get("BNAUTH_REDIS_HOST", "rpi53"),
        port=int(os.environ.get("BNAUTH_REDIS_PORT", "6379")),
        password=os.environ.get("REDISCLI_AUTH"),
        decode_responses=True,
        socket_connect_timeout=3,
    )


def create_app() -> Flask:
    _check_required_env_vars()

    app = Flask(__name__, template_folder="../templates")
    app.secret_key = os.environ["FLASK_SECRET_KEY"]

    client_id = os.environ["BATTLENET_CLIENT_ID"]
    client_secret = os.environ["BATTLENET_CLIENT_SECRET"]
    flask_port = os.environ.get("BNAUTH_FLASK_PORT", "5050")
    # Use 127.0.0.1 not localhost — Windows resolves localhost to ::1 (IPv6)
    # which Flask does not listen on. Override with BNAUTH_CALLBACK_HOST if needed.
    callback_host = os.environ.get("BNAUTH_CALLBACK_HOST", "127.0.0.1")
    redirect_uri = f"http://{callback_host}:{flask_port}/callback"

    @app.route("/")
    def index():
        token_status = None
        try:
            r = _get_redis_client()
            token = r.get("bnauth:access_token")
            if token:
                expires_at_str = r.get("bnauth:expires_at")
                scope = r.get("bnauth:scope")
                remaining = (
                    int(str(expires_at_str)) - int(time.time()) if expires_at_str else 0
                )
                token_status = {
                    "exists": True,
                    "scope": scope,
                    "expires_in_hours": round(remaining / 3600, 1),
                }
        except redis.RedisError:
            pass
        return render_template("index.html", token_status=token_status)

    @app.route("/authorize")
    def authorize():
        state = secrets.token_urlsafe(32)
        session["oauth_state"] = state
        params = {
            "response_type": "code",
            "client_id": client_id,
            "scope": "wow.profile openid",
            "state": state,
            "redirect_uri": redirect_uri,
        }
        query = "&".join(f"{k}={v}" for k, v in params.items())
        return redirect(f"{OAUTH_AUTHORIZE_URL}?{query}")

    @app.route("/callback")
    def callback():
        # Validate state (FR-003)
        returned_state = request.args.get("state", "")
        expected_state = session.pop("oauth_state", None)
        if not expected_state or returned_state != expected_state:
            return render_template(
                "error.html", error="State mismatch — possible CSRF attack."
            ), 400

        error = request.args.get("error")
        if error:
            desc = request.args.get("error_description", error)
            return render_template("error.html", error=desc)

        code = request.args.get("code")
        if not code:
            return render_template(
                "error.html", error="No authorization code received."
            )

        # Exchange code for token (FR-004)
        try:
            resp = requests.post(
                OAUTH_TOKEN_URL,
                data={
                    "redirect_uri": redirect_uri,
                    "grant_type": "authorization_code",
                    "code": code,
                },
                auth=(client_id, client_secret),
                timeout=10,
            )
            resp.raise_for_status()
            token_data = resp.json()
        except (requests.RequestException, ValueError) as e:
            return render_template("error.html", error=f"Token exchange failed: {e}")

        access_token = token_data.get("access_token", "")
        token_type = token_data.get("token_type", "bearer")
        expires_in = int(token_data.get("expires_in", 86399))
        scope = token_data.get("scope", "")
        now = int(time.time())
        expires_at = now + expires_in

        # Store in Redis (FR-005, FR-006)
        try:
            r = _get_redis_client()
            r.set("bnauth:access_token", access_token, ex=expires_in)
            r.set("bnauth:token_type", token_type, ex=expires_in)
            r.set("bnauth:expires_at", str(expires_at), ex=expires_in)
            r.set("bnauth:scope", scope, ex=expires_in)
            r.set("bnauth:obtained_at", str(now), ex=expires_in)
        except redis.RedisError as e:
            return render_template(
                "error.html",
                error=f"Token exchange succeeded but Redis storage failed: {e}",
            )

        return render_template(
            "success.html",
            expires_at=expires_at,
            expires_in_hours=round(expires_in / 3600, 1),
            scope=scope,
        )

    return app


if __name__ == "__main__":
    app = create_app()
    port = int(os.environ.get("BNAUTH_FLASK_PORT", "5050"))
    app.run(host="0.0.0.0", port=port, debug=True)
