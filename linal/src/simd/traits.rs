#![allow(dead_code)]

use crate::simd::errors::SimdResult;

pub(crate) trait SimdOps: Sized + Copy {
    fn add(a: &[Self], b: &[Self], out: &mut [Self]) -> SimdResult;
    fn sub(a: &[Self], b: &[Self], out: &mut [Self]) -> SimdResult;
    fn mul(a: &[Self], b: &[Self], out: &mut [Self]) -> SimdResult;
    fn div(a: &[Self], b: &[Self], out: &mut [Self]) -> SimdResult;
    // inplace operations
    fn add_inplace(out: &mut [Self], value: &[Self]) -> SimdResult;
    fn sub_inplace(out: &mut [Self], value: &[Self]) -> SimdResult;
    fn mul_inplace(out: &mut [Self], value: &[Self]) -> SimdResult;
    fn div_inplace(out: &mut [Self], value: &[Self]) -> SimdResult;
}
