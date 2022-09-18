use std::env;

use reqwest::Result;
use utils::oauth::core::OAuth1;

#[tokio::main]
async fn main() -> Result<()> {
    let consumer_key = env::var("TWITTER_CONSUMER_KEY").unwrap();
    let consumer_secret = env::var("TWITTER_CONSUMER_SECRET").unwrap();
    let mut oauth = OAuth1::new_without_token(&consumer_key, &consumer_secret);
    oauth
        .set_secret_by_oauth_session("http://localhost/callback")
        .await
}
