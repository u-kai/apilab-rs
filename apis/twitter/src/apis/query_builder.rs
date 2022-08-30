use utils::url_encode::core::UrlEncoder;
pub struct SearchQueryBuilder {
    url_encoder: UrlEncoder,
}

impl SearchQueryBuilder {
    fn new() -> Self {
        Self {
            url_encoder: UrlEncoder::new(),
        }
    }
    fn encode_query(&self, query: &str) -> String {
        let e = self.url_encoder.encode(query);
        println!("{}", e);
        e
    }
    fn hash(&self, data: &str) -> String {
        let encoded = self.url_encoder.encode(format!("#{}", data).as_str());
        format!("{}", encoded)
    }
}
