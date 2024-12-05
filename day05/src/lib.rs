use std::{cmp::Ordering, collections::HashMap};

pub fn part_one() -> i32 {
    let input = include_str!("../input.txt");

    let mut sum = 0;

    let mut split = input.split("\n\n");
    let rules_input = split.next().unwrap();
    let updates_input = split
        .next()
        .unwrap()
        .lines()
        .map(|line| line.split(',').map(|number| number.parse::<i32>().unwrap()));

    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();

    // rules
    for line in rules_input.lines() {
        let mut parts = line.split('|');
        let key = parts.next().unwrap().parse::<i32>().unwrap();
        let value = parts.next().unwrap().parse::<i32>().unwrap();

        rules
            .entry(key)
            .and_modify(|values| values.push(value))
            .or_insert(vec![value]);
    }

    let mut valid_updates: Vec<Vec<i32>> = Vec::new();

    // updates
    'outer: for update in updates_input {
        let mut previous_numbers = Vec::new();
        for number in update.clone() {
            if let Some(numbers_after) = rules.get(&number) {
                for number_after in numbers_after {
                    if previous_numbers.contains(number_after) {
                        continue 'outer;
                    }
                }
            }

            previous_numbers.push(number);
        }

        valid_updates.push(update.collect());
    }

    for valid_update in valid_updates {
        let mid_number = valid_update[valid_update.len() / 2];
        sum += mid_number;
    }

    sum
}

pub fn part_two() -> i32 {
    let input = include_str!("../input.txt");

    let mut sum = 0;

    let mut split = input.split("\n\n");
    let rules_input = split.next().unwrap();
    let updates_input = split
        .next()
        .unwrap()
        .lines()
        .map(|line| line.split(',').map(|number| number.parse::<i32>().unwrap()));

    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();

    // rules
    for line in rules_input.lines() {
        let mut parts = line.split('|');
        let key = parts.next().unwrap().parse::<i32>().unwrap();
        let value = parts.next().unwrap().parse::<i32>().unwrap();

        rules
            .entry(key)
            .and_modify(|values| values.push(value))
            .or_insert(vec![value]);
    }

    let mut invalid_updates: Vec<Vec<i32>> = Vec::new();

    // updates
    'outer: for update in updates_input {
        let mut previous_numbers = Vec::new();
        for number in update.clone() {
            if let Some(numbers_after) = rules.get(&number) {
                for number_after in numbers_after {
                    if previous_numbers.contains(number_after) {
                        invalid_updates.push(update.collect());

                        continue 'outer;
                    }
                }
            }

            previous_numbers.push(number);
        }
    }

    let mut reordered_updates: Vec<Vec<i32>> = Vec::new();

    // order
    for mut update in invalid_updates {
        update.sort_by(|a, b| {
            if let Some(values) = rules.get(a) {
                if values.contains(b) {
                    return Ordering::Less;
                }
            }

            if let Some(values) = rules.get(b) {
                if values.contains(a) {
                    return Ordering::Greater;
                }
            }

            Ordering::Equal
        });

        reordered_updates.push(update);
    }

    for valid_update in reordered_updates {
        let mid_number = valid_update[valid_update.len() / 2];
        sum += mid_number;
    }

    sum
}
