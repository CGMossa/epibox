use criterion::{black_box, criterion_group, criterion_main, Criterion};

use epibox::simple_forest_fire::percolation_threshold;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("percolation thr. 50 x 1", |b| {
        b.iter(|| percolation_threshold(black_box(50), black_box(0.5), black_box(1)))
    });
    c.bench_function("percolation thr. 50 x 200", |b| {
        b.iter(|| percolation_threshold(black_box(50), black_box(0.5), black_box(200)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
