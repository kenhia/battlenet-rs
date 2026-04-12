#[cfg(all(feature = "db-sqlite", feature = "db-postgres"))]
compile_error!("Features `db-sqlite` and `db-postgres` are mutually exclusive. Enable only one.");

#[cfg(feature = "db-sqlite")]
pub mod sqlite;

#[cfg(feature = "db-postgres")]
pub mod postgres;

pub mod cached_client;

use async_trait::async_trait;
use chrono::{DateTime, Utc};

/// A cached API response entry.
#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub cache_key: String,
    pub namespace: String,
    pub response: String,
    pub fetched_at: DateTime<Utc>,
    pub character_id: Option<i64>,
    pub realm_slug: Option<String>,
    pub char_name: Option<String>,
}

/// Errors from cache operations.
#[derive(Debug, thiserror::Error)]
pub enum CacheError {
    #[error("database error: {0}")]
    DatabaseError(String),
    #[error("serialization error: {0}")]
    SerializationError(String),
    #[error("schema initialization error: {0}")]
    SchemaInitError(String),
}

/// Abstract cache storage backend.
#[async_trait]
pub trait CacheStore: Send + Sync {
    /// Initialize the cache schema (create tables if not exist).
    async fn initialize(&self) -> Result<(), CacheError>;

    /// Get a cached entry by key.
    async fn get(&self, key: &str) -> Result<Option<CacheEntry>, CacheError>;

    /// Store or update a cache entry.
    async fn put(&self, entry: &CacheEntry) -> Result<(), CacheError>;

    /// Delete a cache entry by key.
    async fn delete(&self, key: &str) -> Result<(), CacheError>;

    /// Delete all cache entries for a specific character.
    async fn delete_character(&self, realm_slug: &str, char_name: &str) -> Result<(), CacheError>;

    /// Update the `fetched_at` timestamp for all entries matching a character.
    async fn refresh_character_timestamp(
        &self,
        realm_slug: &str,
        char_name: &str,
    ) -> Result<(), CacheError>;
}
