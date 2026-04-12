#![cfg(all(feature = "db-sqlite", feature = "wow"))]

use async_trait::async_trait;
use battlenet_rs::cache::cached_client::CachedClient;
use battlenet_rs::cache::sqlite::SqliteCacheStore;
use battlenet_rs::cache::{CacheEntry, CacheError, CacheStore};
use chrono::Utc;
use std::sync::Mutex as StdMutex;
use std::time::Duration;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

async fn test_store() -> SqliteCacheStore {
    let store = SqliteCacheStore::new("sqlite::memory:").await.unwrap();
    store.initialize().await.unwrap();
    store
}

fn sample_entry(key: &str, namespace: &str) -> CacheEntry {
    CacheEntry {
        cache_key: key.to_string(),
        namespace: namespace.to_string(),
        response: r#"{"id":1,"name":"test"}"#.to_string(),
        fetched_at: Utc::now(),
        character_id: None,
        realm_slug: None,
        char_name: None,
    }
}

fn sample_profile_entry(key: &str, char_id: i64, realm: &str, name: &str) -> CacheEntry {
    CacheEntry {
        cache_key: key.to_string(),
        namespace: "profile".to_string(),
        response: r#"{"id":1,"name":"test"}"#.to_string(),
        fetched_at: Utc::now(),
        character_id: Some(char_id),
        realm_slug: Some(realm.to_string()),
        char_name: Some(name.to_string()),
    }
}

// ---------------------------------------------------------------------------
// SqliteCacheStore — CacheStore trait implementation tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn sqlite_initialize() {
    // Should create schema without error; second call is idempotent.
    let store = SqliteCacheStore::new("sqlite::memory:").await.unwrap();
    store.initialize().await.unwrap();
    store.initialize().await.unwrap(); // idempotent
}

