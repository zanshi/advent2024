use glam::IVec2;
use std::{cmp::Reverse, collections::BinaryHeap};

const DIRECTIONS: [IVec2; 4] = [
    IVec2::new(0, -1),
    IVec2::new(1, 0),
    IVec2::new(0, 1),
    IVec2::new(-1, 0),
];

struct Map<'a> {
    data: &'a [u8],
    width: usize,
}

impl Map<'_> {
    #[inline(always)]
    fn coord_to_index(&self, coord: IVec2) -> usize {
        (coord.y * self.width as i32 + coord.x) as usize
    }

    #[inline(always)]
    fn index_to_coord(&self, index: usize) -> IVec2 {
        let x = index % self.width;
        let y = index / self.width;

        IVec2::new(x as i32, y as i32)
    }

    #[inline(always)]
    fn get_from_coord(&self, coord: IVec2) -> u8 {
        *self.data.get(self.coord_to_index(coord)).unwrap()
    }

    // #[inline(always)]
    // fn get(&self, index: usize) -> u8 {
    //     *self.data.get(index).unwrap()
    // }

    // fn viz(&self, pos: (i32, i32)) {
    //     for y in 0..self.width {
    //         println!();
    //         for x in 0..self.width {
    //             let coord = IVec2::new(x as i32, y as i32);
    //             let c = self.get_from_coord(coord) as char;

    //             if pos.0 == x as i32 && pos.1 == y as i32 {
    //                 print!("O");
    //             } else {
    //                 print!("{}", c);
    //             }
    //         }
    //     }
    //     println!();
    //     println!();
    // }
}

fn shortest_path_steps(map: &Map, start_pos: IVec2, end_pos: IVec2) -> i32 {
    let mut open_set: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();

    let start_index = map.coord_to_index(start_pos);
    let end_index = map.coord_to_index(end_pos);

    let mut steps = vec![i32::MAX; map.width * map.width];

    open_set.push(Reverse((0, start_index)));
    steps[start_index] = 0;

    let mut minimum_steps = i32::MAX;

    while !open_set.is_empty() {
        let curr = open_set.pop().unwrap();
        let curr_steps = curr.0 .0;
        let curr_index = curr.0 .1;

        if curr_index == end_index {
            minimum_steps = minimum_steps.min(curr_steps);
        }

        let curr_pos = map.index_to_coord(curr_index);

        for direction in DIRECTIONS {
            let next_pos = curr_pos + direction;

            if next_pos.x >= map.width as i32
                || next_pos.x < 0
                || next_pos.y >= map.width as i32
                || next_pos.y < 0
            {
                continue;
            }

            if map.get_from_coord(next_pos) == b'#' {
                continue;
            }

            let next_index = map.coord_to_index(next_pos);
            let next_steps = curr_steps + 1;

            if next_steps < steps[next_index] {
                steps[next_index] = next_steps;

                open_set.push(Reverse((next_steps, next_index)));
            }
        }
    }

    minimum_steps
}

struct State {
    steps: Vec<i32>,
    in_open_set: Vec<u8>,
    closed_set: Vec<u8>,
}

fn find_path(map: &Map, start_pos: IVec2, end_pos: IVec2, state: &mut State) -> i32 {
    let mut open_set: BinaryHeap<(i32, usize)> = BinaryHeap::new();

    let start_index = map.coord_to_index(start_pos);
    let end_index = map.coord_to_index(end_pos);

    open_set.push((0, start_index));
    state.in_open_set[start_index] = 1;
    state.steps[start_index] = 0;

    while !open_set.is_empty() {
        let curr = open_set.pop().unwrap();
        let curr_steps = curr.0;
        let curr_index = curr.1;

        state.in_open_set[curr_index] = 0;

        state.closed_set[curr_index] = 1;

        let curr_pos = map.index_to_coord(curr_index);

        for direction in DIRECTIONS {
            let next_pos = curr_pos + direction;

            if next_pos.x >= map.width as i32
                || next_pos.x < 0
                || next_pos.y >= map.width as i32
                || next_pos.y < 0
            {
                continue;
            }

            let next_index = map.coord_to_index(next_pos);

            let next_steps = curr_steps + 1;

            if next_index == end_index {
                return next_steps;
            }

            if map.get_from_coord(next_pos) == b'#' {
                continue;
            }

            if state.closed_set[next_index] == 1 {
                continue;
            }

            if next_steps < state.steps[next_index] {
                state.steps[next_index] = next_steps;

                if state.in_open_set[next_index] == 0 {
                    open_set.push((next_steps, next_index));
                    state.in_open_set[next_index] = 1;
                }
            }
        }
    }

    i32::MAX
}

pub fn part_one(input: &str) -> i32 {
    let width = input.find('\n').unwrap();
    let input = input.split('\n').collect::<String>();
    let input = input.as_bytes();

    let start_index = input.iter().position(|x| *x == b'S').unwrap();
    let end_index = input.iter().position(|x| *x == b'E').unwrap();

    let map = Map { data: input, width };

    shortest_path_steps(
        &map,
        map.index_to_coord(start_index),
        map.index_to_coord(end_index),
    )
}

pub fn part_two(input: &str) -> i32 {
    0
}

#[test]
fn part_1_small_input() {
    let input = include_str!("../input_small_1.txt");
    let out = part_one(input);

    assert_eq!(out, 22);
}

#[test]
fn part_1_input() {
    let input = include_str!("../input.txt");
    let out = part_one(input);

    assert_eq!(out, 436);
}

#[test]
fn part_2_small_input() {
    let input = include_str!("../input_small_1.txt");
    let out = part_two(input);

    assert_eq!(out, 0);
}

// #[test]
// fn part_2_input() {
//     let input = include_str!("../input.txt");
//     let out = part_two(input);

//     assert_eq!(out, 6620);
// }
