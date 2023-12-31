use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[path = "../src/easy/fibonacci_number.rs"]
mod fibonacci_number;

fn nth_fib_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| {
        b.iter(|| fibonacci_number::fib(black_box(20)))
    });
}

criterion_group!(benches, nth_fib_benchmark);
criterion_main!(benches);
