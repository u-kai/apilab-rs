use std::env;

use reqwest::Result;
use utils::base64::core::encode;

pub struct TwitterCunsmerCredentials {
    cunsmer_key: String,
    cunsmer_secret: String,
    beare_token: Option<String>,
}
impl TwitterCunsmerCredentials {
    pub fn from_env() -> Self {
        let cunsmer_key = env::var("TWITTER_CONSUMER_KEY").unwrap();
        let cunsmer_secret = env::var("TWITTER_CONSUMER_SECRET").unwrap();
        Self {
            cunsmer_key,
            cunsmer_secret,
            beare_token: None,
        }
    }
    pub async fn get_beare_token(&self) -> Result<String> {
        let request_data = encode(format!("{}:{}", self.cunsmer_key, self.cunsmer_secret));
        let auth_header = format!("Basic {}", request_data);
        //-H "Authorization: Basic <BEARER_TOKEN_CREDENTIALS>" \
        //-H "Content-Type: application/x-www-form-urlencoded;charset=UTF-8" \
        //-d "grant_type=client_credentials" \
        //"https://api.twitter.com/oauth2/token"
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
#[cfg(test)]
mod twiiter_auth_test {
    use super::*;
}
