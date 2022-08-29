use serde::{Deserialize, Serialize};
use serde_json::Result;

use crate::attach_twitter_response;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TwitterSearchResponse {
    data: Vec<TwitterSearchDefaultResponse>,
}
impl TwitterSearchResponse {
    pub fn new(response: &str) -> Result<Self> {
        serde_json::from_str(response)
    }
    pub fn print(&self) {
        self.data.iter().for_each(|response| {
            if let Some(urls) = &response.entities {
                if let Some(urls) = &urls.urls {
                    urls.iter().for_each(|url| {
                        if let Some(images) = &url.images {
                            images.iter().for_each(|image| {
                                println!("id : {}", response.id);
                                println!("text : {}", response.text);
                                println!("image_url = {:?}", image.url)
                            })
                        }
                    })
                }
            }
        })
    }
}
attach_twitter_response!(TwitterSearchDefaultResponse);
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TwitterSearchDefaultResponse {
    id: String,
    text: String,
    entities: Option<TwitterFieldsEntitiesUrls>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TwitterFieldsEntitiesUrls {
    urls: Option<Vec<TwitterImagesField>>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TwitterImagesField {
    images: Option<Vec<TwitterImageField>>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TwitterImageField {
    url: String,
    width: usize,
    height: usize,
}
