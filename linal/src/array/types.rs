use crate::{
    array::traits::ElementArithmetcs,
    simd::traits::{ArithmeticOperation, SimdOps},
};

impl ElementArithmetcs for i32 {
    fn arith(&self, b: Self, operation: ArithmeticOperation) -> Self {
        match operation {
            ArithmeticOperation::Addition => self + b,
            ArithmeticOperation::Subtraction => self - b,
            ArithmeticOperation::Multiplication => self * b,
            _ => self / b,
        }
    }
}

impl ElementArithmetcs for f32 {
    fn arith(&self, b: Self, operation: ArithmeticOperation) -> Self {
        match operation {
            ArithmeticOperation::Addition => self + b,
            ArithmeticOperation::Subtraction => self - b,
            ArithmeticOperation::Multiplication => self * b,
            _ => self / b,
        }
    }
    fn slice_arith(a: &[Self], b: &[Self], out: &mut [Self], operation: ArithmeticOperation) {
        SimdOps::arithmetic(&a, &b, out, operation).unwrap(); // add error handling later
    }
}
