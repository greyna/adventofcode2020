use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day4::get_input;
use day4::parse_input;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse_input", |b| {
        b.iter(|| parse_input(black_box(&get_input())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
