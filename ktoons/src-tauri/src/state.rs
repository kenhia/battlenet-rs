use battlenet_rs::cache::cached_client::CachedClient;
use battlenet_rs::cache::sqlite::SqliteCacheStore;
use tokio::sync::Mutex;

/// Application state managed by Tauri.
///
/// `client` is thread-safe internally (sqlx pool + internal mutex for access token).
/// `user_token` is wrapped in an async Mutex because token exchange involves await.
pub struct AppState {
    pub client: CachedClient<SqliteCacheStore>,
    pub user_token: Mutex<Option<UserToken>>,
}

/// In-memory user token obtained via OAuth.
/// Separate from the library's `UserAccessToken` (which reads from Redis).
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct UserToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_at: i64,
    pub scope: String,
    pub obtained_at: i64,
}
