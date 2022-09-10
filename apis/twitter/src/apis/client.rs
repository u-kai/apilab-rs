use super::{
    auth::{
        auth::{TwitterBeareTokenResponse, TwitterConsumerCredentials},
        oauth1::TwitterOAuth1Handler,
    },
    query::query::SearchQuery,
    responses::search::TwitterSearchResponse,
};
use reqwest::{
    header::{HeaderMap, CONTENT_TYPE},
    Result,
};

pub struct TwitterClient {
    token: TwitterBeareTokenResponse,
    oauth1: TwitterOAuth1Handler,
}
impl TwitterClient {
    pub async fn from_env() -> Result<Self> {
        let auth = TwitterConsumerCredentials::from_env();
        let token = auth.get_access_token().await?;
        let oauth1 = TwitterOAuth1Handler::from_env();
        Ok(Self { token, oauth1 })
    }
    pub async fn tweet(&self, tweet: &str) -> Result<()> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        let body = format!(r#"{{ "text":"{}" }}"#, tweet);
        let _ = self
            .oauth1
            .post(&Self::gen_twitter_url("tweets"), headers, &body)
            .await?;
        Ok(())
    }
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
}
