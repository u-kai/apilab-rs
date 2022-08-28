use reqwest::Result;
use twitter::auth::TwitterCunsmerCredentials;

#[tokio::main]
async fn main() -> Result<()> {
    let t_auth = TwitterCunsmerCredentials::from_env();
    let text = t_auth.get_beare_token().await?;
    println!("{}", text);
    Ok(())
}
