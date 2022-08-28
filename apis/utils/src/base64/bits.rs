use super::base64_encode_map::Base64BitMap;

pub struct Base64BitStreamIter {
    bit_iter: BitIter,
}
impl Base64BitStreamIter {
    pub fn new(source: &str) -> Self {
        Self {
            bit_iter: Bit::str_to_bit_iter(source),
        }
    }
    pub fn encode(&mut self, map: Base64BitMap) -> String {
        let mut result = String::new();
        let mut base64 = self.pop_base64_bit();
        while base64.is_some() {
            let key = base64.unwrap();
            let value = map.0.get(&key).unwrap();
            result.push(*value);
            base64 = self.pop_base64_bit();
        }
        let mod_len = result.len() % 4;
        for _ in 0..mod_len {
            result.push('=')
        }
        result
    }
    fn pop_base64_bit(&mut self) -> Option<Base64BitStream> {
        let mut stream = [Bit::Zero; 6];
        for i in 0..6 {
            let bit = self.bit_iter.next();
            if bit.is_none() {
                if i == 0 {
                    return None;
                }
                return Some(Base64BitStream { stream });
            }
            stream[i] = bit.unwrap().clone()
        }
        Some(Base64BitStream { stream })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Base64BitStream {
    stream: [Bit; 6],
}
impl Base64BitStream {
    pub fn new(stream: [Bit; 6]) -> Self {
        Self { stream }
    }
}
pub struct BitIter {
    bits: Vec<Bit>,
}
impl BitIter {
    pub fn new(source: &str) -> Self {
        let bits =
            source
                .bytes()
                .map(|byte| Bit::from_byte(byte))
                .fold(Vec::new(), |mut acc, mut cur| {
                    for c in cur {
                        acc.push(c);
                    }
                    acc
                });
        Self { bits }
    }
}
impl Iterator for BitIter {
    type Item = Bit;
    fn next(&mut self) -> Option<Self::Item> {
        if self.bits.len() == 0 {
            return None;
        };
        Some(self.bits.remove(0))
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Bit {
    Zero,
    One,
}
impl Bit {
    fn str_to_bit_iter(source: &str) -> BitIter {
        BitIter::new(source)
    }
    fn from_byte(byte: u8) -> [Bit; 8] {
        let mut bits = [Bit::Zero; 8];
        (0..8).for_each(|i| {
            let bit = (byte >> (7 - i)) & 1;
            match bit {
                0 => bits[i] = Bit::Zero,
                1 => bits[i] = Bit::One,
                _ => panic!("not 1 or 0 {}", bit),
            }
        });
        bits
    }
}
#[cfg(test)]
mod bit_test {
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
}
