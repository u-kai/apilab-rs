use std::{thread, time};

use reqwest::Result;
//use serde::Serialize;
//use utils::oauth::core::OAuth1RequestToken;

use super::{
    auth::{TwitterBeareTokenResponse, TwitterConsumerCredentials},
    oauth1::TwitterOAuth1Handler,
    responses::search::TwitterSearchResponse,
};
#[derive(Debug)]
pub struct SearchQuery {
    query: String,
    origin_query: String,
}
impl SearchQuery {
    pub fn new(query: String) -> Self {
        Self {
            origin_query: query.clone(),
            query,
        }
    }
    fn set_next_token(&mut self, next_token: &str) {
        self.query = format!("{}&next_token={}", self.origin_query, next_token)
    }
    fn use_query(&self) -> &str {
        println!("use query = {}", self.query);
        &self.query
    }
}
pub struct TwitterClient {
    token: TwitterBeareTokenResponse,
    oauth1_handler: TwitterOAuth1Handler,
}
impl TwitterClient {
    pub async fn from_env() -> Result<Self> {
        let auth = TwitterConsumerCredentials::from_env();
        let token = auth.get_access_token().await?;
        // let token_ge = OAuth1TokenGetter::new()
        let oauth1_handler = TwitterOAuth1Handler::from_env_2();
        //let oauth1_handler = TwitterOAuth1Handler::from_env().await?;
        Ok(Self {
            token,
            oauth1_handler, //: TwitterOAuth1Handler::from_env(),
        })
    }
    pub async fn delete(&self, id: &str) -> Result<()> {
        let url = format!("{}/{}", Self::gen_twitter_url("tweets"), id);
        let client = reqwest::Client::new();
        let response = client.delete(url).send().await?;
        //let response = self
        //.oauth1_handler
        //.request(&Self::gen_twitter_url("tweets"), headers, "")
        //.await?;
        println!("success {:?}", response.text().await?);
        Ok(())
    }
    //pub async fn tweet(&self, tweet: &str) -> Result<()> {
    //let mut headers = HeaderMap::new();
    //headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    //let body = format!(r#"{{ "text":"{}" }}"#, tweet);
    //let response = self
    //.oauth1_handler
    //.request(&Self::gen_twitter_url("tweets"), headers, &body)
    //.await?;
    //println!("success {:?}", response);
    //Ok(())
    //}
    pub async fn search_rec(
        &self,
        mut query: SearchQuery,
        count: usize,
    ) -> Result<TwitterSearchResponse> {
        let mut search_result = TwitterSearchResponse::new();
        for _ in 0..count {
            let response = self.search(query.use_query()).await?;
            let response = TwitterSearchResponse::from_response(&response).unwrap();
            search_result.concat_other(response);
            let next_token = search_result.get_next_token();
            query.set_next_token(next_token);
        }
        Ok(search_result)
    }
    pub async fn search(&self, free_query: &str) -> Result<String> {
        let url = format!(
            "https://api.twitter.com/2/tweets/search/recent/?query={}",
            free_query
        );
        let header = self.token.create_auth_header();
        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .header(header.0, header.1.as_str())
            .header(
                "Content-Type",
                "application/x-www-form-urlencoded;charset=UTF-8",
            )
            .send()
            .await?;
        let text = response.text().await?;
        Ok(text)
    }
    fn gen_twitter_url(path: &str) -> String {
        format!("https://api.twitter.com/2/{}", path)
    }
    fn sleep() {
        thread::sleep(time::Duration::from_secs(900))
    }
}
