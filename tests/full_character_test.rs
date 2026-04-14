#![cfg(feature = "wow")]

mod common;

use battlenet_rs::wow_models::full_character::*;

// =============================================================================
// Phase 2: Foundational Tests (T004, T005)
// =============================================================================

#[test]
fn test_endpoint_error_construction() {
    let err = EndpointError {
        endpoint: "hunter_pets".to_string(),
        message: "Not Found".to_string(),
    };
    assert_eq!(err.endpoint, "hunter_pets");
    assert_eq!(err.message, "Not Found");
}

#[test]
fn test_endpoint_error_serialize_roundtrip() {
    let err = EndpointError {
        endpoint: "pvp_2v2".to_string(),
        message: "404: Not Found".to_string(),
    };
    let json = serde_json::to_string(&err).unwrap();
    let deserialized: EndpointError = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.endpoint, err.endpoint);
    assert_eq!(deserialized.message, err.message);
}

#[test]
fn test_full_character_construction_all_none() {
    let fc = FullCharacter {
        realm_slug: "trollbane".to_string(),
        character_name: "belarsa".to_string(),
        has_profile_data: false,
        fetched_at: chrono::Utc::now(),
        errors: vec![],
        profile: None,
        achievements: None,
        achievement_statistics: None,
        appearance: None,
        collections: None,
        mounts_collection: None,
        pets_collection: None,
        heirlooms_collection: None,
        toys_collection: None,
        encounters: None,
        dungeons: None,
        raids: None,
        equipment: None,
        hunter_pets: None,
        media: None,
        mythic_keystone_profile: None,
        mythic_keystone_season: None,
        professions: None,
        pvp_summary: None,
        pvp_2v2: None,
        pvp_3v3: None,
        pvp_rbg: None,
        quests: None,
        completed_quests: None,
        reputations: None,
        soulbinds: None,
        specializations: None,
        statistics: None,
        titles: None,
    };
    assert_eq!(fc.realm_slug, "trollbane");
    assert_eq!(fc.character_name, "belarsa");
    assert!(!fc.has_profile_data);
    assert!(fc.errors.is_empty());
    assert!(fc.profile.is_none());
    assert!(fc.mythic_keystone_season.is_none());
}

// =============================================================================
// Phase 3: US1 Tests (T010–T014) — live API tests
// =============================================================================

#[tokio::test]
async fn test_full_character_populates_common_endpoints() {
    let client = common::setup_client();
    let result = full_character(&client, "trollbane", "belarsa", None).await;
    assert!(
        result.is_ok(),
        "full_character() should succeed: {:?}",
        result.err()
    );
    let fc = result.unwrap();

    assert_eq!(fc.realm_slug, "trollbane");
    assert_eq!(fc.character_name, "belarsa");
    assert!(!fc.has_profile_data);
    assert!(fc.profile.is_some(), "profile should be populated");

    // Count populated endpoint fields (excluding metadata)
    let populated = count_populated_fields(&fc);
    assert!(
        populated >= 20,
        "Expected at least 20 populated fields, got {populated}. Errors: {:?}",
        fc.errors
    );
}

#[tokio::test]
async fn test_full_character_nonexistent_returns_err() {
    let client = common::setup_client();
    let result = full_character(&client, "trollbane", "zzzznotacharzzzzz", None).await;
    assert!(result.is_err(), "non-existent character should return Err");
}

#[tokio::test]
async fn test_full_character_graceful_degradation() {
    let client = common::setup_client();
    let fc = full_character(&client, "trollbane", "belarsa", None)
        .await
        .expect("should succeed");

    // At least some endpoints may be None (class-specific) — check errors are captured
    // Even if all endpoints succeed, the errors vec should be present (empty)
    // If hunter_pets is None for a non-hunter, there should be a corresponding error
    if fc.hunter_pets.is_none() {
        let has_hunter_pets_error = fc.errors.iter().any(|e| e.endpoint == "hunter_pets");
        assert!(
            has_hunter_pets_error,
            "hunter_pets is None but no corresponding error found"
        );
    }
}

#[tokio::test]
async fn test_full_character_has_profile_data_flag() {
    let client = common::setup_client();

    // Without token
    let fc = full_character(&client, "trollbane", "belarsa", None)
        .await
        .expect("should succeed");
    assert!(
        !fc.has_profile_data,
        "has_profile_data should be false without token"
    );
}

#[tokio::test]
async fn test_full_character_mythic_keystone_season() {
    let client = common::setup_client();
    let fc = full_character(&client, "trollbane", "belarsa", None)
        .await
        .expect("should succeed");

    // M+ season should be auto-detected and populated (or None with error if character has no M+ data)
    if let Some(season) = &fc.mythic_keystone_season {
        // Season was fetched — verify it has a season ID
        assert!(season.season.id > 0, "season ID should be positive");
    }
    // If None, there should be an error entry (or the character has no M+ season data)
}

