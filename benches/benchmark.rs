use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hamming_code_simd::hamming::*;
use rand::Rng;

pub fn benchmark_encode(c: &mut Criterion) {
    let mut input = rand::thread_rng().gen::<u64>();
    c.bench_function("hamming encode", |b| {
        b.iter(|| encode(black_box(input)))
    });
}

pub fn benchmark_decode(c: &mut Criterion) {
    let mut input = rand::thread_rng().gen::<u64>();
    c.bench_function("hamming decode", |b| {
        b.iter(|| decode(black_box(input)))
    });
}

pub fn benchmark_fast_parity(c: &mut Criterion) {
    let input = rand::thread_rng().gen::<u64>();
    c.bench_function("fast_parity check", |b| {
        b.iter(|| fast_parity(black_box(input)))
    });
}

pub fn benchmark_slow_parity(c: &mut Criterion) {
    let input = rand::thread_rng().gen::<u64>();
    c.bench_function("slow_parity check", |b| {
        b.iter(|| slow_parity(black_box(input)))
    });
}

criterion_group!(
    benches,
    benchmark_encode,
    benchmark_decode,
    benchmark_fast_parity,
    benchmark_slow_parity
);
criterion_main!(benches);
