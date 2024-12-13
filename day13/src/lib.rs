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

    let prize = prize
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

    Input { a, b, prize }
}

pub fn part_one(input: &str) -> i64 {
    let mut input = input.lines();

    let mut total_tokens = 0;

    while let Some(a) = input.next() {
        let b = input.next().unwrap();
        let prize = input.next().unwrap();

        let p = parse_input(a, b, prize);

        let a = Matrix2::new(p.a.0 as f64, p.b.0 as f64, p.a.1 as f64, p.b.1 as f64);

        let det_a = a.determinant();
        let mut d_x = a;
        d_x.set_column(0, &Vector2::new(p.prize.0 as f64, p.prize.1 as f64));

        let det_x = d_x.determinant();

        let mut d_y = a;
        d_y.set_column(1, &Vector2::new(p.prize.0 as f64, p.prize.1 as f64));

        let det_y = d_y.determinant();

        let x = det_x / det_a;
        let y = det_y / det_a;

        let a_presses = x as i64;
        let b_presses = y as i64;

        if p.a.0 * a_presses + p.b.0 * b_presses == p.prize.0
            && p.a.1 * a_presses + p.b.1 * b_presses == p.prize.1
        {
            let tokens = a_presses * 3 + b_presses as i64;
            total_tokens += tokens;
        }

        let Some(_empty_line) = input.next() else {
            break;
        };
    }

    total_tokens
}

pub fn part_two(input: &str) -> i64 {
    let mut input = input.lines();

    let mut total_tokens = 0;

    while let Some(a) = input.next() {
        let b = input.next().unwrap();
        let prize = input.next().unwrap();

        let mut p = parse_input(a, b, prize);
        p.prize.0 += 10000000000000;
        p.prize.1 += 10000000000000;

        let a = Matrix2::new(p.a.0 as f64, p.b.0 as f64, p.a.1 as f64, p.b.1 as f64);

        let det_a = a.determinant();
        let mut d_x = a;
        d_x.set_column(0, &Vector2::new(p.prize.0 as f64, p.prize.1 as f64));

        let det_x = d_x.determinant();

        let mut d_y = a;
        d_y.set_column(1, &Vector2::new(p.prize.0 as f64, p.prize.1 as f64));

        let det_y = d_y.determinant();

        let x = det_x / det_a;
        let y = det_y / det_a;

        let a_presses = x as i64;
        let b_presses = y as i64;

        if p.a.0 * a_presses + p.b.0 * b_presses == p.prize.0
            && p.a.1 * a_presses + p.b.1 * b_presses == p.prize.1
        {
            let tokens = a_presses * 3 + b_presses as i64;
            total_tokens += tokens;
        }

        let Some(_empty_line) = input.next() else {
            break;
        };
    }

    total_tokens
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

    assert_eq!(out, 36838);
}

#[test]
fn part_2_input() {
    let input = include_str!("../input.txt");
    let out = part_two(input);

    assert_eq!(out, 83029436920891);
}
