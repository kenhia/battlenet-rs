use redis::Commands;

use crate::errors::{BattleNetClientError, BattlenetClientResult};

/// User access token read from Redis, obtained via the bnauth OAuth helper.
#[derive(Debug)]
pub struct UserAccessToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_at: i64,
    pub scope: String,
    pub obtained_at: i64,
}

/// Read the user access token from Redis.
///
/// Connects to Redis using `BNAUTH_REDIS_HOST`, `BNAUTH_REDIS_PORT`, and
/// `REDISCLI_AUTH` environment variables. Returns the token stored by the
/// bnauth Flask app, or an error if the token is unavailable or Redis is
/// unreachable.
pub fn read_user_token() -> BattlenetClientResult<UserAccessToken> {
    let host = std::env::var("BNAUTH_REDIS_HOST").unwrap_or_else(|_| "rpi53".to_string());
    let port = std::env::var("BNAUTH_REDIS_PORT").unwrap_or_else(|_| "6379".to_string());
    let password = std::env::var("REDISCLI_AUTH").unwrap_or_default();

    let client = redis::Client::open(format!("redis://:{password}@{host}:{port}"))?;
    let mut con = client.get_connection()?;

    let access_token: Option<String> = con.get("bnauth:access_token")?;
    let access_token = access_token.ok_or(BattleNetClientError::UserTokenNotAvailable)?;

    let token_type: String = con
        .get("bnauth:token_type")
        .unwrap_or_else(|_| "bearer".to_string());
    let expires_at: String = con
        .get("bnauth:expires_at")
        .unwrap_or_else(|_| "0".to_string());
    let scope: String = con.get("bnauth:scope").unwrap_or_default();
    let obtained_at: String = con
        .get("bnauth:obtained_at")
        .unwrap_or_else(|_| "0".to_string());

    Ok(UserAccessToken {
        access_token,
        token_type,
        expires_at: expires_at.parse().unwrap_or(0),
        scope,
        obtained_at: obtained_at.parse().unwrap_or(0),
    })
}
