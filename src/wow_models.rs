use crate::client::BattleNetClient;

pub mod character_profile;
pub mod wow_token;

pub mod prelude {
    pub use super::character_profile::*;
    pub use super::wow_token::*;
}

pub enum UrlArgs {
    None,
    Player {realm_slug: String, name: String},
}

pub trait GenerateUrl {
    fn url(client: &BattleNetClient, url_args: &UrlArgs) -> String;
}
