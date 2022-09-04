use serde_json::Result;

use crate::apis::responses::search::{TwitterSearchResponse, TwitterSearchResponseMeta};

impl TwitterSearchResponse {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            meta: TwitterSearchResponseMeta {
                newest_id: None,
                next_token: None,
                oldest_id: None,
                result_count: None,
            },
        }
    }
    pub fn from_response(response: &str) -> Result<Self> {
        serde_json::from_str(response)
    }
    pub fn concat_other(&mut self, mut other: Self) {
        self.data.append(&mut other.data);
        self.meta = other.meta;
    }
    pub fn get_next_token(&self) -> &str {
        self.meta.next_token.as_ref().unwrap()
    }
}