/// Helper to count populated Option<T> endpoint fields.
fn count_populated_fields(fc: &FullCharacter) -> usize {
    let mut count = 0;
    if fc.profile.is_some() {
        count += 1;
    }
    if fc.achievements.is_some() {
        count += 1;
    }
    if fc.achievement_statistics.is_some() {
        count += 1;
    }
    if fc.appearance.is_some() {
        count += 1;
    }
    if fc.collections.is_some() {
        count += 1;
    }
    if fc.mounts_collection.is_some() {
        count += 1;
    }
    if fc.pets_collection.is_some() {
        count += 1;
    }
    if fc.heirlooms_collection.is_some() {
        count += 1;
    }
    if fc.toys_collection.is_some() {
        count += 1;
    }
    if fc.encounters.is_some() {
        count += 1;
    }
    if fc.dungeons.is_some() {
        count += 1;
    }
    if fc.raids.is_some() {
        count += 1;
    }
    if fc.equipment.is_some() {
        count += 1;
    }
    if fc.hunter_pets.is_some() {
        count += 1;
    }
    if fc.media.is_some() {
        count += 1;
    }
    if fc.mythic_keystone_profile.is_some() {
        count += 1;
    }
    if fc.mythic_keystone_season.is_some() {
        count += 1;
    }
    if fc.professions.is_some() {
        count += 1;
    }
    if fc.pvp_summary.is_some() {
        count += 1;
    }
    if fc.pvp_2v2.is_some() {
        count += 1;
    }
    if fc.pvp_3v3.is_some() {
        count += 1;
    }
    if fc.pvp_rbg.is_some() {
        count += 1;
    }
    if fc.quests.is_some() {
        count += 1;
    }
    if fc.completed_quests.is_some() {
        count += 1;
    }
    if fc.reputations.is_some() {
        count += 1;
    }
    if fc.soulbinds.is_some() {
        count += 1;
    }
    if fc.specializations.is_some() {
        count += 1;
    }
    if fc.statistics.is_some() {
        count += 1;
    }
    if fc.titles.is_some() {
        count += 1;
    }
    count
}

// =============================================================================
// Phase 4: US2 Tests (T019-T020) — JSON serialization
// =============================================================================

#[tokio::test]
async fn test_full_character_json_returns_valid_json() {
    let client = common::setup_client();
    let result = full_character_json(&client, "trollbane", "belarsa", None).await;
    assert!(
        result.is_ok(),
        "full_character_json() should succeed: {:?}",
        result.err()
    );
    let json_str = result.unwrap();
    assert!(!json_str.is_empty(), "JSON string should not be empty");
    // Verify it's valid JSON
    let parsed: serde_json::Value =
        serde_json::from_str(&json_str).expect("Output should be valid JSON");
    assert!(parsed.is_object(), "Top-level should be an object");
    assert!(
        parsed.get("realm_slug").is_some(),
        "Should contain realm_slug"
    );
    assert!(parsed.get("profile").is_some(), "Should contain profile");
}

#[tokio::test]
async fn test_full_character_json_roundtrip() {
    let client = common::setup_client();
    let json_str = full_character_json(&client, "trollbane", "belarsa", None)
        .await
        .expect("should succeed");
    // Deserialize back to FullCharacter
    let deserialized: FullCharacter =
        serde_json::from_str(&json_str).expect("JSON should round-trip to FullCharacter");
    assert_eq!(deserialized.realm_slug, "trollbane");
    assert_eq!(deserialized.character_name, "belarsa");
    assert!(
        deserialized.profile.is_some(),
        "profile should survive round-trip"
    );
    let populated = count_populated_fields(&deserialized);
    assert!(
        populated >= 20,
        "Round-trip should preserve at least 20 populated fields, got {populated}"
    );
}

// =============================================================================
// Phase 5: US3 Tests (T023-T024) — Cache integration
// =============================================================================

#[cfg(feature = "db-sqlite")]
mod cache_tests {
    use super::*;
    use battlenet_rs::cache::cached_client::CachedClient;
    use battlenet_rs::cache::sqlite::SqliteCacheStore;

    async fn setup_cached_client() -> CachedClient<SqliteCacheStore> {
        let client = common::setup_client();
        let store = SqliteCacheStore::new(":memory:").await.unwrap();
        CachedClient::new(client, store).await.unwrap()
    }

    #[tokio::test]
    async fn test_cached_client_implements_character_fetcher() {
        // T023: CachedClient must impl CharacterFetcher — compile-time check + basic call
        let cached = setup_cached_client().await;
        let result = full_character(&cached, "trollbane", "belarsa", None).await;
        assert!(
            result.is_ok(),
            "CachedClient should work with full_character: {:?}",
            result.err()
        );
        let fc = result.unwrap();
        assert_eq!(fc.realm_slug, "trollbane");
        assert!(fc.profile.is_some());
    }

    #[tokio::test]
    async fn test_cached_full_character_second_call_faster() {
        // T024: Second call should be ≥10x faster due to cache
        let cached = setup_cached_client().await;

        let start1 = std::time::Instant::now();
        let fc1 = full_character(&cached, "trollbane", "belarsa", None)
            .await
            .expect("first call should succeed");
        let dur1 = start1.elapsed();

        let start2 = std::time::Instant::now();
        let fc2 = full_character(&cached, "trollbane", "belarsa", None)
            .await
            .expect("second call should succeed");
        let dur2 = start2.elapsed();

        // Second call should be significantly faster (cache hit)
        assert!(
            dur2 < dur1 / 10,
            "Second call ({dur2:?}) should be at least 10x faster than first ({dur1:?})"
        );
        // Both should have the same data
        assert_eq!(count_populated_fields(&fc1), count_populated_fields(&fc2));
    }
}
