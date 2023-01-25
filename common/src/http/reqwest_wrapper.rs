use std::collections::HashMap;

use reqwest::{header::CONTENT_TYPE, Client};

use super::client::HttpClient;

pub struct ReqwestWrapper {
    inner: Client,
}
impl ReqwestWrapper {
    pub fn new() -> Self {
        Self {
            inner: Client::new(),
        }
    }
}

impl HttpClient for ReqwestWrapper {
    fn get(&self, url: &str) -> reqwest::RequestBuilder {
        self.inner.get(url)
    }
    fn get_with_param<P: super::client::ToUrlParameter>(
        &self,
        url: &str,
        param: P,
    ) -> reqwest::RequestBuilder {
        self.inner.get(format!("{}?{}", url, param.to_parameter()))
    }
    fn post_with_form_param(
        &self,
        url: &str,
        param: HashMap<String, String>,
    ) -> reqwest::RequestBuilder {
        self.inner.post(url).form(&param)
    }
    fn post_with_json<P: super::client::ToJson>(
        &self,
        url: &str,
        param: P,
    ) -> reqwest::RequestBuilder {
        self.inner
            .post(url)
            .body(param.to_json())
            .header(CONTENT_TYPE, "application/json")
    }
}
