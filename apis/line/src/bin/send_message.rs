use line::apis::requests::send_message::LineSendMessage;
use reqwest::header::CONTENT_TYPE;

#[tokio::main]
async fn main() {
    let user_id = std::env::var("LINE_USER_ID").unwrap();
    let secret = std::env::var("LINE_CHANEL_ACCESS_TOKEN").unwrap();
    let message = LineSendMessage::new("hello".to_string(), user_id);
    let message = serde_json::to_string(&message).unwrap();
    let text = reqwest::Client::new()
        .post("https://api.line.me/v2/bot/message/push")
        .bearer_auth(secret)
        .body(message)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("{}", text);
}
