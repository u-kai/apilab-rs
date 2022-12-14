use super::base64_encode_map::Base64BitMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Base64BitStreamIter {
    bit_iter: BitIter,
}
impl Base64BitStreamIter {
    pub fn new(source: &str) -> Self {
        Self {
            bit_iter: Bit::str_to_bit_iter(source),
        }
    }
    pub fn from_bytes(byte: &[u8]) -> Self {
        Self {
            bit_iter: BitIter {
                bits: byte.iter().map(|b| Bit::from_byte(b)).flatten().collect(),
            },
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
        if mod_len % 4 != 0 {
            for _ in 0..(4 - mod_len) {
                result.push('=')
            }
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
    pub fn get(&self, i: usize) -> Bit {
        self.stream[i]
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct BitIter {
    bits: Vec<Bit>,
}
impl BitIter {
    pub fn new(source: &str) -> Self {
        let bits =
            source
                .bytes()
                .map(|byte| Bit::from_byte(&byte))
                .fold(Vec::new(), |mut acc, cur| {
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
    pub fn to_u8(bits: &[Bit; 8]) -> u8 {
        (0..8).fold(0, |acc, cur| {
            acc + match bits[cur] {
                Self::Zero => 0,
                Self::One => 2_i32.pow(7 - cur as u32) as u8,
            }
        })
    }
    pub fn str_to_bit_iter(source: &str) -> BitIter {
        BitIter::new(source)
    }
    pub fn from_byte(byte: &u8) -> [Bit; 8] {
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
    fn new_bit_iter_test() {
        assert_eq!(
            Base64BitStreamIter::new("hello world"),
            Base64BitStreamIter::from_bytes("hello world".as_bytes())
        );
    }
    #[test]
    fn to_u8_test() {
        assert_eq!(Bit::to_u8(&[Bit::Zero; 8]), 0);
        assert_eq!(
            Bit::to_u8(&[
                Bit::Zero,
                Bit::Zero,
                Bit::Zero,
                Bit::Zero,
                Bit::Zero,
                Bit::Zero,
                Bit::Zero,
                Bit::One,
            ]),
            1
        );
        assert_eq!(
            Bit::to_u8(&[
                Bit::One,
                Bit::One,
                Bit::One,
                Bit::One,
                Bit::One,
                Bit::One,
                Bit::One,
                Bit::One,
            ]),
            255
        );
    }
    #[test]
    fn from_byte_test() {
        assert_eq!(Bit::from_byte(&0), [Bit::Zero; 8]);
        assert_eq!(
            Bit::from_byte(&1),
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
            Bit::from_byte(&254),
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
