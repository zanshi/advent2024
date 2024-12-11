use std::collections::HashSet;

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
    // let input = "8435 234 928434 14 0 7 92446 8992692";
    // let input = "1 2024 1 0 9 9 2021976";
    // let input = "125 17";
    // let input = "1";
    let input = "8992692";

    let mut stones = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    println!("1");

    for i in 0..35 {
        let mut set = HashSet::new();

        for stone in &stones {
            set.insert(stone);
        }

        let previous_uniques = set.len() as i64;

        let stones_len_before = stones.len();
        blink(&mut stones);

        print!("{}", stones.len());
        print!(", diff: {}", stones.len() - stones_len_before);

        let mut set = HashSet::new();

        for stone in &stones {
            set.insert(stone);
        }

        print!(", uniques: {}", set.len());
        print!(", uniques diff: {}", set.len() as i64 - previous_uniques);

        // if set.len() == 54 {
        //     print!(", uniques: ");
        //     for unique in set {
        //         print!("{}, ", unique);
        //     }

        //     break;
        // }

        println!();

        // println!();
        // for stone in stones.iter() {
        //     print!("{stone} ");
        // }
        // println!();

        // for stone in stones2.iter() {
        //     if stones.contains(stone)
        // }
    }

    let mut total_count = 0;

    for stone in stones {
        total_count += blink_recursive(stone, 0, 75);
    }

    total_count
}

pub fn part_two() -> i64 {
    let input = "8435 234 928434 14 0 7 92446 8992692";

    // 0 -> 1 -> 2024 -> 20 24 -> 2 0 2 4 -> 4048 1 4048 8096 -> 40 48 2024 40 48 80 96 -> 4 0 4 8 4048 4 0 4 8 8 0 9 6 -> 16192

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
