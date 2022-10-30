use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LineSendMessage {
    pub messages: Option<Vec<LineSendMessageMessages>>,
    pub to: Option<String>,
}
impl LineSendMessage {
    pub fn new(message: String, to: String) -> Self {
        let message = LineSendMessageMessages {
            text: Some(message),
            r#type: Some("text".to_string()),
        };
        Self {
            messages: Some(vec![message]),
            to: Some(to),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LineSendMessageMessages {
    pub text: Option<String>,
    pub r#type: Option<String>,
}
