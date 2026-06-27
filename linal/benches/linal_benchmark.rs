#![allow(unused)]

use std::ops::Add;

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use linal::array::core::Array;

#[inline(never)]
fn add_vecs<T>(v1: &Vec<T>, v2: &Vec<T>) -> Vec<T>
where
    T: Copy + Add<Output = T> + Default,
{
    assert_eq!(v1.len(), v2.len());
    let mut out = vec![T::default(); v1.len()];
    for i in 0..v1.len() {
        out[i] = v1[i] + v2[i];
    }
    out
}

const N: usize = 1000;

fn simd_arrayf32_benchmark(c: &mut Criterion) {
    let arr_1_f32: Array<f32> = black_box(Array::from(vec![3.14f32; N]));
    let arr_2_f32: Array<f32> = black_box(Array::from(vec![3.14f32; N]));

    c.bench_function(
        &format!("simd f32 addition of two Array<f32> of size {}", N),
        |b| {
            b.iter(|| &arr_1_f32 + &arr_2_f32);
        },
    );
}

fn arrayi32_benchmark(c: &mut Criterion) {
    let arr_1_i32: Array<i32> = black_box(Array::from(vec![3i32; N]));
    let arr_2_i32: Array<i32> = black_box(Array::from(vec![3i32; N]));

    c.bench_function(&format!("Array<i32> scalar add, N={N}"), |b| {
        b.iter(|| &arr_1_i32 + &arr_2_i32);
    });
}

criterion_group!(simd_benches, simd_arrayf32_benchmark, arrayi32_benchmark);
criterion_main!(simd_benches);
