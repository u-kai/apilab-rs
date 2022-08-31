use std::collections::HashMap;

enum JsonType {
    String,
    Number,
    Array(Vec<JsonType>),
    Objcet(HashMap<String, JsonType>),
    Boolean,
    Null,
}
#[cfg(test)]
mod json_define_to_struct {
    #[test]
    fn test_json_define_to_struct() {
        let json = r#"{
            "test":"test_value"
        }"#;
        let struct_name = "TestJson";
        let tobe = r#"#[derive(Serialize, Deserialize)]
struct TestJson {
    test:String
}
"#
        .to_string();
        //assert!(false);
        //assert_eq!(parse_and_define_struct_result, tobe);
    }
}
