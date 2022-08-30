use std::{thread, time};

use reqwest::Result;

use crate::apis::responses::meta::TwitterApiResponseLimitMeta;

use super::{
    auth::{TwitterBeareTokenResponse, TwitterCunsmerCredentials},
    responses::{meta::TwitterApiResponseMeta, search::TwitterSearchResponse},
};
pub struct SearchQuery {
    query: String,
    origin_query: String,
}
impl SearchQuery {
    fn new(query: String) -> Self {
        Self {
            origin_query: query.clone(),
            query,
        }
    }
    fn set_next_token(&mut self, next_token: &str) {
        self.query = format!("{}&next_token={}", self.origin_query, next_token)
    }
    fn use_query(&self) -> &str {
        &self.query
    }
}
pub struct TwitterClient {
    token: TwitterBeareTokenResponse,
}
impl TwitterClient {
    pub async fn from_env() -> Result<Self> {
        let auth = TwitterCunsmerCredentials::from_env();
        let token = auth.get_access_token().await?;
        Ok(Self { token })
    }
    pub async fn search_rec(&self, mut query: SearchQuery, count: usize) -> Result<String> {
        let mut result = String::new();
        for i in 0..count {
            let response = self.search(query.use_query()).await?;
            let meta = TwitterApiResponseMeta::new(&response);
            match meta {
                Ok(meta) => {
                    query.set_next_token(meta.next_token());
                    result = format!("{} time {}\n", i, response);
                }
                Err(_) => {
                    let meta = TwitterApiResponseLimitMeta::new(&response)
                        .expect(&format!("{}", response));
                    if meta.is_limit() {
                        println!("Reach Twitter Limit");
                        println!("So Wait 15 minite");
                        Self::sleep();
                    } else {
                        println!("rare case meta = {:#?}", meta);
                    }
                }
            }
            let data =
                TwitterSearchResponse::new(&response).expect(&format!("not parse \n{}", response));
            data.print();
        }
        Ok(result)
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
    fn sleep() {
        thread::sleep(time::Duration::from_secs(900))
    }
}
