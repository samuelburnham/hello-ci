use criterion::{black_box, criterion_group, criterion_main, Criterion};
use crate::{fib_iterative, fib_recursive};

fn iterative_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fib_iterative(black_box(20))));
}

fn recursive_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fib_recursive(black_box(20))));
}

criterion_group!(benches, iterative_benchmark);
criterion_group!(benches, recursive_benchmark);
criterion_main!(benches);
