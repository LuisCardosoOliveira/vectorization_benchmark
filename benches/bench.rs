use criterion::{black_box, criterion_group, criterion_main, Criterion};
pub fn mul_arrays_non_vectorized(a: &[f32], b: &[f32]) -> f32 {
    let mut result = 0.0;
    for i in 0..a.len() {
        result += a[i] * b[i];
    }
    result
}

pub fn mul_arrays_vectorized(a: &[f32], b: &[f32], c: &mut [f32]) {
    for ((&x, &y), z) in a.iter().zip(b.iter()).zip(c.iter_mut()) {
        *z = x * y;
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum_arrays");

    let len = 1024;
    let a: Vec<f32> = vec![1.0; len];
    let y: Vec<f32> = vec![2.0; len];
    let mut c: Vec<f32> = vec![0.0; len];

    group.bench_function("non_vectorized", |b| {
        b.iter(|| mul_arrays_non_vectorized(black_box(&a), black_box(&y)))
    });

    group.bench_function("vectorized", |b| {
        b.iter(|| mul_arrays_vectorized(black_box(&a), black_box(&y), black_box(&mut c)))
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
