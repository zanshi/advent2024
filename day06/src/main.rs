use std::time::Instant;

use day06::part_one;
use day06::part_two;

fn main() {
    // let start = Instant::now();
    // let out = part_one();
    // let elapsed = start.elapsed().as_micros();

    // println!("Part One: {out}, time: {elapsed} us");

    let start = Instant::now();
    let out = part_two();
    let elapsed = start.elapsed().as_micros();
    println!("Part Two: {out}, time: {elapsed} us");
}
