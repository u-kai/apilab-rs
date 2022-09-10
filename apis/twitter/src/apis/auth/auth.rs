use std::env;

use reqwest::Result;
use serde::{Deserialize, Serialize};
use utils::base64::core::encode;

#[allow(unused)]
pub struct TwitterAccessToken {
    pub access_token_key: String,
    pub access_token_secret: String,
}
impl TwitterAccessToken {
    #[allow(unused)]
    pub fn new(access_token_key: String, access_token_secret: String) -> Self {
        Self {
            access_token_key,
            access_token_secret,
        }
    }
    /// please set env TWITTER_API_KEY and TWITTER_API_SECRET
    #[allow(unused)]
    pub fn from_env() -> Self {
        let access_token_key = env::var("TWITTER_ACCESS_TOKEN_KEY").unwrap();
        let access_token_secret = env::var("TWITTER_ACCESS_TOKEN_SECRET").unwrap();
        Self {
            access_token_key,
            access_token_secret,
        }
    }
}

pub struct TwitterConsumerCredentials {
    pub consumer_key: String,
    pub consumer_secret: String,
}
impl TwitterConsumerCredentials {
    #[allow(unused)]
    pub fn new(consumer_key: String, consumer_secret: String) -> Self {
        Self {
            consumer_key,
            consumer_secret,
        }
    }
    /// please set env TWITTER_CONSUMER_KEY and TWITTER_CONSUMER_SECRET
    pub fn from_env() -> Self {
        let consumer_key = env::var("TWITTER_CONSUMER_KEY").unwrap();
        let consumer_secret = env::var("TWITTER_CONSUMER_SECRET").unwrap();
        Self {
            consumer_key,
            consumer_secret,
        }
    }
    pub async fn get_access_token(&self) -> Result<TwitterBeareTokenResponse> {
        let response = self.request_access_token().await?;
        Ok(serde_json::from_str(response.as_str()).unwrap())
    }
    ///-H "Authorization: Basic <BEARER_TOKEN_CREDENTIALS>" \
    ///-H "Content-Type: application/x-www-form-urlencoded;charset=UTF-8" \
    ///-d "grant_type=client_credentials" \
    ///"https://api.twitter.com/oauth2/token"
    pub async fn request_access_token(&self) -> Result<String> {
        let request_data = encode(format!("{}:{}", self.consumer_key, self.consumer_secret));
        let auth_header = format!("Basic {}", request_data);
        let response = reqwest::Client::new()
            .post("https://api.twitter.com/oauth2/token")
            .header(
                "Content-Type",
                "application/x-www-form-urlencoded;charset=UTF-8",
            )
            .header("Authorization", auth_header)
            .body("grant_type=client_credentials")
            .send()
            .await?;
        response.text().await
    }
}

#[derive(Serialize, Deserialize)]
pub struct TwitterBeareTokenResponse {
    pub token_type: String,
    pub access_token: String,
}
impl TwitterBeareTokenResponse {
    pub fn create_auth_header(&self) -> (&str, String) {
        ("Authorization", format!("Bearer {}", self.access_token))
    }
}
