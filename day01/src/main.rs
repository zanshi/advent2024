use std::{collections::HashMap, fs::read_to_string, time::Instant};

fn part_one() {
    let start = Instant::now();

    let input = read_to_string("./input.txt").unwrap();

    let lines = input.lines();

    let mut left_list: Vec<i32> = Vec::with_capacity(1024);
    let mut right_list: Vec<i32> = Vec::with_capacity(1024);

    for line in lines {
        let mut numbers = line
            .split_whitespace()
            .map(|number| number.parse::<i32>().unwrap());

        left_list.push(numbers.next().unwrap());
        right_list.push(numbers.next().unwrap());
    }

    left_list.sort();
    right_list.sort();

    let combined_distance = left_list
        .iter()
        .zip(right_list.iter())
        .fold(0, |acc, (l, r)| acc + l.abs_diff(*r));

    let elapsed = start.elapsed();

    println!(
        "Part One: Combined distance: {combined_distance}, time: {} us",
        elapsed.as_micros()
    );
}

fn part_two() {
    let start = Instant::now();

    let input = read_to_string("./input.txt").unwrap();

    let lines = input.lines();

    let mut left_list: Vec<i32> = Vec::with_capacity(1024);
    let mut right_list: HashMap<i32, i32> = HashMap::new();

    for line in lines {
        let mut numbers = line
            .split_whitespace()
            .map(|number| number.parse::<i32>().unwrap());

        left_list.push(numbers.next().unwrap());

        let right_number = numbers.next().unwrap();

        right_list
            .entry(right_number)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    let similarity_score = left_list
        .iter()
        .fold(0, |acc, l| acc + l * right_list.get(l).unwrap_or(&0));

    let elapsed = start.elapsed();

    println!(
        "Part Two: Similarity score: {similarity_score}, time: {} us",
        elapsed.as_micros()
    );
}

fn main() {
    part_one();
    part_two();
}
