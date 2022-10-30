use tools::auto_type_define::json_to_struct::JsonStructBuilder;

fn main() {
    let json = r#"
    {
        "to": "U4af4980629...",
        "messages":[
            {
                "type":"text",
                "text":"Hello, world1"
            },
            {
                "type":"text",
                "text":"Hello, world2"
            }
        ]
    }"#;
    let file_path = "./apis/line/src/apis/requests/send_message.rs";
    let root_name = "LineSendMessage";
    let mut builder = JsonStructBuilder::new(root_name);
    builder
        .add_derive("Debug")
        .add_derive("Clone")
        .set_require(root_name, "data")
        .set_require(root_name, "meta")
        .set_all_pub();
    builder.from_json_example_to_file(json, file_path).unwrap();
}
