use std::env;

use reqwest::Result;
use serde::{Deserialize, Serialize};
use utils::base64::core::encode;

pub struct TwitterAccessToken {
    access_token_key: String,
    access_token_secret: String,
}
impl TwitterAccessToken {
    pub fn new(access_token_key: String, access_token_secret: String) -> Self {
        Self {
            access_token_key,
            access_token_secret,
        }
    }
    /// please set env TWITTER_API_KEY and TWITTER_API_SECRET
    pub fn from_env() -> Self {
        let access_token_key = env::var("TWITTER_ACCESS_TOKEN_KEY").unwrap();
        let access_token_secret = env::var("TWITTER_ACCESS_TOKEN_SECRET").unwrap();
        Self {
            access_token_key,
            access_token_secret,
        }
    }
}

pub struct TwitterCunsmerCredentials {
    cunsmer_key: String,
    cunsmer_secret: String,
}
impl TwitterCunsmerCredentials {
    pub fn new(cunsmer_key: String, cunsmer_secret: String) -> Self {
        Self {
            cunsmer_key,
            cunsmer_secret,
        }
    }
    /// please set env TWITTER_CONSUMER_KEY and TWITTER_CONSUMER_SECRET
    pub fn from_env() -> Self {
        let cunsmer_key = env::var("TWITTER_CONSUMER_KEY").unwrap();
        let cunsmer_secret = env::var("TWITTER_CONSUMER_SECRET").unwrap();
        Self {
            cunsmer_key,
            cunsmer_secret,
        }
    }
    pub(super) async fn get_access_token(&self) -> Result<TwitterBeareTokenResponse> {
        let response = self.request_access_token().await?;
        Ok(serde_json::from_str(response.as_str()).unwrap())
    }

    ///-H "Authorization: Basic <BEARER_TOKEN_CREDENTIALS>" \
    ///-H "Content-Type: application/x-www-form-urlencoded;charset=UTF-8" \
    ///-d "grant_type=client_credentials" \
    ///"https://api.twitter.com/oauth2/token"
    pub async fn request_access_token(&self) -> Result<String> {
        let request_data = encode(format!("{}:{}", self.cunsmer_key, self.cunsmer_secret));
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
pub(super) struct TwitterBeareTokenResponse {
    pub token_type: String,
    pub access_token: String,
}
impl TwitterBeareTokenResponse {
    pub(super) fn create_auth_header(&self) -> (&str, String) {
        ("Authorization", format!("Bearer {}", self.access_token))
    }
}
