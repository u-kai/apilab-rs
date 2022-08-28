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
    fn from_byte_test() {
        assert_eq!(Bit::from_byte(0), [Bit::Zero; 8]);
        assert_eq!(
            Bit::from_byte(1),
            [
                Bit::Zero,
                Bit::Zero,
                Bit::Zero,
                Bit::Zero,
                Bit::Zero,
                Bit::Zero,
                Bit::Zero,
                Bit::One,
            ]
        );
        assert_eq!(
            Bit::from_byte(254),
            [
                Bit::One,
                Bit::One,
                Bit::One,
                Bit::One,
                Bit::One,
                Bit::One,
                Bit::One,
                Bit::Zero,
            ]
        );
    }
    #[test]
    fn base64_test() {
        let source = "Hello,World!";
        let encode = encode(source);
        assert_eq!(encode, "SGVsbG8sV29ybGQh".to_string());
    }
}
struct Base64BitStreamIter {
    bit_iter: BitIter,
}
impl Base64BitStreamIter {
    fn new(source: &str) -> Self {
        Self {
            bit_iter: Bit::str_to_bit_iter(source),
        }
    }
    fn encode(&mut self, map: Base64BitMap) -> String {
        let mut result = String::new();
        let mut base64 = self.pop_base64_bit();
        while base64.is_some() {
            let key = base64.unwrap();
            let value = map.0.get(&key).unwrap();
            result.push(*value);
            base64 = self.pop_base64_bit();
        }
        result
    }
    fn pop_base64_bit(&mut self) -> Option<Base64BitStream> {
        let mut stream = [Bit::Zero; 6];
        for i in 0..6 {
            let bit = self.bit_iter.next();
            if bit.is_none() && i != 0 {
                return None;
            }
            stream[i] = bit.unwrap().clone()
        }
        Some(Base64BitStream { stream })
    }
    //fn
}
