use crate::{
    array::traits::ElementArithmetics,
    simd::traits::{ArithmeticOperation, SimdOps},
};

impl ElementArithmetics for i32 {
    fn arith(&self, b: Self, operation: ArithmeticOperation) -> Self {
        match operation {
            ArithmeticOperation::Addition => self + b,
            ArithmeticOperation::Subtraction => self - b,
            ArithmeticOperation::Multiplication => self * b,
            ArithmeticOperation::Division => self / b,
        }
    }
}

impl ElementArithmetics for f32 {
    fn arith(&self, b: Self, operation: ArithmeticOperation) -> Self {
        match operation {
            ArithmeticOperation::Addition => self + b,
            ArithmeticOperation::Subtraction => self - b,
            ArithmeticOperation::Multiplication => self * b,
            ArithmeticOperation::Division => self / b,
        }
    }
    fn slice_arith(a: &[Self], b: &[Self], out: &mut [Self], operation: ArithmeticOperation) {
        SimdOps::arithmetic(a, b, out, operation)
            .expect("slice length mismatch in SIMD arithmetic");
    }
}
