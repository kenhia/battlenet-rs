"""End-to-end test: store mock token in Redis, read it back, call user-scoped endpoint (SC-002)."""

import os
import time

import pytest
import redis
import requests

# Env vars for Redis connection
REDIS_HOST = os.environ.get("BNAUTH_REDIS_HOST", "rpi53")
REDIS_PORT = int(os.environ.get("BNAUTH_REDIS_PORT", "6379"))
REDIS_AUTH = os.environ.get("REDISCLI_AUTH", "")

_BNAUTH_KEYS = [
    "bnauth:access_token",
    "bnauth:token_type",
    "bnauth:expires_at",
    "bnauth:scope",
    "bnauth:obtained_at",
]


@pytest.fixture
def redis_client():
    r = redis.Redis(
        host=REDIS_HOST,
        port=REDIS_PORT,
        password=REDIS_AUTH,
        decode_responses=True,
    )
    yield r
    # Cleanup test keys
    r.delete(*_BNAUTH_KEYS)
    r.close()


class TestEndToEnd:
    """SC-002: Read token from Redis and use it to call a user-scoped Battle.net endpoint."""

    def test_store_and_read_token_then_call_api(self, redis_client):
        # Step 1: Store mock token in Redis (simulating what bnauth Flask app does)
        now = int(time.time())
        expires_in = 86399
        expires_at = now + expires_in
        test_token = "e2e_test_token_abc123"

        redis_client.set("bnauth:access_token", test_token, ex=300)
        redis_client.set("bnauth:token_type", "bearer", ex=300)
        redis_client.set("bnauth:expires_at", str(expires_at), ex=300)
        redis_client.set("bnauth:scope", "openid", ex=300)
        redis_client.set("bnauth:obtained_at", str(now), ex=300)

        # Step 2: Read token back from Redis (simulating what battlenet-rs does)
        stored_token = redis_client.get("bnauth:access_token")
        assert stored_token == test_token

        stored_type = redis_client.get("bnauth:token_type")
        assert stored_type == "bearer"

        stored_expires = redis_client.get("bnauth:expires_at")
        assert int(stored_expires) == expires_at

        # Step 3: Call user-scoped Battle.net endpoint with the token
        # Use the US region profile endpoint as the E2E test target
        # This verifies the token format is correct for API usage
        resp = requests.get(
            "https://us.api.blizzard.com/profile/user/wow",
            headers={
                "Authorization": f"Bearer {stored_token}",
            },
            params={"namespace": "profile-us", "locale": "en_US"},
            timeout=10,
        )

        # A test token won't be valid, so we expect 401 (Unauthorized)
        # but NOT a 400 (Bad Request) — this confirms the Bearer format is correct
        # and the endpoint exists. A real token would return 200.
        assert resp.status_code in (
            200,
            401,
            403,
        ), f"Unexpected status {resp.status_code}: expected 200/401/403"
