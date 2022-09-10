use super::auth::{TwitterAccessToken, TwitterConsumerCredentials};
use reqwest::{header::HeaderMap, Result};
use utils::oauth::core::OAuth1;

pub struct TwitterOAuth1Handler {
    oauth1: OAuth1,
}
impl TwitterOAuth1Handler {
    pub fn from_env() -> Self {
        let consumer_credentials = TwitterConsumerCredentials::from_env();
        let access_token = TwitterAccessToken::from_env();
        Self {
            oauth1: OAuth1::new(
                consumer_credentials.consumer_key,
                consumer_credentials.consumer_secret,
                access_token.access_token_key,
                access_token.access_token_secret,
            ),
        }
    }
    pub async fn post(&self, endpoint: &str, headers: HeaderMap, body: &str) -> Result<String> {
        self.oauth1
            .post(endpoint, Some(headers), Some(body))
            .await?;
        Ok(String::new())
    }
}
