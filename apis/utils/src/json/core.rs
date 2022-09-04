use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum JSON {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<JSON>),
    Object(HashMap<String, JSON>),
}

impl JSON {
    pub fn parse(source: &str) -> Self {
        JSON::Null
    }
}

#[cfg(test)]
mod json_test {
    use super::*;
    #[test]
    fn test_parse() {
        let source = r#"
            {
                "data":[
                    {
                        "id":103,
                        "name":"kai",
                    }
                ]
            }
        "#;
        let tobe = JSON::Array(vec![
            (JSON::Object(
                [
                    (String::from("name"), JSON::String(String::from("kai"))),
                    (String::from("id"), JSON::Number(103_f64)),
                ]
                .into_iter()
                .collect(),
            )),
        ]);
        let tobe = JSON::Object([(String::from("data"), tobe)].into_iter().collect());
        println!("{:#?}", tobe);
        assert_eq!(JSON::parse(source), tobe);
    }
}
