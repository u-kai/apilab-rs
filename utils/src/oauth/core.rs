use super::util::{gen_timestamp, SIGNATURE_METHOD};
use crate::{base64::core::encode_from_byte, url_encode::core::UrlEncoder};
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Result,
};
use std::{
    collections::BTreeMap,
    io::{stdin, BufRead},
};

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
#[derive(Clone, Debug)]
pub struct OAuth1 {
    consumer_key: String,
    consumer_secret: String,
    access_token: String,
    access_token_secret: Option<String>,
    access_token_verifier: Option<String>,
    url_encoder: UrlEncoder,
}
impl OAuth1 {
    pub fn new(
        consumer_key: String,
        consumer_secret: String,
        access_token: String,
        access_token_secret: String,
    ) -> Self {
        Self {
            consumer_key,
            consumer_secret,
            access_token,
            access_token_secret: Some(access_token_secret),
            access_token_verifier: None,
            url_encoder: UrlEncoder::for_oauth(),
        }
    }
    pub fn new_without_token(consumer_key: &str, consumer_secret: &str) -> Self {
        Self {
            consumer_secret: consumer_secret.to_string(),
            consumer_key: consumer_key.to_string(),
            url_encoder: UrlEncoder::for_oauth(),
            access_token: "".to_string(),
            access_token_verifier: None,
            access_token_secret: None,
        }
    }
    pub fn set_access_token(&mut self, access_token: &str) {
        self.access_token = access_token.to_string();
    }
    pub fn set_access_token_secret(&mut self, access_token_secret: &str) {
        self.access_token_secret = Some(access_token_secret.to_string());
    }
    pub async fn set_secret_by_oauth_session(&mut self, callback_url: &str) -> Result<()> {
        fn get_line() -> String {
            let mut line = String::new();
            let stdin = stdin();
            let mut stdin = stdin.lock();
            let _ = stdin.read_line(&mut line).unwrap();
            let line = line.trim();
            line.to_string()
        }
        println!("please enter request url");
        let request_url = get_line();
        self.fetch_and_set_request_token(request_url.as_str(), None)
            .await?;
        println!("please enter authorize url");
        let authorize_url = get_line();
        println!(
            "click here: {}",
            self.authorization_url(authorize_url.as_str(), callback_url)
        );
        println!("please enter redirect response");
        let redirect_response = get_line();
        self.set_authorization_response(redirect_response.as_str());
        println!("please enter access url");
        let access_url = get_line();
        self.fetch_and_set_access_token(access_url.as_str(), callback_url)
            .await?;
        Ok(())
    }
    pub async fn post(
        &self,
        endpoint: &str,
        headers: Option<HeaderMap>,
        body: Option<&str>,
    ) -> Result<String> {
        let mut add_header = BTreeMap::new();
        add_header.insert("oauth_token", self.access_token.as_str());
        let auth_header = self.create_header_value(endpoint, Some(add_header));
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
    pub async fn fetch_and_set_access_token(
        &mut self,
        endpoint: &str,
        callback_url: &str,
    ) -> Result<()> {
        let mut add_header = BTreeMap::new();
        add_header.insert("oauth_token", self.access_token.as_str());
        add_header.insert(
            "oauth_verifier",
            self.access_token_verifier
                .as_ref()
                .map(|s| s.as_str())
                .unwrap(),
        );
        add_header.insert("oauth_callback", callback_url);
        let headers = self.create_header_value(endpoint, Some(add_header));
        let response = Self::request(endpoint, self.create_auth_header(headers.as_str())).await?;
        let mut response = response
            .split('&')
            .map(|kv| kv.split('=').skip(1).next().unwrap());
        self.access_token = response.next().as_ref().unwrap().to_string();
        self.access_token_secret = Some(response.next().as_ref().unwrap().to_string());
        Ok(())
    }
    pub fn set_authorization_response(&mut self, redirect_response: &str) {
        let params = redirect_response
            .split('?')
            .skip(1)
            .next()
            .expect(&format!("{} is not redirect_response", redirect_response));
        let mut params = params
            .split('&')
            .map(|kv| kv.split('=').skip(1).next().unwrap());
        let fetched_access_token = params.next().unwrap().to_string();
        if self.access_token != fetched_access_token {
            panic!("not same token");
        };
        self.access_token_verifier = Some(params.next().unwrap().to_string());
    }
    pub async fn fetch_and_set_request_token(
        &mut self,
        endpoint: &str,
        custom_params: Option<BTreeMap<&str, &str>>,
    ) -> Result<()> {
        let header_auth = self.create_header_value(endpoint, custom_params);
        let headers = self.create_auth_header(&header_auth);
        let response = Self::request(endpoint, headers).await?;
        let mut response = response.split('&').map(|s| {
            let mut splited = s.split('=');
            splited.next();
            splited.next().expect(&format!("error {}", response))
        });
        let oauth_token = response.next().unwrap().to_string();
        self.access_token = oauth_token;
        Ok(())
    }
    pub fn authorization_url(&self, authorization_url: &str, callback_url: &str) -> String {
        format!(
            "{}?oauth_token={}&oauth_callback={}",
            authorization_url,
            self.url_encoder.encode(&self.access_token),
            self.url_encoder.encode(callback_url)
        )
    }
    async fn request(endpoint: &str, headers: HeaderMap) -> Result<String> {
        let client = reqwest::Client::new();
        client
            .post(endpoint)
            .headers(headers)
            .send()
            .await?
            .text()
            .await
    }
    fn create_header_value(
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
        //params.insert("oauth_callback", callback_url);
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
        let token_secret_encoded = self.url_encoder.encode(
            &self
                .access_token_secret
                .as_ref()
                .map(|s| s.as_str())
                .unwrap_or(""),
        );
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
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OAuth1AuthorizationResponse {
    pub oauth_token: String,
    pub oauth_token_secret: String,
    pub oauth_verifier: String,
}

mod oauth_test {
    #[test]
    fn fetch_request_token_header_test() {
        use super::*;
        //this secret is example at https://developer.twitter.com/ja/docs/authentication/oauth-1-0a/creating-a-signature
        let consumer_key = "xvz1evFS4wEEPTGEFPHBog";
        let consumer_secret = "kAcSOqF21Fu85e7zjz7ZN2U4ZRhfV3WpwPAoE3Z7kBw";
        let oauth = OAuth1::new_without_token(consumer_key, consumer_secret);
        let mut headers = BTreeMap::new();
        headers.insert("oauth_callback", "http://localhost");
        assert_eq!(
            oauth.create_header_value("https://test",Some(headers)),
            r#"OAuth oauth_callback="http%3A%2F%2Flocalhost", oauth_consumer_key="xvz1evFS4wEEPTGEFPHBog", oauth_nonce="nonce1600000000", oauth_signature="WiPE4O+xuu1addbb1tInN5xgyTc%3D", oauth_signature_method="HMAC-SHA1", oauth_timestamp="1600000000", oauth_version="1.0""#.to_string()
        )
    }
    #[test]
    fn authorization_url_test() {
        use super::*;
        let consumer_key = "xvz1evFS4wEEPTGEFPHBog";
        let consumer_secret = "kAcSOqF21Fu85e7zjz7ZN2U4ZRhfV3WpwPAoE3Z7kBw";
        let oauth = OAuth1::new_without_token(consumer_key, consumer_secret);
        assert_eq!(
            oauth.authorization_url("https://test", "http://localhost"),
            r#"https://test?oauth_token=&oauth_callback=http%3A%2F%2Flocalhost"#.to_string()
        )
    }
}
