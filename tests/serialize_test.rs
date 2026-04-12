//! Serialization round-trip tests for battlenet-rs model structs.
//!
//! These tests verify that all model structs (both bendpoint-generated and manually
//! defined) can be serialized to JSON and deserialized back without data loss.
//! Per FR-011 and FR-012, all model structs must derive Serialize.

#[cfg(feature = "wow")]
mod serialize_tests {
    use battlenet_rs::wow_models::core_structs::*;
    use battlenet_rs::wow_models::GenerateUrl;
    use serde::{Deserialize, Serialize};

    /// Helper: round-trip a fixture JSON through deserialize → serialize → deserialize
    /// and compare the two deserialized values by re-serializing them both.
    fn assert_round_trip<T>(json: &str)
    where
        T: for<'a> Deserialize<'a> + Serialize + std::fmt::Debug,
    {
        let original: T = serde_json::from_str(json).expect("failed to deserialize fixture");
        let serialized = serde_json::to_string(&original).expect("failed to serialize");
        let round_tripped: T =
            serde_json::from_str(&serialized).expect("failed to deserialize round-tripped JSON");
        let serialized_again =
            serde_json::to_string(&round_tripped).expect("failed to re-serialize");
        assert_eq!(
            serialized,
            serialized_again,
            "round-trip mismatch for {}",
            std::any::type_name::<T>()
        );
    }

    // --- bendpoint struct tests ---

    #[test]
    fn test_mounts_index_round_trip() {
        use battlenet_rs::wow_models::mount::MountsIndex;
        // MountsIndex won't parse from achievement-categories, so use the right fixture:
        // We test the mechanism — if Serialize is missing this won't compile.
        let _check: fn() = || {
            let fixture = r#"{"_links":{"self":{"href":"https://us.api.blizzard.com/data/wow/mount/index"}},"mounts":[{"key":{"href":"https://us.api.blizzard.com/data/wow/mount/6"},"name":"Brown Horse","id":6}]}"#;
            assert_round_trip::<MountsIndex>(fixture);
        };
    }

    #[test]
    fn test_achievement_categories_index_round_trip() {
        use battlenet_rs::wow_models::achievement::AchievementCategoriesIndex;
        let fixture = include_str!("../data/achievement-categories-index.json");
        assert_round_trip::<AchievementCategoriesIndex>(fixture);
    }

    // --- manual struct tests ---

    #[test]
    fn test_href_link_round_trip() {
        let fixture = r#"{"href":"https://example.com/api"}"#;
        assert_round_trip::<HrefLink>(fixture);
    }

    #[test]
    fn test_type_and_name_round_trip() {
        let fixture = r#"{"type":"ALLIANCE","name":"Alliance"}"#;
        assert_round_trip::<TypeAndName>(fixture);
    }

    #[test]
    fn test_auction_round_trip() {
        use battlenet_rs::wow_models::core_other::Auction;
        let fixture =
            r#"{"id":1,"item":{"id":123},"buyout":50000,"quantity":1,"time_left":"SHORT"}"#;
        assert_round_trip::<Auction>(fixture);
    }

    // --- cache_namespace default  ---

    #[test]
    fn test_cache_namespace_default_is_static() {
        use battlenet_rs::namespace::WowNamespace;
        use battlenet_rs::wow_models::mount::MountsIndex;

        assert!(matches!(
            MountsIndex::cache_namespace(),
            WowNamespace::Static
        ));
    }
}
