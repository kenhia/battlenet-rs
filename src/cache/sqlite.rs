use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Row, SqlitePool};
use std::str::FromStr;

use super::{CacheEntry, CacheError, CacheStore};

pub struct SqliteCacheStore {
    pool: SqlitePool,
}

impl SqliteCacheStore {
    pub async fn new(url: &str) -> Result<Self, CacheError> {
        let options = SqliteConnectOptions::from_str(url)
            .map_err(|e| CacheError::DatabaseError(e.to_string()))?
            .create_if_missing(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await
            .map_err(|e| CacheError::DatabaseError(e.to_string()))?;

        Ok(Self { pool })
    }
}

#[async_trait]
impl CacheStore for SqliteCacheStore {
    async fn initialize(&self) -> Result<(), CacheError> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS cache_entries (
                cache_key    TEXT PRIMARY KEY,
                namespace    TEXT NOT NULL CHECK(namespace IN ('static', 'dynamic', 'profile')),
                response     TEXT NOT NULL,
                fetched_at   TEXT NOT NULL,
                character_id INTEGER,
                realm_slug   TEXT,
                char_name    TEXT
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| CacheError::SchemaInitError(e.to_string()))?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_cache_namespace ON cache_entries(namespace)")
            .execute(&self.pool)
            .await
            .map_err(|e| CacheError::SchemaInitError(e.to_string()))?;

        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_cache_character ON cache_entries(realm_slug, char_name)",
        )
        .execute(&self.pool)
        .await
        .map_err(|e| CacheError::SchemaInitError(e.to_string()))?;

        Ok(())
    }

    async fn get(&self, key: &str) -> Result<Option<CacheEntry>, CacheError> {
        let row = sqlx::query(
            "SELECT cache_key, namespace, response, fetched_at, character_id, realm_slug, char_name FROM cache_entries WHERE cache_key = ?",
        )
        .bind(key)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| CacheError::DatabaseError(e.to_string()))?;

        match row {
            None => Ok(None),
            Some(row) => {
                let fetched_at_str: String = row
                    .try_get("fetched_at")
                    .map_err(|e| CacheError::DatabaseError(e.to_string()))?;
                let fetched_at = DateTime::parse_from_rfc3339(&fetched_at_str)
                    .map_err(|e| CacheError::SerializationError(e.to_string()))?
                    .with_timezone(&Utc);

                Ok(Some(CacheEntry {
                    cache_key: row
                        .try_get("cache_key")
                        .map_err(|e| CacheError::DatabaseError(e.to_string()))?,
                    namespace: row
                        .try_get("namespace")
                        .map_err(|e| CacheError::DatabaseError(e.to_string()))?,
                    response: row
                        .try_get("response")
                        .map_err(|e| CacheError::DatabaseError(e.to_string()))?,
                    fetched_at,
                    character_id: row
                        .try_get("character_id")
                        .map_err(|e| CacheError::DatabaseError(e.to_string()))?,
                    realm_slug: row
                        .try_get("realm_slug")
                        .map_err(|e| CacheError::DatabaseError(e.to_string()))?,
                    char_name: row
                        .try_get("char_name")
                        .map_err(|e| CacheError::DatabaseError(e.to_string()))?,
                }))
            }
        }
    }

    async fn put(&self, entry: &CacheEntry) -> Result<(), CacheError> {
        let fetched_at_str = entry.fetched_at.to_rfc3339();

        sqlx::query(
            r#"
            INSERT INTO cache_entries (cache_key, namespace, response, fetched_at, character_id, realm_slug, char_name)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(cache_key) DO UPDATE SET
                namespace = excluded.namespace,
                response = excluded.response,
                fetched_at = excluded.fetched_at,
                character_id = excluded.character_id,
                realm_slug = excluded.realm_slug,
                char_name = excluded.char_name
            "#,
        )
        .bind(&entry.cache_key)
        .bind(&entry.namespace)
        .bind(&entry.response)
        .bind(&fetched_at_str)
        .bind(entry.character_id)
        .bind(&entry.realm_slug)
        .bind(&entry.char_name)
        .execute(&self.pool)
        .await
        .map_err(|e| CacheError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<(), CacheError> {
        sqlx::query("DELETE FROM cache_entries WHERE cache_key = ?")
            .bind(key)
            .execute(&self.pool)
            .await
            .map_err(|e| CacheError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn delete_character(&self, realm_slug: &str, char_name: &str) -> Result<(), CacheError> {
        sqlx::query("DELETE FROM cache_entries WHERE realm_slug = ? AND char_name = ?")
            .bind(realm_slug)
            .bind(char_name)
            .execute(&self.pool)
            .await
            .map_err(|e| CacheError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn refresh_character_timestamp(
        &self,
        realm_slug: &str,
        char_name: &str,
    ) -> Result<(), CacheError> {
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            "UPDATE cache_entries SET fetched_at = ? WHERE realm_slug = ? AND char_name = ?",
        )
        .bind(&now)
        .bind(realm_slug)
        .bind(char_name)
        .execute(&self.pool)
        .await
        .map_err(|e| CacheError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
