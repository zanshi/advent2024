use std::time::Instant;

use day12::{part_one, part_two};

fn main() {
    let input = include_str!("../input.txt");

    let now = Instant::now();
    let out = part_one(input);
    let elapsed = now.elapsed().as_micros();

    println!("Part One: {out}, time: {elapsed} us");

    assert_eq!(out, 1546338);

    let now = Instant::now();
    let out = part_two(input);
    let elapsed = now.elapsed().as_micros();

    println!("Part Two: {out}, time: {elapsed} us");

    // assert_eq!(out, 182081);
}
