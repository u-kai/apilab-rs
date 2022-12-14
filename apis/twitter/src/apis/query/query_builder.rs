use chrono::{Date, Local};
use utils::url_encode::core::UrlEncoder;

use super::query::SearchQuery;

#[derive(Default, Debug)]
#[allow(unused)]
pub struct SearchQueryBuilder {
    query: String,
    url_encoder: UrlEncoder,
    start_time: Option<Date<Local>>,
    end_time: Option<Date<Local>>,
    since_id: Option<String>,
    source: Option<String>,
    until_id: Option<String>,
    max_results: Option<usize>,
    tweet_fileds: Option<TweetsFieldQueryParameters>,
}

#[allow(unused)]
impl SearchQueryBuilder {
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            url_encoder: UrlEncoder::new(),
            ..Default::default()
        }
    }
    pub fn add_entities_filed(&mut self) -> &mut Self {
        self.tweet_fileds = Some(TweetsFieldQueryParameters {
            entities: Some(TweetsFieldEntities { annotations: None }),
            ..Default::default()
        });
        self
    }
    pub fn add_hash(&mut self) -> &mut Self {
        self.query = format!("{}{}", self.url_encoder.encode("#"), self.query);
        self
    }
    pub fn add_max_results(&mut self, max_results: usize) -> &mut Self {
        self.max_results = Some(max_results);
        self
    }
    pub fn build_query(self) -> SearchQuery {
        SearchQuery::new(format!(
            "{}&{}&max_results={}",
            self.query,
            self.tweet_fileds.unwrap().to_query(),
            self.max_results.unwrap()
        ))
    }
    fn encode_query(&self, query: &str) -> String {
        self.url_encoder.encode(query)
    }
    fn hash(&self, data: &str) -> String {
        let encoded = self.url_encoder.encode(format!("#{}", data).as_str());
        format!("{}", encoded)
    }
}

trait ToQuery {
    fn to_query(&self) -> String;
}
#[derive(Default, Debug)]
#[allow(unused)]
struct TweetsFieldQueryParameters {
    attachments: Option<TweetsFieldAttachments>,
    author_id: Option<String>,
    created_at: Option<Date<Local>>,
    context_annotations: Option<TweetsFieldContextAnnotations>,
    public_metrics: Option<PublicMetrics>,
    entities: Option<TweetsFieldEntities>,
}
impl ToQuery for TweetsFieldQueryParameters {
    fn to_query(&self) -> String {
        let entities = match &self.entities {
            Some(entities) => entities.to_query(),
            None => String::new(),
        };
        format!("tweet.fields={}", entities)
    }
}
#[derive(Default, Debug)]
#[allow(unused)]
struct PublicMetrics {
    retweet_count: usize,
    reply_count: usize,
    like_count: usize,
    quote_count: usize,
}
#[derive(Default, Debug)]
#[allow(unused)]
struct TweetsFieldEntities {
    annotations: Option<Vec<EntitiesAnnotation>>,
}
impl ToQuery for TweetsFieldEntities {
    fn to_query(&self) -> String {
        "entities".to_string()
    }
}
#[derive(Default, Debug)]
#[allow(unused)]
struct EntitiesAnnotation {
    start: usize,
    end: usize,
    probability: f64,
}
#[derive(Default, Debug)]
#[allow(unused)]
struct TweetsFieldAttachments {
    media_keys: Vec<String>,
}
#[derive(Default, Debug)]
struct TweetsFieldContextAnnotations {}
