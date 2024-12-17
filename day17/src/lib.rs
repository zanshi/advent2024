use std::fmt::Write;
enum Opcode {
    Adv = 0,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u8> for Opcode {
    fn from(opcode: u8) -> Self {
        match opcode {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => unreachable!(),
        }
    }
}

fn combo_operand(operand: u8, reg_a: i64, reg_b: i64, reg_c: i64) -> i64 {
    match operand {
        0..=3 => operand as i64,
        4 => reg_a,
        5 => reg_b,
        6 => reg_c,
        _ => unreachable!(),
    }
}

struct Input {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    program: Vec<u8>,
}

fn parse_input(input: &str) -> Input {
    let mut input = input.split('\n');
    let reg_a = input
        .next()
        .unwrap()
        .strip_prefix("Register A: ")
        .unwrap()
        .parse::<i64>()
        .unwrap();
    let reg_b = input
        .next()
        .unwrap()
        .strip_prefix("Register B: ")
        .unwrap()
        .parse::<i64>()
        .unwrap();
    let reg_c = input
        .next()
        .unwrap()
        .strip_prefix("Register C: ")
        .unwrap()
        .parse::<i64>()
        .unwrap();

    let program = input
        .nth(1)
        .unwrap()
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    Input {
        reg_a,
        reg_b,
        reg_c,
        program,
    }
}

#[derive(Debug)]
enum Error {
    Wrong,
}

fn run_program(input: &Input, check: bool) -> Result<Vec<u8>, Error> {
    let mut reg_a = input.reg_a;
    let mut reg_b = input.reg_b;
    let mut reg_c = input.reg_c;
    let program = &input.program;

    let mut instruction_ptr = 0;

    let mut output: Vec<u8> = Vec::new();

    while instruction_ptr < (program.len() - 1) {
        let opcode = Opcode::from(program[instruction_ptr]);
        let operand = program[instruction_ptr + 1];

        match opcode {
            Opcode::Adv => {
                let combo = combo_operand(operand, reg_a, reg_b, reg_c);

                let num = reg_a;
                let denum = 2i64.pow(combo as u32);

                let res = num / denum;

                reg_a = res;
            }
            Opcode::Bxl => {
                let res = reg_b ^ operand as i64;

                reg_b = res;
            }
            Opcode::Bst => {
                let res = combo_operand(operand, reg_a, reg_b, reg_c) % 8;

                reg_b = res;
            }
            Opcode::Jnz => {
                if reg_a != 0 {
                    let instruction_ptr_before = instruction_ptr;
                    instruction_ptr = operand as usize;

                    // skip += 2 if set
                    if instruction_ptr != instruction_ptr_before {
                        continue;
                    }
                }
            }
            Opcode::Bxc => {
                let res = reg_b ^ reg_c;
                reg_b = res;
            }
            Opcode::Out => {
                let val = combo_operand(operand, reg_a, reg_b, reg_c) % 8;
                output.push(val as u8);

                if check {
                    if output != input.program[..output.len()] {
                        return Err(Error::Wrong);
                    }

                    if output.len() == input.program.len() {
                        return Ok(output);
                    }
                }
            }
            Opcode::Bdv => {
                let combo = combo_operand(operand, reg_a, reg_b, reg_c);

                let num = reg_a;
                let denum = 2i64.pow(combo as _);

                let res = num / denum;

                reg_b = res;
            }
            Opcode::Cdv => {
                let combo = combo_operand(operand, reg_a, reg_b, reg_c);

                let num = reg_a;
                let denum = 2i64.pow(combo as _);

                let res = num / denum;

                reg_c = res;
            }
        }

        instruction_ptr += 2;
    }

    if check {
        return Err(Error::Wrong);
    }

    Ok(output)
}

pub fn part_one(input: &str) -> String {
    let input = parse_input(input);

    let output = run_program(&input, false).unwrap();

    let mut output_string = String::new();

    for i in output.iter().take(output.len() - 1) {
        write!(output_string, "{},", i).unwrap();
    }
    write!(output_string, "{}", output.last().unwrap()).unwrap();

    output_string
}

pub fn part_two(input: &str) -> i64 {
    let mut input = parse_input(input);
    input.reg_a = 0;

    loop {
        input.reg_a += 1;

        if let Ok(output) = run_program(&input, true) {
            if input.program == output {
                return input.reg_a;
            }
        }
    }
}

#[test]
fn part_1_small_input() {
    let input = include_str!("../input_small_1.txt");
    let out = part_one(input);

    assert_eq!(out, "4,6,3,5,6,3,5,2,1,0");
}

#[test]
fn part_1_input() {
    let input = include_str!("../input.txt");
    let out = part_one(input);

    assert_eq!(out, "4,6,1,4,2,1,3,1,6");
}

#[test]
fn part_2_small_input() {
    let input = include_str!("../input_small_2.txt");
    let out = part_two(input);

    assert_eq!(out, 117440);
}

#[test]
fn part_2_input() {
    let input = include_str!("../input.txt");
    let out = part_two(input);

    // assert_eq!(out, 6620);
}
