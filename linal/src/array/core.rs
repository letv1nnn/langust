use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Sub},
};

use crate::array::{
    null::NullBuffer,
    traits::{ArrayElement, SliceArith},
};

// primitive array to store data and their state (present or not)
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct Array<T: ArrayElement> {
    data: Vec<T>,
    nulls: NullBuffer,
}

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
    pub fn null_count(&self) -> usize {
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
    pub const fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn sum(&self) -> T
    where
        T: std::iter::Sum,
    {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(i, v)| (!self.nulls.is_null(i)).then_some(*v))
            .sum()
    }
    pub fn min(&self) -> Option<T>
    where
        T: std::cmp::Ord,
    {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(i, v)| (!self.nulls.is_null(i)).then_some(*v))
            .min()
    }
    pub fn max(&self) -> Option<T>
    where
        T: std::cmp::Ord,
    {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(i, v)| (!self.nulls.is_null(i)).then_some(*v))
            .max()
    }
    pub fn mean(&self) -> Option<f64>
    where
        T: Into<f64>,
    {
        let valid_len = self.len() - self.nulls.count_nulls();
        if valid_len == 0usize {
            return None;
        }
        let sum: f64 = self
            .data
            .iter()
            .enumerate()
            .filter_map(|(i, v)| (!self.nulls.is_null(i)).then_some((*v).into()))
            .sum();
        Some(sum / valid_len as f64)
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
            panic!("index {index} is null")
        } else {
            &self.data[index]
        }
    }
}

impl<T: ArrayElement> IndexMut<usize> for Array<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if self.nulls.is_null(index) {
            panic!("index {index} is null");
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

impl<'a, T: ArrayElement> IntoIterator for &'a Array<T> {
    type Item = Option<&'a T>;
    type IntoIter = ArrayIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        ArrayIter {
            array: self,
            index: 0,
        }
    }
}

impl_array_op!(Add, add, add_slices);
impl_array_op!(Sub, sub, sub_slices);
impl_array_op!(Mul, mul, mul_slices);
impl_array_op!(Div, div, div_slices);

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

    #[test]
    fn sum_max_min_mean() {
        let arr: Array<f32> = vec![Some(12.34), None, Some(56.67), None].into();

        const EXPONENT: f64 = 1e-5;

        assert!(f32::abs(arr.sum() - 69.01) < EXPONENT as f32);
        assert!(f64::abs(arr.mean().unwrap() - 34.505) < EXPONENT);

        let arr: Array<i32> = vec![None, Some(1), None, Some(42), Some(5), Some(-14), None].into();
        assert_eq!(arr.max(), Some(42));
        assert_eq!(arr.min(), Some(-14));
    }

    #[test]
    fn iterator_capabilities() {
        unimplemented!("test main iterator capabilities")
    }
}
