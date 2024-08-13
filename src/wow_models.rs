use crate::client::BattleNetClient;

pub mod achievement;
pub mod character_profile;
pub mod connected_realm;
pub mod core_other;
pub mod core_structs;
pub mod wow_token;

/// ## some desc
pub mod prelude {
    pub use super::achievement::*;
    pub use super::character_profile::*;
    pub use super::connected_realm::*;
    pub use super::core_other::*;
    pub use super::core_structs::*;
    pub use super::wow_token::*;
    pub use crate::wow_models::UrlArgs;
}

pub enum UrlArgs {
    None,
    Player { realm_slug: String, name: String },
    Id { id: u64 },
}

pub trait GenerateUrl {
    fn url(client: &BattleNetClient, url_args: &UrlArgs) -> String;
}
