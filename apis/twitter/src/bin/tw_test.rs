use reqwest::Result;
use twitter::apis::client::TwitterClient;

#[tokio::main]
async fn main() -> Result<()> {
    //let client = TwitterClient::from_env().await?;
    //let response = client
    //.search_rec_with_hash("浜辺美波&tweet.fields=entities&max_results=10", 5)
    //.await?;
    Ok(())
}
