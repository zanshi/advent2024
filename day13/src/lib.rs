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

#[inline]
fn solve(p: Input) -> i64 {
    let a1 = p.a.0;
    let a2 = p.a.1;

    let b1 = p.b.0;
    let b2 = p.b.1;

    let c1 = p.prize.0;
    let c2 = p.prize.1;

    let det = a1 * b2 - b1 * a2;

    let x = (c1 * b2 - b1 * c2) / det;
    let y = (a1 * c2 - c1 * a2) / det;

    if a1 * x + b1 * y == c1 && a2 * x + b2 * y == c2 {
        x * 3 + y
    } else {
        0
    }
}

pub fn part_one(input: &str) -> i64 {
    let mut input = input.split('\n');

    let mut total_tokens = 0;

    while let Some(a) = input.next() {
        let b = input.next().unwrap();
        let prize = input.next().unwrap();

        let p = parse_input(a, b, prize);

        total_tokens += solve(p);

        let Some(_empty_line) = input.next() else {
            break;
        };
    }

    total_tokens
}

// 94 * a + 22 * b = 8400
// 34 * a + 67 * b = 5400

// 94 * x + 22 * y = 8400
// 34 * x + 67 * y = 5400

// a1 * x + b1 * y = c1
// a2 * x + b2 * y = c2

pub fn part_two(input: &str) -> i64 {
    let mut input = input.split('\n');

    let mut total_tokens = 0;

    while let Some(a) = input.next() {
        let b = input.next().unwrap();
        let prize = input.next().unwrap();

        let mut p = parse_input(a, b, prize);
        p.prize.0 += 10000000000000;
        p.prize.1 += 10000000000000;

        total_tokens += solve(p);

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
