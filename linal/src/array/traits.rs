use crate::simd::traits::ArithmeticOperation;

// restrictions for types that array can store
pub trait ArrayElement: Copy + Default + PartialEq + std::fmt::Debug + 'static {}

// trait for arithmetic operations on different types, since f32, f64 implement SIMD vectorization
pub(crate) trait ElementArithmetcs: Copy {
    fn arith(&self, b: Self, operation: ArithmeticOperation) -> Self;

    // default implementation
    fn slice_arith(a: &[Self], b: &[Self], out: &mut [Self], operation: ArithmeticOperation) {
        out.iter_mut()
            .zip(a.into_iter().zip(b))
            .for_each(|(o, (&a, &b))| {
                *o = match operation {
                    ArithmeticOperation::Addition => a.arith(b, ArithmeticOperation::Addition),
                    ArithmeticOperation::Subtraction => {
                        a.arith(b, ArithmeticOperation::Subtraction)
                    }
                    ArithmeticOperation::Multiplication => {
                        a.arith(b, ArithmeticOperation::Multiplication)
                    }
                    _ => a.arith(b, ArithmeticOperation::Division),
                }
            });
    }
}
