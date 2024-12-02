use std::{
    collections::{BinaryHeap, HashMap},
    fs::read_to_string,
};

pub fn part_one() -> u32 {
    let input = read_to_string("./input.txt").unwrap();

    let lines = input.lines();

    let mut left_list = BinaryHeap::with_capacity(1024);
    let mut right_list = BinaryHeap::with_capacity(1024);

    for line in lines {
        let mut numbers = line
            .split_whitespace()
            .map(|number| number.parse::<i32>().unwrap());

        left_list.push(numbers.next().unwrap());
        right_list.push(numbers.next().unwrap());
    }

    left_list
        .iter()
        .zip(right_list.iter())
        .fold(0, |acc, (l, r)| acc + l.abs_diff(*r))
}

pub fn part_two() -> i32 {
    let input = read_to_string("./input.txt").unwrap();

    let lines = input.lines();

    let mut left_list: Vec<i32> = Vec::with_capacity(1024);
    let mut right_list: HashMap<i32, i32> = HashMap::with_capacity(1024);

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

    left_list
        .iter()
        .fold(0, |acc, l| acc + l * right_list.get(l).unwrap_or(&0))
}
