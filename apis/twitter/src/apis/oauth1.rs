use reqwest::{
    header::{HeaderMap, CONTENT_TYPE},
    Result,
};
use utils::oauth::core::OAuth1;

//use crate::apis::auth::TwitterAccessToken;

use crate::apis::auth::TwitterAccessToken;

use super::auth::TwitterConsumerCredentials;

pub struct TwitterOAuth1Handler {
    //oauth_handler: OAuth1Handler,
}
impl TwitterOAuth1Handler {
    pub fn from_env_2() -> Self {
        //let consumer_credentials = TwitterConsumerCredentials::from_env();
        //let access_token = TwitterAccessToken::from_env();
        //let oauth_handler = OAuth1Handler::new(
        //&consumer_credentials.consumer_key,
        //&consumer_credentials.consumer_secret,
        //&access_token.access_token_key,
        //&access_token.access_token_secret,
        //);
        //println!("oauth_handler: {:#?}", oauth_handler);
        Self {}
    }
    pub async fn from_env() -> Result<()> {
        let consumer_credentials = TwitterConsumerCredentials::from_env();
        let mut oauth = OAuth1::new(
            &consumer_credentials.consumer_key,
            &consumer_credentials.consumer_secret,
            "http://localhost",
        );
        let ta = TwitterAccessToken::from_env();
        oauth.set_access_token(&ta.access_token_key);
        oauth.set_access_token_secret(&ta.access_token_secret);

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        let body = Some(r#"{"text":"good!"}"#);
        let res = oauth
            .post("https://api.twitter.com/2/tweets", Some(headers), body)
            .await?;
        println!("response {}", res);
        Ok(())
    }
    //pub async fn post(&self, endpoint: &str, headers: HeaderMap, body: &str) -> Result<String> {
    ////self.oauth_handler.post(endpoint, headers, body).await
    //Ok(String::new())
    //}
}
