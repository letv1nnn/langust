use std::arch::x86_64::*;

#[target_feature(enable = "avx512f")]
unsafe fn add8_f64(v1: *const f64, v2: *const f64, out: *mut f64) {
    unsafe {
        let va = _mm512_loadu_pd(v1);
        let vb = _mm512_loadu_pd(v2);
        _mm512_storeu_pd(out, _mm512_add_pd(va, vb))
    }
}

unsafe fn add_simd_x86_64(v1: *const f64, v2: *const f64, out: *mut f64, len: usize) {
    let mut i = 0usize;
    while i + 8 <= len {
        unsafe {
            add8_f64(v1.add(i), v2.add(i), out.add(i));
        }
        i += 8;
    }
    while i < len {
        unsafe {
            *out.add(i) = *v1.add(i) + *v2.add(i);
        }
        i += 1;
    }
}

pub(crate) fn add_kernel(a: &[f64], b: &[f64], out: &mut [f64]) {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx512f") {
            return unsafe { add_simd_x86_64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len()) };
        }
    }

    for i in 0..a.len() {
        out[i] = a[i] + b[i];
    }
}

#[cfg(test)]
mod simd_tests {
    use super::*;

    fn reference_add(a: &[f64], b: &[f64], out: &mut [f64]) {
        for i in 0..a.len() {
            out[i] = a[i] + b[i];
        }
    }

    #[test]
    fn addition_small() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        let mut actual = vec![0.0; 3];
        let mut expected = vec![0.0; 3];

        add_kernel(&a, &b, &mut actual);
        reference_add(&a, &b, &mut expected);

        assert_eq!(actual, expected);
    }

    #[test]
    fn addition_exact_simd_block() {
        let a: Vec<f64> = (0..8).map(|x| x as f64).collect();
        let b: Vec<f64> = (0..8).map(|x| (x * 2) as f64).collect();

        let mut actual = vec![0.0; 8];
        let mut expected = vec![0.0; 8];

        add_kernel(&a, &b, &mut actual);
        reference_add(&a, &b, &mut expected);

        assert_eq!(actual, expected);
    }
}
