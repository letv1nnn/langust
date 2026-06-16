use std::ops::{Add, Index, IndexMut, Sub};

use crate::matrix::errors::MatrixError;

// TODO: Change to f32, so SIMD would calculate twice as faster.
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
    pub fn try_add(&self, rhs: &Self) -> Result<Self, MatrixError> {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            return Err(MatrixError::ShapeMismatch);
        }
        Ok(self + rhs)
    }
    pub fn try_sub(&self, rhs: &Self) -> Result<Self, MatrixError> {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            return Err(MatrixError::ShapeMismatch);
        }
        Ok(self - rhs)
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
    fn add(self, _rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

impl Add for &Matrix {
    type Output = Matrix;
    fn add(self, _rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

impl Sub for Matrix {
    type Output = Matrix;
    fn sub(self, _rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

impl Sub for &Matrix {
    type Output = Matrix;
    fn sub(self, _rhs: Self) -> Self::Output {
        unimplemented!()
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
