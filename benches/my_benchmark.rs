use criterion::{criterion_group, criterion_main, Criterion};

use hello_raytracing::main as main2;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("render", |b| b.iter(main2));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
