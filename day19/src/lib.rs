use std::collections::BTreeSet;

use rustc_hash::FxHashSet as HashSet;

pub fn part_one(input: &str) -> i32 {
    let mut lines = input.lines();

    let available_patterns = lines.next().unwrap().split(", ").collect::<Vec<&str>>();

    let _empty = lines.next().unwrap();

    let desired_patterns = lines.collect::<Vec<&str>>();

    let mut possible = 0;

    // println!("{:?}", available_patterns);
    // println!("{:?}", desired_patterns);

    // goal: find how many designs are possible (or how many are impossible)
    // the available patterns are substrings of the desired ones

    // 199 - wrong

    // let mut possible_patterns = Vec::new();

    for desired_pattern in desired_patterns {
        let mut available_patterns_in_desired_pattern = Vec::new();

        for pattern in available_patterns.iter() {
            if desired_pattern.contains(pattern) {
                available_patterns_in_desired_pattern.push(pattern);
            }
        }

        for c in desired_pattern.chars() {
            if available_patterns_in_desired_pattern.contains(x)
        }

        println!(
            "Patterns in {}: {:?}",
            desired_pattern, available_patterns_in_desired_pattern
        );
    }

    // 'outer: for desired_pattern in desired_patterns {
    //     let mut start_end = 1;
    //     let mut last_found_start = 0;

    //     let mut start = 0;
    //     let mut end = start_end;

    //     while start_end <= desired_pattern.len() && end <= desired_pattern.len() {
    //         let substr = &desired_pattern[start..end];

    //         if available_patterns.contains(&substr) {
    //             start += substr.len();
    //             end = start + 1;

    //             last_found_start = start;

    //             if start >= desired_pattern.len() {
    //                 possible += 1;
    //                 possible_patterns.push(desired_pattern);
    //                 continue 'outer;
    //             }
    //         } else {
    //             end += 1;
    //         }

    //         if end > desired_pattern.len() {
    //             start_end += 1;
    //             start = last_found_start;
    //             end = start + start_end;
    //         }
    //     }
    // }

    // println!("{:?}", possible_patterns);

    possible
}

pub fn part_two(input: &str) -> i32 {
    0
}

#[test]
fn part_1_small_input() {
    let input = include_str!("../input_small_1.txt");
    let out = part_one(input);

    assert_eq!(out, 6);
}

#[test]
fn part_1_input() {
    let input = include_str!("../input.txt");
    let out = part_one(input);

    assert_eq!(out, 436);
}

// #[test]
// fn part_2_small_input() {
//     let input = include_str!("../input_small_1.txt");
//     let out = part_two(input, 7);

//     assert_eq!(out, (6, 1));
// }

// #[test]
// fn part_2_input() {
//     let input = include_str!("../input.txt");
//     let out = part_two(input);

//     assert_eq!(out, 6620);
// }
