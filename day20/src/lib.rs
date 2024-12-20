use std::{cmp::Reverse, collections::BinaryHeap};

use glam::IVec2;
use rustc_hash::{FxBuildHasher, FxHashSet as HashSet};

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

    fn is_inside(&self, coord: IVec2) -> bool {
        !(coord.x >= self.width as i32
            || coord.x < 0
            || coord.y >= self.width as i32
            || coord.y < 0)
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

    fn viz(&self) {
        for y in 0..self.width {
            println!();
            for x in 0..self.width {
                let coord = IVec2::new(x as i32, y as i32);
                let c = if self.get_from_coord(coord) == 1 {
                    '*'
                } else {
                    '.'
                };

                print!("{}", c);
            }
        }
        println!();
        println!();
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Node {
    index: usize,
    prev: usize,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            index: usize::MAX,
            prev: usize::MAX,
        }
    }
}

fn find_path_len(map: &Map, open_set: &mut OpenSet, start_pos: IVec2, end_pos: IVec2) -> usize {
    let start_index = map.coord_to_index(start_pos);
    let end_index = map.coord_to_index(end_pos);

    let mut visited = vec![Node::default(); map.width * map.width];

    let mut steps = vec![i32::MAX; map.width * map.width];

    let start_node = Node {
        index: start_index,
        prev: usize::MAX,
    };

    open_set.push(Reverse((0, start_node)));
    steps[start_index] = 0;

    visited[start_index] = start_node;

    while !open_set.is_empty() {
        let curr = open_set.pop().unwrap();
        let curr_steps = curr.0 .0;
        let curr_index = curr.0 .1.index;

        if curr_index == end_index {
            let mut node = curr.0 .1;

            let mut path_len = 0;

            loop {
                path_len += 1;

                if node.prev == usize::MAX {
                    return path_len;
                }

                node = visited[node.prev];
            }
        }

        visited[curr_index] = curr.0 .1;

        let curr_pos = map.index_to_coord(curr_index);

        for direction in DIRECTIONS {
            let next_pos = curr_pos + direction;

            if !map.is_inside(next_pos) {
                continue;
            }

            if map.get_from_coord(next_pos) == b'#' {
                continue;
            }

            let next_index = map.coord_to_index(next_pos);
            let next_steps = curr_steps + 1;

            if next_steps < steps[next_index] {
                steps[next_index] = next_steps;

                open_set.push(Reverse((
                    next_steps,
                    Node {
                        index: next_index,
                        prev: curr_index,
                    },
                )));
            }
        }
    }

    unreachable!()
}

fn find_path(map: &Map, open_set: &mut OpenSet, start_pos: IVec2, end_pos: IVec2) -> Vec<usize> {
    let start_index = map.coord_to_index(start_pos);
    let end_index = map.coord_to_index(end_pos);

    let mut visited = vec![Node::default(); map.width * map.width];

    let mut steps = vec![i32::MAX; map.width * map.width];

    let start_node = Node {
        index: start_index,
        prev: usize::MAX,
    };

    open_set.push(Reverse((0, start_node)));
    steps[start_index] = 0;

    while !open_set.is_empty() {
        let curr = open_set.pop().unwrap();
        let curr_steps = curr.0 .0;
        let curr_index = curr.0 .1.index;

        if curr_index == end_index {
            let mut reconstructed_path = Vec::new();

            let mut node = curr.0 .1;

            loop {
                reconstructed_path.push(node.index);

                if node.prev == usize::MAX {
                    return reconstructed_path;
                }

                node = visited[node.prev];
            }
        }

        visited[curr_index] = curr.0 .1;

        let curr_pos = map.index_to_coord(curr_index);

        for direction in DIRECTIONS {
            let next_pos = curr_pos + direction;

            if !map.is_inside(next_pos) {
                continue;
            }

            if map.get_from_coord(next_pos) == b'#' {
                continue;
            }

            let next_index = map.coord_to_index(next_pos);

            let next_steps = curr_steps + 1;

            if next_steps < steps[next_index] {
                steps[next_index] = next_steps;

                open_set.push(Reverse((
                    next_steps,
                    Node {
                        index: next_index,
                        prev: curr_index,
                    },
                )));
            }
        }
    }

    unreachable!()
}

