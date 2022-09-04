use serde_json::{Result, Value};

pub struct JsonStructBuilder {
    derive: String,
    struct_name: String,
    child_struct_buffer: Vec<Box<JsonStructBuilder>>,
}
impl JsonStructBuilder {
    pub fn new(derives: Vec<&str>, struct_name: impl Into<String>) -> Self {
        Self {
            derive: derives.join(",").to_string(),
            struct_name: struct_name.into(),
            child_struct_buffer: Vec::new(),
        }
    }
    fn from_derive(derive: impl Into<String>, struct_name: impl Into<String>) -> Self {
        Self {
            derive: derive.into(),
            struct_name: struct_name.into(),
            child_struct_buffer: Vec::new(),
        }
    }
    pub fn from_json_example(&mut self, source: &str) -> Result<String> {
        let json_value = Self::json_to_object(source)?;
        let s = match json_value {
            Value::Object(object) => {
                let init_string = format!(
                    "{}\nstruct {} {{\n\t",
                    self.derive_statement(),
                    self.struct_name,
                );
                let mut result = object.keys().fold(init_string, |acc, key| {
                    let child_object = object.get(key).unwrap();
                    let child_object_value = match child_object {
                        Value::Object(object) => {
                            let child_object_struct_name = key
                                .chars()
                                .enumerate()
                                .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
                                .collect::<String>();
                            let child_builder =
                                Self::from_derive(&self.derive, &child_object_struct_name);
                            self.child_struct_buffer.push(Box::new(child_builder));
                            child_object_struct_name
                        }
                        Value::String(_) => self.case_string(),
                        Value::Null => self.case_null(),
                        Value::Bool(_) => self.case_bool(),
                        Value::Number(_) => self.case_number(),
                        _ => todo!("impl not yet"),
                    };
                    format!("{}{}: {},\n\t", acc, key, child_object_value)
                });
                result.pop();
                format!("{}}}", result)
            }
            Value::String(_) => self.case_string(),
            _ => {
                todo!("impl not yet")
            }
        };
        Ok(s)
    }
    fn json_to_object(source: &str) -> Result<Value> {
        let json: Value = serde_json::from_str(source)?;
        Ok(json)
    }
    fn derive_statement(&self) -> String {
        format!("#[derive({})]", self.derive)
    }
    fn case_null(&self) -> String {
        String::from("Option<null>")
    }
    fn case_bool(&self) -> String {
        String::from("Option<bool>")
    }
    fn case_number(&self) -> String {
        String::from("Option<f64>")
    }
    fn case_string(&self) -> String {
        String::from("Option<String>")
    }
}
pub fn json_define_to_struct(source: &str) -> Result<String> {
    let json: Value = serde_json::from_str(source)?;
    Ok(String::new())
}
#[cfg(test)]
mod json_define_to_struct {
    use super::*;
    const FIELD_SPACE: &str = "\n\t";
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
        let mut builder = JsonStructBuilder::new(vec!["Serialize", "Desrialize"], struct_name);
        println!("{:#?}", JsonStructBuilder::json_to_object(flat_json));
        assert_eq!(builder.from_json_example(flat_json).unwrap(), tobe);
    }
    //#[test]
    //fn test_json_define_to_struct() {
    //let json = r#"{
    //"data":[
    //{
    //"test":"test_value"
    //}
    //]
    //}"#;
    //let struct_name = "TestJson";
    //let tobe = r#"#[derive(Serialize, Deserialize)]
    //struct TestJson {
    //data:Vec<TestJsonData>
    //}
    //#[derive(Serialize,Desialize)]
    //struct TestJsonData {
    //test:Option<String>
    //}
    //"#
    //.to_string();
    //let builder = JsonStructBuilder::new(vec!["Serialize", "Deserialize"], struct_name);
    //println!("{:#?}", JsonStructBuilder::json_to_value(json));
    //assert_eq!(builder.from_json_example(json).unwrap(), tobe);
    //}
}
