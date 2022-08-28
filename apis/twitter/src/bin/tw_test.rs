use reqwest::Result;
use twitter::apis::client::TwitterClient;

#[tokio::main]
async fn main() -> Result<()> {
    let client = TwitterClient::from_env().await?;
    let covid_19 = client.search_hash("rust").await?;
    println!("{:#?}", covid_19);
    Ok(())
}
