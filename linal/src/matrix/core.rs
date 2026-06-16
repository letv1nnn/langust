use std::ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign};

use crate::matrix::{
    errors::MatrixError,
    simd::{ArithmeticOperation, Simd},
};

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

    pub fn transpose(&self) -> Matrix {
        let mut out = vec![0.0f32; self.data.len()];
        for row in 0..self.rows {
            for col in 0..self.cols {
                out[col * self.rows + row] = self.data[row * self.cols + col];
            }
        }
        Matrix {
            data: out,
            rows: self.cols,
            cols: self.rows,
        }
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

impl TryFrom<Vec<Vec<f32>>> for Matrix {
    type Error = MatrixError;
    fn try_from(value: Vec<Vec<f32>>) -> Result<Self, Self::Error> {
        let row_len = value[0].len();
        for row in value.iter() {
            if row_len != row.len() {
                return Err(MatrixError::ShapeMismatch);
            }
        }
        let (rows, cols) = (value.len(), value[0].len());
        let data = value.into_iter().flatten().collect::<Vec<f32>>();
        Ok(Self { data, rows, cols })
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
        self.try_add(rhs).expect("matrix dimensions do not match")
    }
}

impl AddAssign for Matrix {
    fn add_assign(&mut self, rhs: Self) {
        assert_eq!(self.rows, rhs.rows, "matrix dimensions do not match");
        assert_eq!(self.cols, rhs.cols, "matrix dimensions do not match");
        Simd::arithmetics_f32_inplace(&mut self.data, &rhs.data, ArithmeticOperation::Addition);
    }
}

// need to find a way to not clone the initial vector
impl Sub for Matrix {
    type Output = Matrix;
    fn sub(self, rhs: Self) -> Self::Output {
        self.try_sub(&rhs).expect("matrix dimensions do not match")
    }
}

impl Sub for &Matrix {
    type Output = Matrix;
    fn sub(self, rhs: Self) -> Self::Output {
        self.try_sub(rhs).expect("matrix dimensions do not match")
    }
}

impl SubAssign for Matrix {
    fn sub_assign(&mut self, rhs: Self) {
        assert_eq!(self.rows, rhs.rows, "matrix dimensions do not match");
        assert_eq!(self.cols, rhs.cols, "matrix dimensions do not match");
        Simd::arithmetics_f32_inplace(&mut self.data, &rhs.data, ArithmeticOperation::Subtraction);
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

    #[test]
    fn transpose_matrix() {
        let im = Matrix::identity(3usize, 3usize);
        let im_t = im.transpose();
        assert_eq!(im_t, im);
        assert_eq!(im_t.rows(), im.cols());
        assert_eq!(im.rows(), im_t.cols());

        let m = Matrix::try_from(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();
        let m_t = m.transpose();
        assert_eq!(m_t, Matrix::try_from(vec![vec![1.0, 3.0], vec![2.0, 4.0]]).unwrap());
        assert_eq!(m.rows(), m_t.cols());
        assert_eq!(m_t.rows(), m.cols());
    }
}

#[cfg(test)]
mod matrix_arithmetic_operations_tests {
    use super::*;

    #[test]
    fn addition() {
        let (m1, m2) = (
            Matrix::from(vec![1.0, 2.0, 3.0, 4.0]),
            Matrix::from(vec![4.0, 3.0, 2.0, 1.0]),
        );

        let expected = Matrix::from(vec![5.0; 4usize]);
        assert_eq!(m1 + m2, expected);

        let (mut m1, m2) = (
            Matrix::from(vec![1.0, 2.0, 3.0, 4.0]),
            Matrix::from(vec![0.0; 4usize]),
        );

        let expected = Matrix::from(vec![1.0, 2.0, 3.0, 4.0]);
        m1 += m2;
        assert_eq!(m1, expected);
    }

    #[test]
    fn subtraction() {
        let (m1, m2) = (
            Matrix::from(vec![1.0, 2.0, 3.0, 4.0]),
            Matrix::from(vec![4.0, 3.0, 2.0, 1.0]),
        );

        let expected = Matrix::from(vec![-3.0, -1.0, 1.0, 3.0]);
        assert_eq!(m1 - m2, expected);

        let (mut m1, m2) = (
            Matrix::from(vec![1.0, 2.0, 3.0, 4.0]),
            Matrix::from(vec![4.0, 3.0, 2.0, 1.0]),
        );

        let expected = Matrix::from(vec![-3.0, -1.0, 1.0, 3.0]);
        m1 -= m2;
        assert_eq!(m1, expected);
    }
}

#[cfg(test)]
mod matrix_operation_expected_to_fail {
    use super::*;

    #[test]
    #[should_panic]
    fn arithmetics() {
        let (m1, m2) = (
            Matrix::from(vec![0.0; 10usize]),
            Matrix::from(vec![3.14; 1usize]),
        );

        let _ = &m1 + &m2;
        let _ = &m1 - &m2;
        let _ = m2 + m1;
    }
}
