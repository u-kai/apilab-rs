use super::base64_encode_map::Base64BitMap;
use crate::base64::bits::Base64BitStreamIter;
use std::collections::HashMap;
#[cfg(test)]
pub fn encode<T: AsRef<str>>(source: T) -> String {
    let source = source.as_ref();
    let map = Base64BitMap::new();
    let mut iter = Base64BitStreamIter::new(source);
    iter.encode(map)
}

mod base64_tests {
    use super::*;
    #[test]
    fn base64_test() {
        let source = "Hello,World!";
        let encode = encode(source);
        assert_eq!(encode, "SGVsbG8sV29ybGQh".to_string());
    }
}
