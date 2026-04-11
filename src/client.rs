#[cfg(feature = "wow")]
use crate::wow_models::{GenerateUrl, UrlArgs};
use crate::{
    auth::AccessTokenResponse,
    errors::{BattleNetClientError, BattlenetClientResult},
    region::BattleNetRegion,
};
use serde::Deserialize;
use std::env;
use std::ops::Add;
use std::sync::Mutex;
use std::time::Duration;
use time::OffsetDateTime;

/// The BattleNetClient
#[derive(Debug)]
pub struct BattleNetClient {
    /// The reqwest client.
    pub http: reqwest::Client,
    pub region: BattleNetRegion,
    pub locale: String,
    client_id: String,
    client_secret: String,
    access_token: Mutex<Option<String>>,
    expires_at: Mutex<time::OffsetDateTime>,
}

const DEFAULT_API_TIMEOUT_SECONDS: u64 = 5;

impl BattleNetClient {
    /// Construct a new client.
    pub fn new(
        region: BattleNetRegion,
        locale: &str,
        client_id: &str,
        client_secret: &str,
    ) -> Self {
        let timeout = Duration::from_secs(DEFAULT_API_TIMEOUT_SECONDS);
        Self::new_with_timeout(region, locale, client_id, client_secret, timeout)
    }

    /// Construct a new client specifying the API timeout.
    pub fn new_with_timeout(
        region: BattleNetRegion,
        locale: &str,
        client_id: &str,
        client_secret: &str,
        timeout: std::time::Duration,
    ) -> Self {
        Self {
            http: reqwest::Client::builder().timeout(timeout).build().unwrap(),
            region,
            locale: locale.to_string(),
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            access_token: Mutex::new(None),
            expires_at: Mutex::new(time::OffsetDateTime::now_utc()),
        }
    }

    /// Construct a new client from environment variables.
    /// # Example
    /// Uses crate `dotenvy` to load environment variables from ".env" file.
    /// ```rust
    /// use battlenet_rs::client::BattleNetClient;
    ///
    /// let _ = dotenvy::from_filename(".env");
    /// let client = BattleNetClient::new_from_environment();
    /// println!("{client:?}");
    /// ```
    pub fn new_from_environment() -> Self {
        // `client_id` and `client_secret` are required.
        let client_id =
            std::env::var("BATTLENET_CLIENT_ID").expect("BATTLENET_CLIENT_ID must be set");
        let client_secret =
            std::env::var("BATTLENET_CLIENT_SECRET").expect("BATTLENET_CLIENT_SECRET must be set");

        // `region` and `locale` are optional and will default to US, en_US.
        // TODO: Add tracing if we override the region or locale (code will become
        // TODO: messier, but ultimately worth it)
        let region = std::env::var("BATTLENET_REGION").unwrap_or_else(|_| "US".to_string());
        let mut locale = std::env::var("BATTLENET_LOCALE").unwrap_or_else(|_| "en_US".to_string());

        let region = BattleNetRegion::new_region_from_str(&region);
        if !region.check_locale(locale.as_str()) {
            // TODO: Add tracing that we overrode the locale.
            locale = region.default_locale().to_string();
        }

        let mut timeout = Duration::from_secs(DEFAULT_API_TIMEOUT_SECONDS);
        if let Ok(env_timeout) = env::var("BATTLENET_API_TIMEOUT") {
            let timeout_seconds: u64 = env_timeout.parse().unwrap();
            timeout = Duration::from_secs(timeout_seconds);
        }

        Self::new_with_timeout(region, &locale, &client_id, &client_secret, timeout)
    }

    /// Get a mutable copy of the client's access token. If the token has not
    /// been set or is expired, `None` is returned.
    fn try_access_token(&self) -> BattlenetClientResult<Option<String>> {
        match self.access_token.try_lock() {
            Ok(token_lock) => match token_lock.as_ref() {
                None => Err(BattleNetClientError::ClientTokenNotAvailable),
                Some(token) => match self.try_refresh_required() {
                    Ok(refresh_required) => {
                        if refresh_required {
                            Ok(None)
                        } else {
                            Ok(Some(token.to_owned()))
                        }
                    }
                    Err(e) => Err(BattleNetClientError::ClientTokenMutex(e.to_string())),
                },
            },
            Err(e) => Err(BattleNetClientError::ClientTokenMutex(e.to_string())),
        }
    }

    // TODO: rename to `token_valid` or `is_token_valid`
    /// Is the access token expired?
    fn try_refresh_required(&self) -> BattlenetClientResult<bool> {
        match self.expires_at.try_lock() {
            Ok(expiration) => {
                let now = time::OffsetDateTime::now_utc();
                Ok(expiration.le(&now))
            }
            Err(e) => Err(BattleNetClientError::ClientTokenMutex(e.to_string())),
        }
    }

    /// Request a new client access token. Token is cached within the client.
    pub async fn get_access_token(&self) -> BattlenetClientResult<String> {
        // If we have a good token, return it.
        if let Ok(Some(token)) = self.try_access_token() {
            return Ok(token);
        }

        let form = reqwest::multipart::Form::new().text("grant_type", "client_credentials");
        let token_response = self
            .http
            .post(self.region.client_token_endpoint())
            .multipart(form)
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .send()
            .await?
            .json::<AccessTokenResponse>()
            .await?;
        let access_token = token_response.access_token;

        if let Ok(mut token_lock) = self.access_token.try_lock() {
            *token_lock = Some(access_token.clone());
        } else {
            return Err(BattleNetClientError::ClientTokenMutex(
                "Could not lock token".to_string(),
            ));
        }

        if let Ok(mut expiration_lock) = self.expires_at.try_lock() {
            let expires_in_duration = Duration::from_secs(token_response.expires_in);
            *expiration_lock = OffsetDateTime::now_utc().add(expires_in_duration);
        } else {
            return Err(BattleNetClientError::ClientTokenMutex(
                "Could not lock expiration".to_string(),
            ));
        }

        Ok(access_token)
    }

