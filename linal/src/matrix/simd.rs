use std::arch::x86_64::*;

// TODO: implement for AVX registers.

#[derive(Clone, Copy)]
pub(crate) enum ArithmeticOperation {
    Addition,
    Subtraction,
}

#[target_feature(enable = "avx512f")]
fn raw_simd_arithmetics_on_8_f64(
    v1: *const f64,
    v2: *const f64,
    out: *mut f64,
    operation: ArithmeticOperation,
) {
    unsafe {
        let va = _mm512_loadu_pd(v1);
        let vb = _mm512_loadu_pd(v2);
        match operation {
            ArithmeticOperation::Addition => _mm512_storeu_pd(out, _mm512_add_pd(va, vb)),
            ArithmeticOperation::Subtraction => _mm512_storeu_pd(out, _mm512_sub_pd(va, vb)),
        }
    }
}

fn raw_simd_arithmetics_x86_64(
    v1: *const f64,
    v2: *const f64,
    out: *mut f64,
    len: usize,
    operation: ArithmeticOperation,
) {
    let mut i = 0usize;
    while i + 8 <= len {
        unsafe {
            raw_simd_arithmetics_on_8_f64(v1.add(i), v2.add(i), out.add(i), operation);
        }
        i += 8;
    }
    while i < len {
        unsafe {
            *out.add(i) = match operation {
                ArithmeticOperation::Addition => *v1.add(i) + *v2.add(i),
                ArithmeticOperation::Subtraction => *v1.add(i) - *v2.add(i),
            };
        }
        i += 1;
    }
}

pub(crate) fn kernel_arithmetics(
    a: &[f64],
    b: &[f64],
    out: &mut [f64],
    operation: ArithmeticOperation,
) {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx512f") {
            return raw_simd_arithmetics_x86_64(
                a.as_ptr(),
                b.as_ptr(),
                out.as_mut_ptr(),
                a.len(),
                operation,
            );
        }
    }

    for i in 0..a.len() {
        out[i] = match operation {
            ArithmeticOperation::Addition => a[i] + b[i],
            ArithmeticOperation::Subtraction => a[i] - b[i],
        }
    }
}

#[cfg(test)]
mod simd_tests {
    use super::*;

    fn reference_arithmetics(
        a: &[f64],
        b: &[f64],
        out: &mut [f64],
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
    fn addition_small() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        let mut actual = vec![0.0; 3];
        let mut expected = vec![0.0; 3];

        kernel_arithmetics(&a, &b, &mut actual, ArithmeticOperation::Addition);
        reference_arithmetics(&a, &b, &mut expected, ArithmeticOperation::Addition);

        assert_eq!(actual, expected);
    }

    #[test]
    fn subtraction_small() {
        let a = vec![10.0, 20.0, 30.0];
        let b = vec![1.0, 2.0, 3.0];

        let mut actual = vec![0.0; 3];
        let mut expected = vec![0.0; 3];

        kernel_arithmetics(&a, &b, &mut actual, ArithmeticOperation::Subtraction);
        reference_arithmetics(&a, &b, &mut expected, ArithmeticOperation::Subtraction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn addition_exact_simd_block() {
        let a: Vec<f64> = (0..8).map(|x| x as f64).collect();
        let b: Vec<f64> = (0..8).map(|x| (x * 2) as f64).collect();

        let mut actual = vec![0.0; 8];
        let mut expected = vec![0.0; 8];

        kernel_arithmetics(&a, &b, &mut actual, ArithmeticOperation::Addition);
        reference_arithmetics(&a, &b, &mut expected, ArithmeticOperation::Addition);

        assert_eq!(actual, expected);
    }

    #[test]
    fn subtraction_exact_simd_block() {
        let a: Vec<f64> = (0..8).map(|x| (x * 5) as f64).collect();
        let b: Vec<f64> = (0..8).map(|x| x as f64).collect();

        let mut actual = vec![0.0; 8];
        let mut expected = vec![0.0; 8];

        kernel_arithmetics(&a, &b, &mut actual, ArithmeticOperation::Subtraction);
        reference_arithmetics(&a, &b, &mut expected, ArithmeticOperation::Subtraction);

        assert_eq!(actual, expected);
    }
}
