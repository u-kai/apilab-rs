use super::url_encode_map::UrlEncodeExceptionMap;
#[derive(Debug, Default)]
pub struct UrlEncoder {
    map: UrlEncodeExceptionMap,
}
impl UrlEncoder {
    pub fn new() -> Self {
        Self {
            map: UrlEncodeExceptionMap::new(),
        }
    }
    pub fn encode(&self, source: &str) -> String {
        source.chars().fold(String::new(), |acc, c| {
            let mut dst = [0; 4];
            c.encode_utf8(&mut dst);
            let add = if c.len_utf8() > 1 {
                (0..c.len_utf8()).fold(String::new(), |acc, cur| format!("{}%{:X}", acc, dst[cur]))
            } else {
                match self.map.0.get(&c) {
                    Some(encoded) => encoded.to_string(),
                    None => format!("{}", c),
                }
            };
            format!("{}{}", acc, add)
        })
    }
}

#[cfg(test)]
mod url_encode_test {
    use super::*;
    #[test]
    fn encode_test() {
        let url_encode = UrlEncoder::new();
        let source = "#こんにちは世界";
        let tobe = "%23%E3%81%93%E3%82%93%E3%81%AB%E3%81%A1%E3%81%AF%E4%B8%96%E7%95%8C";
        assert_eq!(url_encode.encode(source), tobe.to_string());
        let source = "(Hello World)";
        let tobe = "%28Hello+World%29";
        assert_eq!(url_encode.encode(source), tobe.to_string())
    }
}
