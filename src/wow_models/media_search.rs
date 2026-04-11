use serde::Deserialize;

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::namespace::WowNamespace;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

// --- Media Search ---

#[derive(Debug, Deserialize)]
pub struct MediaSearchData {
    pub id: u32,
}

pub type MediaSearchResult = Result<SearchResult<MediaSearchData>, BattleNetClientError>;
pub type MediaSearchJsonResult = Result<String, BattleNetClientError>;

impl GenerateUrl for SearchResult<MediaSearchData> {
    fn url(client: &BattleNetClient, url_args: &UrlArgs) -> String {
        let params = match url_args {
            UrlArgs::Search { params } => params,
            _ => panic!("UrlArgs::Search expected"),
        };
        let endpoint = "data/wow/search/media";
        let namespace = WowNamespace::Static.to_region_string(&client.region);
        let base = client.region.base_url();
        let locale = &client.locale;
        let mut url = format!("{base}/{endpoint}?namespace={namespace}&locale={locale}");
        for (key, value) in params {
            url.push_str(&format!("&{key}={value}"));
        }
        url
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::json_to_struct;

    #[test]
    fn test_media_search_data() {
        let json = r#"{
            "page": 1,
            "pageSize": 100,
            "maxPageSize": 1000,
            "pageCount": 1,
            "results": [
                {"key": {"href": "https://test"}, "data": {"id": 42}}
            ]
        }"#;
        let result: SearchResult<MediaSearchData> = json_to_struct(json).unwrap();
        assert_eq!(result.results.len(), 1);
        assert_eq!(result.results[0].data.id, 42);
    }
}
