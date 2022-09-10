use reqwest::{header::HeaderMap, Result};
use utils::oauth::core::{OAuth1, OAuth1Handler, OAuth1RequestTokenFetcher};

use crate::apis::auth::TwitterAccessToken;

use super::auth::TwitterConsumerCredentials;

pub struct TwitterOAuth1Handler {
    oauth_handler: OAuth1Handler,
}
impl TwitterOAuth1Handler {
    pub fn from_env_2() -> Self {
        let consumer_credentials = TwitterConsumerCredentials::from_env();
        let access_token = TwitterAccessToken::from_env();
        let oauth_handler = OAuth1Handler::new(
            &consumer_credentials.consumer_key,
            &consumer_credentials.consumer_secret,
            &access_token.access_token_key,
            &access_token.access_token_secret,
        );
        println!("oauth_handler: {:#?}", oauth_handler);
        Self { oauth_handler }
    }
    pub async fn from_env() -> Result<()> {
        let consumer_credentials = TwitterConsumerCredentials::from_env();
        let oauth = OAuth1::new(
            &consumer_credentials.consumer_key,
            &consumer_credentials.consumer_secret,
        );
        let response = oauth
            .fetch_request_token("https://api.twitter.com/oauth/request_token", None)
            .await?;
        println!("success {:?}", response);
        Ok(())
        //let oauth_authorizer = oauth_token_getter
        //.create_oauth1_authorizer("https://api.twitter.com/oauth/authorize")
        //.await?;
        //let res = oauth_authorizer.request().await?;
        //println!("auth response {}", res);
        //let oauth_handler = oauth_token_getter.create_oauth1_handler().await?;
        //println!("oauth_handler: {:#?}", oauth_handler);
        //Ok(Self {})
    }
    pub async fn post(&self, endpoint: &str, headers: HeaderMap, body: &str) -> Result<String> {
        //self.oauth_handler.post(endpoint, headers, body).await
        Ok(String::new())
    }
}
