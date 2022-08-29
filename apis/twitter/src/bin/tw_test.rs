use reqwest::Result;
use twitter::apis::client::TwitterClient;

#[tokio::main]
async fn main() -> Result<()> {
    let client = TwitterClient::from_env().await?;
    let response = client
        .search_rec("#浜辺美波&media.fields=url&max_results=50", 5)
        .await?;
    Ok(())
}
