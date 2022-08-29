use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TwitterApiResponseMeta {
    meta: TwitterApiResponseMetaData,
}
impl TwitterApiResponseMeta {
    pub fn new(response_text: &str) -> Result<Self> {
        serde_json::from_str(response_text)
    }
    pub fn next_token(&self) -> &str {
        &self.meta.next_token
    }
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TwitterApiResponseLimitMeta {
    meta: TwitterApiResponseLimitMetaData,
}
impl TwitterApiResponseLimitMeta {
    pub fn new(response_text: &str) -> Result<Self> {
        serde_json::from_str(response_text)
    }

    pub fn is_limit(&self) -> bool {
        self.meta.result_count == 0
    }
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct TwitterApiResponseLimitMetaData {
    result_count: usize,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct TwitterApiResponseMetaData {
    newest_id: String,
    oldest_id: String,
    result_count: usize,
    next_token: String,
}

mod meta_test {
    use crate::apis::responses::example::RESPONSE_EXAMPLE;

    use super::*;
    #[test]
    fn test() {
        let t = serde_json::from_str::<TwitterApiResponseMeta>(RESPONSE_EXAMPLE);
        match t {
            Ok(t) => {
                assert_eq!(
                    t,
                    TwitterApiResponseMeta {
                        meta: TwitterApiResponseMetaData {
                            newest_id: "1275840892285399041".to_string(),
                            oldest_id: "1275840875466305542".to_string(),
                            result_count: 10,
                            next_token: "b26v89c19zqg8o3fo7gesq314zlbjb2xlwutmy72r47lp".to_string()
                        }
                    }
                )
            }
            Err(e) => {
                println!("fjaklsfjals;df{:#?}", e);
                assert!(false)
            }
        }
    }
}
