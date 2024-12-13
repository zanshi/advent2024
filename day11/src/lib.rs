fn split_digits(number: i64) -> (i64, i64) {
    let mut divisor = 10;

    while number / divisor > divisor {
        divisor *= 10;
    }

    (number / divisor, number % divisor)
}

fn blink_recursive(counts: &mut Vec<i64>, stone: i64, blink_count: i32, max_blinks: i32) {
    counts[blink_count as usize] += 1;

    if blink_count == max_blinks - 1 {
        return;
    }

    if stone == 0 {
        blink_recursive(counts, 1, blink_count + 1, max_blinks);
        return;
    }

    let digits = (stone as f32).log10().floor() as i32 + 1;

    // even digits
    if digits % 2 == 0 {
        let (left, right) = split_digits(stone);
        blink_recursive(counts, left, blink_count + 1, max_blinks);
        blink_recursive(counts, right, blink_count + 1, max_blinks);
        return;
    }

    blink_recursive(counts, stone * 2024, blink_count + 1, max_blinks);
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
            let (left, right) = split_digits(*stone);

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
    let input = "125 17";
    // let input = "1";
    // let input = "8992692";

    let mut stones = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    // for i in 0..35 {
    //     let mut set = HashSet::new();

    //     for stone in &stones {
    //         set.insert(stone);
    //     }

    //     let previous_uniques = set.len() as i64;

    //     let stones_len_before = stones.len();
    //     blink(&mut stones);

    //     print!("{}", stones.len());
    //     print!(", diff: {}", stones.len() - stones_len_before);

    //     let mut set = HashSet::new();

    //     for stone in &stones {
    //         set.insert(stone);
    //     }

    //     print!(", uniques: {}", set.len());
    //     print!(", uniques diff: {}", set.len() as i64 - previous_uniques);

    //     // if set.len() == 54 {
    //     //     print!(", uniques: ");
    //     //     for unique in set {
    //     //         print!("{}, ", unique);
    //     //     }

    //     //     break;
    //     // }

    //     println!();

    //     // println!();
    //     // for stone in stones.iter() {
    //     //     print!("{stone} ");
    //     // }
    //     // println!();

    //     // for stone in stones2.iter() {
    //     //     if stones.contains(stone)
    //     // }
    // }

    let mut total_count = 0;

    let max_blink_count: i32 = 76;

    let mut counts = vec![0i64; max_blink_count as usize];

    for stone in stones {
        blink_recursive(&mut counts, stone, 0, max_blink_count);
    }

    total_count = *counts.last().unwrap();

    // for count in counts {
    //     total_count += count;
    // }

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

    // for stone in stones {
    //     total_count += blink_recursive(stone, 0, 75);
    // }

    total_count
}
