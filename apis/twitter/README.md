# Twitter API

- use twitter api
- you must register twitter api

# Example

## search

```rust
use reqwest::Result;
use twitter::apis::{client::TwitterClient, query::query_builder::SearchQueryBuilder};

#[tokio::main]
async fn main() -> Result<()> {
    let client = TwitterClient::from_env().await?;
    let query = "浜辺美波";
    let mut query_builder = SearchQueryBuilder::new(query);
    query_builder
        .add_hash()
        .add_entities_filed()
        .add_max_results(10);
    let query = query_builder.build_query();
    let data = client.search_rec(query, 5).await?;
    println!("{:#?}", data);
    Ok(())
}
```

## twieet

- twieet api is use access_token and access_token_secret
- twieet is use oauth1

```rust
use reqwest::Result;
use twitter::apis::{client::TwitterClient, query::query_builder::SearchQueryBuilder};

#[tokio::main]
async fn main() -> Result<()> {
    let client = TwitterClient::from_env().await?;
    client.tweet("rust lang is very nice!!").await?;
    Ok(())
}
```
