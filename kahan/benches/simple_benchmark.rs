use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::*;

use kahan::{sum, sum_simd};

pub fn criterion_benchmark(c: &mut Criterion) {
    // Generate a vector with random float values.
    let mut rng = thread_rng();
    let size = 1875000;
    let mut input = vec![0.0; size];
    for i in 0..size {
        input[i] = rng.gen();
    }

    // Run benchmark on the simple and SIMD implementation.
    let mut group = c.benchmark_group("Kahan Summation");

    group.bench_with_input(BenchmarkId::new("Simple", input.len()), &input, |b, i| {
        b.iter(|| sum(black_box(i)))
    });
    group.bench_with_input(BenchmarkId::new("SIMD", input.len()), &input, |b, i| {
        b.iter(|| sum_simd(black_box(i)))
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
