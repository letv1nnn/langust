use std::ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign};

use crate::matrix::{
    errors::MatrixError,
    simd::{ArithmeticOperation, Simd},
};

// TODO: Change to f32, so SIMD would calculate twice as faster.
#[repr(C)]
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Matrix {
    data: Vec<f32>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![0.0f32; rows * cols],
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
    pub const fn is_square(&self) -> bool {
        self.rows == self.cols
    }
}

// linear algebra operations
impl Matrix {
    pub fn identity(rows: usize, cols: usize) -> Self {
        let mut identity_matrix = Self::new(rows, cols);

        let (mut row, mut col) = (0usize, 0usize);
        while row < identity_matrix.rows && col < identity_matrix.cols {
            identity_matrix[(row, col)] = 1.0f32;
            row += 1;
            col += 1;
        }
        identity_matrix
    }

    pub fn transpose(&mut self) {
        unimplemented!()
    }
}

// arithmetics
impl Matrix {
    pub fn try_add(&self, rhs: &Self) -> Result<Self, MatrixError> {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            return Err(MatrixError::ShapeMismatch);
        }
        let mut out = Matrix::new(self.rows, self.cols);

        Simd::arithmetics_f32(
            &self.data,
            &rhs.data,
            &mut out.data,
            ArithmeticOperation::Addition,
        );

        Ok(out)
    }
    pub fn try_sub(&self, rhs: &Self) -> Result<Self, MatrixError> {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            return Err(MatrixError::ShapeMismatch);
        }
        let mut out = Matrix::new(self.rows, self.cols);

        Simd::arithmetics_f32(
            &self.data,
            &rhs.data,
            &mut out.data,
            ArithmeticOperation::Subtraction,
        );

        Ok(out)
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f32;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row * self.cols + col]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.data[row * self.cols + col]
    }
}

impl From<Vec<f32>> for Matrix {
    fn from(value: Vec<f32>) -> Self {
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
        self.try_add(&rhs).expect("matrix dimensions do not match")
    }
}

impl Add for &Matrix {
    type Output = Matrix;
    fn add(self, rhs: Self) -> Self::Output {
        self.try_add(&rhs).expect("matrix dimensions do not match")
    }
}

impl AddAssign for Matrix {
    fn add_assign(&mut self, _rhs: Self) {
        unimplemented!()
    }
}

impl AddAssign for &Matrix {
    fn add_assign(&mut self, _rhs: Self) {
        unimplemented!()
    }
}

impl Sub for Matrix {
    type Output = Matrix;
    fn sub(self, rhs: Self) -> Self::Output {
        self.try_sub(&rhs).expect("matrix dimensions do not match")
    }
}

impl Sub for &Matrix {
    type Output = Matrix;
    fn sub(self, rhs: Self) -> Self::Output {
        self.try_sub(&rhs).expect("matrix dimensions do not match")
    }
}

impl SubAssign for Matrix {
    fn sub_assign(&mut self, _rhs: Self) {
        unimplemented!()
    }
}

impl SubAssign for &Matrix {
    fn sub_assign(&mut self, _rhs: Self) {
        unimplemented!()
    }
}

#[cfg(test)]
mod matrix_general_tests {
    use std::cmp::min;

    use super::*;

    fn is_identity(matrix: &Matrix) -> bool {
        let range = min(matrix.rows, matrix.cols);
        for i in 0..range {
            if matrix[(i, i)] != 1.0f32 {
                return false;
            }
        }
        true
    }

    #[test]
    fn identity_matrix_initialization() {
        let im1 = Matrix::identity(3usize, 3usize);
        assert!(is_identity(&im1));

        let im2 = Matrix::identity(5usize, 1usize);
        assert!(is_identity(&im2));

        let im3 = Matrix::identity(2usize, 7usize);
        assert!(is_identity(&im3));
    }
}

#[cfg(test)]
mod matrix_arithmetic_operations_tests {
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
