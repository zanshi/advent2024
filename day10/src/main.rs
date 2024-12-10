use std::time::Instant;

use day10::part_one;
use day10::part_two;

fn main() {
    let input = include_str!("../input.txt");

    let start = Instant::now();
    let out = part_one(input);
    let elapsed = start.elapsed().as_micros();

    println!("Part One: {out}, time: {elapsed} us");

    assert_eq!(out, 782);

    let start = Instant::now();
    let out = part_two(input);
    let elapsed = start.elapsed().as_micros();
    println!("Part Two: {out}, time: {elapsed} us");

    assert_eq!(out, 1694);
}
