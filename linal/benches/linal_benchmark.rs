#![allow(unused)]

use std::ops::Add;

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use linal::array::core::Array;

#[inline]
fn add_vecsf32(v1: &Vec<f32>, v2: &Vec<f32>) -> Vec<f32> {
    assert_eq!(v1.len(), v2.len());
    let mut out = vec![f32::default(); v1.len()];
    for i in 0..v1.len() {
        out[i] = v1[i] + v2[i];
    }
    out
}

fn simd_f32_benchmark(c: &mut Criterion) {
    const N: usize = 1000;

    let (v1, v2) = black_box((vec![3.14f32; N], vec![3.14f32; N]));

    let arr_1: Array<f32> = black_box(Array::from(vec![3.14f32; N]));
    let arr_2: Array<f32> = black_box(Array::from(vec![3.14f32; N]));

    c.bench_function(
        &format!("simd f32 addition of two Array<f32> of size {}", N),
        |b| {
            b.iter(|| &arr_1 + &arr_2);
        },
    );

    c.bench_function(&format!("Vec<f32> scalar add, N={N}"), |b| {
        b.iter(|| black_box(add_vecsf32(&v1, &v2)))
    });
}

criterion_group!(simd_benches, simd_f32_benchmark);
criterion_main!(simd_benches);
