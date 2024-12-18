use glam::IVec2;
use std::{cmp::Reverse, collections::BinaryHeap};

const DIRECTIONS: [IVec2; 4] = [
    IVec2::new(0, -1),
    IVec2::new(1, 0),
    IVec2::new(0, 1),
    IVec2::new(-1, 0),
];

struct Map {
    data: Vec<u8>,
    width: usize,
}

impl Map {
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

pub fn part_one(input: &str, map_width: usize, number_of_bytes: usize) -> i32 {
    let mut map = Map {
        data: vec![b'.'; map_width * map_width],
        width: map_width,
    };

    let mut count = 0;
    for line in input.lines() {
        let mut split = line.split(',');
        let x = split.next().unwrap().parse::<i32>().unwrap();
        let y = split.next().unwrap().parse::<i32>().unwrap();

        let coord = IVec2::new(x, y);
        let index = map.coord_to_index(coord);

        map.data[index] = b'#';
        count += 1;

        if count >= number_of_bytes {
            break;
        }
    }

    // map.viz((0, 0));

    shortest_path_steps(
        &map,
        IVec2::new(0, 0),
        IVec2::new((map_width - 1) as i32, (map_width - 1) as i32),
    )
}

pub fn part_two(_input: &str) -> i64 {
    0
}

#[test]
fn part_1_small_input() {
    let input = include_str!("../input_small_1.txt");
    let out = part_one(input, 7, 12);

    assert_eq!(out, 22);
}

#[test]
fn part_1_input() {
    let input = include_str!("../input.txt");
    let out = part_one(input, 71, 1024);

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
