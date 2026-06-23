// restrictions for types that array can store
pub trait ArrayElement: Copy + Default + PartialEq + std::fmt::Debug + 'static {}

// trait for arithmetic operations on different types, since f32, f64 implement SIMD vectorization
pub(crate) trait ElementArithmetcs: Copy {
    fn add(a: Self, b: Self) -> Self;
    fn sub(a: Self, b: Self) -> Self;
    fn mul(a: Self, b: Self) -> Self;
    fn div(a: Self, b: Self) -> Self;

    // default implementation
    fn add_slices(a: &[Self], b: &[Self], out: &mut [Self]) {
        out.iter_mut()
            .zip(a.into_iter().zip(b))
            .for_each(|(o, (&a, &b))| *o = Self::add(a, b));
    }
    fn sub_slices(a: &[Self], b: &[Self], out: &mut [Self]) {
        out.iter_mut()
            .zip(a.into_iter().zip(b))
            .for_each(|(o, (&a, &b))| *o = Self::sub(a, b));
    }
    fn mul_slices(a: &[Self], b: &[Self], out: &mut [Self]) {
        out.iter_mut()
            .zip(a.into_iter().zip(b))
            .for_each(|(o, (&a, &b))| *o = Self::mul(a, b));
    }
    fn div_slices(a: &[Self], b: &[Self], out: &mut [Self]) {
        out.iter_mut()
            .zip(a.into_iter().zip(b))
            .for_each(|(o, (&a, &b))| *o = Self::div(a, b));
    }
}
