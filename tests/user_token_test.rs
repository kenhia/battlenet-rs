#[cfg(feature = "redis")]
mod user_token_tests {
    use battlenet_rs::errors::BattleNetClientError;
    use battlenet_rs::user_token;
    use std::sync::Mutex;

    /// Mutex used to serialize tests that read or write environment variables.
    /// `test_read_user_token_redis_unreachable` mutates env vars globally,
    /// which races with tests that connect using the same vars.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    /// T027: Read UserAccessToken from Redis when bnauth:access_token exists.
    /// Requires live Redis with token keys set.
    #[test]
    fn test_read_user_token_success() {
        let _guard = ENV_LOCK.lock().unwrap();
        // Set up test keys in Redis
        let host = std::env::var("BNAUTH_REDIS_HOST").unwrap_or_else(|_| "rpi53".to_string());
        let port = std::env::var("BNAUTH_REDIS_PORT").unwrap_or_else(|_| "6379".to_string());
        let password = std::env::var("REDISCLI_AUTH").unwrap_or_default();

        let client = redis::Client::open(format!("redis://:{password}@{host}:{port}"))
            .expect("Failed to create Redis client");
        let mut con = client.get_connection().expect("Failed to connect to Redis");

        // Store test token data
        let _: () = redis::cmd("SET")
            .arg("bnauth:access_token")
            .arg("test_token_abc123")
            .arg("EX")
            .arg(300)
            .query(&mut con)
            .expect("Failed to SET access_token");
        let _: () = redis::cmd("SET")
            .arg("bnauth:token_type")
            .arg("bearer")
            .arg("EX")
            .arg(300)
            .query(&mut con)
            .expect("Failed to SET token_type");
        let _: () = redis::cmd("SET")
            .arg("bnauth:expires_at")
            .arg("1744243200")
            .arg("EX")
            .arg(300)
            .query(&mut con)
            .expect("Failed to SET expires_at");
        let _: () = redis::cmd("SET")
            .arg("bnauth:scope")
            .arg("openid")
            .arg("EX")
            .arg(300)
            .query(&mut con)
            .expect("Failed to SET scope");
        let _: () = redis::cmd("SET")
            .arg("bnauth:obtained_at")
            .arg("1744156800")
            .arg("EX")
            .arg(300)
            .query(&mut con)
            .expect("Failed to SET obtained_at");

        let token = user_token::read_user_token().expect("read_user_token should succeed");
        assert_eq!(token.access_token, "test_token_abc123");
        assert_eq!(token.token_type, "bearer");
        assert_eq!(token.expires_at, 1744243200);
        assert_eq!(token.scope, "openid");
        assert_eq!(token.obtained_at, 1744156800);

        // Clean up test keys
        let _: () = redis::cmd("DEL")
            .arg("bnauth:access_token")
            .arg("bnauth:token_type")
            .arg("bnauth:expires_at")
            .arg("bnauth:scope")
            .arg("bnauth:obtained_at")
            .query(&mut con)
            .expect("Failed to clean up test keys");
    }

    /// T028: Return UserTokenNotAvailable when bnauth:access_token does not exist.
    #[test]
    fn test_read_user_token_not_available() {
        let _guard = ENV_LOCK.lock().unwrap();
        // Ensure keys don't exist
        let host = std::env::var("BNAUTH_REDIS_HOST").unwrap_or_else(|_| "rpi53".to_string());
        let port = std::env::var("BNAUTH_REDIS_PORT").unwrap_or_else(|_| "6379".to_string());
        let password = std::env::var("REDISCLI_AUTH").unwrap_or_default();

        let client = redis::Client::open(format!("redis://:{password}@{host}:{port}"))
            .expect("Failed to create Redis client");
        let mut con = client.get_connection().expect("Failed to connect to Redis");

        let _: () = redis::cmd("DEL")
            .arg("bnauth:access_token")
            .arg("bnauth:token_type")
            .arg("bnauth:expires_at")
            .arg("bnauth:scope")
            .arg("bnauth:obtained_at")
            .query(&mut con)
            .expect("Failed to DEL keys");

        let result = user_token::read_user_token();
        assert!(result.is_err());
        match result.unwrap_err() {
            BattleNetClientError::UserTokenNotAvailable => {} // expected
            other => panic!("Expected UserTokenNotAvailable, got: {other:?}"),
        }
    }

    /// T029: Return RedisError when Redis is unreachable.
    #[test]
    fn test_read_user_token_redis_unreachable() {
        let _guard = ENV_LOCK.lock().unwrap();
        // Save original env vars
        let orig_host = std::env::var("BNAUTH_REDIS_HOST").ok();
        let orig_port = std::env::var("BNAUTH_REDIS_PORT").ok();
        let orig_auth = std::env::var("REDISCLI_AUTH").ok();

        // Point to a non-existent Redis
        std::env::set_var("BNAUTH_REDIS_HOST", "localhost");
        std::env::set_var("BNAUTH_REDIS_PORT", "59999");
        std::env::set_var("REDISCLI_AUTH", "fake");

        let result = user_token::read_user_token();

        // Restore env for other tests
        match orig_host {
            Some(v) => std::env::set_var("BNAUTH_REDIS_HOST", v),
            None => std::env::remove_var("BNAUTH_REDIS_HOST"),
        }
        match orig_port {
            Some(v) => std::env::set_var("BNAUTH_REDIS_PORT", v),
            None => std::env::remove_var("BNAUTH_REDIS_PORT"),
        }
        match orig_auth {
            Some(v) => std::env::set_var("REDISCLI_AUTH", v),
            None => std::env::remove_var("REDISCLI_AUTH"),
        }

        assert!(result.is_err());
        match result.unwrap_err() {
            BattleNetClientError::RedisError(_) => {} // expected
            other => panic!("Expected RedisError, got: {other:?}"),
        }
    }
}
