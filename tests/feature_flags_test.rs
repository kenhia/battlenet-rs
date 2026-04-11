// Feature flag compile-gate tests.
//
// These tests verify that modules are available (or not) under various
// feature combinations. Each test is gated so it only runs when the
// appropriate features are enabled.

// T016: With `wow` feature, Game Data models are available
#[cfg(feature = "wow")]
mod wow_available {
    use battlenet_rs::wow_models::prelude::*;

    #[test]
    fn game_data_models_compile() {
        // If this compiles, achievement types are available under `wow`
        let _: fn() -> AchievementCategoriesIndexResult = || unreachable!();
        let _: fn() -> WowTokenIndexResult = || unreachable!();
        let _: fn() -> ConnectedRealmsIndexResult = || unreachable!();
        let _: fn() -> AuctionsIndexResult = || unreachable!();
    }
}

// T017: With `wow` feature, UrlArgs variants are available
#[cfg(feature = "wow")]
mod urlargs_available {
    use battlenet_rs::wow_models::UrlArgs;

    #[test]
    fn all_urlargs_variants_exist() {
        let _ = UrlArgs::None;
        let _ = UrlArgs::Id { id: 1 };
        let _ = UrlArgs::Player {
            realm_slug: "test".into(),
            name: "test".into(),
        };
        let _ = UrlArgs::Guild {
            realm_slug: "test".into(),
            name_slug: "test".into(),
        };
        let _ = UrlArgs::TwoIds { id1: 1, id2: 2 };
        let _ = UrlArgs::ThreeIds {
            id1: 1,
            id2: 2,
            id3: 3,
        };
        let _ = UrlArgs::PlayerExtra {
            realm_slug: "test".into(),
            name: "test".into(),
            extra: "test".into(),
        };
        let _ = UrlArgs::TwoStrings {
            first: "test".into(),
            second: "test".into(),
        };
        let _ = UrlArgs::Search { params: vec![] };
    }
}

// T018: With `wow` + `user`, Profile models are available
#[cfg(all(feature = "wow", feature = "user"))]
mod profile_available {
    use battlenet_rs::wow_models::prelude::*;

    #[test]
    fn profile_models_compile() {
        // CharacterProfile and CharacterProfileStatus should be available
        // when both wow and user features are enabled
        let _: fn() -> CharacterProfileStatusResult = || unreachable!();
    }
}

// T019: Stub features compile with no effect
#[test]
fn stub_features_compile() {
    // This test always runs. The fact that compilation succeeds with
    // any combination of stub features proves they have no effect.
    // When run with --all-features, wow-classic/diablo/hearthstone/starcraft
    // are all enabled and compilation still succeeds.
}

// T020: Redis feature works alongside new features
#[cfg(feature = "redis")]
mod redis_works {
    use battlenet_rs::user_token;

    #[test]
    fn redis_module_available() {
        // If this compiles, the redis feature still works
        let _ = std::any::type_name::<user_token::UserAccessToken>();
    }
}
