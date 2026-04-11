#![cfg(feature = "wow")]

use battlenet_rs::wow_models::prelude::*;

mod common;

#[tokio::test]
async fn wow_token_test() {
    let client = common::setup_client();

    // Price returned from the API is in copper, so multiply gold by 10K
    let price_min = std::env::var("BATTLENET_TEST_WOW_TOKEN_MIN")
        .expect("BATTLENET_TEST_WOW_TOKEN_MIN must be set in .env")
        .parse::<i64>()
        .expect("BATTLENET_TEST_WOW_TOKEN_MIN must be a i64")
        * 10000;
    let price_max = std::env::var("BATTLENET_TEST_WOW_TOKEN_MAX")
        .expect("BATTLENET_TEST_WOW_TOKEN_MAX must be set in .env")
        .parse::<i64>()
        .expect("BATTLENET_TEST_WOW_TOKEN_MAX must be a i64")
        * 10000;

    let wow_token_index_result: WowTokenIndexResult = client.get_data(&UrlArgs::None).await;
    match wow_token_index_result {
        Ok(wow_token_index) => {
            assert!(wow_token_index.price >= price_min);
            assert!(wow_token_index.price <= price_max);
        }
        Err(e) => {
            panic!("wow_token_index_result error {:?}", e);
        }
    }
    // TODO: Add check that the last_updated_timestamp is within last hour
}
