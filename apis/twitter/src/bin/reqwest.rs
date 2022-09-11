use std::{
    fs::File,
    io::{BufWriter, Write},
};

use reqwest::Result;
use twitter::apis::{client::TwitterClient, query::query_builder::SearchQueryBuilder};

#[tokio::main]
async fn main() -> Result<()> {
    let twitter = TwitterClient::from_env().await?;
    let mut query_builder = SearchQueryBuilder::new("浜辺美波");
    query_builder
        .add_entities_filed()
        .add_max_results(100)
        .add_hash();
    let query = query_builder.build_query();
    let _ = twitter
        .search_rec(query, 10, |f| println!("{:?}", f.image_urls()))
        .await?;
    //let images = data.image_urls();
    //let client = reqwest::Client::new();
    //for (i, image) in images.iter().enumerate() {
    //let mut response = client.get(*image).send().await?.bytes().await?;
    //let mut file = BufWriter::new(File::create(format!("pngs/{}.png", i)).unwrap());
    //let _ = file.write_all(&mut response).unwrap();
    //}
    Ok(())
}
