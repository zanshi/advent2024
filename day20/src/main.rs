use std::time::Instant;

use day20::{part_one, part_two};

fn main() {
    let input = include_str!("../input_small_1.txt");

    let now = Instant::now();
    let out = part_one(input); // change
    let elapsed = now.elapsed().as_micros();

    println!("Part One: {out}, time: {elapsed} us");

    assert_eq!(out, 436);

    let now = Instant::now();
    let out = part_two(input);
    let elapsed = now.elapsed().as_micros();

    println!("Part Two: {out:?}, time: {elapsed} us");

    assert_eq!(out, 0);
}
