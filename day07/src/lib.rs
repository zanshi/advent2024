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
            let mut result = numbers[0];

            for (number, bit) in numbers[1..].iter().zip(bits.iter()) {
                if *bit {
                    result *= number;
                } else {
                    result += number;
                }

                if result > test_value {
                    break;
                }
            }

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

    // 370631131361420 - wrong, too high
    // 333027885676693

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
            let mut result = numbers[0];

            for (number, bit) in numbers[1..].iter().zip(bits.iter()) {
                if *bit {
                    result *= number;
                } else {
                    result += number;
                }

                if result > test_value {
                    break;
                }
            }

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

        fn increment_ternary(operator: &mut [u8]) -> bool {
            let mut carry = true;

            for bit in operator.iter_mut() {
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
                }
            }

            !carry
        }

        loop {
            let mut result = numbers[0];

            for (number, operator) in numbers[1..].iter().zip(operators.iter()) {
                if *operator == 0 {
                    result += number;
                } else if *operator == 1 {
                    result *= number;
                } else if *operator == 2 {
                    result = (result.to_string() + &number.to_string())
                        .parse::<i64>()
                        .unwrap();
                }

                if result > test_value {
                    break;
                }
            }

            if result == test_value {
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
