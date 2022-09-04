use serde::{Deserialize, Serialize};
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct TwitterSearchResponse {
    pub data: Vec<TwitterSearchResponseData>,
    pub meta: TwitterSearchResponseMeta,
}
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct TwitterSearchResponseMeta {
    pub newest_id: Option<String>,
    pub next_token: Option<String>,
    pub oldest_id: Option<String>,
    pub result_count: Option<f64>,
}
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct TwitterSearchResponseData {
    pub author_id: Option<String>,
    pub created_at: Option<String>,
    pub entities: Option<TwitterSearchResponseDataEntities>,
    pub id: Option<String>,
    pub lang: Option<String>,
    pub possibly_sensitive: Option<bool>,
    pub referenced_tweets: Option<Vec<TwitterSearchResponseDataReferencedTweets>>,
    pub source: Option<String>,
    pub text: Option<String>,
}
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct TwitterSearchResponseDataReferencedTweets {
    pub id: Option<String>,
    pub r#type: Option<String>,
}
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct TwitterSearchResponseDataEntities {
    pub mentions: Option<Vec<TwitterSearchResponseDataEntitiesMentions>>,
}
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct TwitterSearchResponseDataEntitiesMentions {
    pub end: Option<f64>,
    pub start: Option<f64>,
    pub username: Option<String>,
}