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
    pub fn from_response(response: &str) -> Result<Self, String> {
        match serde_json::from_str(response) {
            Ok(res) => Ok(res),
            Err(_) => Err(response.to_string()),
        }
    }
    pub fn concat_other(&mut self, mut other: Self) {
        self.data.append(&mut other.data);
        self.meta = other.meta;
    }
    pub fn get_next_token(&self) -> Option<&str> {
        self.meta.next_token.as_ref().map(|s| s.as_str())
    }
}
