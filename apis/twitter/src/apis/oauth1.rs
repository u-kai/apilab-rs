use reqwest::{header::HeaderMap, Result};
use utils::oauth::core::{OAuth1Handler, OAuth1RequestTokenFetcher};

use crate::apis::auth::TwitterAccessToken;

use super::auth::TwitterConsumerCredentials;

pub struct TwitterOAuth1Handler {
    oauth_handler: OAuth1Handler,
}
impl TwitterOAuth1Handler {
    pub fn from_env_2() -> Self {
        let cumsumer_credentials = TwitterConsumerCredentials::from_env();
        let access_token = TwitterAccessToken::from_env();
        let oauth_handler = OAuth1Handler::new(
            &cumsumer_credentials.consumer_key,
            &cumsumer_credentials.consumer_secret,
            &access_token.access_token_key,
            &access_token.access_token_secret,
        );
        println!("oauth_handler: {:#?}", oauth_handler);
        Self { oauth_handler }
    }
    //pub async fn from_env() -> Result<Self> {
    //let cumsumer_credentials = TwitterConsumerCredentials::from_env();
    //let oauth_token_getter = OAuth1TokenGetter::new(
    //"https://api.twitter.com/oauth/request_token",
    //"http://localhost",
    //"HMAC-SHA1",
    //&cumsumer_credentials.consumer_key,
    //&cumsumer_credentials.consumer_secret,
    //);
    //println!("success ",);
    //let oauth_authorizer = oauth_token_getter
    //.create_oauth1_authorizer("https://api.twitter.com/oauth/authorize")
    //.await?;
    //let res = oauth_authorizer.request().await?;
    //println!("auth response {}", res);
    ////let oauth_handler = oauth_token_getter.create_oauth1_handler().await?;
    ////println!("oauth_handler: {:#?}", oauth_handler);
    //Ok(Self {})
    //}
    pub async fn post(&self, endpoint: &str, headers: HeaderMap, body: &str) -> Result<String> {
        //self.oauth_handler.post(endpoint, headers, body).await
        Ok(String::new())
    }
}
