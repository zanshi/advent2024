use std::time::Instant;

fn split_digit(number: i64) -> (i64, i64) {
    let mut divisor = 10;

    while number / divisor > divisor {
        divisor *= 10;
    }

    (number / divisor, number % divisor)
}

fn blink_recursive(stone: i64, depth: i32, max_depth: i32) -> i64 {
    if depth == max_depth {
        return 1;
    }

    if stone == 0 {
        return blink_recursive(1, depth + 1, max_depth);
    }

    let digits = (stone as f32).log10().floor() as i32 + 1;

    // even digits
    if digits % 2 == 0 {
        let (left, right) = split_digit(stone);
        return blink_recursive(left, depth + 1, max_depth)
            + blink_recursive(right, depth + 1, max_depth);
    }

    blink_recursive(stone * 2024, depth + 1, max_depth)
}

fn blink(stones: &mut Vec<i64>) {
    let mut new_stones = Vec::with_capacity(stones.len() * 2);

    for stone in stones.iter() {
        if *stone == 0 {
            new_stones.push(1);
            continue;
        }

        let digits = (*stone as f32).log10().floor() as i32 + 1;

        // even digits
        if digits % 2 == 0 {
            let (left, right) = split_digit(*stone);

            new_stones.push(left);
            new_stones.push(right);

            continue;
        }

        new_stones.push(stone * 2024);
    }

    *stones = new_stones;
}

pub fn part_one() -> i64 {
    let input = "8435 234 928434 14 0 7 92446 8992692";
    // let input = "1 2024 1 0 9 9 2021976";
    // let input = "125 17";
    // let input = "17";

    let stones = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut total_count = 0;

    for stone in stones {
        total_count += blink_recursive(stone, 0, 25);
    }

    total_count
}

pub fn part_two() -> i64 {
    let input = "8435 234 928434 14 0 7 92446 8992692";

    let stones = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut total_count = 0;

    for stone in stones {
        total_count += blink_recursive(stone, 0, 75);
    }

    total_count
}
