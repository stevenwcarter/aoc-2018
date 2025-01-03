use advent_of_code::{check_for_repeats, check_for_repeats_vec};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("check_for_repeats", |b| {
        b.iter(|| check_for_repeats(black_box("abcdeffghi")))
    });
    c.bench_function("check_for_repeats_vec", |b| {
        b.iter(|| check_for_repeats_vec(black_box("abcdeffghi")))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
