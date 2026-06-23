use crate::{
    array::traits::ElementArithmetcs,
    simd::traits::{ArithmeticOperation, SimdOps},
};

impl ElementArithmetcs for i32 {
    fn add(a: Self, b: Self) -> Self {
        a + b
    }
    fn sub(a: Self, b: Self) -> Self {
        a - b
    }
    fn mul(a: Self, b: Self) -> Self {
        a * b
    }
    fn div(a: Self, b: Self) -> Self {
        a / b
    }
}

// TODO: implement for other primitives
// for i64, u32, u64
impl ElementArithmetcs for f32 {
    fn add(a: Self, b: Self) -> Self {
        a + b
    }
    fn sub(a: Self, b: Self) -> Self {
        a - b
    }
    fn mul(a: Self, b: Self) -> Self {
        a * b
    }
    fn div(a: Self, b: Self) -> Self {
        a / b
    }
    // implementing SIMD vectorization
    fn add_slices(a: &[Self], b: &[Self], out: &mut [Self]) {
        SimdOps::arithmetic(&a, &b, out, ArithmeticOperation::Addition).unwrap(); // unwrap for now, need to find a way to handle the SIMD ops error
    }
    fn sub_slices(a: &[Self], b: &[Self], out: &mut [Self]) {
        SimdOps::arithmetic(&a, &b, out, ArithmeticOperation::Subtraction).unwrap(); // unwrap for now, need to find a way to handle the SIMD ops error
    }
    fn mul_slices(a: &[Self], b: &[Self], out: &mut [Self]) {
        SimdOps::arithmetic(&a, &b, out, ArithmeticOperation::Multiplication).unwrap(); // unwrap for now, need to find a way to handle the SIMD ops error
    }
    fn div_slices(a: &[Self], b: &[Self], out: &mut [Self]) {
        SimdOps::arithmetic(&a, &b, out, ArithmeticOperation::Division).unwrap(); // unwrap for now, need to find a way to handle the SIMD ops error
    }
}