fn solve_maze(
    map: &Map,
    was_here: &mut [bool],
    correct_path: &mut [u8],
    correct_path_len: &mut i32,
    pos: IVec2,
    end_pos: IVec2,
) -> bool {
    if pos == end_pos {
        return true;
    }
    if !map.is_inside(pos) {
        return false;
    }

    let index = map.coord_to_index(pos);
    if map.get_from_coord(pos) == b'#' || was_here[index] {
        return false;
    }

    was_here[index] = true;

    for direction in DIRECTIONS {
        let next_pos = pos + direction;

        if !map.is_inside(pos) {
            continue;
        }

        let next_index = map.coord_to_index(next_pos);

        if solve_maze(
            map,
            was_here,
            correct_path,
            correct_path_len,
            next_pos,
            end_pos,
        ) {
            *correct_path_len += 1;
            correct_path[next_index] = 1;
            return true;
        }
    }

    false
}

#[derive(Clone, Copy)]
struct Cheat {
    start: usize,
    end: usize,
}

impl Default for Cheat {
    fn default() -> Self {
        Self {
            start: usize::MAX,
            end: usize::MAX,
        }
    }
}

type OpenSet = BinaryHeap<Reverse<(i32, Node)>>;

pub fn part_one(input: &str) -> i32 {
    let width = input.find('\n').unwrap();
    let input = input
        .split('\n')
        .flat_map(|s| s.as_bytes())
        .copied()
        .collect::<Vec<u8>>();

    let start_index = input.iter().position(|x| *x == b'S').unwrap();
    let end_index = input.iter().position(|x| *x == b'E').unwrap();

    let mut map = Map { data: input, width };

    let start_pos = map.index_to_coord(start_index);
    let end_pos = map.index_to_coord(end_index);

    let mut open_set: OpenSet = BinaryHeap::new();
    let path = find_path(&map, &mut open_set, start_pos, end_pos);

    let path_len = path.len();

    let mut faster_paths_count = 0;

    let mut was_here = vec![false; map.width * map.width];
    let mut correct_path = vec![0i32; map.width * map.width];
    // let mut initial_path_len = 0;

    // let solved_maze = solve_maze(
    //     &map,
    //     &mut was_here,
    //     &mut correct_path,
    //     &mut initial_path_len,
    //     start_pos,
    //     end_pos,
    // );

    // assert_eq!(initial_path_len as usize + 1, path_len);

    // let path_len = correct_path.iter().fold(0i32, |acc, x| acc + (*x as i32));

    for (i, index) in path.iter().enumerate() {
        correct_path[*index] = (path.len() - i) as i32 - 1;
    }

    let mut cheat_set = HashSet::with_hasher(FxBuildHasher);
    for (i, index) in path.iter().enumerate() {
        let coord = map.index_to_coord(*index);

        for direction in DIRECTIONS {
            let next_pos = coord + direction;

            if !map.is_inside(next_pos) {
                continue;
            }

            if map.get_from_coord(next_pos) == b'#' {
                let after_wall_coord = next_pos + direction;

                if !map.is_inside(after_wall_coord) {
                    continue;
                }

                if map.get_from_coord(after_wall_coord) != b'#' {
                    let cheat_start_index = map.coord_to_index(next_pos);
                    let cheat_end_index = map.coord_to_index(after_wall_coord);

                    if cheat_set.insert((cheat_start_index, cheat_end_index)) {
                        let new_path_len = i as i32 + correct_path[cheat_end_index] + 2;
                        let saved_picoseconds = path_len as i32 - new_path_len;

                        if saved_picoseconds >= 100 {
                            faster_paths_count += 1;
                        }
                    }
                }
            }
        }
    }

    faster_paths_count
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
