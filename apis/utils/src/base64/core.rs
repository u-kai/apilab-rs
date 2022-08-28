use super::{base64_encode_map::Base64BitMap, bits::Bit};

pub fn encode<T: AsRef<str>>(source: T) -> String {
    use super::base64_encode_map::Base64BitMap;
    use crate::base64::bits::Base64BitStreamIter;
    let source = source.as_ref();
    let map = Base64BitMap::new();
    let mut iter = Base64BitStreamIter::new(source);
    iter.encode(map)
}
pub fn decode<T: AsRef<str>>(source: T) -> String {
    let source = source.as_ref();
    let map = super::base64_decode_map::Base64BitMap::new();
    let mut v = vec![];
    for c in source.chars() {
        if c == '=' && v.len() % 8 != 0 {
            let remove_len = v.len() % 8;
            for _ in 0..remove_len {
                v.pop();
            }
            break;
        }
        let bit_stream = map.0.get(&c).unwrap();
        for i in 0..6 {
            v.push(bit_stream.get(i));
        }
    }
    let mut tmp = [Bit::Zero; 8];
    let mut byte = vec![];
    for i in 1..=v.len() {
        tmp[(i - 1) % 8] = v[i - 1];
        if i % 8 == 0 {
            byte.push(Bit::to_u8(&tmp));
        }
    }
    String::from_utf8(byte).unwrap()
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
        let source = r#""goodlack1234""#;
        let encoded = encode(source);
        assert_eq!(encoded, "Imdvb2RsYWNrMTIzNCI=".to_string());
    }
    #[test]
    fn base64_decode_test() {
        let source = "SGVsbG8sV29ybGQh";
        let decoded = decode(source);
        assert_eq!(decoded, "Hello,World!".to_string());
        let source = "YWJjZGVmZw==";
        let decoded = decode(source);
        assert_eq!(decoded, "abcdefg".to_string());
        let source = "Imdvb2RsYWNrMTIzNCI=";
        let encoded = decode(source);
        assert_eq!(encoded, r#""goodlack1234""#.to_string());
    }
}
