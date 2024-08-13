use battlenet_rs::client::BattleNetClient;

/// Get client for tests using `CLIENT_ID` and `CLIENT_SECRET`
/// from .env in the root of the project
pub fn setup_client() -> BattleNetClient {
    let _ = dotenvy::from_filename(".env");
    BattleNetClient::new_from_environment()
}
