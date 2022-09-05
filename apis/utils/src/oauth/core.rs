use std::collections::BTreeMap;

use chrono::Utc;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Result,
};

use crate::{base64::core::encode_from_byte, url_encode::core::UrlEncoder};

#[derive(Clone, Debug)]
pub struct OAuth1Handler {
    endpoint: String,
    oauth_callback: String,
    oauth_signature_method: String,
    oauth_consumer_key: String,
    oauth_consumer_secret: String,
    oauth_token: Option<String>,
    oauth_token_secret: Option<String>,
    url_encoder: UrlEncoder,
}
impl OAuth1Handler {
    pub fn new(
        endpoint: &str,
        oauth_callback: &str,
        oauth_signature_method: &str,
        oauth_consumer_key: &str,
        oauth_consumer_secret: &str,
    ) -> Self {
        let mut url_encoder = UrlEncoder::new();
        url_encoder
            .regist_non_encode('*')
            .regist_non_encode('-')
            .regist_non_encode('.')
            .regist_non_encode('_');
        Self {
            endpoint: endpoint.to_string(),
            oauth_callback: oauth_callback.to_string(),
            oauth_signature_method: oauth_signature_method.to_string(),
            oauth_consumer_key: oauth_consumer_key.to_string(),
            oauth_consumer_secret: oauth_consumer_secret.to_string(),
            oauth_token: None,
            oauth_token_secret: None,
            url_encoder,
        }
    }
    pub async fn request(&self, body: &str) -> Result<String> {
        let header_auth = self.get_request_header();
        let headers = self.gen_header(&header_auth);
        let client = reqwest::Client::new();
        client
            .post(&self.endpoint)
            .headers(headers)
            .body(body.to_string())
            .send()
            .await?
            .text()
            .await
    }
    pub async fn get_request_token(&self) -> Result<OAuth1RequestToken> {
        let header_auth = self.get_request_token_header();
        let header = self.gen_header(&header_auth);
        println!("header_auth {}", header_auth);
        println!("headers {:#?}", header);
        let client = reqwest::Client::new();
        let response = client
            .post(&self.endpoint)
            .headers(header)
            .send()
            .await?
            .text()
            .await?;
        println!("response {}", response);
        let mut response = response.split('&').map(|s| {
            let mut splited = s.split('=');
            splited.next();
            splited.next().unwrap()
        });
        Ok(OAuth1RequestToken {
            oauth_token: response.next().unwrap().to_string(),
            oauth_token_secret: response.next().unwrap().to_string(),
            oauth_callback_confirmed: response.next().unwrap().to_string(),
        })
    }
    pub fn change_endpoint(&mut self, endpoint: &str) {
        self.endpoint = endpoint.to_string()
    }
    fn create_oauth_signature(&self, oauth_header: &BTreeMap<&str, &str>) -> String {
        let consumer_secret_encoded = self.url_encoder.encode(&self.oauth_consumer_secret);
        let token_secret_encoded = self.url_encoder.encode(
            self.oauth_token_secret
                .as_ref()
                .map(|s| s.as_str())
                .unwrap_or(""),
        );
        let key = format!("{}&{}", consumer_secret_encoded, token_secret_encoded);
        let params = oauth_header.iter().fold(String::new(), |acc, (k, v)| {
            format!(
                "{}{}={}&",
                acc,
                self.url_encoder.encode(k),
                self.url_encoder.encode(v)
            )
        });
        let params = &params[..params.len() - 1];
        let http_method_encoded = self.url_encoder.encode("POST");
        let endpoint_encoded = self.url_encoder.encode(&self.endpoint);
        let params_encoded = self.url_encoder.encode(params);
        let message = format!(
            "{}&{}&{}",
            http_method_encoded, endpoint_encoded, params_encoded
        );
        let hash = hmacsha1::hmac_sha1(key.as_bytes(), message.as_bytes());
        encode_from_byte(&hash)
    }
    fn get_request_header(&self) -> String {
        let oauth_nonce = &format!("nonce{}", Utc::now().timestamp());
        let oauth_timestamp = &format!("{}", Utc::now().timestamp());
        let oauth_version = "1.0";
        let mut params = BTreeMap::new();
        params.insert("oauth_nonce", oauth_nonce.as_str());
        params.insert("oauth_callback", &self.oauth_callback);
        params.insert("oauth_signature_method", &self.oauth_signature_method);
        params.insert("oauth_timestamp", &oauth_timestamp);
        params.insert("oauth_version", &oauth_version);
        params.insert("oauth_consumer_key", &self.oauth_consumer_key);
        let oauth_signature = self.create_oauth_signature(&params);
        let oauth_nonce = self
            .url_encoder
            .encode(&format!("nonce{}", Utc::now().timestamp()));
        let oauth_timestamp = self
            .url_encoder
            .encode(&format!("{}", Utc::now().timestamp()));
        let oauth_token = self.url_encoder.encode(&self.oauth_token.as_ref().unwrap());
        let oauth_version = self.url_encoder.encode("1.0");
        let oauth_callback = self.url_encoder.encode(&self.oauth_callback);
        let oauth_signature_method = self.url_encoder.encode(&self.oauth_signature_method);
        let oauth_consumer_key = self.url_encoder.encode(&self.oauth_consumer_key);
        let oauth_signature = self.url_encoder.encode(oauth_signature.as_str());
        format!(
            r#"OAuth oauth_consumer_key="{}", oauth_nonce="{}", oauth_signature_method="{}", oauth_timestamp="{}", oauth_token="{}", oauth_version="{}", oauth_signature="{}""#,
            oauth_consumer_key,
            oauth_nonce,
            oauth_signature_method,
            oauth_timestamp,
            oauth_token,
            oauth_version,
            oauth_signature,
        )
    }
    fn get_request_token_header(&self) -> String {
        let oauth_nonce = &format!("nonce{}", Utc::now().timestamp());
        let oauth_timestamp = &format!("{}", Utc::now().timestamp());
        let oauth_version = "1.0";
        let mut params = BTreeMap::new();
        params.insert("oauth_nonce", oauth_nonce.as_str());
        params.insert("oauth_callback", &self.oauth_callback);
        params.insert("oauth_signature_method", &self.oauth_signature_method);
        params.insert("oauth_timestamp", &oauth_timestamp);
        params.insert("oauth_version", &oauth_version);
        params.insert("oauth_consumer_key", &self.oauth_consumer_key);
        let oauth_signature = self.create_oauth_signature(&params);
        let oauth_nonce = self
            .url_encoder
            .encode(&format!("nonce{}", Utc::now().timestamp()));
        let oauth_timestamp = self
            .url_encoder
            .encode(&format!("{}", Utc::now().timestamp()));
        let oauth_version = self.url_encoder.encode("1.0");
        let oauth_callback = self.url_encoder.encode(&self.oauth_callback);
        let oauth_signature_method = self.url_encoder.encode(&self.oauth_signature_method);
        let oauth_consumer_key = self.url_encoder.encode(&self.oauth_consumer_key);
        let oauth_signature = self.url_encoder.encode(oauth_signature.as_str());
        format!(
            r#"OAuth oauth_nonce="{}", oauth_callback="{}", oauth_signature_method="{}", oauth_timestamp="{}", oauth_consumer_key="{}", oauth_signature="{}", oauth_version="{}""#,
            oauth_nonce,
            oauth_callback,
            oauth_signature_method,
            oauth_timestamp,
            oauth_consumer_key,
            oauth_signature,
            oauth_version,
        )
    }
    fn gen_header(&self, header_auth: &str) -> HeaderMap {
        let mut header = HeaderMap::new();
        header.insert(AUTHORIZATION, header_auth.parse().unwrap());
        header.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );
        //header.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        header
    }
}

#[derive(Clone, Debug)]
pub struct OAuth1RequestToken {
    pub oauth_token: String,
    pub oauth_token_secret: String,
    pub oauth_callback_confirmed: String,
}
