use reqwest::Result;
use utils::oauth::core::{OAuth1Handler, OAuth1RequestToken};

use super::auth::{TwitterAccessToken, TwitterConsumerCredentials};

pub struct TwitterOAuth1Handler {
    oauth_handler: OAuth1Handler,
}
impl TwitterOAuth1Handler {
    pub fn from_env() -> Self {
        let cumsumer_credentials = TwitterConsumerCredentials::from_env();
        Self {
            oauth_handler: OAuth1Handler::new(
                "https://api.twitter.com/oauth/request_token",
                "http://localhost",
                "HMAC-SHA1",
                &cumsumer_credentials.consumer_key,
                &cumsumer_credentials.consumer_secret,
            ),
        }
    }
    pub async fn get_request_token(&self) -> Result<OAuth1RequestToken> {
        self.oauth_handler.get_request_token().await
    }
    pub async fn request(&self, body: &str) -> Result<String> {
        self.oauth_handler.request(body).await
    }
    pub fn change_endpoint(&mut self, endpoint: &str) {
        self.oauth_handler.change_endpoint(endpoint);
    }
}
