pub mod auth;
pub mod client;
pub mod errors;
pub mod namespace;
pub mod region;

#[cfg(feature = "wow")]
pub mod wow_models;

#[cfg(feature = "redis")]
pub mod user_token;

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
