use serde::{Deserialize, Serialize};

use crate::cache::{CacheEntry, CacheStore};
use crate::client::{json_to_struct, BattleNetClient};
use crate::errors::BattleNetClientError;
#[cfg(feature = "wow")]
use crate::wow_models::{GenerateUrl, UrlArgs};

use chrono::Utc;

/// Cache-aware wrapper around BattleNetClient.
pub struct CachedClient<S: CacheStore> {
    pub client: BattleNetClient,
    store: S,
    ttl_days: u32,
}

impl<S: CacheStore> CachedClient<S> {
    /// Create a new CachedClient wrapping an existing client and cache store.
    /// Calls `store.initialize()` to ensure the schema exists.
    pub async fn new(client: BattleNetClient, store: S) -> Result<Self, crate::cache::CacheError> {
        store.initialize().await?;
        Ok(Self {
            client,
            store,
            ttl_days: 30,
        })
    }

    /// Create with custom TTL (default: 30 days).
    pub async fn new_with_ttl(
        client: BattleNetClient,
        store: S,
        ttl_days: u32,
    ) -> Result<Self, crate::cache::CacheError> {
        store.initialize().await?;
        Ok(Self {
            client,
            store,
            ttl_days,
        })
    }

    /// Get the configured TTL in days.
    pub fn ttl_days(&self) -> u32 {
        self.ttl_days
    }

    /// Access the underlying cache store (for testing/diagnostics).
    pub fn store(&self) -> &S {
        &self.store
    }

    /// Build a cache key from the URL that would be generated for this type + args.
    #[cfg(feature = "wow")]
    fn cache_key<T: GenerateUrl>(&self, url_args: &UrlArgs) -> String {
        T::url(&self.client, url_args)
    }

    /// Check whether a cache entry's TTL has expired.
    fn is_expired(&self, entry: &CacheEntry) -> bool {
        let age = Utc::now() - entry.fetched_at;
        age.num_days() >= self.ttl_days as i64
    }

    /// Attempt to store a response in the cache. On failure, log a warning
    /// but do NOT propagate the error (FR-025).
    #[cfg(feature = "wow")]
    async fn try_put(&self, key: &str, namespace: &str, json: &str, url_args: &UrlArgs) {
        let (realm_slug, char_name) = match url_args {
            UrlArgs::Player { realm_slug, name } => (Some(realm_slug.clone()), Some(name.clone())),
            _ => (None, None),
        };
        let entry = CacheEntry {
            cache_key: key.to_string(),
            namespace: namespace.to_string(),
            response: json.to_string(),
            fetched_at: Utc::now(),
            character_id: None,
            realm_slug,
            char_name,
        };
        if let Err(e) = self.store.put(&entry).await {
            log::warn!("cache write failed for key '{}': {}", key, e);
        }
    }

    /// Validate a profile cache entry whose TTL has expired.
    /// Calls CharacterProfileStatus to check validity.
    /// Returns Ok(true) if entry is still valid, Ok(false) if purged.
    /// On transient failure, returns Err (caller should return stale data).
    #[cfg(all(feature = "wow", feature = "user"))]
    async fn validate_profile_entry(
        &self,
        entry: &CacheEntry,
    ) -> Result<bool, BattleNetClientError> {
        use crate::wow_models::character_profile::CharacterProfileStatus;

        let realm_slug = match &entry.realm_slug {
            Some(r) => r.clone(),
            None => return Ok(false), // can't validate without realm
        };
        let char_name = match &entry.char_name {
            Some(n) => n.clone(),
            None => return Ok(false),
        };

        let args = UrlArgs::Player {
            realm_slug: realm_slug.clone(),
            name: char_name.clone(),
        };

        // Try to call CharacterProfileStatus
        match self.client.get_data::<CharacterProfileStatus>(&args).await {
            Ok(status) => {
                if !status.is_valid {
                    // Invalid character → purge all entries
                    let _ = self.store.delete_character(&realm_slug, &char_name).await;
                    return Ok(false);
                }

                // Check character_id match (if we have a stored ID)
                if let Some(cached_id) = entry.character_id {
                    if cached_id != status.id as i64 {
                        // ID mismatch → purge all entries
                        let _ = self.store.delete_character(&realm_slug, &char_name).await;
                        return Ok(false);
                    }
                }

                // Valid + matching → refresh timestamp for all character entries
                let _ = self
                    .store
                    .refresh_character_timestamp(&realm_slug, &char_name)
                    .await;
                Ok(true)
            }
            Err(BattleNetClientError::ClientRequestFailed(e)) => {
                // Check for 404
                if e.status() == Some(reqwest::StatusCode::NOT_FOUND) {
                    let _ = self.store.delete_character(&realm_slug, &char_name).await;
                    return Ok(false);
                }
                // Transient failure → return stale data
                Err(BattleNetClientError::ClientRequestFailed(e))
            }
            Err(e) => {
                // Other transient failure → return stale data
                Err(e)
            }
        }
    }

