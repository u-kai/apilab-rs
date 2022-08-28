use reqwest::Result;

use super::auth::{TwitterBeareTokenResponse, TwitterCunsmerCredentials};

pub struct TwitterClient {
    token: TwitterBeareTokenResponse,
}
impl TwitterClient {
    pub async fn from_env() -> Result<Self> {
        let auth = TwitterCunsmerCredentials::from_env();
        println!("send access token request");
        let token = auth.get_access_token().await?;
        Ok(Self { token })
    }
    pub async fn search_hash(&self, searched: &str) -> Result<String> {
        let query = SearchQueryBuilder::hash(searched);
        let url = format!(
            "https://api.twitter.com/2/tweets/search/recent/?query={}",
            query
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
}

struct SearchQueryBuilder;

impl SearchQueryBuilder {
    fn hash(data: &str) -> String {
        format!(r#"{}"#, data)
    }
}
