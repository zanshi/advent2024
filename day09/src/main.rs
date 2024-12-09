use std::time::Instant;

use day09::part_one;
use day09::part_two;

fn main() {
    let start = Instant::now();
    let out = part_one();
    let elapsed = start.elapsed().as_micros();

    println!("Part One: {out}, time: {elapsed} us");

    // assert_eq!(out, 381);

    let start = Instant::now();
    let out = part_two();
    let elapsed = start.elapsed().as_micros();
    println!("Part Two: {out}, time: {elapsed} us");

    // assert_eq!(out, 1184);
}
