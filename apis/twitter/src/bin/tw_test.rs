use reqwest::Result;
use twitter::apis::{
    client::TwitterClient, oauth1::TwitterOAuth1Handler, query_builder::SearchQueryBuilder,
};

#[tokio::main]
async fn main() -> Result<()> {
    let client = TwitterClient::from_env().await?;
    //let query = "浜辺美波"; tweet

    //println!("{:#?}", client.tweet("hello").await?);

    //let mut query_builder = SearchQueryBuilder::new(query);
    //query_builder
    //.add_hash()
    //.add_entities_filed()
    //.add_max_results(10);
    //let query = query_builder.build_query();
    //let data = client.search_rec(query, 5).await?;
    Ok(())
}
