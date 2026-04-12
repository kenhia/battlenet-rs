use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};

use super::{CacheEntry, CacheError, CacheStore};

pub struct PostgresCacheStore {
    pool: PgPool,
}

impl PostgresCacheStore {
    pub async fn new(url: &str) -> Result<Self, CacheError> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(url)
            .await
            .map_err(|e| CacheError::DatabaseError(e.to_string()))?;

        Ok(Self { pool })
    }
}

#[async_trait]
impl CacheStore for PostgresCacheStore {
    async fn initialize(&self) -> Result<(), CacheError> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS cache_entries (
                cache_key    TEXT PRIMARY KEY,
                namespace    TEXT NOT NULL CHECK(namespace IN ('static', 'dynamic', 'profile')),
                response     TEXT NOT NULL,
                fetched_at   TIMESTAMPTZ NOT NULL,
                character_id BIGINT,
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
            "SELECT cache_key, namespace, response, fetched_at, character_id, realm_slug, char_name FROM cache_entries WHERE cache_key = $1",
        )
        .bind(key)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| CacheError::DatabaseError(e.to_string()))?;

        match row {
            None => Ok(None),
            Some(row) => {
                let fetched_at: DateTime<Utc> = row
                    .try_get("fetched_at")
                    .map_err(|e| CacheError::DatabaseError(e.to_string()))?;

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
        sqlx::query(
            r#"
            INSERT INTO cache_entries (cache_key, namespace, response, fetched_at, character_id, realm_slug, char_name)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT(cache_key) DO UPDATE SET
                namespace = EXCLUDED.namespace,
                response = EXCLUDED.response,
                fetched_at = EXCLUDED.fetched_at,
                character_id = EXCLUDED.character_id,
                realm_slug = EXCLUDED.realm_slug,
                char_name = EXCLUDED.char_name
            "#,
        )
        .bind(&entry.cache_key)
        .bind(&entry.namespace)
        .bind(&entry.response)
        .bind(entry.fetched_at)
        .bind(entry.character_id)
        .bind(&entry.realm_slug)
        .bind(&entry.char_name)
        .execute(&self.pool)
        .await
        .map_err(|e| CacheError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<(), CacheError> {
        sqlx::query("DELETE FROM cache_entries WHERE cache_key = $1")
            .bind(key)
            .execute(&self.pool)
            .await
            .map_err(|e| CacheError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn delete_character(&self, realm_slug: &str, char_name: &str) -> Result<(), CacheError> {
        sqlx::query("DELETE FROM cache_entries WHERE realm_slug = $1 AND char_name = $2")
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
        let now = Utc::now();

        sqlx::query(
            "UPDATE cache_entries SET fetched_at = $1 WHERE realm_slug = $2 AND char_name = $3",
        )
        .bind(now)
        .bind(realm_slug)
        .bind(char_name)
        .execute(&self.pool)
        .await
        .map_err(|e| CacheError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
