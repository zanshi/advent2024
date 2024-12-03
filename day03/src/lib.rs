use std::fs::read_to_string;

pub fn part_one() -> i32 {
    let input = read_to_string("./input.txt").unwrap();

    let mut product = 0;

    for mul_inputs in input.split("mul(").skip(1) {
        let mut numbers = mul_inputs.split(',');

        let left_number = numbers.next().and_then(|number| number.parse::<i32>().ok());
        let right_number = numbers
            .next()
            .and_then(|number| number.split_once(')'))
            .and_then(|number| number.0.parse::<i32>().ok());

        if let (Some(left), Some(right)) = (left_number, right_number) {
            product += left * right;
        }
    }

    product
}

pub fn part_two() -> i32 {
    let input = read_to_string("./input.txt").unwrap();

    let mut input_slice = &input[..];

    let mut product = 0;

    let mut enabled = true;

    loop {
        let mul_index = input_slice.find("mul(");

        if mul_index.is_none() {
            break;
        }

        let mul_index = mul_index.unwrap();

        let mut do_index = input_slice.find("do()").unwrap_or(usize::MAX);
        let mut dont_index = input_slice.find("don't()").unwrap_or(usize::MAX);

        while do_index < mul_index {
            let index = input_slice[(do_index + 4)..]
                .find("do()")
                .unwrap_or(usize::MAX);

            if index == 0 || index == usize::MAX {
                break;
            }

            if do_index + index < mul_index {
                do_index += index;
            } else {
                break;
            }
        }

        while dont_index < mul_index {
            let index = input_slice[(dont_index + 7)..]
                .find("don't()")
                .unwrap_or(usize::MAX);

            if index == 0 || index == usize::MAX {
                break;
            }

            if dont_index + index < mul_index {
                dont_index += index;
            } else {
                break;
            }
        }

        if do_index < dont_index && do_index < mul_index {
            enabled = true;
        } else if dont_index < do_index && dont_index < mul_index {
            enabled = false;
        }

        input_slice = &input_slice[(mul_index + 4)..];

        let mut numbers = input_slice.split(',');

        let left_number = numbers.next().and_then(|number| number.parse::<i32>().ok());
        let right_number = numbers
            .next()
            .and_then(|number| number.split_once(')'))
            .and_then(|number| number.0.parse::<i32>().ok());

        if let (Some(left), Some(right)) = (left_number, right_number) {
            if enabled {
                product += left * right;
            }
        }
    }

    product
}
