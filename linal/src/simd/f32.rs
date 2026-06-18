use crate::simd::errors::{SimdResult, check_for_length_mismatch};
use crate::simd::traits::ArithmeticOperation;

use super::traits::SimdOps;

#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

impl SimdOps for f32 {
    fn arithmetic(a: &[Self], b: &[Self], out: &mut [Self], op: ArithmeticOperation) -> SimdResult {
        check_for_length_mismatch(a, b)?;
        check_for_length_mismatch(a, out)?;

        #[cfg(target_arch = "aarch64")]
        unsafe {
            dispatch_aarch64(a, b, out, op);
        }
        #[cfg(target_arch = "x86_64")]
        unsafe {
            dispatch_x86_64(a, b, out, op);
        }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        scalar_tail(a, b, out, 0, op);

        Ok(())
    }

    fn arithmetic_inplace(out: &mut [Self], value: &[Self], op: ArithmeticOperation) -> SimdResult {
        check_for_length_mismatch(out, value)?;

        #[cfg(target_arch = "aarch64")]
        unsafe {
            dispatch_inplace_aarch64(out, value, op);
        }
        #[cfg(target_arch = "x86_64")]
        unsafe {
            dispatch_inplace_x86_64(out, value, op);
        }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        scalar_tail_inplace(out, value, 0, op);

        Ok(())
    }
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
fn dispatch_aarch64(a: &[f32], b: &[f32], out: &mut [f32], op: ArithmeticOperation) {
    let mut i = 0usize;
    while i + 4usize <= a.len() {
        let a_simd = unsafe { vld1q_f32(a.as_ptr().add(i)) };
        let b_simd = unsafe { vld1q_f32(b.as_ptr().add(i)) };

        let result = match op {
            ArithmeticOperation::Addition => vaddq_f32(a_simd, b_simd),
            ArithmeticOperation::Subtraction => vsubq_f32(a_simd, b_simd),
            ArithmeticOperation::Multiplication => vmulq_f32(a_simd, b_simd),
            ArithmeticOperation::Division => vdivq_f32(a_simd, b_simd),
        };

        unsafe {
            vst1q_f32(out.as_mut_ptr().add(i), result);
        }
        i += 4usize;
    }

    scalar_tail(a, b, out, i, op);
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
fn dispatch_x86_64(a: &[f32], b: &[f32], out: &mut [f32], op: ArithmeticOperation) {
    let mut i = 0usize;
    while i + 8usize <= a.len() {
        let a_simd = unsafe { _mm256_loadu_ps(a.as_ptr().add(i)) };
        let b_simd = unsafe { _mm256_loadu_ps(b.as_ptr().add(i)) };

        let result = match op {
            ArithmeticOperation::Addition => unsafe { _mm256_add_ps(a_simd, b_simd) },
            ArithmeticOperation::Subtraction => unsafe { _mm256_sub_ps(a_simd, b_simd) },
            ArithmeticOperation::Multiplication => unsafe { _mm256_mul_ps(a_simd, b_simd) },
            ArithmeticOperation::Division => unsafe { _mm256_div_ps(a_simd, b_simd) },
        };

        unsafe {
            _mm256_storeu_ps(out.as_mut_ptr().add(i), result);
        }
        i += 8usize;
    }

    scalar_tail(a, b, out, i, op);
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
fn dispatch_inplace_aarch64(out: &mut [f32], value: &[f32], op: ArithmeticOperation) {
    let mut i = 0usize;
    while i + 4usize <= out.len() {
        let out_simd = unsafe { vld1q_f32(out.as_ptr().add(i)) };
        let value_simd = unsafe { vld1q_f32(value.as_ptr().add(i)) };

        let result = match op {
            ArithmeticOperation::Addition => vaddq_f32(out_simd, value_simd),
            ArithmeticOperation::Subtraction => vsubq_f32(out_simd, value_simd),
            ArithmeticOperation::Multiplication => vmulq_f32(out_simd, value_simd),
            ArithmeticOperation::Division => vdivq_f32(out_simd, value_simd),
        };

        unsafe {
            vst1q_f32(out.as_mut_ptr().add(i), result);
        }
        i += 4usize;
    }

    scalar_tail_inplace(out, value, i, op);
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
fn dispatch_inplace_x86_64(out: &mut [f32], value: &[f32], op: ArithmeticOperation) {
    let mut i = 0usize;
    while i + 8usize <= out.len() {
        let out_simd = unsafe { _mm256_loadu_ps(out.as_ptr().add(i)) };
        let value_simd = unsafe { _mm256_loadu_ps(value.as_ptr().add(i)) };

        let result = match op {
            ArithmeticOperation::Addition => unsafe { _mm256_add_ps(out_simd, value_simd) },
            ArithmeticOperation::Subtraction => unsafe { _mm256_sub_ps(out_simd, value_simd) },
            ArithmeticOperation::Multiplication => unsafe { _mm256_mul_ps(out_simd, value_simd) },
            ArithmeticOperation::Division => unsafe { _mm256_div_ps(out_simd, value_simd) },
        };

        unsafe {
            _mm256_storeu_ps(out.as_mut_ptr().add(i), result);
        }
        i += 8usize;
    }

    scalar_tail_inplace(out, value, i, op);
}

fn scalar_tail(a: &[f32], b: &[f32], out: &mut [f32], start: usize, op: ArithmeticOperation) {
    for i in start..a.len() {
        out[i] = match op {
            ArithmeticOperation::Addition => a[i] + b[i],
            ArithmeticOperation::Subtraction => a[i] - b[i],
            ArithmeticOperation::Multiplication => a[i] * b[i],
            ArithmeticOperation::Division => a[i] / b[i],
        };
    }
}

fn scalar_tail_inplace(out: &mut [f32], value: &[f32], start: usize, op: ArithmeticOperation) {
    for i in start..out.len() {
        match op {
            ArithmeticOperation::Addition => out[i] += value[i],
            ArithmeticOperation::Subtraction => out[i] -= value[i],
            ArithmeticOperation::Multiplication => out[i] *= value[i],
            ArithmeticOperation::Division => out[i] /= value[i],
        }
    }
}

#[cfg(test)]
mod f32_simdops_tests {
    use super::*;
    use crate::simd::errors::SimdError;

    fn reference_op(a: &[f32], b: &[f32], out: &mut [f32], op: ArithmeticOperation) {
        assert!(
            a.len() == b.len() && a.len() == out.len(),
            "vectors have different length"
        );
        scalar_tail(a, b, out, 0usize, op);
    }

    fn reference_op_inplace(out: &mut [f32], b: &[f32], op: ArithmeticOperation) {
        assert!(out.len() == b.len(), "vectors have different length");
        scalar_tail_inplace(out, b, 0usize, op);
    }

    #[test]
    fn addition() {
        let v1 = vec![1.2, 2.3, 3.4, 4.5, 5.6, 6.7, 7.8, 8.9, 9.0];
        let v2 = vec![9.0, 8.9, 7.8, 6.7, 5.6, 4.5, 3.4, 2.3, 1.2];

        let (mut actual, mut expected) = (vec![0.0f32; v1.len()], vec![0.0f32; v1.len()]);
        reference_op(&v1, &v2, &mut expected, ArithmeticOperation::Addition);
        f32::arithmetic(&v1, &v2, &mut actual, ArithmeticOperation::Addition).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn addition_inplace() {
        let v1 = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let v2 = vec![9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];

        let mut expected = v1.clone();
        reference_op_inplace(&mut expected, &v2, ArithmeticOperation::Addition);

        let mut actual = v1;
        f32::arithmetic_inplace(&mut actual, &v2, ArithmeticOperation::Addition).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn subtraction() {
        let v1 = vec![10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0];
        let v2 = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];

        let (mut actual, mut expected) = (vec![0.0f32; v1.len()], vec![0.0f32; v1.len()]);
        reference_op(&v1, &v2, &mut expected, ArithmeticOperation::Subtraction);
        f32::arithmetic(&v1, &v2, &mut actual, ArithmeticOperation::Subtraction).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn subtraction_inplace() {
        let v1 = vec![10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0];
        let v2 = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];

        let mut expected = v1.clone();
        reference_op_inplace(&mut expected, &v2, ArithmeticOperation::Subtraction);

        let mut actual = v1;
        f32::arithmetic_inplace(&mut actual, &v2, ArithmeticOperation::Subtraction).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn multiplication() {
        let v1 = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let v2 = vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];

        let (mut actual, mut expected) = (vec![0.0f32; v1.len()], vec![0.0f32; v1.len()]);
        reference_op(&v1, &v2, &mut expected, ArithmeticOperation::Multiplication);
        f32::arithmetic(&v1, &v2, &mut actual, ArithmeticOperation::Multiplication).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn multiplication_inplace() {
        let v1 = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let v2 = vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];

        let mut expected = v1.clone();
        reference_op_inplace(&mut expected, &v2, ArithmeticOperation::Multiplication);

        let mut actual = v1;
        f32::arithmetic_inplace(&mut actual, &v2, ArithmeticOperation::Multiplication).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn division() {
        let v1 = vec![10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0];
        let v2 = vec![2.0, 4.0, 5.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0];

        let (mut actual, mut expected) = (vec![0.0f32; v1.len()], vec![0.0f32; v1.len()]);
        reference_op(&v1, &v2, &mut expected, ArithmeticOperation::Division);
        f32::arithmetic(&v1, &v2, &mut actual, ArithmeticOperation::Division).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn division_inplace() {
        let v1 = vec![10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0];
        let v2 = vec![2.0, 4.0, 5.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0];

        let mut expected = v1.clone();
        reference_op_inplace(&mut expected, &v2, ArithmeticOperation::Division);

        let mut actual = v1;
        f32::arithmetic_inplace(&mut actual, &v2, ArithmeticOperation::Division).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn division_by_zero_produces_inf() {
        let v1 = vec![1.0, 2.0, 3.0];
        let v2 = vec![0.0, 0.0, 0.0];
        let mut out = vec![0.0f32; 3];

        f32::arithmetic(&v1, &v2, &mut out, ArithmeticOperation::Division).unwrap();

        for val in &out {
            assert!(val.is_infinite());
        }
    }

    #[test]
    fn empty_vectors() {
        let (v1, v2): (Vec<f32>, Vec<f32>) = (vec![], vec![]);
        let mut out: Vec<f32> = vec![];

        f32::arithmetic(&v1, &v2, &mut out, ArithmeticOperation::Addition).unwrap();
        assert!(out.is_empty());

        f32::arithmetic_inplace(&mut out, &v2, ArithmeticOperation::Addition).unwrap();
        assert!(out.is_empty());
    }

    #[test]
    fn length_one() {
        let (v1, v2) = (vec![3.0f32], vec![7.0f32]);
        let mut out = vec![0.0f32; 1];

        f32::arithmetic(&v1, &v2, &mut out, ArithmeticOperation::Addition).unwrap();
        assert_eq!(out, vec![10.0]);

        f32::arithmetic(&v1, &v2, &mut out, ArithmeticOperation::Multiplication).unwrap();
        assert_eq!(out, vec![21.0]);
    }

    #[test]
    fn length_exact_simd_width() {
        let v1 = vec![1.0f32; 8];
        let v2 = vec![2.0f32; 8];
        let mut out = vec![0.0f32; 8];

        f32::arithmetic(&v1, &v2, &mut out, ArithmeticOperation::Addition).unwrap();
        assert_eq!(out, vec![3.0f32; 8]);

        f32::arithmetic(&v1, &v2, &mut out, ArithmeticOperation::Multiplication).unwrap();
        assert_eq!(out, vec![2.0f32; 8]);
    }

    #[test]
    fn length_simd_width_plus_tail() {
        let v1 = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0];
        let v2 = vec![1.0f32; 11];
        let mut out = vec![0.0f32; 11];

        f32::arithmetic(&v1, &v2, &mut out, ArithmeticOperation::Subtraction).unwrap();
        let expected: Vec<f32> = (0..11).map(|i| i as f32).collect();
        assert_eq!(out, expected);
    }

    #[test]
    fn large_vector() {
        let n = 1024;
        let v1: Vec<f32> = (0..n).map(|i| i as f32).collect();
        let v2 = vec![1.0f32; n];
        let mut out = vec![0.0f32; n];

        f32::arithmetic(&v1, &v2, &mut out, ArithmeticOperation::Addition).unwrap();
        for i in 0..n {
            assert_eq!(out[i], i as f32 + 1.0);
        }
    }

    #[test]
    fn length_mismatch_returns_error() {
        let (v1, v2) = (vec![1.0f32; 4], vec![1.0f32; 3]);
        let mut out = vec![0.0f32; 4];

        let err = f32::arithmetic(&v1, &v2, &mut out, ArithmeticOperation::Addition).unwrap_err();
        assert_eq!(
            err,
            SimdError::LengthMismatch {
                expected: 4,
                got: 3
            }
        );
    }

    #[test]
    fn length_mismatch_out_returns_error() {
        let (v1, v2) = (vec![1.0f32; 4], vec![1.0f32; 4]);
        let mut out = vec![0.0f32; 2];

        let err = f32::arithmetic(&v1, &v2, &mut out, ArithmeticOperation::Addition).unwrap_err();
        assert_eq!(
            err,
            SimdError::LengthMismatch {
                expected: 4,
                got: 2
            }
        );
    }

    #[test]
    fn length_mismatch_inplace_returns_error() {
        let v2 = vec![1.0f32; 3];
        let mut out = vec![0.0f32; 5];

        let err =
            f32::arithmetic_inplace(&mut out, &v2, ArithmeticOperation::Addition).unwrap_err();
        assert_eq!(
            err,
            SimdError::LengthMismatch {
                expected: 5,
                got: 3
            }
        );
    }

    #[test]
    fn all_ops_length_mismatch() {
        let (v1, v2) = (vec![1.0f32; 4], vec![1.0f32; 2]);
        let mut out = vec![0.0f32; 4];

        assert!(f32::arithmetic(&v1, &v2, &mut out, ArithmeticOperation::Subtraction).is_err());
        assert!(f32::arithmetic(&v1, &v2, &mut out, ArithmeticOperation::Multiplication).is_err());
        assert!(f32::arithmetic(&v1, &v2, &mut out, ArithmeticOperation::Division).is_err());
        assert!(f32::arithmetic_inplace(&mut out, &v2, ArithmeticOperation::Subtraction).is_err());
        assert!(
            f32::arithmetic_inplace(&mut out, &v2, ArithmeticOperation::Multiplication).is_err()
        );
        assert!(f32::arithmetic_inplace(&mut out, &v2, ArithmeticOperation::Division).is_err());
    }

    #[test]
    fn negative_values() {
        let v1 = vec![-1.0, -2.0, -3.0, -4.0, -5.0, -6.0, -7.0, -8.0, -9.0];
        let v2 = vec![-9.0, -8.0, -7.0, -6.0, -5.0, -4.0, -3.0, -2.0, -1.0];

        let (mut actual, mut expected) = (vec![0.0f32; v1.len()], vec![0.0f32; v1.len()]);
        reference_op(&v1, &v2, &mut expected, ArithmeticOperation::Multiplication);
        f32::arithmetic(&v1, &v2, &mut actual, ArithmeticOperation::Multiplication).unwrap();

        assert_eq!(actual, expected);
    }
}
