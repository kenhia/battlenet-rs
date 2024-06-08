/// Blizzard API data is region-specific. This enum represents the different
/// regions that the API supports.
#[derive(Debug)]
pub enum BattleNetRegion {
    US,
    EU,
    KR,
    TW,
    CN,
}

impl BattleNetRegion {
    pub fn new_region_from_str(region: &str) -> Self {
        match region.to_lowercase().as_str() {
            "us" => Self::US,
            "eu" => Self::EU,
            "kr" => Self::KR,
            "tw" => Self::TW,
            "cn" => Self::CN,
            _ => panic!("Invalid region"),
        }
    }
    /// Returns the base URL for the region.
    pub fn base_url(&self) -> &str {
        match self {
            Self::US => "https://us.api.blizzard.com",
            Self::EU => "https://eu.api.blizzard.com",
            Self::KR => "https://kr.api.blizzard.com",
            Self::TW => "https://tw.api.blizzard.com",
            Self::CN => "https://gateway.battlenet.com.cn",
        }
    }

    /// Returns the OAuth client ID endpoint for the region.
    pub fn client_token_endpoint(&self) -> &str {
        match self {
            Self::CN => "https://www.battlenet.com.cn/oauth/token",
            _ => "https://oauth.battle.net/token",
        }
    }

    /// Returns the OAuth authorize endpoint for the region.
    pub fn authorize_endpoint(&self) -> &str {
        match self {
            Self::CN => "https://www.battlenet.com.cn/oauth/authorize",
            _ => "https://oauth.battle.net/authorize",
        }
    }

    /// Returns the region as a `&str`.
    pub fn to_str(&self) -> &str {
        match self {
            Self::US => "us",
            Self::EU => "eu",
            Self::KR => "kr",
            Self::TW => "tw",
            Self::CN => "cn",
        }
    }

    /// Returns a vector of the available locales for the region.
    pub fn available_locales(&self) -> Vec<&str> {
        match self {
            Self::US => vec!["en_US", "es_MX", "pt_BR"],
            Self::EU => vec![
                "en_GB", "es_ES", "fr_FR", "ru_RU", "de_DE", "pt_PT", "it_IT",
            ],
            Self::KR => vec!["ko_KR"],
            Self::TW => vec!["zh_TW"],
            Self::CN => vec!["zh_CN"],
        }
    }

    /// Returns the default locale for the region.
    pub fn default_locale(&self) -> &str {
        self.available_locales()[0]
    }

    /// Checks if the locale is available for the region.
    pub fn check_locale(&self, locale: &str) -> bool {
        self.available_locales().contains(&locale)
    }
}
