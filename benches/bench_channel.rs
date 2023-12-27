use bench_crates::use_flume_mpmc;
use bench_crates::use_flume_mpsc;
use bench_crates::use_flume_spsc;
use bench_crates::use_tokio_mpsc;
use bench_crates::use_tokio_spsc;
use bench_crates::DataProvider;
use bench_crates::StringData;
use bench_crates::U8Data;
use criterion::{criterion_group, criterion_main, Criterion};

fn get_name<T: DataProvider>(name: &str) -> String {
    format!("{name}-{}", T::TYPE_LITERAL)
}

pub fn flume_benchmark<T: DataProvider>(c: &mut Criterion) {
    let name1 = get_name::<T>("flume-mpmc");
    let name2 = get_name::<T>("flume-mpsc");
    let name3 = get_name::<T>("flume-spsc");
    c.bench_function(&name1, |b| b.iter(use_flume_mpmc::<T>));
    c.bench_function(&name2, |b| b.iter(use_flume_mpsc::<T>));
    c.bench_function(&name3, |b| b.iter(use_flume_spsc::<T>));
}

pub fn tokio_benchmark<T: DataProvider>(c: &mut Criterion) {
    let name1 = get_name::<T>("tokio-mpsc");
    let name2 = get_name::<T>("tokio-spsc");
    c.bench_function(&name1, |b| b.iter(use_tokio_mpsc::<T>));
    c.bench_function(&name2, |b| b.iter(use_tokio_spsc::<T>));
}

criterion_group!(
    benches,
    flume_benchmark::<StringData>,
    tokio_benchmark::<StringData>,
    flume_benchmark::<U8Data>,
    tokio_benchmark::<U8Data>
);

criterion_main!(benches);
