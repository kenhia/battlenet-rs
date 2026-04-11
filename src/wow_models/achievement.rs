use crate::namespace::WowNamespace;
use serde::Deserialize;

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

#[bendpoint(endpoint = "data/wow/achievement-category/index" namespace = "static")]
struct AchievementCategoriesIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub categories: Vec<NameAndId>,
    pub root_categories: Vec<NameAndId>,
    pub guild_categories: Vec<NameAndId>,
}

#[bendpoint(endpoint = "data/wow/achievement-category/{id}" url_args = "Id" namespace = "static")]
struct AchievementCategory {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: String,
    pub achievements: Vec<NameAndId>,
    pub subcategories: Option<Vec<NameAndId>>,
    pub parent_category: Option<NameAndId>,
    pub is_guild_category: bool,
    pub aggregates_by_faction: AggregatesByFaction,
    pub display_order: u32,
}

#[bendpoint(endpoint = "data/wow/achievement/index" namespace = "static")]
struct AchievementsIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub achievements: Vec<NameAndId>,
}

#[bendpoint(endpoint = "data/wow/achievement/{id}" url_args = "Id" namespace = "static")]
struct Achievement {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub category: NameAndId,
    pub name: String,
    pub description: String,
    pub points: u32,
    pub is_account_wide: bool,
    pub criteria: AchievementCriteria,
    pub prerequisite_achievement: Option<NameAndId>,
    pub next_achievement: Option<NameAndId>,
    pub reward_description: Option<String>,
    pub media: KeyAndId,
    pub display_order: u32,
}

#[bendpoint(endpoint = "data/wow/media/achievement/{id}" url_args = "Id" namespace = "static")]
struct AchievementMedia {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub assets: Vec<Asset>,
    pub id: u32,
}
