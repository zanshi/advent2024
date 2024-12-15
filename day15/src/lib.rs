use std::fmt::Display;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl From<Direction> for (i32, i32) {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "^"),
            Direction::Right => write!(f, ">"),
            Direction::Down => write!(f, "v"),
            Direction::Left => write!(f, "<"),
        }
    }
}

struct Map {
    input: Vec<u8>,
    width: usize,
    height: usize,
}

impl Map {
    #[inline(always)]
    fn coord_to_index(&self, x: i32, y: i32) -> usize {
        (y * self.width as i32 + x) as usize
    }

    #[inline(always)]
    fn index_to_coord(&self, index: usize) -> (i32, i32) {
        let x = index % self.width;
        let y = index / self.width;

        (x as i32, y as i32)
    }

    #[inline(always)]
    fn get_from_coord(&self, x: i32, y: i32) -> u8 {
        *self.input.get(self.coord_to_index(x, y)).unwrap()
    }

    // #[inline(always)]
    // fn get(&self, index: usize) -> u8 {
    //     *self.input.get(index).unwrap()
    // }

    fn calculate_gps_box_sum(&self) -> usize {
        let mut sum = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                if self.get_from_coord(x as i32, y as i32) == b'O' {
                    sum += 100 * y + x;
                }
            }
        }

        sum
    }

    fn calculate_gps_large_box_sum(&self) -> usize {
        let mut sum = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                if self.get_from_coord(x as i32, y as i32) == b'[' {
                    sum += 100 * y + x;
                }
            }
        }

        sum
    }

    fn viz(&self, pos: (i32, i32)) {
        for y in 0..self.height {
            println!();
            for x in 0..self.width {
                if pos.0 == x as i32 && pos.1 == y as i32 {
                    print!("@");
                } else {
                    print!("{}", self.get_from_coord(x as i32, y as i32) as char);
                }
            }
        }
        println!();
        println!();
    }
}

fn parse_input(input: &str) -> (Map, Vec<Direction>, usize) {
    let width = input.find('\n').unwrap();

    let lines = input.lines();

    let mut movement_line_start = 0;

    for (i, line) in lines.enumerate() {
        if line.is_empty() {
            movement_line_start = i + 1;
            break;
        }
    }

    let lines = input.lines();

    let mut map = lines
        .take_while(|l| !l.is_empty())
        .flat_map(|l| l.as_bytes().to_owned())
        .collect::<Vec<u8>>();

    let start_index = map.iter().position(|c| *c == b'@').unwrap();
    map[start_index] = b'.';

    let map = Map {
        input: map,
        width,
        height: width,
    };

    let lines = input.lines();

    let movements = lines
        .skip(movement_line_start)
        .flat_map(|l| {
            l.chars().map(|c| match c {
                '^' => Direction::Up,
                '>' => Direction::Right,
                'v' => Direction::Down,
                '<' => Direction::Left,
                _ => unreachable!(),
            })
        })
        .collect();

    (map, movements, start_index)
}

fn parse_input_b(input: &str) -> (Map, Vec<Direction>, usize) {
    let width = input.find('\n').unwrap();

    let lines = input.lines();

    let mut movement_line_start = 0;

    for (i, line) in lines.enumerate() {
        if line.is_empty() {
            movement_line_start = i + 1;
            break;
        }
    }

    let lines = input.lines();

    let mut map = lines
        .take_while(|l| !l.is_empty())
        .flat_map(|l| l.as_bytes().to_owned())
        .flat_map(|c| match c {
            b'#' => *b"##",
            b'O' => *b"[]",
            b'.' => *b"..",
            b'@' => *b"@.",
            _ => unreachable!(),
        })
        .collect::<Vec<u8>>();

    let start_index = map.iter().position(|c| *c == b'@').unwrap();
    map[start_index] = b'.';

    let map = Map {
        input: map,
        width: width * 2,
        height: width,
    };

    let lines = input.lines();

    let movements = lines
        .skip(movement_line_start)
        .flat_map(|l| {
            l.chars().map(|c| match c {
                '^' => Direction::Up,
                '>' => Direction::Right,
                'v' => Direction::Down,
                '<' => Direction::Left,
                _ => unreachable!(),
            })
        })
        .collect();

    (map, movements, start_index)
}

