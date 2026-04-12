use std::time::Instant;

use battlenet_rs::cache::cached_client::CachedClient;
use battlenet_rs::cache::sqlite::SqliteCacheStore;
use battlenet_rs::client::BattleNetClient;
use battlenet_rs::user_token::read_user_token;
use battlenet_rs::wow_models::prelude::*;

#[tokio::main]
async fn main() {
    let _ = dotenvy::from_filename(".env");

    // Read user token from Redis (bnauth OAuth helper)
    let user_token = read_user_token().expect("Failed to read user token from Redis");
    println!("Token obtained at: {}", user_token.obtained_at);
    println!("Token expires at:  {}", user_token.expires_at);
    println!();

    // Set up SQLite-backed cache
    let client = BattleNetClient::new_from_environment();
    let store = SqliteCacheStore::new("sqlite:battlenet_cache.db")
        .await
        .expect("Failed to create SQLite cache store");
    let cached = CachedClient::new(client, store)
        .await
        .expect("Failed to initialize CachedClient");

    // First fetch — will hit the Blizzard API and cache the result
    let t1 = Instant::now();
    let profile: AccountProfileSummary = cached
        .get_data_with_token_force(&UrlArgs::None, &user_token.access_token)
        .await
        .expect("Failed to fetch account profile");
    let elapsed1 = t1.elapsed();

    let char_count: usize = profile
        .wow_accounts
        .as_ref()
        .map(|accts| accts.iter().map(|a| a.characters.len()).sum())
        .unwrap_or(0);
    println!(
        "1st fetch (API):   {} characters across {} account(s) in {:.1?}",
        char_count,
        profile.wow_accounts.as_ref().map(|a| a.len()).unwrap_or(0),
        elapsed1,
    );

    // Second fetch — should return from SQLite cache (no API call)
    let t2 = Instant::now();
    let profile2: AccountProfileSummary = cached
        .get_data_with_token(&UrlArgs::None, &user_token.access_token)
        .await
        .expect("Failed to fetch account profile (cached)");
    let elapsed2 = t2.elapsed();

    let char_count2: usize = profile2
        .wow_accounts
        .as_ref()
        .map(|accts| accts.iter().map(|a| a.characters.len()).sum())
        .unwrap_or(0);
    println!(
        "2nd fetch (cache): {} characters across {} account(s) in {:.1?}",
        char_count2,
        profile2.wow_accounts.as_ref().map(|a| a.len()).unwrap_or(0),
        elapsed2,
    );

    println!(
        "\nSpeedup: {:.0}x faster from cache",
        elapsed1.as_secs_f64() / elapsed2.as_secs_f64()
    );
}
