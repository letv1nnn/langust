use crate::{
    array::traits::SliceArith,
    simd::traits::{ArithmeticOperation, SimdOps},
};

impl SliceArith for i32 {}
impl SliceArith for i64 {}
impl SliceArith for u32 {}
impl SliceArith for u64 {}

impl SliceArith for f32 {
    fn add_slices(a: &[Self], b: &[Self], out: &mut [Self]) {
        SimdOps::arithmetic(a, b, out, ArithmeticOperation::Addition)
            .expect("slice length mismatch in SIMD arithmetic");
    }
    fn sub_slices(a: &[Self], b: &[Self], out: &mut [Self]) {
        SimdOps::arithmetic(a, b, out, ArithmeticOperation::Subtraction)
            .expect("slice length mismatch in SIMD arithmetic");
    }
    fn mul_slices(a: &[Self], b: &[Self], out: &mut [Self]) {
        SimdOps::arithmetic(a, b, out, ArithmeticOperation::Multiplication)
            .expect("slice length mismatch in SIMD arithmetic");
    }
    fn div_slices(a: &[Self], b: &[Self], out: &mut [Self]) {
        SimdOps::arithmetic(a, b, out, ArithmeticOperation::Division)
            .expect("slice length mismatch in SIMD arithmetic");
    }
}
