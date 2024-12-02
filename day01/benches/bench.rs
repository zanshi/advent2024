use criterion::{criterion_group, criterion_main, Criterion};
use std::{collections::HashMap, fs::read_to_string, hint::black_box};

use day01::part_one;
use day01::part_two;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day01 part1", |b| b.iter(|| part_one()));
    c.bench_function("day01 part2", |b| b.iter(|| part_two()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
