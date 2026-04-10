"""Tests for bnauth Flask app — User Story 1: Authorize and Store User Token."""

import os
from unittest.mock import MagicMock, patch

import pytest
import requests as requests_lib

# Set required env vars before importing app
_TEST_ENV = {
    "BATTLENET_CLIENT_ID": "test_client_id",
    "BATTLENET_CLIENT_SECRET": "test_client_secret",
    "FLASK_SECRET_KEY": "test_secret_key",
    "REDISCLI_AUTH": "test_redis_password",
    "BNAUTH_REDIS_HOST": "localhost",
    "BNAUTH_REDIS_PORT": "6379",
    "BNAUTH_FLASK_PORT": "5050",
}


@pytest.fixture
def app():
    with patch.dict(os.environ, _TEST_ENV):
        from bnauth.app import create_app

        app = create_app()
        app.config["TESTING"] = True
        yield app


@pytest.fixture
def client(app):
    return app.test_client()


class TestIndexRoute:
    """T011: Verify index route returns 200 with auth button."""

    @patch("bnauth.app._get_redis_client")
    def test_index_returns_200_with_auth_button(self, mock_redis, client):
        mock_r = MagicMock()
        mock_r.get.return_value = None
        mock_redis.return_value = mock_r

        response = client.get("/")
        assert response.status_code == 200
        assert b"Get Battle.net Auth" in response.data


class TestAuthorizeRoute:
    """T012: Verify /authorize redirects to oauth.battle.net with correct params."""

    def test_authorize_redirects_to_battlenet(self, client):
        response = client.get("/authorize")
        assert response.status_code == 302
        location = response.headers["Location"]
        assert "oauth.battle.net/authorize" in location
        assert "response_type=code" in location
        assert "client_id=test_client_id" in location
        assert (
            "scope=wow.profile+openid" in location
            or "scope=wow.profile%20openid" in location
        )
        assert "state=" in location
        assert "redirect_uri=" in location


class TestCallbackCSRF:
    """T013: Verify /callback rejects mismatched state (CSRF protection)."""

    def test_callback_rejects_mismatched_state(self, client):
        # Set a state in session via authorize first
        with client.session_transaction() as sess:
            sess["oauth_state"] = "expected_state"

        response = client.get("/callback?state=wrong_state&code=test_code")
        assert response.status_code == 400
        assert b"State mismatch" in response.data

    def test_callback_rejects_missing_state(self, client):
        response = client.get("/callback?code=test_code")
        assert response.status_code == 400
        assert b"State mismatch" in response.data


class TestCallbackTokenExchange:
    """T014: Verify /callback exchanges code and stores 5 keys in Redis with TTL."""

    @patch("bnauth.app._get_redis_client")
    @patch("bnauth.app.requests.post")
    def test_callback_stores_token_in_redis(self, mock_post, mock_redis, client):
        # Mock Battle.net token response
        mock_response = MagicMock()
        mock_response.status_code = 200
        mock_response.json.return_value = {
            "access_token": "test_access_token",
            "token_type": "bearer",
            "expires_in": 86399,
            "scope": "openid",
        }
        mock_response.raise_for_status = MagicMock()
        mock_post.return_value = mock_response

        # Mock Redis
        mock_r = MagicMock()
        mock_redis.return_value = mock_r

        # Set state in session
        with client.session_transaction() as sess:
            sess["oauth_state"] = "valid_state"

        response = client.get("/callback?state=valid_state&code=auth_code_123")
        assert response.status_code == 200
        assert b"Authorization Successful" in response.data

        # Verify all 5 keys were stored with TTL
        set_calls = mock_r.set.call_args_list
        keys_set = {call[0][0] for call in set_calls}
        assert keys_set == {
            "bnauth:access_token",
            "bnauth:token_type",
            "bnauth:expires_at",
            "bnauth:scope",
            "bnauth:obtained_at",
        }

        # Verify TTL (ex=86399) was set on each key
        for call in set_calls:
            assert call[1].get("ex") == 86399 or call.kwargs.get("ex") == 86399


class TestCallbackError:
    """T015: Verify /callback shows error page when token exchange fails."""

    @patch("bnauth.app.requests.post")
    def test_callback_shows_error_on_exchange_failure(self, mock_post, client):
        mock_post.side_effect = requests_lib.ConnectionError("Connection refused")

        with client.session_transaction() as sess:
            sess["oauth_state"] = "valid_state"

        response = client.get("/callback?state=valid_state&code=auth_code_123")
        assert b"Token exchange failed" in response.data
        assert b"Try Again" in response.data


class TestEnvVarValidation:
    """T016: Verify app fails fast when required env vars are missing."""

    def test_missing_client_id_raises(self):
        env = {k: v for k, v in _TEST_ENV.items() if k != "BATTLENET_CLIENT_ID"}
        with patch.dict(os.environ, env, clear=True):
            from bnauth.app import create_app

            with pytest.raises(RuntimeError, match="BATTLENET_CLIENT_ID"):
                create_app()

    def test_missing_client_secret_raises(self):
        env = {k: v for k, v in _TEST_ENV.items() if k != "BATTLENET_CLIENT_SECRET"}
        with patch.dict(os.environ, env, clear=True):
            from bnauth.app import create_app

            with pytest.raises(RuntimeError, match="BATTLENET_CLIENT_SECRET"):
                create_app()

    def test_missing_flask_secret_key_raises(self):
        env = {k: v for k, v in _TEST_ENV.items() if k != "FLASK_SECRET_KEY"}
        with patch.dict(os.environ, env, clear=True):
            from bnauth.app import create_app

            with pytest.raises(RuntimeError, match="FLASK_SECRET_KEY"):
                create_app()

    def test_missing_rediscli_auth_raises(self):
        env = {k: v for k, v in _TEST_ENV.items() if k != "REDISCLI_AUTH"}
        with patch.dict(os.environ, env, clear=True):
            from bnauth.app import create_app

            with pytest.raises(RuntimeError, match="REDISCLI_AUTH"):
                create_app()


class TestReauthorization:
    """T036: Verify re-authorization overwrites existing Redis keys with fresh TTLs."""

    @patch("bnauth.app._get_redis_client")
    @patch("bnauth.app.requests.post")
    def test_reauth_overwrites_existing_keys(self, mock_post, mock_redis, client):
        mock_r = MagicMock()
        mock_redis.return_value = mock_r

        # First auth
        mock_response = MagicMock()
        mock_response.json.return_value = {
            "access_token": "first_token",
            "token_type": "bearer",
            "expires_in": 86399,
            "scope": "openid",
        }
        mock_response.raise_for_status = MagicMock()
        mock_post.return_value = mock_response

        with client.session_transaction() as sess:
            sess["oauth_state"] = "state1"
        client.get("/callback?state=state1&code=code1")

        # Second auth (re-authorization)
        mock_response2 = MagicMock()
        mock_response2.json.return_value = {
            "access_token": "second_token",
            "token_type": "bearer",
            "expires_in": 86399,
            "scope": "openid",
        }
        mock_response2.raise_for_status = MagicMock()
        mock_post.return_value = mock_response2

        with client.session_transaction() as sess:
            sess["oauth_state"] = "state2"
        response = client.get("/callback?state=state2&code=code2")
        assert response.status_code == 200

        # The last SET for access_token should contain "second_token"
        access_token_calls = [
            c for c in mock_r.set.call_args_list if c[0][0] == "bnauth:access_token"
        ]
        assert len(access_token_calls) == 2
        assert access_token_calls[-1][0][1] == "second_token"
