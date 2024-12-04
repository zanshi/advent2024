use std::time::Instant;

use day04::part_one;
use day04::part_two;

fn main() {
    let start = Instant::now();
    let out = part_one();
    let elapsed = start.elapsed().as_micros();

    println!("Part One: XMAS Count: {out}, time: {elapsed} us");

    let start = Instant::now();
    let out = part_two();
    let elapsed = start.elapsed().as_micros();
    println!("Part Two: X-MAS Count: {out}, time: {elapsed} us");
}
