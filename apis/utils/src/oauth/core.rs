use std::collections::{BTreeMap, HashMap};

use chrono::Utc;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Result,
};

use crate::{base64::core::encode_from_byte, url_encode::core::UrlEncoder};

use super::{
    authorizer::OAuth1Authorizer,
    util::{gen_timestamp, SIGNATURE_METHOD},
};

#[derive(Clone, Debug)]
pub struct OAuth1Handler {
    oauth_consumer_key: String,
    oauth_consumer_secret: String,
    oauth_token: String,
    oauth_token_secret: String,
    oauth_signature_method: &'static str,
    url_encoder: UrlEncoder,
}
impl OAuth1Handler {
    pub fn new(
        oauth_consumer_key: &str,
        oauth_consumer_secret: &str,
        oauth_token: &str,
        oauth_token_secret: &str,
    ) -> Self {
        let url_encoder = UrlEncoder::for_oauth();
        Self {
            oauth_consumer_key: oauth_consumer_key.to_string(),
            oauth_consumer_secret: oauth_consumer_secret.to_string(),
            oauth_token: oauth_token.to_string(),
            oauth_token_secret: oauth_token_secret.to_string(),
            oauth_signature_method: SIGNATURE_METHOD,
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
        let url_encoder = UrlEncoder::for_oauth();
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
            oauth_signature_method: SIGNATURE_METHOD,
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
struct OAuth1Signature;
impl OAuth1Signature {
    fn to_string(self) -> String {
        format!("")
    }
}
pub struct OAuth1 {
    consumer_key: String,
    consumer_secret: String,
    access_token: String,
    access_token_secret: Option<String>,
    url_encoder: UrlEncoder,
}
impl OAuth1 {
    pub fn new(consumer_key: &str, consumer_secret: &str) -> Self {
        Self {
            consumer_secret: consumer_secret.to_string(),
            consumer_key: consumer_key.to_string(),
            url_encoder: UrlEncoder::for_oauth(),
            access_token: "".to_string(),
            access_token_secret: None,
        }
    }
    pub async fn fetch_request_token(
        &self,
        endpoint: &str,
        custom_params: Option<BTreeMap<&str, &str>>,
    ) -> Result<OAuth1RequestToken> {
        let header_auth = self.fetch_request_token_header(endpoint, custom_params);
        let header = self.create_auth_header(&header_auth);
        let client = reqwest::Client::new();
        let response = client
            .post(endpoint)
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
    fn fetch_request_token_header(
        &self,
        endpoint: &str,
        custom_params: Option<BTreeMap<&str, &str>>,
    ) -> String {
        let mut params = BTreeMap::new();
        let timestamp = Self::create_timestamp_param();
        let nonce = Self::create_nonce_param(&timestamp);
        if let Some(mut custom_params) = custom_params {
            params.append(&mut custom_params)
        }
        params.insert("oauth_nonce", nonce.as_str());
        params.insert("oauth_signature_method", SIGNATURE_METHOD);
        params.insert("oauth_timestamp", timestamp.as_str());
        params.insert("oauth_version", "1.0");
        params.insert("oauth_consumer_key", &self.consumer_key);
        let signature = self.create_oauth_signature(endpoint, &params);
        params.insert("oauth_signature", &signature);
        self.params_to_authrization_header(params)
    }
    fn params_to_authrization_header(&self, params: BTreeMap<&str, &str>) -> String {
        let mut header_auth = params
            .iter()
            .fold(String::from("OAuth "), |acc, (param, value)| {
                format!(
                    r#"{}{}="{}", "#,
                    acc,
                    self.url_encoder.encode(param),
                    self.url_encoder.encode(value)
                )
            });
        header_auth.pop();
        header_auth.pop();
        header_auth
    }
    fn create_oauth_signature(&self, endpoint: &str, params: &BTreeMap<&str, &str>) -> String {
        let consumer_secret_encoded = self.url_encoder.encode(&self.consumer_secret);
        let token_secret_encoded = self.url_encoder.encode(&self.access_token);
        let key = format!("{}&{}", consumer_secret_encoded, token_secret_encoded);
        let params = params.iter().fold(String::new(), |acc, (k, v)| {
            format!(
                "{}{}={}&",
                acc,
                self.url_encoder.encode(k),
                self.url_encoder.encode(v)
            )
        });
        let params = &params[..params.len() - 1];
        let http_method_encoded = self.url_encoder.encode("POST");
        let endpoint_encoded = self.url_encoder.encode(endpoint);
        let params_encoded = self.url_encoder.encode(params);
        let message = format!(
            "{}&{}&{}",
            http_method_encoded, endpoint_encoded, params_encoded
        );
        let hash = hmacsha1::hmac_sha1(key.as_bytes(), message.as_bytes());
        encode_from_byte(&hash)
    }
    fn set_access_token(&mut self, access_token: String) {
        self.access_token = access_token
    }
    fn create_auth_header(&self, auth_header: &str) -> HeaderMap {
        let mut header = HeaderMap::new();
        header.insert(AUTHORIZATION, auth_header.parse().unwrap());
        header.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );
        header
    }
    fn create_nonce_param(timestamp: &str) -> String {
        format!("nonce{}", timestamp)
    }
    fn create_timestamp_param() -> String {
        gen_timestamp()
    }
}

mod oauth_test {
    use super::*;
    #[test]
    fn fetch_request_token_header_test() {
        //this secret is example at https://developer.twitter.com/ja/docs/authentication/oauth-1-0a/creating-a-signature
        let consumer_key = "xvz1evFS4wEEPTGEFPHBog";
        let consumer_secret = "kAcSOqF21Fu85e7zjz7ZN2U4ZRhfV3WpwPAoE3Z7kBw";
        let oauth = OAuth1::new(consumer_key, consumer_secret);
        assert_eq!(
            oauth.fetch_request_token_header("https://test", None),
            r#"OAuth oauth_consumer_key="xvz1evFS4wEEPTGEFPHBog", oauth_nonce="nonce1600000000", oauth_signature="7rOW2w89OgncjDmosV0lASI0ChA%3D", oauth_signature_method="HMAC-SHA1", oauth_timestamp="1600000000", oauth_version="1.0""#.to_string()
        )
    }
}
