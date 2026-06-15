use std::ops::{Add, Index, IndexMut, Sub};

use crate::matrix::simd::{ArithmeticOperation, kernel_arithmetics};

#[repr(C)]
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Matrix {
    data: Vec<f64>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![0.0f64; rows * cols],
            rows,
            cols,
        }
    }
    pub const fn rows(&self) -> usize {
        self.rows
    }
    pub const fn cols(&self) -> usize {
        self.cols
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row * self.cols + col]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.data[row * self.cols + col]
    }
}

impl From<Vec<f64>> for Matrix {
    fn from(value: Vec<f64>) -> Self {
        Self {
            rows: 1usize,
            cols: value.len(),
            data: value,
        }
    }
}

impl Add for Matrix {
    type Output = Matrix;
    fn add(self, rhs: Self) -> Self::Output {
        let len = self.rows * self.cols;
        let mut out = vec![0.0f64; len];

        kernel_arithmetics(
            &self.data,
            &rhs.data,
            &mut out,
            ArithmeticOperation::Addition,
        );

        Matrix {
            data: out,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl Sub for Matrix {
    type Output = Matrix;
    fn sub(self, rhs: Self) -> Self::Output {
        let len = self.rows * self.cols;
        let mut out = vec![0.0f64; len];

        kernel_arithmetics(
            &self.data,
            &rhs.data,
            &mut out,
            ArithmeticOperation::Subtraction,
        );

        Matrix {
            data: out,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

#[cfg(test)]
mod matrix_tests {
    use super::*;

    #[test]
    fn addition() {
        let m1 = Matrix::from(vec![1.0, 2.0, 3.0, 4.0]);
        let m2 = Matrix::from(vec![4.0, 3.0, 2.0, 1.0]);

        let expected = Matrix::from(vec![5.0; 4usize]);
        assert_eq!(m1 + m2, expected);
    }

    #[test]
    fn subtraction() {
        let m1 = Matrix::from(vec![1.0, 2.0, 3.0, 4.0]);
        let m2 = Matrix::from(vec![4.0, 3.0, 2.0, 1.0]);

        let expected = Matrix::from(vec![-3.0, -1.0, 1.0, 3.0]);
        assert_eq!(m1 - m2, expected);
    }
}
