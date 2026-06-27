use std::{
    fmt::Display,
    ops::{Add, Index, IndexMut, Sub},
};

use crate::{
    array::{
        null::NullBuffer,
        traits::{ArrayElement, ElementArithmetcs},
    },
    simd::traits::ArithmeticOperation,
};

// primitive array to store data and their state (present or not)
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct Array<T: ArrayElement> {
    data: Vec<T>,
    nulls: NullBuffer,
}

impl ArrayElement for f32 {}
impl ArrayElement for f64 {}
impl ArrayElement for i32 {}
impl ArrayElement for i64 {}
impl ArrayElement for u32 {}
impl ArrayElement for u64 {}

impl<T: ArrayElement> Array<T> {
    pub fn new() -> Self {
        Self {
            data: vec![],
            nulls: NullBuffer::new(),
        }
    }
    pub fn with_nulls(data: Vec<T>, nulls: NullBuffer) -> Self {
        assert_eq!(
            data.len(),
            nulls.len(),
            "data and null buffer length mismatch"
        );
        Self { data, nulls }
    }
    pub fn zeros(&self) -> usize {
        self.nulls.count_nulls()
    }
    pub fn get(&self, idx: usize) -> Option<&T> {
        (!self.nulls.is_null(idx)).then(|| &self.data[idx])
    }
    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        (!self.nulls.is_null(idx)).then(|| &mut self.data[idx])
    }
    pub const fn len(&self) -> usize {
        self.data.len()
    }
}

impl<T: ArrayElement> Default for Array<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: ArrayElement> Index<usize> for Array<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        if self.nulls.is_null(index) {
            panic!("value at {index} index is abscent")
        } else {
            &self.data[index]
        }
    }
}

impl<T: ArrayElement> IndexMut<usize> for Array<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if self.nulls.is_null(index) {
            panic!("value at {index} index is abscent");
        } else {
            &mut self.data[index]
        }
    }
}

// assuming all values are valid
impl<T: ArrayElement> From<Vec<T>> for Array<T> {
    fn from(data: Vec<T>) -> Self {
        Self {
            nulls: NullBuffer::with_len(data.len()),
            data,
        }
    }
}

impl<T: ArrayElement> From<Vec<Option<T>>> for Array<T> {
    fn from(value: Vec<Option<T>>) -> Self {
        let (mut data, mut nulls) = (
            vec![T::default(); value.len()],
            NullBuffer::with_len(value.len()),
        );

        for i in 0usize..value.len() {
            match value[i] {
                Some(value) => data[i] = value,
                _ => nulls.set_null(i),
            }
        }

        Self { data, nulls }
    }
}

impl<T: ArrayElement + Display> Display for Array<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ ")?;

        for idx in 0..self.data.len() {
            if idx > 0 {
                write!(f, ", ")?;
            }

            if self.nulls.is_null(idx) {
                write!(f, "NaN")?;
            } else {
                write!(f, "{}", self.data[idx])?;
            }
        }

        write!(f, " ]")
    }
}

pub struct ArrayIter<'a, T: ArrayElement> {
    array: &'a Array<T>,
    index: usize,
}

impl<'a, T: ArrayElement> Iterator for ArrayIter<'a, T> {
    type Item = Option<&'a T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.array.len() {
            None
        } else {
            let value = self.array.get(self.index);
            self.index += 1;
            Some(value)
        }
    }
}

// arithmetics

// F32
impl<T: ArrayElement + ElementArithmetcs> Add<Array<T>> for Array<T> {
    type Output = Self;
    fn add(self, rhs: Array<T>) -> Self::Output {
        assert_eq!(
            self.data.len(),
            rhs.data.len(),
            "data length of the left hand side does not match the length of the data at the right hand side"
        );

        let mut data = vec![T::default(); self.data.len()];
        ElementArithmetcs::slice_arith(
            &self.data,
            &rhs.data,
            &mut data,
            ArithmeticOperation::Addition,
        );
        let nulls = self.nulls.union(&rhs.nulls);

        Self { data, nulls }
    }
}

impl<T: ArrayElement + ElementArithmetcs> Add<&Array<T>> for &Array<T> {
    type Output = Array<T>;
    fn add(self, rhs: &Array<T>) -> Self::Output {
        assert_eq!(
            self.data.len(),
            rhs.data.len(),
            "data length of the left hand side does not match the length of the data at the right hand side"
        );

        let mut data = vec![T::default(); self.data.len()];
        ElementArithmetcs::slice_arith(
            &self.data,
            &rhs.data,
            &mut data,
            ArithmeticOperation::Addition,
        );
        let nulls = self.nulls.union(&rhs.nulls);

        Array { data, nulls }
    }
}

impl<T: ArrayElement + ElementArithmetcs> Sub<Array<T>> for Array<T> {
    type Output = Array<T>;
    fn sub(self, rhs: Array<T>) -> Self::Output {
        assert_eq!(
            self.data.len(),
            rhs.data.len(),
            "data length of the left hand side does not match the length of the data at the right hand side"
        );

        let mut data = vec![T::default(); self.data.len()];
        ElementArithmetcs::slice_arith(
            &self.data,
            &rhs.data,
            &mut data,
            ArithmeticOperation::Subtraction,
        );
        let nulls = self.nulls.union(&rhs.nulls);

        Array { data, nulls }
    }
}

impl<T: ArrayElement + ElementArithmetcs> Sub<&Array<T>> for &Array<T> {
    type Output = Array<T>;
    fn sub(self, rhs: &Array<T>) -> Self::Output {
        assert_eq!(
            self.data.len(),
            rhs.data.len(),
            "data length of the left hand side does not match the length of the data at the right hand side"
        );

        let mut data = vec![T::default(); self.data.len()];
        ElementArithmetcs::slice_arith(
            &self.data,
            &rhs.data,
            &mut data,
            ArithmeticOperation::Subtraction,
        );
        let nulls = self.nulls.union(&rhs.nulls);

        Array { data, nulls }
    }
}

#[cfg(test)]
mod core_array_tests {
    use super::*;

    #[test]
    fn vector_into_and_from_array() {
        let arr: Array<i32> = vec![Some(1), None, Some(32), None].into();

        let expected_nb = NullBuffer::from(vec![false, true, false, true]);
        let expected_data = vec![1i32, i32::default(), 32i32, i32::default()];

        assert_eq!(
            arr.nulls, expected_nb,
            "actual array null buffer differs from the expected one"
        );
        assert_eq!(
            arr.data, expected_data,
            "actual array data differs from the expected one"
        );
    }

    #[test]
    fn array_f32_simd_arithmetics() {
        let arr1: Array<f32> = vec![Some(1.0f32), None, Some(32.0f32), None].into();
        let arr2: Array<f32> = vec![Some(2.0f32), None, None, Some(3.14f32)].into();

        let (actual, expected): (Array<f32>, Array<f32>) =
            (arr1 + arr2, vec![Some(3.0f32), None, None, None].into());

        assert_eq!(actual[0], expected[0]);
        assert!(!actual.nulls.is_null(0usize));
    }
}
