// bit 1 = null, bit 0 = valid

use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct NullBuffer {
    bits: Vec<u8>,
    len: usize,
}

impl NullBuffer {
    pub fn new() -> Self {
        Self {
            bits: Vec::default(),
            len: 0usize,
        }
    }
    pub fn with_len(len: usize) -> Self {
        Self {
            bits: vec![0u8; len.div_ceil(len)],
            len,
        }
    }
    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len == 0usize
    }
    pub fn count_nulls(&self) -> usize {
        if self.bits.is_empty() {
            return 0;
        }
        let last = self.bits.len() - 1;
        let mut count: usize = self.bits[..last]
            .iter()
            .map(|b| b.count_ones() as usize)
            .sum();
        let remaining = self.len % 8;
        let mask = if remaining == 0 {
            0xFF
        } else {
            (1u8 << remaining) - 1
        };
        count += (self.bits[last] & mask).count_ones() as usize;
        count
    }
    pub fn any_null(&self) -> bool {
        self.bits.iter().any(|b| *b != 0u8)
    }
    #[inline]
    pub fn is_null(&self, idx: usize) -> bool {
        assert!(idx < self.len);
        let (byte, bit) = (idx / 8usize, idx % 8usize);
        (self.bits[byte] & (1u8 << bit)) != 0u8
    }
    #[inline]
    pub fn set_null(&mut self, idx: usize) {
        assert!(idx < self.len);
        let (byte, bit) = (idx / 8usize, idx % 8usize);
        self.bits[byte] |= 1u8 << bit;
    }
    #[inline]
    pub fn set_valid(&mut self, idx: usize) {
        assert!(idx < self.len);
        let (byte, bit) = (idx / 8usize, idx % 8usize);
        self.bits[byte] &= !(1u8 << bit);
    }
}

impl From<Vec<bool>> for NullBuffer {
    fn from(value: Vec<bool>) -> Self {
        let len = value.len();
        let mut bits = vec![0u8; len.div_ceil(8usize)];

        for (i, is_null) in value.into_iter().enumerate() {
            if is_null {
                let (byte, bit) = (i >> 3usize, i % 8usize);
                bits[byte] |= 1u8 << bit;
            }
        }

        Self { bits, len }
    }
}

impl From<Vec<u8>> for NullBuffer {
    fn from(value: Vec<u8>) -> Self {
        let len = value.len();
        let mut bits = vec![0u8; len.div_ceil(8usize)];

        for (i, val) in value.into_iter().enumerate() {
            if val == 0u8 {
                let (byte, bit) = (i >> 3usize, i % 8usize);
                bits[byte] |= 1u8 << bit;
            }
        }

        Self { bits, len }
    }
}

// impl FromIterator<bool> for NullBuffer { 
//     fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
//         let values: Vec<bool> = iter.into_iter().collect();
//         values.into()
//     }
// }

impl Display for NullBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.len {
            writeln!(f, "{}", if self.is_null(i) { "NaN" } else { "Value" })?;
        }
        Ok(())
    }
}

// `cargo test -- --nocapture` to see the debug output
#[cfg(test)]
mod nullbuffer_array_tests {
    use super::*;

    #[test]
    fn contruction_from_vector() {
        let nb = NullBuffer::from(vec![true, false, true, false]);
        println!("{:08b}", &nb.bits[0]);
        // count null/NaN values
        assert_eq!(nb.count_nulls(), 2usize);
        // check each value
        assert!(nb.is_null(0usize));
        assert_eq!(nb.is_null(1usize), false);
        assert!(nb.is_null(2usize));
        assert_eq!(nb.is_null(3usize), false);
        // check length
        assert_eq!(nb.len(), 4usize);
        // any nulls
        assert!(nb.any_null());

        let nb = NullBuffer::from(vec![0u8, 1u8, 0u8, 1u8, 1u8]);
        println!("{:08b}", &nb.bits[0]);
        // count null/NaN values
        assert_eq!(nb.count_nulls(), 2usize);
        // check each value
        assert!(nb.is_null(0usize));
        assert_eq!(nb.is_null(1usize), false);
        assert!(nb.is_null(2usize));
        assert_eq!(nb.is_null(3usize), false);
        assert_eq!(nb.is_null(4usize), false);
        // check length
        assert_eq!(nb.len(), 5usize);
        // any nulls
        assert!(nb.any_null());

        let nb = NullBuffer::from(vec![0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8]);
        println!("{:08b} {:08b}", &nb.bits[0], &nb.bits[1]);
        // check each value
        for bit in 0..8usize {
            assert!(nb.is_null(bit));
        }
        assert!(!nb.is_null(8usize));
        assert!(nb.is_null(9usize));
        // check length
        assert_eq!(nb.len(), 10usize);
        // any nulls
        assert!(nb.any_null());
    }

    #[test]
    fn setting_values() {
        let mut nb = NullBuffer::from(vec![0u8]);
        assert!(nb.is_null(0usize));
        
        nb.set_valid(0usize);
        assert!(!nb.is_null(0usize));

        nb.set_null(0usize);
        assert!(nb.is_null(0usize));
    }
}
