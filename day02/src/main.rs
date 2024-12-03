use std::time::Instant;

fn part_one() {
    let start = Instant::now();

    let input = include_str!("../input.txt");

    let lines = input.lines();

    let mut safe_reports = 0;

    'outer: for line in lines {
        let numbers = line
            .split(' ')
            .map(|number| number.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let numbers_off = numbers.iter().skip(1);

        let mut previous_diff: i32 = 0;

        for (curr, next) in numbers.iter().zip(numbers_off) {
            let diff = next - curr;

            let abs_diff = diff.abs();
            let valid_diff = abs_diff > 0 && abs_diff < 4;

            if diff == 0 || !valid_diff || previous_diff.signum() + diff.signum() == 0 {
                continue 'outer;
            }

            previous_diff = diff;
        }

        safe_reports += 1;
    }

    let elapsed = start.elapsed();

    println!(
        "Part One: Safe reports: {safe_reports}, time: {} us",
        elapsed.as_micros()
    );
}

fn part_two() {
    let start = Instant::now();

    let input = include_str!("../input.txt");

    let lines = input.lines();

    let mut safe_reports = 0;

    'outer: for line in lines {
        let numbers = line
            .split(' ')
            .map(|number| number.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let numbers_off = numbers.iter().skip(1);

        let mut has_bad_level = false;

        let mut previous_diff: i32 = 0;

        for (curr, next) in numbers.iter().zip(numbers_off) {
            let diff = next - curr;

            let abs_diff = diff.abs();
            let valid_diff = abs_diff > 0 && abs_diff < 4;

            if diff == 0 || !valid_diff || previous_diff.signum() + diff.signum() == 0 {
                if has_bad_level {
                    continue 'outer;
                } else {
                    has_bad_level = true;
                    continue;
                }
            }

            previous_diff = diff;
        }

        safe_reports += 1;
    }

    let elapsed = start.elapsed();

    println!(
        "Part Two: Safe reports: {safe_reports}, time: {} us",
        elapsed.as_micros()
    );
}

fn main() {
    part_one();
    part_two();
}
