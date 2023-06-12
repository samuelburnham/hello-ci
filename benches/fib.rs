use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use hello_ci::fib::{fib_iterative, fib_recursive};

fn bench_fibs(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci");
    for i in [20usize, 21usize].iter() {
        group.bench_with_input(BenchmarkId::new("Recursive", i), i,
            |b, i| b.iter(|| fib_recursive(*i)));
        group.bench_with_input(BenchmarkId::new("Iterative", i), i,
            |b, i| b.iter(|| fib_iterative(*i)));
    }
    group.finish();
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);
