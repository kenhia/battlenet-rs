use crate::client::BattleNetClient;
use crate::namespace::WowNamespace;

pub mod achievement;
pub mod auction_house;
pub mod azerite_essence;
pub mod connected_realm;
pub mod core_other;
pub mod core_structs;
pub mod covenant;
pub mod creature;
pub mod guild;
pub mod heirloom;
pub mod item;
pub mod journal;
pub mod media_search;
pub mod modified_crafting;
pub mod mount;
pub mod mythic_keystone;
pub mod pet;
pub mod playable_class;
pub mod playable_race;
pub mod playable_spec;
pub mod power_type;
pub mod profession;
pub mod pvp;
pub mod quest;
pub mod realm;
pub mod region_api;
pub mod reputation;
pub mod spell;
pub mod talent;
pub mod title;
pub mod toy;
pub mod wow_token;

// Profile API modules (require user feature)
#[cfg(feature = "user")]
pub mod account_profile;
#[cfg(feature = "user")]
pub mod character_achievements;
#[cfg(feature = "user")]
pub mod character_appearance;
#[cfg(feature = "user")]
pub mod character_collections;
#[cfg(feature = "user")]
pub mod character_encounters;
#[cfg(feature = "user")]
pub mod character_equipment;
#[cfg(feature = "user")]
pub mod character_hunter_pets;
#[cfg(feature = "user")]
pub mod character_media;
#[cfg(feature = "user")]
pub mod character_mythic_keystone;
#[cfg(feature = "user")]
pub mod character_professions;
#[cfg(feature = "user")]
pub mod character_profile;
#[cfg(feature = "user")]
pub mod character_pvp;
#[cfg(feature = "user")]
pub mod character_quests;
#[cfg(feature = "user")]
pub mod character_reputations;
#[cfg(feature = "user")]
pub mod character_soulbinds;
#[cfg(feature = "user")]
pub mod character_specializations;
#[cfg(feature = "user")]
pub mod character_statistics;
#[cfg(feature = "user")]
pub mod character_titles;

#[cfg(feature = "user")]
pub mod full_character;

/// ## some desc
pub mod prelude {
    pub use super::achievement::*;
    pub use super::auction_house::*;
    pub use super::azerite_essence::*;
    pub use super::connected_realm::*;
    pub use super::core_other::*;
    pub use super::core_structs::*;
    pub use super::covenant::*;
    pub use super::creature::*;
    pub use super::guild::*;
    pub use super::heirloom::*;
    pub use super::item::*;
    pub use super::journal::*;
    pub use super::media_search::*;
    pub use super::modified_crafting::*;
    pub use super::mount::*;
    pub use super::mythic_keystone::*;
    pub use super::pet::*;
    pub use super::playable_class::*;
    pub use super::playable_race::*;
    pub use super::playable_spec::*;
    pub use super::power_type::*;
    pub use super::profession::*;
    pub use super::pvp::*;
    pub use super::quest::*;
    pub use super::realm::*;
    pub use super::region_api::*;
    pub use super::reputation::*;
    pub use super::spell::*;
    pub use super::talent::*;
    pub use super::title::*;
    pub use super::toy::*;
    pub use super::wow_token::*;
    pub use crate::wow_models::UrlArgs;

    #[cfg(feature = "user")]
    pub use super::account_profile::*;
    #[cfg(feature = "user")]
    pub use super::character_achievements::*;
    #[cfg(feature = "user")]
    pub use super::character_appearance::*;
    #[cfg(feature = "user")]
    pub use super::character_collections::*;
    #[cfg(feature = "user")]
    pub use super::character_encounters::*;
    #[cfg(feature = "user")]
    pub use super::character_equipment::*;
    #[cfg(feature = "user")]
    pub use super::character_hunter_pets::*;
    #[cfg(feature = "user")]
    pub use super::character_media::*;
    #[cfg(feature = "user")]
    pub use super::character_mythic_keystone::*;
    #[cfg(feature = "user")]
    pub use super::character_professions::*;
    #[cfg(feature = "user")]
    pub use super::character_profile::*;
    #[cfg(feature = "user")]
    pub use super::character_pvp::*;
    #[cfg(feature = "user")]
    pub use super::character_quests::*;
    #[cfg(feature = "user")]
    pub use super::character_reputations::*;
    #[cfg(feature = "user")]
    pub use super::character_soulbinds::*;
    #[cfg(feature = "user")]
    pub use super::character_specializations::*;
    #[cfg(feature = "user")]
    pub use super::character_statistics::*;
    #[cfg(feature = "user")]
    pub use super::character_titles::*;

    #[cfg(feature = "user")]
    pub use super::full_character::*;
}

pub enum UrlArgs {
    None,
    Player {
        realm_slug: String,
        name: String,
    },
    Id {
        id: u64,
    },
    Guild {
        realm_slug: String,
        name_slug: String,
    },
    TwoIds {
        id1: u64,
        id2: u64,
    },
    ThreeIds {
        id1: u64,
        id2: u64,
        id3: u64,
    },
    PlayerExtra {
        realm_slug: String,
        name: String,
        extra: String,
    },
    TwoStrings {
        first: String,
        second: String,
    },
    Search {
        params: Vec<(String, String)>,
    },
}

pub trait GenerateUrl {
    fn url(client: &BattleNetClient, url_args: &UrlArgs) -> String;

    /// Returns the namespace for cache behavior. Default: Static.
    fn cache_namespace() -> WowNamespace {
        WowNamespace::Static
    }
}
