use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hamming_code_simd::hamming::{decode, encode};

pub fn benchmark_encode(c: &mut Criterion) {
    c.bench_function("hamming encode", |b| b.iter(|| encode(black_box(&mut 20))));
}

criterion_group!(benches, benchmark_encode);
criterion_main!(benches);
