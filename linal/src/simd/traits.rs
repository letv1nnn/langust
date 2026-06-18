#![allow(dead_code)]

use crate::simd::errors::SimdResult;

#[derive(Clone, Copy)]
pub(crate) enum ArithmeticOperation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

pub(crate) trait SimdOps: Sized + Copy {
    fn arithmetic(a: &[Self], b: &[Self], out: &mut [Self], op: ArithmeticOperation) -> SimdResult;
    fn arithmetic_inplace(out: &mut [Self], value: &[Self], op: ArithmeticOperation) -> SimdResult;
}
