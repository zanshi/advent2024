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

fn increment_ternary(operators: &mut [u8]) -> bool {
    let mut carry = true;

    for bit in operators.iter_mut() {
        if carry {
            if *bit == 2 {
                *bit = 0;
                carry = true;
            } else if *bit == 1 {
                *bit = 2;
                carry = false;
            } else {
                *bit = 1;
                carry = false;
                break;
            }
        } else {
            break;
        }
    }

    !carry
}

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

        loop {
            let result =
                numbers[1..]
                    .iter()
                    .zip(bits.iter())
                    .fold(
                        numbers[0],
                        |acc, (number, bit)| {
                            if *bit {
                                acc * number
                            } else {
                                acc + number
                            }
                        },
                    );

            if result == test_value {
                total_calibration_result += test_value;

                continue 'line_loop;
            }

            if !increment_bitset(&mut bits) {
                break;
            }
        }
    }

    total_calibration_result
}

pub fn part_two() -> i64 {
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

        // fast check
        let mut bits = vec![false; numbers.len() - 1];

        loop {
            let result =
                numbers[1..]
                    .iter()
                    .zip(bits.iter())
                    .fold(
                        numbers[0],
                        |acc, (number, bit)| {
                            if *bit {
                                acc * number
                            } else {
                                acc + number
                            }
                        },
                    );

            if result == test_value {
                total_calibration_result += test_value;

                continue 'line_loop;
            }

            if !increment_bitset(&mut bits) {
                break;
            }
        }

        let mut operators = vec![0; numbers.len() - 1];
        // 0 == add
        // 1 == mul
        // 2 == concat

        loop {
            let result = numbers[1..].iter().zip(operators.iter()).try_fold(
                numbers[0],
                |acc, (number, operator)| {
                    let res = if *operator == 0 {
                        acc + number
                    } else if *operator == 1 {
                        acc * number
                    } else if *number >= 100 {
                        acc * 1000 + number
                    } else if *number >= 10 {
                        acc * 100 + number
                    } else {
                        acc * 10 + number
                    };

                    (res <= test_value).then_some(res)
                },
            );

            if result == Some(test_value) {
                total_calibration_result += test_value;

                continue 'line_loop;
            }

            if !increment_ternary(&mut operators) {
                break;
            }
        }
    }

    total_calibration_result
}