#[tokio::test]
async fn sqlite_put_and_get() {
    let store = test_store().await;
    let entry = sample_entry("key-1", "static");
    store.put(&entry).await.unwrap();

    let retrieved = store
        .get("key-1")
        .await
        .unwrap()
        .expect("entry should exist");
    assert_eq!(retrieved.cache_key, "key-1");
    assert_eq!(retrieved.namespace, "static");
    assert_eq!(retrieved.response, r#"{"id":1,"name":"test"}"#);
}

#[tokio::test]
async fn sqlite_get_missing_key() {
    let store = test_store().await;
    let result = store.get("nonexistent").await.unwrap();
    assert!(result.is_none());
}

#[tokio::test]
async fn sqlite_put_overwrite() {
    let store = test_store().await;
    let entry1 = sample_entry("ow-key", "static");
    store.put(&entry1).await.unwrap();

    let mut entry2 = sample_entry("ow-key", "static");
    entry2.response = r#"{"id":2,"name":"updated"}"#.to_string();
    store.put(&entry2).await.unwrap();

    let retrieved = store.get("ow-key").await.unwrap().unwrap();
    assert_eq!(retrieved.response, r#"{"id":2,"name":"updated"}"#);
}

#[tokio::test]
async fn sqlite_delete() {
    let store = test_store().await;
    let entry = sample_entry("del-key", "dynamic");
    store.put(&entry).await.unwrap();

    store.delete("del-key").await.unwrap();
    assert!(store.get("del-key").await.unwrap().is_none());
}

#[tokio::test]
async fn sqlite_delete_nonexistent_is_ok() {
    let store = test_store().await;
    // Deleting a key that does not exist should succeed (no-op).
    store.delete("ghost").await.unwrap();
}

#[tokio::test]
async fn sqlite_delete_character() {
    let store = test_store().await;
    let e1 = sample_profile_entry("char-1", 100, "area-52", "belarsa");
    let e2 = sample_profile_entry("char-2", 100, "area-52", "belarsa");
    let e3 = sample_profile_entry("char-3", 200, "stormrage", "other");
    store.put(&e1).await.unwrap();
    store.put(&e2).await.unwrap();
    store.put(&e3).await.unwrap();

    store.delete_character("area-52", "belarsa").await.unwrap();

    assert!(store.get("char-1").await.unwrap().is_none());
    assert!(store.get("char-2").await.unwrap().is_none());
    assert!(store.get("char-3").await.unwrap().is_some());
}

#[tokio::test]
async fn sqlite_refresh_character_timestamp() {
    let store = test_store().await;
    let mut entry = sample_profile_entry("ts-key", 100, "area-52", "belarsa");
    // Set fetched_at in the past to guarantee delta.
    entry.fetched_at = Utc::now() - chrono::Duration::seconds(60);
    store.put(&entry).await.unwrap();

    let original = store.get("ts-key").await.unwrap().unwrap();

    // Small sleep so Utc::now() inside refresh differs.
    tokio::time::sleep(Duration::from_millis(10)).await;

    store
        .refresh_character_timestamp("area-52", "belarsa")
        .await
        .unwrap();

    let refreshed = store.get("ts-key").await.unwrap().unwrap();
    assert!(
        refreshed.fetched_at > original.fetched_at,
        "fetched_at should be updated: original={}, refreshed={}",
        original.fetched_at,
        refreshed.fetched_at
    );
}

// ---------------------------------------------------------------------------
// CachedClient — construction tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn cached_client_new_initializes_store() {
    let store = SqliteCacheStore::new("sqlite::memory:").await.unwrap();
    let client = battlenet_rs::client::BattleNetClient::new(
        battlenet_rs::region::BattleNetRegion::US,
        "en_US",
        "fake_id",
        "fake_secret",
    );
    // new() should call store.initialize() internally.
    let _cached = CachedClient::new(client, store).await.unwrap();
}

#[tokio::test]
async fn cached_client_new_with_custom_ttl() {
    let store = SqliteCacheStore::new("sqlite::memory:").await.unwrap();
    let client = battlenet_rs::client::BattleNetClient::new(
        battlenet_rs::region::BattleNetRegion::US,
        "en_US",
        "fake_id",
        "fake_secret",
    );
    let _cached = CachedClient::new_with_ttl(client, store, 7).await.unwrap();
}

// ---------------------------------------------------------------------------
// FR-025: Cache write failure — API response still returned
// ---------------------------------------------------------------------------

/// A CacheStore that always fails on put() — used to verify FR-025.
struct FailingStore;

#[async_trait]
impl CacheStore for FailingStore {
    async fn initialize(&self) -> Result<(), CacheError> {
        Ok(())
    }
    async fn get(&self, _key: &str) -> Result<Option<CacheEntry>, CacheError> {
        Ok(None) // cache miss → forces API fetch
    }
    async fn put(&self, _entry: &CacheEntry) -> Result<(), CacheError> {
        Err(CacheError::DatabaseError("disk full".to_string()))
    }
    async fn delete(&self, _key: &str) -> Result<(), CacheError> {
        Ok(())
    }
    async fn delete_character(&self, _realm: &str, _name: &str) -> Result<(), CacheError> {
        Ok(())
    }
    async fn refresh_character_timestamp(
        &self,
        _realm: &str,
        _name: &str,
    ) -> Result<(), CacheError> {
        Ok(())
    }
}

/// FR-025: When cache put() fails, CachedClient should still return the API
/// response (log::warn! internally). Requires valid API credentials.
#[tokio::test]
#[ignore] // requires BATTLENET_CLIENT_ID / BATTLENET_CLIENT_SECRET
async fn cached_client_cache_write_failure_returns_data() {
    let _ = dotenvy::from_filename(".env");
    let client = battlenet_rs::client::BattleNetClient::new_from_environment();
    let cached = CachedClient::new(client, FailingStore).await.unwrap();

    // Static endpoint fetch should succeed even though put() will fail.
    use battlenet_rs::wow_models::prelude::*;
    let result: Result<MountsIndex, _> = cached.get_data(&UrlArgs::None).await;
    assert!(
        result.is_ok(),
        "API response must be returned despite cache write failure"
    );
}

// ---------------------------------------------------------------------------
// US3: Dynamic and Profile namespace caching behavior
// ---------------------------------------------------------------------------

/// A CacheStore that records put() calls so we can inspect what was cached.
struct RecordingStore {
    inner: SqliteCacheStore,
    put_calls: StdMutex<Vec<CacheEntry>>,
}

impl RecordingStore {
    async fn new() -> Self {
        let inner = SqliteCacheStore::new("sqlite::memory:").await.unwrap();
        inner.initialize().await.unwrap();
        Self {
            inner,
            put_calls: StdMutex::new(Vec::new()),
        }
    }

    fn put_count(&self) -> usize {
        self.put_calls.lock().unwrap().len()
    }

    fn last_put(&self) -> Option<CacheEntry> {
        self.put_calls.lock().unwrap().last().cloned()
    }
}

#[async_trait]
impl CacheStore for RecordingStore {
    async fn initialize(&self) -> Result<(), CacheError> {
        self.inner.initialize().await
    }
    async fn get(&self, key: &str) -> Result<Option<CacheEntry>, CacheError> {
        self.inner.get(key).await
    }
    async fn put(&self, entry: &CacheEntry) -> Result<(), CacheError> {
        self.put_calls.lock().unwrap().push(entry.clone());
        self.inner.put(entry).await
    }
    async fn delete(&self, key: &str) -> Result<(), CacheError> {
        self.inner.delete(key).await
    }
    async fn delete_character(&self, realm: &str, name: &str) -> Result<(), CacheError> {
        self.inner.delete_character(realm, name).await
    }
    async fn refresh_character_timestamp(&self, realm: &str, name: &str) -> Result<(), CacheError> {
        self.inner.refresh_character_timestamp(realm, name).await
    }
}

/// Static endpoint: first call should cache, second should return from cache (no extra put).
#[tokio::test]
#[ignore] // requires API credentials
async fn cached_client_static_cache_hit() {
    let _ = dotenvy::from_filename(".env");
    let client = battlenet_rs::client::BattleNetClient::new_from_environment();
    let store = RecordingStore::new().await;
    let cached = CachedClient::new(client, store).await.unwrap();

    use battlenet_rs::wow_models::prelude::*;

    // First call — cache miss, fetches from API, caches
    let _: MountsIndex = cached.get_data(&UrlArgs::None).await.unwrap();
    assert_eq!(cached.store().put_count(), 1, "should cache on first call");

    // Second call — cache hit, no additional put
    let _: MountsIndex = cached.get_data(&UrlArgs::None).await.unwrap();
    assert_eq!(
        cached.store().put_count(),
        1,
        "should NOT put again on cache hit"
    );
}

/// Dynamic endpoint: always fetches from API and caches (two calls → two puts).
#[tokio::test]
#[ignore] // requires API credentials
async fn cached_client_dynamic_always_fetches() {
    let _ = dotenvy::from_filename(".env");
    let client = battlenet_rs::client::BattleNetClient::new_from_environment();
    let store = RecordingStore::new().await;
    let cached = CachedClient::new(client, store).await.unwrap();

    use battlenet_rs::wow_models::prelude::*;

    // ConnectedRealmsIndex uses dynamic namespace
    let _: ConnectedRealmsIndex = cached.get_data(&UrlArgs::None).await.unwrap();
    assert_eq!(cached.store().put_count(), 1);

    // Second call should still fetch from API and put again
    let _: ConnectedRealmsIndex = cached.get_data(&UrlArgs::None).await.unwrap();
    assert_eq!(
        cached.store().put_count(),
        2,
        "dynamic should always fetch and cache"
    );

    // Verify the cached entry has "dynamic" namespace
    let last = cached.store().last_put().unwrap();
    assert_eq!(last.namespace, "dynamic");
}

/// Profile endpoint: always fetches, caches with profile metadata.
#[tokio::test]
#[ignore] // requires API credentials + user token
async fn cached_client_profile_stores_character_metadata() {
    let _ = dotenvy::from_filename(".env");
    let client = battlenet_rs::client::BattleNetClient::new_from_environment();
    let store = RecordingStore::new().await;
    let cached = CachedClient::new(client, store).await.unwrap();

    use battlenet_rs::wow_models::prelude::*;

    let args = UrlArgs::Player {
        realm_slug: "area-52".to_string(),
        name: "belarsa".to_string(),
    };

    // This requires a valid user token — skeleton test for structure.
    // In CI, this would be marked #[ignore].
    // The key assertion: put() is called with namespace="profile" and
    // character metadata fields populated.
    let token = std::env::var("BATTLENET_USER_TOKEN").unwrap_or_default();
    if token.is_empty() {
        eprintln!("BATTLENET_USER_TOKEN not set, skipping profile metadata test");
        return;
    }

    let _: CharacterProfileStatus = cached.get_data_with_token(&args, &token).await.unwrap();

    let last = cached.store().last_put().unwrap();
    assert_eq!(last.namespace, "profile");
}

/// Unit test: dynamic namespace entry stored in SQLite has correct namespace tag.
#[tokio::test]
async fn sqlite_dynamic_namespace_stored_correctly() {
    let store = test_store().await;
    let entry = sample_entry("dyn-key", "dynamic");
    store.put(&entry).await.unwrap();

    let retrieved = store.get("dyn-key").await.unwrap().unwrap();
    assert_eq!(retrieved.namespace, "dynamic");
}

/// Unit test: profile namespace entry stored in SQLite preserves character metadata.
#[tokio::test]
async fn sqlite_profile_entry_preserves_metadata() {
    let store = test_store().await;
    let entry = sample_profile_entry("prof-key", 12345, "area-52", "belarsa");
    store.put(&entry).await.unwrap();

    let retrieved = store.get("prof-key").await.unwrap().unwrap();
    assert_eq!(retrieved.namespace, "profile");
    assert_eq!(retrieved.character_id, Some(12345));
    assert_eq!(retrieved.realm_slug.as_deref(), Some("area-52"));
    assert_eq!(retrieved.char_name.as_deref(), Some("belarsa"));
}

// ---------------------------------------------------------------------------
// US6: Concurrent read/write test
// ---------------------------------------------------------------------------

/// Spawn concurrent writer + reader tasks to verify no corruption or blocking.
#[tokio::test]
async fn sqlite_concurrent_read_write() {
    use std::sync::Arc;

    let store = Arc::new(test_store().await);

    // Writer: insert 50 entries
    let writer_store = Arc::clone(&store);
    let writer = tokio::spawn(async move {
        for i in 0..50u32 {
            let entry = CacheEntry {
                cache_key: format!("concurrent-{i}"),
                namespace: "static".to_string(),
                response: format!(r#"{{"n":{i}}}"#),
                fetched_at: Utc::now(),
                character_id: None,
                realm_slug: None,
                char_name: None,
            };
            writer_store.put(&entry).await.unwrap();
        }
    });

    // Reader: continuously read (some will hit, some will miss)
    let reader_store = Arc::clone(&store);
    let reader = tokio::spawn(async move {
        let mut hits = 0u32;
        for i in 0..50u32 {
            if reader_store
                .get(&format!("concurrent-{i}"))
                .await
                .unwrap()
                .is_some()
            {
                hits += 1;
            }
        }
        hits
    });

    writer.await.unwrap();
    let hits = reader.await.unwrap();

    // After both complete, all 50 should be readable
    for i in 0..50u32 {
        let entry = store.get(&format!("concurrent-{i}")).await.unwrap();
        assert!(entry.is_some(), "entry concurrent-{i} should exist");
    }

    // Reader may have seen some entries during write — at minimum 0 is fine
    assert!(hits <= 50, "hits should be <= 50");
}

// ---------------------------------------------------------------------------
// US4: 30-Day TTL enforcement tests
// ---------------------------------------------------------------------------

/// Unit test: profile entry within TTL should be returned from cache
/// without any API call. We verify by using a store pre-populated with data,
/// and a client with fake credentials — if the cache-first path works,
/// no HTTP call is needed and no error occurs.
#[tokio::test]
async fn ttl_not_expired_returns_cached_profile() {
    use battlenet_rs::wow_models::prelude::*;

    let store = SqliteCacheStore::new("sqlite::memory:").await.unwrap();
    store.initialize().await.unwrap();

    // Pre-populate with a valid CharacterProfileStatus response, recent fetched_at
    let url = "https://us.api.blizzard.com/profile/wow/character/area-52/belarsa/status?namespace=profile-us&locale=en_US";
    let json = r#"{"id":123456,"is_valid":true}"#;
    let entry = CacheEntry {
        cache_key: url.to_string(),
        namespace: "profile".to_string(),
        response: json.to_string(),
        fetched_at: Utc::now(), // fresh — within TTL
        character_id: Some(123456),
        realm_slug: Some("area-52".to_string()),
        char_name: Some("belarsa".to_string()),
    };
    store.put(&entry).await.unwrap();

    // Create CachedClient with fake credentials — if it tries to make an HTTP
    // call, it will fail. If profile cache-first works, it returns from cache.
    let client = battlenet_rs::client::BattleNetClient::new(
        battlenet_rs::region::BattleNetRegion::US,
        "en_US",
        "fake_id",
        "fake_secret",
    );
    let cached = CachedClient::new_with_ttl(client, store, 30).await.unwrap();

    let args = UrlArgs::Player {
        realm_slug: "area-52".to_string(),
        name: "belarsa".to_string(),
    };

    // This should return from cache without making an API call
    let result: CharacterProfileStatus = cached.get_data(&args).await.unwrap();
    assert_eq!(result.id, 123456);
    assert!(result.is_valid);
}

/// Unit test: profile entry beyond TTL should trigger validation.
/// With fake credentials the validation call will fail, but with
/// transient-failure handling it should return the stale cached data.
#[tokio::test]
async fn ttl_expired_transient_failure_returns_stale_cached() {
    use battlenet_rs::wow_models::prelude::*;

    let store = SqliteCacheStore::new("sqlite::memory:").await.unwrap();
    store.initialize().await.unwrap();

    // Pre-populate with a profile entry that is 31 days old
    let url = "https://us.api.blizzard.com/profile/wow/character/area-52/belarsa/status?namespace=profile-us&locale=en_US";
    let json = r#"{"id":123456,"is_valid":true}"#;
    let entry = CacheEntry {
        cache_key: url.to_string(),
        namespace: "profile".to_string(),
        response: json.to_string(),
        fetched_at: Utc::now() - chrono::Duration::days(31), // expired
        character_id: Some(123456),
        realm_slug: Some("area-52".to_string()),
        char_name: Some("belarsa".to_string()),
    };
    store.put(&entry).await.unwrap();

    // Fake credentials → validation API call will fail (transient failure)
    let client = battlenet_rs::client::BattleNetClient::new(
        battlenet_rs::region::BattleNetRegion::US,
        "en_US",
        "fake_id",
        "fake_secret",
    );
    let cached = CachedClient::new_with_ttl(client, store, 30).await.unwrap();

    let args = UrlArgs::Player {
        realm_slug: "area-52".to_string(),
        name: "belarsa".to_string(),
    };

    // Transient failure: should return stale cached data, NOT error
    let result: CharacterProfileStatus = cached.get_data(&args).await.unwrap();
    assert_eq!(result.id, 123456);
}

/// Integration: expired entry with valid+matching validation refreshes timestamp.
#[tokio::test]
#[ignore] // requires API credentials
async fn ttl_expired_valid_matching_refreshes_timestamp() {
    use battlenet_rs::wow_models::prelude::*;

    let _ = dotenvy::from_filename(".env");
    let client = battlenet_rs::client::BattleNetClient::new_from_environment();
    let store = SqliteCacheStore::new("sqlite::memory:").await.unwrap();
    store.initialize().await.unwrap();

    let args = UrlArgs::Player {
        realm_slug: "area-52".to_string(),
        name: "belarsa".to_string(),
    };

    // First, do a real fetch to get valid data and character_id
    let cached = CachedClient::new_with_ttl(client, store, 30).await.unwrap();
    let status: CharacterProfileStatus = cached.get_data(&args).await.unwrap();
    let char_id = status.id as i64;

    // Manually set fetched_at to 31 days ago to simulate expiry
    let key = "https://us.api.blizzard.com/profile/wow/character/area-52/belarsa/status?namespace=profile-us&locale=en_US";

    // Update fetched_at and character_id directly in the store
    let entry = CacheEntry {
        cache_key: key.to_string(),
        namespace: "profile".to_string(),
        response: serde_json::to_string(&status).unwrap(),
        fetched_at: Utc::now() - chrono::Duration::days(31),
        character_id: Some(char_id),
        realm_slug: Some("area-52".to_string()),
        char_name: Some("belarsa".to_string()),
    };
    cached.store().put(&entry).await.unwrap();

    // Now fetch again — should validate and refresh timestamp
    let result: CharacterProfileStatus = cached.get_data(&args).await.unwrap();
    assert!(result.is_valid);

    // Verify timestamp was refreshed
    let refreshed = cached.store().get(key).await.unwrap().unwrap();
    assert!(
        refreshed.fetched_at > Utc::now() - chrono::Duration::days(1),
        "fetched_at should be refreshed to ~now"
    );
}
