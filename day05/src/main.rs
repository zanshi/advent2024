use std::time::Instant;

use day05::part_one;
use day05::part_two;

fn main() {
    let start = Instant::now();
    let out = part_one();
    let elapsed = start.elapsed().as_micros();

    println!("Part One: Sum: {out}, time: {elapsed} us");

    let start = Instant::now();
    let out = part_two();
    let elapsed = start.elapsed().as_micros();
    println!("Part Two: Sum: {out}, time: {elapsed} us");
}
