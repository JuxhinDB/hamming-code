use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hamming_code_simd::hamming::{decode, encode};
use rand::Rng;

pub fn benchmark_encode(c: &mut Criterion) {
    let mut input = rand::thread_rng().gen::<u64>();
    c.bench_function("hamming encode", |b| {
        b.iter(|| encode(black_box(&mut input)))
    });
}

pub fn benchmark_decode(c: &mut Criterion) {
    let mut input = rand::thread_rng().gen::<u64>();
    c.bench_function("hamming decode", |b| {
        b.iter(|| decode(black_box(&mut input)))
    });
}

criterion_group!(benches, benchmark_encode);
criterion_main!(benches);
