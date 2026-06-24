// bit 1 = null, bit 0 = valid

use std::fmt::Display;

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub struct NullBuffer {
    bits: Vec<u8>,
    len: usize,
}

impl NullBuffer {
    pub fn new() -> Self {
        Self {
            bits: Vec::new(),
            len: 0usize,
        }
    }
    pub fn with_len(len: usize) -> Self {
        Self {
            bits: vec![0u8; len.div_ceil(8usize)],
            len,
        }
    }
    #[inline]
    pub fn clear(&mut self) {
        self.bits.clear();
        self.len = 0usize;
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
    pub fn get(&self, idx: usize) -> Option<bool> {
        if idx >= self.len {
            None
        } else {
            Some(self.is_null(idx))
        }
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
    pub fn union(&self, other: &Self) -> Self {
        assert_eq!(self.len, other.len);
        let bits: Vec<u8> = self
            .bits
            .iter()
            .zip(other.bits.iter())
            .map(|(a, b)| a | b)
            .collect();
        Self {
            bits,
            len: self.len,
        }
    }
    #[inline]
    pub fn iter(&self) -> NullBufferIter<'_> {
        NullBufferIter {
            buffer: self,
            idx: 0,
        }
    }
}

impl Default for NullBuffer {
    fn default() -> Self {
        Self::new()
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

pub struct NullBufferIter<'a> {
    buffer: &'a NullBuffer,
    idx: usize,
}

impl Iterator for NullBufferIter<'_> {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.buffer.len {
            return None;
        }
        let value = self.buffer.is_null(self.idx);
        self.idx += 1;

        Some(value)
    }
}

impl FromIterator<u8> for NullBuffer {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let mut buffer = NullBuffer::new();

        for value in iter {
            let idx = buffer.len;

            buffer.len += 1usize;

            if idx.is_multiple_of(8usize) {
                buffer.bits.push(0u8);
            }

            if value == 0u8 {
                buffer.set_null(idx);
            }
        }

        buffer
    }
}

impl FromIterator<bool> for NullBuffer {
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
        let mut buffer = NullBuffer::default();

        for value in iter {
            let idx = buffer.len;

            buffer.len += 1usize;

            if idx % 8usize == 0usize {
                buffer.bits.push(0u8);
            }

            if value {
                buffer.set_null(idx);
            }
        }

        buffer
    }
}

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

    #[test]
    fn iterators() {
        // FromIterator trait for u8 and bool
        let expected = NullBuffer::from(vec![0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8]);

        let actual_nb_1 = vec![0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8]
            .into_iter()
            .collect::<NullBuffer>();
        println!("{}", actual_nb_1);

        let actual_nb_2 = vec![true, true, true, true, true, true, true, true, false, true]
            .into_iter()
            .collect::<NullBuffer>();
        println!("{}", actual_nb_2);

        assert!(expected == actual_nb_1 && expected == actual_nb_2);
        assert_eq!(actual_nb_1, actual_nb_2);

        // Iterator trait
        let nb = NullBuffer::from(vec![
            true, false, true, false, true, false, false, true, false, false,
        ]);

        let collected: Vec<bool> = nb.iter().collect();

        for i in 0..nb.len() {
            assert_eq!(collected[i], nb.is_null(i));
        }

        assert_eq!(
            collected,
            vec![
                true, false, true, false, true, false, false, true, false, false
            ]
        );

        assert_eq!(collected.len(), nb.len());
    }
}
