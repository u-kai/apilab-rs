use std::collections::HashMap;

use reqwest::RequestBuilder;

pub trait HttpClient {
    fn get(&self, url: &str) -> RequestBuilder;
    fn get_with_param<P: ToUrlParameter>(&self, url: &str, param: P) -> RequestBuilder;
    fn post_with_json<P: ToJson>(&self, url: &str, param: P) -> RequestBuilder;
    fn post_with_form_param(&self, url: &str, param: HashMap<String, String>) -> RequestBuilder;
}

pub trait ToUrlParameter {
    fn to_parameter(&self) -> String;
}
pub trait ToJson {
    fn to_json(&self) -> String;
}
