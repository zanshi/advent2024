pub fn part_one() -> i64 {
    let input = include_str!("../input.txt");

    let mut total_calibration_result = 0;

    'line_loop: for line in input.lines() {
        let mut parts = line.split(':');

        let test_value = parts.next().unwrap().parse::<i64>().unwrap();

        let numbers = parts
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|num| num.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let mut bits = vec![false; numbers.len() - 1];

        fn increment_bitset(bits: &mut [bool]) -> bool {
            let mut carry = true;

            for bit in bits.iter_mut() {
                if carry {
                    if *bit {
                        *bit = false;
                        carry = true;
                    } else {
                        *bit = true;
                        carry = false;
                        break;
                    }
                }
            }

            !carry
        }

        loop {
            let mut result = numbers[0];

            for j in 1..numbers.len() {
                if bits[j - 1] {
                    result *= numbers[j];
                } else {
                    result += numbers[j];
                }

                if result == test_value {
                    total_calibration_result += test_value;

                    continue 'line_loop;
                }

                if result > test_value {
                    break;
                }
            }

            if !increment_bitset(&mut bits) {
                break;
            }
        }
    }

    total_calibration_result
}

pub fn part_two() -> i32 {
    let input = include_str!("../input.txt");
    0
}
