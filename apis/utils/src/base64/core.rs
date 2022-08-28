use super::base64_encode_map::Base64BitMap;

pub fn encode<T: AsRef<str>>(source: T) -> String {
    use super::base64_encode_map::Base64BitMap;
    use crate::base64::bits::Base64BitStreamIter;
    let source = source.as_ref();
    let map = Base64BitMap::new();
    let mut iter = Base64BitStreamIter::new(source);
    iter.encode(map)
}
pub fn decode<T: AsRef<str>>(source: T) -> String {
    let map = Base64BitMap::new();

    "Hello,World!".to_string()
}

#[cfg(test)]
mod base64_tests {
    use super::*;
    #[test]
    fn base64_encode_test() {
        let source = "Hello,World!";
        let encoded = encode(source);
        assert_eq!(encoded, "SGVsbG8sV29ybGQh".to_string());
        let source = "abcdefg";
        let encoded = encode(source);
        assert_eq!(encoded, "YWJjZGVmZw==".to_string());
    }
    fn base64_decode_test() {
        let source = "SGVsbG8sV29ybGQh";
        let decoded = decode(source);
        assert_eq!(decoded, "Hello,World!".to_string());
    }
}
