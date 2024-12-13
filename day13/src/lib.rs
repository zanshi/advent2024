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

    let mut total_tokens = 0;

    while let Some(a) = input.next() {
        let b = input.next().unwrap();
        let prize = input.next().unwrap();

        let p = parse_input(a, b, prize);

        // let mut a_presses = 0;
        // let mut b_presses = 0;

        // let mut x = 0;
        // let mut y = 0;
        // while x < p.prize.0 && y < p.prize.1 {
        //     x += p.a.0;
        //     y += p.a.1;

        //     a_presses += 1;
        //     b_presses += 1;
        // }

        // while a_presses < 100 {

        // }

        let a = Matrix2::new(p.a.0 as f64, p.b.0 as f64, p.a.1 as f64, p.b.1 as f64);

        let b = Vector2::new(p.prize.0 as f64, p.prize.1 as f64);

        let x = a.lu().solve(&b).unwrap();

        let a_presses = x[0] as i64;
        let b_presses = x[1] as i64;

        if (x[0] - x[0].floor()).abs() < f64::EPSILON && (x[1] - x[1].floor()).abs() < f64::EPSILON
        {
            if p.a.0 * a_presses + p.b.0 * b_presses == p.prize.0
                && p.a.1 * a_presses + p.b.1 * b_presses == p.prize.1
            {
                println!("Problem: {:?}", p);

                println!("Adding {}, {}", x[0], x[1]);

                if a_presses > 100 {
                    println!("a > 100");
                }

                if b_presses > 100 {
                    println!("b > 100");
                }

                let tokens = a_presses * 3 + b_presses as i64;
                total_tokens += tokens;
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
