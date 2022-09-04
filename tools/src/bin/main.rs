use tools::auto_type_define::json_to_struct::JsonStructBuilder;

fn main() {
    let file_path = "apis/twitter/src/apis/responses/search.rs";
    let root_name = "TwitterSearchResponse";
    let mut builder = JsonStructBuilder::new(root_name);
    builder
        .add_derive("Debug")
        .add_derive("Clone")
        .set_require(root_name, "data")
        .set_require(root_name, "meta")
        .set_all_pub();

    let source = r#"{
      "data":[
        {
          "author_id": "110578919",
          "in_reply_to_user_id": "13524182",
          "created_at": "2020-06-24T17:19:03.000Z",
          "entities": {
            "hashtags": [
              {
                "start": 0,
                "end": 4,
                "tag": "NYC"
              }
            ],
            "mentions": [
              {
                "start": 3,
                "end": 13,
                "username": "zerohedge"
              }
            ],
            "urls": [
              {
                "start": 68,
                "end": 91,
                "url": "https://t.co/wqV97PbtF9",
                "expanded_url": "https://www.zerohedge.com/political/de-blasio-considers-laying-22000-nyc-employees",
                "display_url": "zerohedge.com/political/de-b…",
                "images": [
                  {
                    "url": "https://pbs.twimg.com/news_img/1275839982771539968/xZNipbjm?format=jpg&name=orig",
                    "width": 650,
                    "height": 447
                  },
                  {
                    "url": "https://pbs.twimg.com/news_img/1275839982771539968/xZNipbjm?format=jpg&name=150x150",
                    "width": 150,
                    "height": 150
                  }
                ],
                "status": 200,
                "title": "De Blasio Considers Laying Off 22,000 NYC Employees ",
                "description": "\"The city may have to lay off workers...\"",
                "unwound_url": "https://www.zerohedge.com/political/de-blasio-considers-laying-22000-nyc-employees"
              }
            ]
          },
          "id": "1275840876997050369",
          "lang": "en",
          "possibly_sensitive": false,
          "referenced_tweets": [
            {
              "type": "retweeted",
              "id": "1275839979772665861"
            }
          ],
          "source": "Twitter for iPhone",
          "text": "RT @zerohedge: De Blasio Considers Laying Off 22,000 NYC Employees  https://t.co/wqV97PbtF9"
        }
      ],
      "meta": {
        "newest_id": "1275840892285399041",
        "oldest_id": "1275840875466305542",
        "result_count": 10,
        "next_token": "b26v89c19zqg8o3fo7gesq314zlbjb2xlwutmy72r47lp"
      }
    }
    "#;
    let _ = builder.from_json_example_to_file(source, file_path);
}
