use std::collections::HashMap;

use reqwest::header::AUTHORIZATION;

pub struct DeepLClient {
    inner: reqwest::Client,
    auth_key_header_value: String,
}

impl DeepLClient {
    const TRANCELATE_URL: &'static str = "https://api-free.deepl.com/v2/translate";
    pub fn new(auth_key: impl Into<String>) -> Self {
        let client = reqwest::Client::new();
        let auth_key_header_value = format!("DeepL-Auth-Key {}", auth_key.into());
        Self {
            inner: client,
            auth_key_header_value,
        }
    }
    pub fn from_env() -> Result<Self, std::env::VarError> {
        let key = std::env::var("DEEPL_AUTH_KEY")?;
        Ok(Self::new(key))
    }
    pub async fn translate(
        &self,
        source: DeepLSource,
        to: impl Into<DeepLTranlatableLang>,
    ) -> Result<String, String> {
        let to = to.into();
        if source.can_translate(to) {
            let mut form = HashMap::new();
            form.insert("text", source.source.as_str());
            let to = to.to_api_string();
            form.insert("target_lang", to.as_str());
            let response = self
                .inner
                .post(Self::TRANCELATE_URL)
                .header(AUTHORIZATION, &self.auth_key_header_value)
                .form(&form)
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            let response = serde_json::from_str::<DeepLTranslatedResults>(&response).unwrap();
            return Ok(response
                .translations
                .into_iter()
                .fold(String::new(), |acc, cur| format!("{}{}\n", acc, cur.text)));
        }
        Err(String::from("todo"))
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeepLTranslatedResults {
    translations: Vec<DeepLTranslatedResult>,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeepLTranslatedResult {
    detected_source_language: String,
    text: String,
}

pub struct DeepLSource {
    lang: DeepLTranlatableLang,
    source: String,
}
impl DeepLSource {
    pub fn new(lang: impl Into<DeepLTranlatableLang>, source: impl Into<String>) -> Self {
        Self {
            lang: lang.into(),
            source: source.into(),
        }
    }
    fn can_translate(&self, to: impl Into<DeepLTranlatableLang>) -> bool {
        let to = to.into();
        DeepLTranlatableLang::can_translate(self.lang, to)
    }
}
#[derive(Debug, Clone, Copy)]
pub enum DeepLTranlatableLang {
    English,
    German,
    French,
    Spanish,
    Japanese,
    Italian,
    Polish,
    Dutch,
}
impl DeepLTranlatableLang {
    fn to_api_string(self) -> String {
        self.into()
    }
    fn can_translate(from: Self, to: Self) -> bool {
        match (from, to) {
            (Self::English, Self::German)
            | (Self::German, Self::English)
            | (Self::English, Self::French)
            | (Self::English, Self::Spanish)
            | (Self::English, Self::Japanese)
            | (Self::Japanese, Self::English)
            | (Self::English, Self::Italian)
            | (Self::Italian, Self::English)
            | (Self::English, Self::Polish)
            | (Self::Polish, Self::English)
            | (Self::English, Self::Dutch)
            | (Self::Dutch, Self::English)
            | (Self::German, Self::French)
            | (Self::French, Self::German) => true,
            _ => false,
        }
    }
}

impl<T> From<T> for DeepLTranlatableLang
where
    T: AsRef<str>,
{
    fn from(s: T) -> Self {
        match s.as_ref() {
            "EN" => Self::English,
            "IT" => Self::Italian,
            "JA" => Self::Japanese,
            "NL" => Self::Dutch,
            "DE" => Self::German,
            "FR" => Self::French,
            "ES" => Self::Spanish,
            "PL" => Self::Polish,
            _ => panic!("{} is not compatible at deepl", s.as_ref()),
        }
    }
}

impl Into<String> for DeepLTranlatableLang {
    fn into(self) -> String {
        match self {
            Self::English => "EN".to_string(),
            Self::Italian => "IT".to_string(),
            Self::Japanese => "JA".to_string(),
            Self::Dutch => "NL".to_string(),
            Self::German => "DE".to_string(),
            Self::French => "FR".to_string(),
            Self::Spanish => "ES".to_string(),
            Self::Polish => "PL".to_string(),
        }
    }
}
