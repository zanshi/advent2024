use std::time::Instant;

use day10::part_one;
use day10::part_two;

fn main() {
    let start = Instant::now();
    let out = part_one();
    let elapsed = start.elapsed().as_micros();

    println!("Part One: {out}, time: {elapsed} us");

    // assert_eq!(out, 6607511583593);

    let start = Instant::now();
    let out = part_two();
    let elapsed = start.elapsed().as_micros();
    println!("Part Two: {out}, time: {elapsed} us");

    // assert_eq!(out, 6636608781232);
}
