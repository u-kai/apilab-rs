use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use serde_json::{Map, Result, Value};

#[derive(Debug, Clone)]
struct RequireFiled {
    struct_name: String,
    filed_name: String,
}
impl RequireFiled {
    fn new(struct_name: impl Into<String>, filed_name: impl Into<String>) -> Self {
        Self {
            struct_name: struct_name.into(),
            filed_name: filed_name.into(),
        }
    }
    fn is_require(&self, struct_name: &str, filed_name: &str) -> bool {
        self.struct_name == struct_name && self.filed_name == filed_name
    }
}
pub struct JsonStructBuilder {
    derive: String,
    struct_name: String,
    require_fileds: Vec<RequireFiled>,
    pub_fileds: Option<Vec<String>>,
}

impl JsonStructBuilder {
    pub fn new(struct_name: impl Into<String>) -> Self {
        Self {
            derive: "Serialize,Desrialize".to_string(),
            struct_name: struct_name.into(),
            require_fileds: Vec::new(),
            pub_fileds: None,
        }
    }
    fn inherit_option_fileds(
        &self,
        derive: impl Into<String>,
        struct_name: impl Into<String>,
    ) -> Self {
        Self {
            derive: derive.into(),
            struct_name: struct_name.into(),
            require_fileds: self.require_fileds.clone(),
            pub_fileds: self.pub_fileds.clone(),
        }
    }
    pub fn new_with_drives(derives: Vec<&str>, struct_name: impl Into<String>) -> Self {
        Self {
            derive: derives.join(",").to_string(),
            struct_name: struct_name.into(),
            require_fileds: Vec::new(),
            pub_fileds: None,
        }
    }
    pub fn set_require(
        &mut self,
        struct_name: impl Into<String>,
        filed_name: impl Into<String>,
    ) -> &mut Self {
        self.require_fileds
            .push(RequireFiled::new(struct_name, filed_name));
        self
    }
    pub fn from_json_example_to_file(
        &self,
        source: &str,
        file_path: impl AsRef<Path>,
    ) -> Result<()> {
        let string = self.from_json_example(source)?;
        let buf = string.as_bytes();
        let file = File::create(file_path).unwrap();
        let mut writer = BufWriter::new(file);
        let _ = writer.write_all(buf);
        Ok(())
    }
    pub fn from_json_example(&self, source: &str) -> Result<String> {
        let json_value = Self::json_to_value(source)?;
        let mut child_buffer = Vec::new();
        let s = match json_value {
            Value::Object(object) => self.case_object(&object, &mut child_buffer),
            Value::String(_) => self.case_string(None),
            Value::Array(array) => self.case_array_with_key("", &array, &mut child_buffer),
            Value::Null => self.case_null(),
            Value::Bool(_) => self.case_bool(None),
            Value::Number(_) => self.case_number(None),
        };
        let s = child_buffer
            .iter()
            .rev()
            .fold(s, |acc, cur| format!("{}\n{}", acc, cur));
        Ok(s)
    }
    fn is_require(&self, filed_name: &str) -> bool {
        self.require_fileds
            .iter()
            .any(|req| req.is_require(&self.struct_name, filed_name))
    }
    fn case_object(&self, object: &Map<String, Value>, child_buffer: &mut Vec<String>) -> String {
        let mut object_string = self.struct_statement();
        for key in object.keys() {
            let child_object = object.get(key).unwrap();
            let child_object_value = match child_object {
                Value::Object(object) => {
                    let child_struct_name = self.key_to_struct_name(key);
                    let child_builder =
                        self.inherit_option_fileds(&self.derive, &child_struct_name);
                    let child_string = child_builder.case_object(object, child_buffer);
                    child_buffer.push(child_string);
                    if self.is_require(key) {
                        child_struct_name
                    } else {
                        format!("Option<{}>", child_struct_name)
                    }
                }
                Value::Array(array) => self.case_array_with_key(key, array, child_buffer),
                Value::String(_) => self.case_string(Some(key)),
                Value::Null => self.case_null(),
                Value::Bool(_) => self.case_bool(Some(key)),
                Value::Number(_) => self.case_number(Some(key)),
            };
            object_string = format!(
                "{}{}: {}{}",
                object_string,
                key,
                child_object_value,
                Self::field_derimita()
            )
        }
        let result = format!("{}}}", &object_string[..(object_string.len() - 4)]);
        result
    }
    fn case_array_with_key(
        &self,
        key: &str,
        array: &Vec<Value>,
        child_buffer: &mut Vec<String>,
    ) -> String {
        if array.len() == 0 {
            println!(
                "{} can not define. because array is empty ",
                self.struct_name
            );
            return String::new();
        }
        if key == "" {
            todo!("not impl yet")
        }
        let represent = &array[0];
        match represent {
            Value::Object(object) => {
                let struct_name = self.key_to_struct_name(key);
                let builder = self.inherit_option_fileds(&self.derive, &struct_name);
                let string = builder.case_object(object, child_buffer);
                child_buffer.push(string);
                if self.is_require(key) {
                    format!("Vec<{}>", struct_name)
                } else {
                    format!("Option<Vec<{}>>", struct_name)
                }
            }
            Value::Array(array) => {
                self.case_array_with_key(&format!("Vec<{}>", key), array, child_buffer)
            }
            Value::Null => self.case_null(),
            Value::Bool(_) => {
                if self.is_require(key) {
                    format!("Vec<bool>")
                } else {
                    String::from("Option<Vec<bool>>")
                }
            }
            Value::String(_) => {
                if self.is_require(key) {
                    format!("Vec<String>")
                } else {
                    String::from("Option<Vec<String>>")
                }
            }
            Value::Number(_) => {
                if self.is_require(key) {
                    format!("Vec<f64>")
                } else {
                    String::from("Option<Vec<f64>>")
                }
            }
        }
    }
    fn case_null(&self) -> String {
        String::new()
    }
    fn case_bool(&self, key: Option<&str>) -> String {
        match key {
            Some(key) if self.is_require(key) => {
                format!("bool")
            }
            _ => String::from("Option<bool>"),
        }
    }
    fn case_number(&self, key: Option<&str>) -> String {
        println!("key {:?} ", key);
        println!("self.is {:?} ", &self.struct_name);
        println!("self.fileds {:?} ", &self.require_fileds);
        match key {
            Some(key) if self.is_require(key) => {
                format!("f64")
            }
            _ => String::from("Option<f64>"),
        }
    }
    fn case_string(&self, key: Option<&str>) -> String {
        match key {
            Some(key) if self.is_require(key) => {
                format!("String")
            }
            _ => String::from("Option<String>"),
        }
    }
    fn json_to_value(source: &str) -> Result<Value> {
        let json: Value = serde_json::from_str(source)?;
        Ok(json)
    }
    fn key_to_struct_name(&self, key: &str) -> String {
        let child_struct_name = key
            .chars()
            .enumerate()
            .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
            .collect::<String>();
        format!("{}{}", self.struct_name, child_struct_name)
    }
    fn struct_statement(&self) -> String {
        format!(
            "{}\nstruct {} {{\n    ",
            self.derive_statement(),
            self.struct_name,
        )
    }
    fn derive_statement(&self) -> String {
        format!("#[derive({})]", self.derive)
    }
    const fn field_derimita() -> &'static str {
        ",\n    "
    }
}
#[cfg(test)]
mod json_define_to_struct {
    use super::*;
    const FIELD_SPACE: &str = "\n    ";
    #[test]
    fn test_add_require() {
        let complicated_json = r#"
            {
                "data":[
                    {
                        "id":12345,
                        "test":"test-string",
                        "entities":{
                            "id":0
                        }
                    }
                ]
            }
        "#;
        let struct_name = "TestJson";
        let tobe = r#"#[derive(Serialize,Desrialize)]
struct TestJson {
    data: Vec<TestJsonData>,
}
#[derive(Serialize,Desrialize)]
struct TestJsonData {
    entities: Option<TestJsonDataEntities>,
    id: f64,
    test: Option<String>,
}
#[derive(Serialize,Desrialize)]
struct TestJsonDataEntities {
    id: Option<f64>,
}"#
        .to_string();
        let mut builder =
            JsonStructBuilder::new_with_drives(vec!["Serialize", "Desrialize"], struct_name);
        builder
            .set_require("TestJson", "data")
            .set_require("TestJsonData", "id");
        assert_eq!(builder.from_json_example(complicated_json).unwrap(), tobe);
    }
    #[test]
    fn test_from_flat_json_example() {
        let flat_json = r#"
            {
                "id":12345,
                "test":"test-string"
            }
        "#;
        let struct_name = "TestJson";
        let tobe = format!("#[derive(Serialize,Desrialize)]\nstruct {} {{{}id: Option<f64>,{}test: Option<String>,\n}}",struct_name,FIELD_SPACE,FIELD_SPACE);
        let builder =
            JsonStructBuilder::new_with_drives(vec!["Serialize", "Desrialize"], struct_name);
        assert_eq!(builder.from_json_example(flat_json).unwrap(), tobe);
    }
    #[test]
    fn test_from_objected_json_example() {
        let complicated_json = r#"
            {
                "data":
                    {
                        "id":12345,
                        "test":"test-string",
                        "entities":{
                            "id":0
                        }
                    }
                
            }
        "#;
        let struct_name = "TestJson";
        let tobe = r#"#[derive(Serialize,Desrialize)]
struct TestJson {
    data: Option<TestJsonData>,
}
#[derive(Serialize,Desrialize)]
struct TestJsonData {
    entities: Option<TestJsonDataEntities>,
    id: Option<f64>,
    test: Option<String>,
}
#[derive(Serialize,Desrialize)]
struct TestJsonDataEntities {
    id: Option<f64>,
}"#
        .to_string();
        let builder =
            JsonStructBuilder::new_with_drives(vec!["Serialize", "Desrialize"], struct_name);
        assert_eq!(builder.from_json_example(complicated_json).unwrap(), tobe);
    }
    #[test]
    fn test_from_complicated_json_example() {
        let complicated_json = r#"
            {
                "data":[
                    {
                        "id":12345,
                        "test":"test-string",
                        "entities":{
                            "id":0
                        }
                    }
                ]
            }
        "#;
        let struct_name = "TestJson";
        let tobe = r#"#[derive(Serialize,Desrialize)]
struct TestJson {
    data: Option<Vec<TestJsonData>>,
}
#[derive(Serialize,Desrialize)]
struct TestJsonData {
    entities: Option<TestJsonDataEntities>,
    id: Option<f64>,
    test: Option<String>,
}
#[derive(Serialize,Desrialize)]
struct TestJsonDataEntities {
    id: Option<f64>,
}"#
        .to_string();
        let builder =
            JsonStructBuilder::new_with_drives(vec!["Serialize", "Desrialize"], struct_name);
        assert_eq!(builder.from_json_example(complicated_json).unwrap(), tobe);
    }
}
