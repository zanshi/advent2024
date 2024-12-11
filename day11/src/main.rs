use std::time::Instant;

use day11::{part_one, part_two};

fn main() {
    let now = Instant::now();
    let out = part_one();
    let elapsed = now.elapsed().as_micros();

    println!("Part One: {out}, time: {elapsed} us");

    assert_eq!(out, 182081);

    let now = Instant::now();
    let out = part_two();
    let elapsed = now.elapsed().as_micros();

    println!("Part Two: {out}, time: {elapsed} us");

    // assert_eq!(out, 182081);
}
