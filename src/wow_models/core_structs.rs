use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HrefLink {
    pub href: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinksRef {
    #[serde(alias = "self")]
    pub self_: HrefLink,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeAndName {
    #[serde(alias = "type")]
    pub type_: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeAndValue {
    #[serde(alias = "type")]
    pub type_: u64,
    pub value: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NameAndId {
    #[serde(default)]
    pub key: Option<HrefLink>,
    pub name: String,
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyAndId {
    pub key: HrefLink,
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Realm {
    pub name: String,
    pub id: u64,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealmLong {
    pub id: u64,
    pub region: NameAndId,
    pub connected_realm: HrefLink,
    pub name: String,
    pub category: String,
    pub locale: String,
    pub timezone: String,
    #[serde(alias = "type")]
    pub type_: TypeAndName,
    pub is_tournament: bool,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterGuild {
    pub name: String,
    pub id: u64,
    pub realm: Realm,
    pub faction: TypeAndName,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterTitle {
    pub name: String,
    pub id: u64,
    pub display_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterCovenantProgress {
    pub chosen_covenant: TypeAndName,
    pub renown_level: u64,
    pub soulbinds: HrefLink,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    pub key: String,
    pub value: String,
    #[serde(default)]
    pub file_data_id: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Aggregates {
    pub quantity: i32,
    pub points: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AggregatesByFaction {
    pub alliance: Aggregates,
    pub horde: Aggregates,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementCriteria {
    pub id: u32,
    pub description: String,
    pub amount: u32,
    pub operator: Option<AchievementCriteriaOperator>,
    pub child_criteria: Option<Vec<AchievementCriteria>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementCriteriaOperator {
    #[serde(alias = "type")]
    pub type_: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyHref {
    pub key: HrefLink,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Media {
    pub key: HrefLink,
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult<T> {
    pub page: u32,
    #[serde(alias = "pageSize")]
    pub page_size: u32,
    #[serde(alias = "maxPageSize")]
    pub max_page_size: u32,
    #[serde(alias = "pageCount")]
    pub page_count: u32,
    pub results: Vec<SearchResultEntry<T>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResultEntry<T> {
    pub key: HrefLink,
    pub data: T,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_result_empty_results() {
        let json = r#"{
            "page": 1,
            "pageSize": 100,
            "maxPageSize": 1000,
            "pageCount": 0,
            "results": []
        }"#;
        let result: SearchResult<serde_json::Value> = serde_json::from_str(json).unwrap();
        assert_eq!(result.page, 1);
        assert_eq!(result.page_size, 100);
        assert_eq!(result.max_page_size, 1000);
        assert_eq!(result.page_count, 0);
        assert!(result.results.is_empty());
    }
}