    /// Try to return a cached entry for static or profile namespaces.
    /// Returns Ok(Some(response)) if cache hit is valid, Ok(None) for cache miss
    /// or if the entry should be re-fetched.
    #[cfg(feature = "wow")]
    async fn try_cache_hit(
        &self,
        key: &str,
        ns_str: &str,
    ) -> Result<Option<String>, BattleNetClientError> {
        // Dynamic: never use cache
        if ns_str == "dynamic" {
            return Ok(None);
        }

        let entry = match self
            .store
            .get(key)
            .await
            .map_err(BattleNetClientError::CacheError)?
        {
            Some(e) => e,
            None => return Ok(None),
        };

        // Static: always return from cache
        if ns_str == "static" {
            return Ok(Some(entry.response));
        }

        // Profile: check TTL
        if ns_str == "profile" {
            if !self.is_expired(&entry) {
                // Within TTL → safe to return
                return Ok(Some(entry.response));
            }

            // Expired → validate
            #[cfg(feature = "user")]
            {
                match self.validate_profile_entry(&entry).await {
                    Ok(true) => {
                        // Valid → return cached data (timestamp already refreshed)
                        return Ok(Some(entry.response));
                    }
                    Ok(false) => {
                        // Purged → re-fetch
                        return Ok(None);
                    }
                    Err(_) => {
                        // Transient failure → return stale data
                        return Ok(Some(entry.response));
                    }
                }
            }

            // Without "user" feature, can't validate → return stale data
            #[cfg(not(feature = "user"))]
            return Ok(Some(entry.response));
        }

        Ok(None)
    }

    /// Get typed data, using cache based on namespace policy.
    /// - Static: return from cache if present; otherwise fetch, cache, return.
    /// - Profile: cache-first with 30-day TTL validation.
    /// - Dynamic: always fetch from API, cache afterward.
    #[cfg(feature = "wow")]
    pub async fn get_data<T>(&self, url_args: &UrlArgs) -> Result<T, BattleNetClientError>
    where
        T: for<'a> Deserialize<'a> + Serialize + GenerateUrl,
    {
        let key = self.cache_key::<T>(url_args);
        let namespace = T::cache_namespace();
        let ns_str = namespace.as_cache_str();

        if let Some(cached_response) = self.try_cache_hit(&key, ns_str).await? {
            let result: T = json_to_struct(&cached_response)?;
            return Ok(result);
        }

        // Fetch from API
        let json_text = self.client.get_json::<T>(url_args).await?;

        // Cache the result (fire-and-forget on failure per FR-025)
        self.try_put(&key, ns_str, &json_text, url_args).await;

        let result: T = json_to_struct(&json_text)?;
        Ok(result)
    }

    /// Get typed data, bypassing cache (force refresh).
    #[cfg(feature = "wow")]
    pub async fn get_data_force<T>(&self, url_args: &UrlArgs) -> Result<T, BattleNetClientError>
    where
        T: for<'a> Deserialize<'a> + Serialize + GenerateUrl,
    {
        let key = self.cache_key::<T>(url_args);
        let namespace = T::cache_namespace();
        let ns_str = namespace.as_cache_str();

        let json_text = self.client.get_json::<T>(url_args).await?;
        self.try_put(&key, ns_str, &json_text, url_args).await;

        let result: T = json_to_struct(&json_text)?;
        Ok(result)
    }

    /// Get raw JSON, using cache based on namespace policy.
    #[cfg(feature = "wow")]
    pub async fn get_json<T>(&self, url_args: &UrlArgs) -> Result<String, BattleNetClientError>
    where
        T: for<'a> Deserialize<'a> + Serialize + GenerateUrl,
    {
        let key = self.cache_key::<T>(url_args);
        let namespace = T::cache_namespace();
        let ns_str = namespace.as_cache_str();

        if let Some(cached_response) = self.try_cache_hit(&key, ns_str).await? {
            return Ok(cached_response);
        }

        let json_text = self.client.get_json::<T>(url_args).await?;
        self.try_put(&key, ns_str, &json_text, url_args).await;
        Ok(json_text)
    }

    /// Get typed data with user token, using cache based on namespace policy.
    #[cfg(feature = "wow")]
    pub async fn get_data_with_token<T>(
        &self,
        url_args: &UrlArgs,
        token: &str,
    ) -> Result<T, BattleNetClientError>
    where
        T: for<'a> Deserialize<'a> + Serialize + GenerateUrl,
    {
        let key = self.cache_key::<T>(url_args);
        let namespace = T::cache_namespace();
        let ns_str = namespace.as_cache_str();

        if let Some(cached_response) = self.try_cache_hit(&key, ns_str).await? {
            let result: T = json_to_struct(&cached_response)?;
            return Ok(result);
        }

        let url = T::url(&self.client, url_args);
        let response = self.client.send_request_with_token(url, token).await?;
        let json_text = response.text().await?;

        self.try_put(&key, ns_str, &json_text, url_args).await;

        let result: T = json_to_struct(&json_text)?;
        Ok(result)
    }

    /// Get typed data with user token, bypassing cache (force refresh).
    #[cfg(feature = "wow")]
    pub async fn get_data_with_token_force<T>(
        &self,
        url_args: &UrlArgs,
        token: &str,
    ) -> Result<T, BattleNetClientError>
    where
        T: for<'a> Deserialize<'a> + Serialize + GenerateUrl,
    {
        let key = self.cache_key::<T>(url_args);
        let namespace = T::cache_namespace();
        let ns_str = namespace.as_cache_str();

        let url = T::url(&self.client, url_args);
        let response = self.client.send_request_with_token(url, token).await?;
        let json_text = response.text().await?;

        self.try_put(&key, ns_str, &json_text, url_args).await;

        let result: T = json_to_struct(&json_text)?;
        Ok(result)
    }
}