    /// send a request to Battlenet
    pub async fn send_request(&self, url: String) -> BattlenetClientResult<reqwest::Response> {
        let token = self.get_access_token().await?;
        let response = self.http.get(url).bearer_auth(token).send().await?;
        Ok(response)
    }

    #[cfg(feature = "wow")]
    // Get data for the object
    pub async fn get_data<T>(&self, url_args: &UrlArgs) -> Result<T, BattleNetClientError>
    where
        T: for<'a> Deserialize<'a> + GenerateUrl,
    {
        let url = T::url(self, url_args);
        let response = self.send_request(url).await?;
        let json_text = response.text().await?;
        let result = json_to_struct(&json_text)?;
        Ok(result)
    }

    #[cfg(feature = "wow")]
    // Get the JSON string for the object
    pub async fn get_json<T>(&self, url_args: &UrlArgs) -> Result<String, BattleNetClientError>
    where
        T: GenerateUrl,
    {
        let url = T::url(self, url_args);
        let response = self.send_request(url).await?;
        let json_text = response.text().await?;
        Ok(json_text)
    }

    /// Send a request using a caller-provided bearer token (e.g., user OAuth token).
    pub async fn send_request_with_token(
        &self,
        url: String,
        token: &str,
    ) -> BattlenetClientResult<reqwest::Response> {
        let response = self.http.get(url).bearer_auth(token).send().await?;
        Ok(response)
    }

    #[cfg(feature = "wow")]
    /// Get typed data using a caller-provided bearer token.
    pub async fn get_data_with_token<T>(
        &self,
        url_args: &UrlArgs,
        token: &str,
    ) -> Result<T, BattleNetClientError>
    where
        T: for<'a> Deserialize<'a> + GenerateUrl,
    {
        let url = T::url(self, url_args);
        let response = self.send_request_with_token(url, token).await?;
        let json_text = response.text().await?;
        let result = json_to_struct(&json_text)?;
        Ok(result)
    }

    #[cfg(feature = "wow")]
    /// Get raw JSON using a caller-provided bearer token.
    pub async fn get_json_with_token<T>(
        &self,
        url_args: &UrlArgs,
        token: &str,
    ) -> Result<String, BattleNetClientError>
    where
        T: GenerateUrl,
    {
        let url = T::url(self, url_args);
        let response = self.send_request_with_token(url, token).await?;
        let json_text = response.text().await?;
        Ok(json_text)
    }
}

pub fn json_to_struct<T: for<'a> Deserialize<'a>>(json: &str) -> Result<T, BattleNetClientError> {
    let result: T = serde_json::from_str(json)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_client() -> BattleNetClient {
        BattleNetClient::new(
            BattleNetRegion::US,
            "en_US",
            "test_client_id",
            "test_client_secret",
        )
    }

    #[tokio::test]
    async fn test_send_request_with_token_constructs_bearer_auth() {
        // This test verifies the method exists and accepts the expected parameters.
        // We can't easily test the actual HTTP call without a mock server,
        // but we can verify it returns a network error (proving the request was attempted
        // with the provided token, not the client-credentials flow).
        let client = test_client();
        let result = client
            .send_request_with_token("http://localhost:1/test".to_string(), "fake_user_token")
            .await;
        // Should fail with a network error (connection refused), not a token error
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, BattleNetClientError::ClientRequestFailed(_)));
    }

    #[cfg(feature = "wow")]
    #[tokio::test]
    async fn test_get_data_with_token_exists() {
        use crate::wow_models::{GenerateUrl, UrlArgs};

        // Minimal struct to test get_data_with_token compilation
        #[derive(Debug, serde::Deserialize)]
        #[allow(dead_code)]
        struct DummyProfile {
            id: u64,
        }

        impl GenerateUrl for DummyProfile {
            fn url(_client: &BattleNetClient, _url_args: &UrlArgs) -> String {
                "http://127.0.0.1:1/test".to_string()
            }
        }

        let client = test_client();
        let result: Result<DummyProfile, BattleNetClientError> = client
            .get_data_with_token(&UrlArgs::None, "fake_token")
            .await;
        // Should fail with network error
        assert!(result.is_err());
    }

    #[cfg(feature = "wow")]
    #[tokio::test]
    async fn test_get_json_with_token_exists() {
        use crate::wow_models::{GenerateUrl, UrlArgs};

        #[derive(Debug)]
        struct DummyProfile;

        impl GenerateUrl for DummyProfile {
            fn url(_client: &BattleNetClient, _url_args: &UrlArgs) -> String {
                "http://127.0.0.1:1/test".to_string()
            }
        }

        let client = test_client();
        let result: Result<String, BattleNetClientError> = client
            .get_json_with_token::<DummyProfile>(&UrlArgs::None, "fake_token")
            .await;
        assert!(result.is_err());
    }
}
