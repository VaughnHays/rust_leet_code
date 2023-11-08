use criterion::{black_box, criterion_group, criterion_main, Criterion};
#[path = "../src/nth_fib.rs"]
mod nth_fib;

fn nth_fib_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| nth_fib::nth_fib(black_box(20))));
}

criterion_group!(benches, nth_fib_benchmark);
criterion_main!(benches);
