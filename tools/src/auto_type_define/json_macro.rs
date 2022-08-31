use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}
impl From<bool> for Json {
    fn from(b: bool) -> Self {
        Json::Boolean(b)
    }
}
impl<'a> From<&'a str> for Json {
    fn from(s: &'a str) -> Self {
        Json::String(s.to_string())
    }
}
#[macro_export]
macro_rules! json {
    (null) => {
        $crate::auto_type_define::json_macro::Json::Null
    };
    ([$($elem:tt),*]) => {
        $crate::auto_type_define::json_macro::Json::Array(vec![$(json!($elem)),*])
    };
    ({ $( $key:tt : $value:tt ), *}) => {
       $crate::auto_type_define::json_macro::Json::Object(Box::new(vec![
            $( (stringify!($key).to_string(),json!($value)) ),*
        ].into_iter().collect()))
    };
    ($other:tt) => {
        $crate::auto_type_define::json_macro::Json::from($other)
    };
}

macro_rules! impl_from_num_for_json {
    ($($t:ident), *) => {
    $(
        impl From<$t> for Json {
            fn from(n:$t)->Json{
                Json::Number(n as f64)
            }
        }
    )*
    };
}
impl_from_num_for_json!(i8, u8, i16, u16, i32, u32, f32, i64, u64, f64, i128, u128, isize, usize);
#[cfg(test)]
mod macro_json_test {
    use super::*;
    #[test]
    fn json_null_test() {
        assert_eq!(json!(null), Json::Null)
    }
    #[test]
    fn json_array_test() {
        let mut data = HashMap::new();
        data.insert("name".to_string(), Json::String("kai".to_string()));
        data.insert("age".to_string(), Json::Number(26_f64));
        assert_eq!(
            json!([{name:"kai",age:26}]),
            Json::Array(vec![Json::Object(Box::new(data))])
        )
    }
    #[test]
    fn json_num_test() {
        let mut data = HashMap::new();
        data.insert("name".to_string(), Json::String("kai".to_string()));
        data.insert("age".to_string(), Json::Number(23_f64));
        assert_eq!(
            json!({name:"kai",age:23_i128}),
            Json::Object(Box::new(data))
        )
    }
}
