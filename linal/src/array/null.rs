// 1 represents 0 or NaN, 0 represents a value
#[derive(Debug, PartialEq, Eq)]
pub struct NullBuffer {
    bits: Vec<u8>,
    len: usize,
}

impl NullBuffer {
    pub fn new() -> Self {
        Self {
            bits: vec![],
            len: 0usize,
        }
    }
    pub const fn len(&self) -> usize {
        self.len
    }
    pub fn count_nulls(&self) -> usize {
        (0..self.len).filter(|&i| self.is_null(i)).count()
    }
    pub fn any_null(&self) -> bool {
        self.count_nulls() > 0usize
    }
    pub fn is_null(&self, idx: usize) -> bool {
        let (byte, bit) = (idx / 8usize, idx % 8usize);
        (self.bits[byte] & (1u8 << bit)) != 0u8
    }
    pub fn set_null(&mut self, idx: usize) {
        let (byte, bit) = (idx / 8usize, idx % 8usize);
        self.bits[byte] |= 1u8 << bit;
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
            if val > 0u8 {
                let (byte, bit) = (i >> 3usize, i % 8usize);
                bits[byte] |= 1u8 << bit;
            }
        }

        Self { bits, len }
    }
}

#[cfg(test)]
mod nullbuffer_array_tests {
    use super::*;

    #[test]
    fn contruction_from_vector() {
        let nb = NullBuffer::from(vec![true, false, true, false]);
        assert_eq!(nb.count_nulls(), 2usize);
        assert!(nb.is_null(0usize));
        assert_eq!(nb.is_null(1usize), false);
        assert!(nb.is_null(2usize));
        assert_eq!(nb.is_null(3usize), false);

        let nb = NullBuffer::from(vec![0u8, 1u8, 0u8, 1u8, 1u8]);
        assert_eq!(nb.count_nulls(), 3usize);
        assert!(!nb.is_null(0));
        assert!(nb.is_null(1));
        assert!(!nb.is_null(2));
        assert!(nb.is_null(3));
        assert!(nb.is_null(4));
    }
}
