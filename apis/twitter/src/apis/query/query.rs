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
    pub fn set_next_token(&mut self, next_token: &str) {
        self.query = format!("{}&next_token={}", self.origin_query, next_token)
    }
    pub fn use_query(&self) -> &str {
        println!("use query = {}", self.query);
        &self.query
    }
}
