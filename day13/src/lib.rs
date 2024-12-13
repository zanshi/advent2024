use std::f32::EPSILON;

use nalgebra::{Matrix2, Vector2};

#[derive(Debug)]
struct Input {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

fn parse_input(a: &str, b: &str, prize: &str) -> Input {
    let a = a
        .strip_prefix("Button A: X+")
        .unwrap()
        .split_once(',')
        .map(|(x, y)| {
            (
                x.parse::<i64>().unwrap(),
                y.strip_prefix(" Y+").unwrap().parse::<i64>().unwrap(),
            )
        })
        .unwrap();

    let b = b
        .strip_prefix("Button B: X+")
        .unwrap()
        .split_once(',')
        .map(|(x, y)| {
            (
                x.parse::<i64>().unwrap(),
                y.strip_prefix(" Y+").unwrap().parse::<i64>().unwrap(),
            )
        })
        .unwrap();

    let pos_prize = prize
        .strip_prefix("Prize: X=")
        .unwrap()
        .split_once(',')
        .map(|(x, y)| {
            (
                x.parse::<i64>().unwrap(),
                y.strip_prefix(" Y=").unwrap().parse::<i64>().unwrap(),
            )
        })
        .unwrap();

    Input {
        a,
        b,
        prize: pos_prize,
    }
}

pub fn part_one(input: &str) -> i64 {
    let mut input = input.lines();

    // 21502
    // 23440 - wrong
    // 29521 - wrong
    // 28834 - wrong
    // 36838

    let mut total_tokens = 0;

    while let Some(a) = input.next() {
        let b = input.next().unwrap();
        let prize = input.next().unwrap();

        let p = parse_input(a, b, prize);

        'outer: for i in 0..100 {
            for j in 0..100 {
                if p.a.0 * i + p.b.0 * j == p.prize.0 && p.a.1 * i + p.b.1 * j == p.prize.1 {
                    total_tokens += 3 * i + j;
                    break 'outer;
                }
            }
        }

        let Some(_empty_line) = input.next() else {
            break;
        };
    }

    total_tokens
}

pub fn part_two(input: &str) -> i64 {
    let width = input.find('\n').unwrap();
    let input = input.split('\n').collect::<String>();
    let input = input.as_bytes();

    0 as i64
}

#[test]
fn part_1_small_1() {
    let input = include_str!("../input_small_1.txt");
    let out = part_one(input);

    assert_eq!(out, 480);
}

#[test]
fn part_1_input() {
    let input = include_str!("../input.txt");
    let out = part_one(input);

    // assert_eq!(out, 1546338);
}

// #[test]
// fn part_2_small_1() {
//     let input = include_str!("../input_small_1.txt");
//     let out = part_two(input);

//     assert_eq!(out, 80);
// }

// #[test]
// fn part_2_input() {
//     let input = include_str!("../input.txt");
//     let out = part_two(input);

//     // assert_eq!(out, 1546338);
// }
