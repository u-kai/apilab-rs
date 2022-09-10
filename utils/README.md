# Utils

## oauth

- oauth is impl OAuth1.

```rust
fn main(){
    let mut oauth1 = OAuth1::new("consumer_key","consumer_secret_key");
    oauth1.set_secret_by_oauth_session();
    // please enter your shell

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    let body = Some(r#"{"text":"good!"}"#);
    let responce = oauth
        .post("https://api.twitter.com/2/tweets", Some(headers), body)
        .await?;
    println!("response {}", responce);
}
```