pub fn part_one(input: &str) -> usize {
    let (mut map, move_attempts, start_index) = parse_input(input);

    // map.viz(map.index_to_coord(start_index));

    let (mut robot_x, mut robot_y) = map.index_to_coord(start_index);

    for movement in move_attempts {
        // println!("Move {}:", movement);

        let (movement_x, movement_y) = movement.into();
        let (mut next_pos_x, mut next_pos_y) = (robot_x + movement_x, robot_y + movement_y);
        let next_cell = map.get_from_coord(next_pos_x, next_pos_y);

        match next_cell {
            b'O' => {
                let (trailing_box_x, trailing_box_y) = (next_pos_x, next_pos_y);
                loop {
                    let (cell_after_box_pos_x, cell_after_box_pos_y) =
                        (next_pos_x + movement_x, next_pos_y + movement_y);
                    let cell_after_box =
                        map.get_from_coord(cell_after_box_pos_x, cell_after_box_pos_y);

                    match cell_after_box {
                        b'.' => {
                            let index_next = map.coord_to_index(trailing_box_x, trailing_box_y);
                            let index_cell_after =
                                map.coord_to_index(cell_after_box_pos_x, cell_after_box_pos_y);
                            map.input.swap(index_next, index_cell_after);

                            robot_x = trailing_box_x;
                            robot_y = trailing_box_y;

                            break;
                        }
                        b'#' => break,
                        b'O' => {
                            next_pos_x = cell_after_box_pos_x;
                            next_pos_y = cell_after_box_pos_y;
                        }
                        _ => unreachable!(),
                    }
                }
            }
            b'#' => (),
            b'.' => {
                robot_x = next_pos_x;
                robot_y = next_pos_y;
            }
            _ => unreachable!(),
        }

        // map.viz((robot_x, robot_y));
    }

    map.calculate_gps_box_sum()
}

pub fn part_two(input: &str) -> usize {
    let (mut map, move_attempts, start_index) = parse_input_b(input);

    map.viz(map.index_to_coord(start_index));

    let (mut robot_x, mut robot_y) = map.index_to_coord(start_index);

    for movement in move_attempts {
        println!("Move {}:", movement);

        let (movement_x, movement_y) = movement.into();
        let (mut next_pos_x, mut next_pos_y) = (robot_x + movement_x, robot_y + movement_y);
        let next_cell = map.get_from_coord(next_pos_x, next_pos_y);

        match next_cell {
            b'[' | b']' => {
                let (trailing_box_x, trailing_box_y) = (next_pos_x, next_pos_y);
                loop {
                    let (cell_after_box_pos_x, cell_after_box_pos_y) =
                        (next_pos_x + movement_x, next_pos_y + movement_y);
                    let cell_after_box =
                        map.get_from_coord(cell_after_box_pos_x, cell_after_box_pos_y);

                    match cell_after_box {
                        b'.' => {
                            let index_next = map.coord_to_index(trailing_box_x, trailing_box_y);
                            let index_cell_after =
                                map.coord_to_index(cell_after_box_pos_x, cell_after_box_pos_y);

                            map.input.swap(index_next, index_cell_after);

                            robot_x = trailing_box_x;
                            robot_y = trailing_box_y;

                            break;
                        }
                        b'#' => break,
                        b'[' | b']' => {
                            next_pos_x = cell_after_box_pos_x;
                            next_pos_y = cell_after_box_pos_y;
                        }
                        _ => unreachable!(),
                    }
                }
            }
            b'#' => (),
            b'.' => {
                robot_x = next_pos_x;
                robot_y = next_pos_y;
            }
            _ => unreachable!(),
        }

        map.viz((robot_x, robot_y));
    }

    map.calculate_gps_large_box_sum()
}

#[test]
fn part_1_small_input() {
    let input = include_str!("../input_small_1.txt");
    let out = part_one(input);

    assert_eq!(out, 2028);
}

#[test]
fn part_1_large_input() {
    let input = include_str!("../input_large_1.txt");
    let out = part_one(input);

    assert_eq!(out, 10092);
}

#[test]
fn part_1_input() {
    let input = include_str!("../input.txt");
    let out = part_one(input);

    assert_eq!(out, 1515788);
}

#[test]
fn part_2_large_input() {
    let input = include_str!("../input_large_1.txt");
    let out = part_two(input);

    assert_eq!(out, 9021);
}

#[test]
fn part_2_input() {
    let input = include_str!("../input.txt");
    let out = part_two(input);

    // assert_eq!(out, 6620);
}
