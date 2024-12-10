use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

use day10::part_one;
use day10::part_two;

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");

    c.bench_function("day10 part1", |b| b.iter(|| part_one(black_box(input))));
    c.bench_function("day10 part2", |b| b.iter(|| part_two(black_box(input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
