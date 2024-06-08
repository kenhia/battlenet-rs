pub mod auth;
pub mod client;
pub mod errors;
pub mod namespace;
pub mod region;
pub mod wow_models;

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
