use bench_crates::use_flume_mpsc;
use bench_crates::use_tokio_mpsc;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn flume_benchmark(c: &mut Criterion) {
    c.bench_function("flume", |b| b.iter(use_flume_mpsc));
}

pub fn tokio_benchmark(c: &mut Criterion) {
    c.bench_function("tokio", |b| b.iter(use_tokio_mpsc));
}

criterion_group!(benches, flume_benchmark, tokio_benchmark);
criterion_main!(benches);
