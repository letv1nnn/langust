use crate::simd::traits::ArithmeticOperation;

pub trait ArrayElement: Copy + Default + PartialEq + std::fmt::Debug + 'static {}

pub(crate) trait ElementArithmetics: Copy {
    fn arith(&self, b: Self, operation: ArithmeticOperation) -> Self;

    fn slice_arith(a: &[Self], b: &[Self], out: &mut [Self], operation: ArithmeticOperation) {
        out.iter_mut()
            .zip(a.iter().zip(b))
            .for_each(|(o, (&a, &b))| *o = a.arith(b, operation));
    }
}
