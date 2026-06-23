#![allow(unused)]

use std::{
    any::TypeId,
    ops::{Add, Index, IndexMut},
};

use crate::{
    array::{null::NullBuffer, traits::ArrayElement},
    simd::{self, traits::SimdOps},
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

// arithmetics
impl<T: ArrayElement> Add<Array<T>> for Array<T> {
    type Output = Self;
    fn add(self, rhs: Array<T>) -> Self::Output {
        unimplemented!()
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
}
