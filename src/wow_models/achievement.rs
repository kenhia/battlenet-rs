use serde::Deserialize;

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::namespace::WowNamespace;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

#[derive(Debug, Deserialize)]
pub struct AchievementCategoriesIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub categories: Vec<NameAndId>,
    pub root_categories: Vec<NameAndId>,
    pub guild_categories: Vec<NameAndId>,
}

pub type AchievementCategoriesIndexResult =
    Result<AchievementCategoriesIndex, BattleNetClientError>;
pub type AchievementCategoriesIndexJsonResult = Result<String, BattleNetClientError>;

impl GenerateUrl for AchievementCategoriesIndex {
    fn url(client: &BattleNetClient, _: &UrlArgs) -> String {
        let endpoint = "data/wow/achievement-category/index";
        let namespace = WowNamespace::Static.to_region_string(&client.region);
        let base = client.region.base_url();
        let locale = &client.locale;

        format!("{base}/{endpoint}?namespace={namespace}&locale={locale}")
    }
}

#[derive(Debug, Deserialize)]
pub struct AchievementCategory {
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

pub type AchievementCategoryResult = Result<AchievementCategory, BattleNetClientError>;
pub type AchievementCategoryJsonResult = Result<String, BattleNetClientError>;

impl GenerateUrl for AchievementCategory {
    fn url(client: &BattleNetClient, url_args: &UrlArgs) -> String {
        let id = match url_args {
            UrlArgs::Id { id } => id,
            _ => panic!("UrlArgs::Id expected"),
        };

        let endpoint = format!("data/wow/achievement-category/{id}");
        let namespace = WowNamespace::Static.to_region_string(&client.region);
        let base = client.region.base_url();
        let locale = &client.locale;

        format!("{base}/{endpoint}?namespace={namespace}&locale={locale}")
    }
}

#[derive(Debug, Deserialize)]
pub struct AchievementsIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub achievements: Vec<NameAndId>,
}

pub type AchievementsIndexResult = Result<AchievementsIndex, BattleNetClientError>;
pub type AchievementsIndexJsonResult = Result<String, BattleNetClientError>;

impl GenerateUrl for AchievementsIndex {
    fn url(client: &BattleNetClient, _: &UrlArgs) -> String {
        let endpoint = "data/wow/achievement/index";
        let namespace = WowNamespace::Static.to_region_string(&client.region);
        let base = client.region.base_url();
        let locale = &client.locale;

        format!("{base}/{endpoint}?namespace={namespace}&locale={locale}")
    }
}

#[derive(Debug, Deserialize)]
pub struct Achievement {
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

pub type AchievementResult = Result<Achievement, BattleNetClientError>;
pub type AchievementJsonResult = Result<String, BattleNetClientError>;

impl GenerateUrl for Achievement {
    fn url(client: &BattleNetClient, url_args: &UrlArgs) -> String {
        let id = match url_args {
            UrlArgs::Id { id } => id,
            _ => panic!("UrlArgs::Id expected"),
        };

        let endpoint = format!("data/wow/achievement/{id}");
        let namespace = WowNamespace::Static.to_region_string(&client.region);
        let base = client.region.base_url();
        let locale = &client.locale;

        format!("{base}/{endpoint}?namespace={namespace}&locale={locale}")
    }
}

#[derive(Debug, Deserialize)]
pub struct AchievementMedia {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub assets: Vec<Asset>,
    pub id: u32,
}

pub type AchievementMediaResult = Result<AchievementMedia, BattleNetClientError>;
pub type AchievementMediaJsonResult = Result<String, BattleNetClientError>;

impl GenerateUrl for AchievementMedia {
    fn url(client: &BattleNetClient, url_args: &UrlArgs) -> String {
        let id = match url_args {
            UrlArgs::Id { id } => id,
            _ => panic!("UrlArgs::Id expected"),
        };

        let endpoint = format!("data/wow/media/achievement/{id}");
        let namespace = WowNamespace::Static.to_region_string(&client.region);
        let base = client.region.base_url();
        let locale = &client.locale;

        format!("{base}/{endpoint}?namespace={namespace}&locale={locale}")
    }
}
