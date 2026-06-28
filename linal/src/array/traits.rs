use std::ops::{Add, Div, Mul, Sub};

pub trait ArrayElement: Copy + Default + PartialEq + std::fmt::Debug + 'static {}

pub(crate) trait SliceArith:
    Copy + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self>
{
    fn add_slices(a: &[Self], b: &[Self], out: &mut [Self]) {
        out.iter_mut()
            .zip(a.iter().zip(b))
            .for_each(|(o, (&a, &b))| *o = a + b);
    }

    fn sub_slices(a: &[Self], b: &[Self], out: &mut [Self]) {
        out.iter_mut()
            .zip(a.iter().zip(b))
            .for_each(|(o, (&a, &b))| *o = a - b);
    }

    fn mul_slices(a: &[Self], b: &[Self], out: &mut [Self]) {
        out.iter_mut()
            .zip(a.iter().zip(b))
            .for_each(|(o, (&a, &b))| *o = a * b);
    }

    fn div_slices(a: &[Self], b: &[Self], out: &mut [Self]) {
        out.iter_mut()
            .zip(a.iter().zip(b))
            .for_each(|(o, (&a, &b))| *o = a / b);
    }
}
