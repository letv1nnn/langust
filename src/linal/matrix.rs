#![allow(dead_code)]

use std::ops::Add;

// STRUCTS:
// StaticMatrix, DynMatrix, DataSet(static/dynamic), ...

// TODO: alignment might be set to 32 or 64 for SIMD
#[repr(align(64))]
#[derive(Debug, PartialEq, PartialOrd)]
pub struct StaticMatrix<const ROWS: usize, const COLS: usize> {
    data: Box<[[f64; COLS]; ROWS]>,
}

// TODO: extend the structure
// #[derive(Debug)]
// pub struct DynMatrix<T: Float> {
//     data: Vec<T>,
// }

impl<const ROWS: usize, const COLS: usize> StaticMatrix<ROWS, COLS> {
    pub fn new(data: Box<[[f64; COLS]; ROWS]>) -> Self {
        Self { data }
    }
}

// TODO: implement SIMD vectorization for all operations.
impl<const ROWS: usize, const COLS: usize> Add for StaticMatrix<ROWS, COLS> {
    type Output = StaticMatrix<ROWS, COLS>;
    fn add(self, rhs: Self) -> Self::Output {
        let mut data = Box::new([[f64::default(); COLS]; ROWS]);
        for (row_idx, row) in self.data.iter().enumerate() {
            for (el_idx, el) in row.iter().enumerate() {
                data[row_idx][el_idx] = *el + rhs.data[row_idx][el_idx];
            }
        }
        StaticMatrix::new(data)
    }
}

#[cfg(test)]
mod linal_static_matrix_tests {
    use super::*;

    #[test]
    fn addition() {
        let m1 = StaticMatrix::new(Box::new([[1.0, 2.0], [3.0, 4.0]]));
        let m2 = StaticMatrix::new(Box::new([[1.0, 2.0], [3.0, 4.0]]));
        let expected = StaticMatrix::new(Box::new([[2.0, 4.0], [6.0, 8.0]]));
        let actual = m1 + m2;
        assert_eq!(expected, actual);
    }
}
