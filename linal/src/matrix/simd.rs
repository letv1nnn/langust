#![allow(unused)]

#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[derive(Clone, Copy)]
pub(crate) enum ArithmeticOperation {
    Addition,
    Subtraction,
}

pub(crate) struct Simd;

// public api
impl Simd {
    pub(crate) fn arithmetics_f32(
        v1: &[f32],
        v2: &[f32],
        out: &mut [f32],
        operation: ArithmeticOperation,
    ) {
        #[cfg(target_arch = "aarch64")]
        unsafe {
            Simd::arithmetics_f32_aarch64(v1, v2, out, operation);
        }
        #[cfg(target_arch = "x86_64")]
        unsafe {
            Simd::arithmetics_f32_x86_64(v1, v2, out, operation);
        }
    }

    pub(crate) fn arithmetics_f32_inplace(
        out: &mut [f32],
        v2: &[f32],
        operation: ArithmeticOperation,
    ) {
        #[cfg(target_arch = "aarch64")]
        unsafe {
            Simd::arithmetics_f32_inplace_aarch64(out, v2, operation);
        }
        #[cfg(target_arch = "x86_64")]
        unsafe {
            Simd::arithmetics_f32_inplace_x86_64(out, v2, operation);
        }
    }
}

// private api
impl Simd {
    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "neon")]
    fn arithmetics_f32_aarch64(
        v1: &[f32],
        v2: &[f32],
        out: &mut [f32],
        operation: ArithmeticOperation,
    ) {
        let mut i = 0usize;
        while i + 4usize <= v1.len() {
            let v1_simd = unsafe { vld1q_f32(v1.as_ptr().add(i)) };
            let v2_simd = unsafe { vld1q_f32(v2.as_ptr().add(i)) };

            let result = match operation {
                ArithmeticOperation::Addition => vaddq_f32(v1_simd, v2_simd),
                ArithmeticOperation::Subtraction => vsubq_f32(v1_simd, v2_simd),
            };

            unsafe {
                vst1q_f32(out.as_mut_ptr().add(i), result);
            }
            i += 4;
        }

        Self::scalar_tail_f32(v1, v2, out, i, operation);
    }

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse")]
    fn arithmetics_f32_x86_64(
        v1: &[f32],
        v2: &[f32],
        out: &mut [f32],
        operation: ArithmeticOperation,
    ) {
        let mut i = 0usize;
        while i + 8usize <= v1.len() {
            let v1_simd = unsafe { _mm256_loadu_ps(v1.as_ptr().add(i)) };
            let v2_simd = unsafe { _mm256_loadu_ps(v2.as_ptr().add(i)) };

            let result = match operation {
                ArithmeticOperation::Addition => unsafe { _mm256_add_ps(v1_simd, v2_simd) },
                ArithmeticOperation::Subtraction => unsafe { _mm256_sub_ps(v1_simd, v2_simd) },
            };

            unsafe {
                _mm256_store_ps(out.as_mut_ptr().add(i), result);
            }
            i += 8usize;
        }

        Self::scalar_tail_f32(v1, v2, out, i, operation);
    }

    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "neon")]
    fn arithmetics_f32_inplace_aarch64(
        out: &mut [f32],
        v2: &[f32],
        operation: ArithmeticOperation,
    ) {
        let mut i = 0usize;
        while i + 4usize <= out.len() {
            let out_simd = unsafe { vld1q_f32(out.as_ptr().add(i)) };
            let v2_simd = unsafe { vld1q_f32(v2.as_ptr().add(i)) };

            let result = match operation {
                ArithmeticOperation::Addition => vaddq_f32(out_simd, v2_simd),
                ArithmeticOperation::Subtraction => vsubq_f32(out_simd, v2_simd),
            };

            unsafe {
                vst1q_f32(out.as_mut_ptr().add(i), result);
            }
            i += 4;
        }

        let op: fn(f32, f32) -> f32 = match operation {
            ArithmeticOperation::Addition => |a, b| a + b,
            ArithmeticOperation::Subtraction => |a, b| a - b,
        };
        while i < out.len() {
            out[i] = op(out[i], v2[i]);
            i += 1;
        }
    }

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse")]
    fn arithmetics_f32_inplace_x86_64(out: &mut [f32], v2: &[f32], operation: ArithmeticOperation) {
        let mut i = 0usize;
        while i + 8usize <= out.len() {
            let out_simd = unsafe { _mm256_loadu_ps(out.as_ptr().add(i)) };
            let v2_simd = unsafe { _mm256_loadu_ps(v2.as_ptr().add(i)) };

            let result = match operation {
                ArithmeticOperation::Addition => unsafe { _mm256_add_ps(out_simd, v2_simd) },
                ArithmeticOperation::Subtraction => unsafe { _mm256_add_ps(out_simd, v2_simd) },
            };

            unsafe {
                _mm256_storeu_ps(out.as_mut_ptr().add(i), result);
            }
            i += 8usize;
        }

        let op: fn(f32, f32) -> f32 = match operation {
            ArithmeticOperation::Addition => |a, b| a + b,
            ArithmeticOperation::Subtraction => |a, b| a - b,
        };
        while i < out.len() {
            out[i] = op(out[i], v2[i]);
            i += 1;
        }
    }
    fn scalar_tail_f32(
        v1: &[f32],
        v2: &[f32],
        out: &mut [f32],
        start: usize,
        operation: ArithmeticOperation,
    ) {
        let op: fn(f32, f32) -> f32 = match operation {
            ArithmeticOperation::Addition => |a, b| a + b,
            ArithmeticOperation::Subtraction => |a, b| a - b,
        };
        for i in start..v1.len() {
            out[i] = op(v1[i], v2[i]);
        }
    }
}

#[cfg(test)]
mod simd_arithmetic_operation_tests {
    use super::*;

    fn reference_arithmetics(
        a: &[f32],
        b: &[f32],
        out: &mut [f32],
        operation: ArithmeticOperation,
    ) {
        for i in 0..a.len() {
            match operation {
                ArithmeticOperation::Addition => out[i] = a[i] + b[i],
                ArithmeticOperation::Subtraction => out[i] = a[i] - b[i],
            }
        }
    }

    #[test]
    fn addition() {
        let (v1, v2) = (vec![1.0f32, 2.0f32, 3.0], vec![4.0f32, 5.0f32, 6.0f32]);
        let (mut actual, mut expected) = (vec![0.0; 3], vec![0.0; 3]);

        reference_arithmetics(&v1, &v2, &mut actual, ArithmeticOperation::Addition);
        Simd::arithmetics_f32(&v1, &v2, &mut expected, ArithmeticOperation::Addition);
    }

    #[test]
    fn subtraction() {
        let (v1, v2) = (vec![1.0f32, 2.0f32, 3.0], vec![4.0f32, 5.0f32, 6.0f32]);
        let (mut actual, mut expected) = (vec![0.0; 3], vec![0.0; 3]);

        reference_arithmetics(&v1, &v2, &mut actual, ArithmeticOperation::Subtraction);
        Simd::arithmetics_f32(&v1, &v2, &mut expected, ArithmeticOperation::Subtraction);
    }
}
