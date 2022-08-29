use std::{thread, time};

use reqwest::Result;
use utils::url_encode::core::UrlEncoder;

use crate::apis::responses::meta::TwitterApiResponseLimitMeta;

use super::{
    auth::{TwitterBeareTokenResponse, TwitterCunsmerCredentials},
    responses::meta::TwitterApiResponseMeta,
};

pub struct TwitterClient {
    query_builder: SearchQueryBuilder,
    token: TwitterBeareTokenResponse,
}
impl TwitterClient {
    pub async fn from_env() -> Result<Self> {
        let auth = TwitterCunsmerCredentials::from_env();
        println!("send access token request");
        let query_builder = SearchQueryBuilder::new();
        let token = auth.get_access_token().await?;
        Ok(Self {
            token,
            query_builder,
        })
    }
    pub async fn search_rec(&self, free_query: &str, count: usize) -> Result<String> {
        let mut result = String::new();
        let mut new_query = format!("{}", free_query);
        for i in 0..count {
            let response = self.search(&new_query).await?;
            let meta = TwitterApiResponseMeta::new(&response);
            match meta {
                Ok(meta) => {
                    new_query = format!("{}&next_token={}", free_query, meta.next_token());
                    result = format!("{} time {}\n", i, response);
                    println!("{}", result)
                }
                Err(_) => {
                    let meta = TwitterApiResponseLimitMeta::new(&response).unwrap();
                    if meta.is_limit() {
                        println!("Reach Twitter Limit");
                        println!("So Wait 15 minite");
                        Self::sleep();
                    } else {
                        println!("rare case meta = {:#?}", meta);
                    }
                }
            }
        }
        Ok(result)
    }
    pub async fn search(&self, free_query: &str) -> Result<String> {
        let url = format!(
            "https://api.twitter.com/2/tweets/search/recent/?query={}",
            self.query_builder.encode_query(free_query)
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
    pub async fn search_hash(&self, searched: &str) -> Result<String> {
        let query = self.query_builder.hash(searched);
        self.search(&query).await
    }
    fn sleep() {
        thread::sleep(time::Duration::from_secs(900))
    }
}

struct SearchQueryBuilder {
    url_encoder: UrlEncoder,
}

impl SearchQueryBuilder {
    fn new() -> Self {
        Self {
            url_encoder: UrlEncoder::new(),
        }
    }
    fn encode_query(&self, query: &str) -> String {
        self.url_encoder.encode(query)
    }
    fn hash(&self, data: &str) -> String {
        let encoded = self.url_encoder.encode(format!("#{}", data).as_str());
        format!(r"{}", encoded)
    }
}
