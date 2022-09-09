use std::collections::BTreeMap;

use chrono::Utc;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Result,
};

use crate::{base64::core::encode_from_byte, url_encode::core::UrlEncoder};

use super::authorizer::OAuth1Authorizer;

#[derive(Clone, Debug)]
pub struct OAuth1Handler {
    oauth_consumer_key: String,
    oauth_consumer_secret: String,
    oauth_token: String,
    oauth_token_secret: String,
    oauth_signature_method: String,
    url_encoder: UrlEncoder,
}
impl OAuth1Handler {
    pub fn new(
        oauth_consumer_key: &str,
        oauth_consumer_secret: &str,
        oauth_token: &str,
        oauth_token_secret: &str,
        oauth_signature_method: &str,
    ) -> Self {
        let mut url_encoder = UrlEncoder::new();
        url_encoder
            .regist_non_encode('*')
            .regist_non_encode('-')
            .regist_non_encode('.')
            .regist_non_encode('_');
        Self {
            oauth_consumer_key: oauth_consumer_key.to_string(),
            oauth_consumer_secret: oauth_consumer_secret.to_string(),
            oauth_token: oauth_token.to_string(),
            oauth_token_secret: oauth_token_secret.to_string(),
            oauth_signature_method: oauth_signature_method.to_string(),
            url_encoder,
        }
    }
    pub async fn post(
        &self,
        endpoint: &str,
        headers: Option<HeaderMap>,
        body: Option<&str>,
    ) -> Result<String> {
        let auth_header = self.get_request_header(endpoint);
        let headers = match headers {
            Some(mut headers) => {
                headers.insert(AUTHORIZATION, auth_header.parse().unwrap());
                headers
            }
            None => {
                let mut headers = HeaderMap::new();
                headers.insert(AUTHORIZATION, auth_header.parse().unwrap());
                headers
            }
        };
        reqwest::Client::new()
            .post(endpoint)
            .headers(headers)
            .body(body.unwrap_or("").to_string())
            .send()
            .await?
            .text()
            .await
    }
    fn get_request_header(&self, endpoint: &str) -> String {
        let oauth_nonce = &format!("nonce{}", Utc::now().timestamp());
        let oauth_timestamp = &format!("{}", Utc::now().timestamp());
        let oauth_version = "1.0";
        let mut params = BTreeMap::new();
        params.insert("oauth_nonce", oauth_nonce.as_str());
        params.insert("oauth_signature_method", &self.oauth_signature_method);
        params.insert("oauth_timestamp", &oauth_timestamp);
        params.insert("oauth_version", &oauth_version);
        params.insert("oauth_consumer_key", &self.oauth_consumer_key);
        params.insert("oauth_token", &self.oauth_token);
        let oauth_signature = self.create_oauth_signature(endpoint, &params);
        params.insert("oauth_signature", &oauth_signature);
        let mut header_auth = params
            .iter()
            .fold(String::from("OAuth "), |acc, (param, value)| {
                format!(r#"{}{}="{}", "#, acc, param, self.url_encoder.encode(value))
            });
        header_auth.pop();
        header_auth.pop();
        header_auth
    }
    fn create_oauth_signature(
        &self,
        endpoint: &str,
        oauth_header: &BTreeMap<&str, &str>,
    ) -> String {
        let consumer_secret_encoded = self.url_encoder.encode(&self.oauth_consumer_secret);
        let token_secret_encoded = self.url_encoder.encode(&self.oauth_token_secret);
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
        let endpoint_encoded = self.url_encoder.encode(&endpoint);
        let params_encoded = self.url_encoder.encode(params);
        let message = format!(
            "{}&{}&{}",
            http_method_encoded, endpoint_encoded, params_encoded
        );
        let hash = hmacsha1::hmac_sha1(key.as_bytes(), message.as_bytes());
        encode_from_byte(&hash)
    }
}
#[derive(Clone, Debug)]
pub struct OAuth1RequestTokenFetcher {
    endpoint: String,
    oauth_consumer_key: String,
    oauth_consumer_secret: String,
    oauth_callback: String,
    oauth_signature_method: String,
    url_encoder: UrlEncoder,
}
impl OAuth1RequestTokenFetcher {
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
            url_encoder,
        }
    }
    async fn create_oauth1_authorizer(self, authorizer_endpoint: &str) -> Result<OAuth1Authorizer> {
        let request_token = self.fetch_request_token().await?;
        Ok(OAuth1Authorizer {
            endpoint: authorizer_endpoint.to_string(),
            oauth_callback: self.oauth_callback,
            oauth_token: request_token.oauth_token,
            url_encoder: self.url_encoder,
        })
    }
    pub async fn create_oauth1_handler(self) -> Result<OAuth1Handler> {
        let request_token = self.fetch_request_token().await?;
        Ok(OAuth1Handler {
            oauth_consumer_key: self.oauth_consumer_key,
            oauth_consumer_secret: self.oauth_consumer_secret,
            oauth_token: request_token.oauth_token,
            oauth_token_secret: request_token.oauth_token_secret,
            oauth_signature_method: self.oauth_signature_method,
            url_encoder: self.url_encoder,
        })
    }
    pub async fn fetch_request_token(&self) -> Result<OAuth1RequestToken> {
        let header_auth = self.get_request_token_header();
        let header = self.gen_header(&header_auth);
        let client = reqwest::Client::new();
        let response = client
            .post(&self.endpoint)
            .headers(header)
            .send()
            .await?
            .text()
            .await?;
        let mut response = response.split('&').map(|s| {
            let mut splited = s.split('=');
            splited.next();
            splited.next().expect(&format!("error {}", response))
        });
        Ok(OAuth1RequestToken {
            oauth_token: response.next().unwrap().to_string(),
            oauth_token_secret: response.next().unwrap().to_string(),
            oauth_callback_confirmed: response.next().unwrap().to_string(),
        })
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
    fn create_oauth_signature(&self, oauth_header: &BTreeMap<&str, &str>) -> String {
        let consumer_secret_encoded = self.url_encoder.encode(&self.oauth_consumer_secret);
        let token_secret_encoded = self.url_encoder.encode("");
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
    fn gen_header(&self, header_auth: &str) -> HeaderMap {
        let mut header = HeaderMap::new();
        header.insert(AUTHORIZATION, header_auth.parse().unwrap());
        header.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );
        header
    }
}

#[derive(Clone, Debug)]
pub struct OAuth1RequestToken {
    pub oauth_token: String,
    pub oauth_token_secret: String,
    pub oauth_callback_confirmed: String,
}

#[derive(Clone, Debug)]
pub struct OAuth1AccessToken {
    pub oauth_token: String,
    pub oauth_token_secret: String,
}
