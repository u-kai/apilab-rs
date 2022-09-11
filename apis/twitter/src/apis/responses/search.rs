use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TwitterSearchResponse {
    pub data: Vec<TwitterSearchResponseData>,
    pub meta: TwitterSearchResponseMeta,
}
impl TwitterSearchResponse {
    pub fn image_urls(&self) -> Vec<&str> {
        let mut result = Vec::new();
        let _ = self.data.iter().for_each(|data| {
            data.entities.iter().for_each(|entiti| {
                if let Some(urls) = &entiti.urls {
                    urls.iter().for_each(|url| {
                        if let Some(images) = &url.images {
                            images.iter().for_each(|image| {
                                if let Some(url) = &image.url {
                                    if url.contains("orig") {
                                        result.push(url.as_str())
                                    }
                                }
                            })
                        }
                    })
                }
            })
        });
        result
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TwitterSearchResponseMeta {
    pub newest_id: Option<String>,
    pub next_token: Option<String>,
    pub oldest_id: Option<String>,
    pub result_count: Option<f64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TwitterSearchResponseData {
    pub author_id: Option<String>,
    pub created_at: Option<String>,
    pub entities: Option<TwitterSearchResponseDataEntities>,
    pub id: Option<String>,
    pub in_reply_to_user_id: Option<String>,
    pub lang: Option<String>,
    pub possibly_sensitive: Option<bool>,
    pub referenced_tweets: Option<Vec<TwitterSearchResponseDataReferencedTweets>>,
    pub source: Option<String>,
    pub text: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TwitterSearchResponseDataReferencedTweets {
    pub id: Option<String>,
    pub r#type: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TwitterSearchResponseDataEntities {
    pub hashtags: Option<Vec<TwitterSearchResponseDataEntitiesHashtags>>,
    pub mentions: Option<Vec<TwitterSearchResponseDataEntitiesMentions>>,
    pub urls: Option<Vec<TwitterSearchResponseDataEntitiesUrls>>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TwitterSearchResponseDataEntitiesUrls {
    pub description: Option<String>,
    pub display_url: Option<String>,
    pub end: Option<f64>,
    pub expanded_url: Option<String>,
    pub images: Option<Vec<TwitterSearchResponseDataEntitiesUrlsImages>>,
    pub start: Option<f64>,
    pub status: Option<f64>,
    pub title: Option<String>,
    pub unwound_url: Option<String>,
    pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TwitterSearchResponseDataEntitiesUrlsImages {
    pub height: Option<f64>,
    pub url: Option<String>,
    pub width: Option<f64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TwitterSearchResponseDataEntitiesMentions {
    pub end: Option<f64>,
    pub start: Option<f64>,
    pub username: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TwitterSearchResponseDataEntitiesHashtags {
    pub end: Option<f64>,
    pub start: Option<f64>,
    pub tag: Option<String>,
}
