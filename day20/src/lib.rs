use std::{cmp::Reverse, collections::BinaryHeap, i32};

use glam::IVec2;
use rustc_hash::{FxBuildHasher, FxHashSet as HashSet};

type OpenSet = BinaryHeap<Reverse<(i32, Node)>>;

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

    #[inline(always)]
    fn is_inside(&self, coord: IVec2) -> bool {
        !(coord.x >= self.width as i32
            || coord.x < 0
            || coord.y >= self.width as i32
            || coord.y < 0)
    }

    // fn viz(&self) {
    //     for y in 0..self.width {
    //         println!();
    //         for x in 0..self.width {
    //             let coord = IVec2::new(x as i32, y as i32);
    //             let c = if self.get_from_coord(coord) == 1 {
    //                 '*'
    //             } else {
    //                 '.'
    //             };

    //             print!("{}", c);
    //         }
    //     }
    //     println!();
    //     println!();
    // }
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

fn find_path(map: &Map, path_map: &mut [i32], start_pos: IVec2, end_pos: IVec2) -> Vec<usize> {
    let mut curr_index = map.coord_to_index(start_pos);
    let end_index = map.coord_to_index(end_pos);

    let mut path = Vec::new();

    let mut path_len = 0;

    loop {
        path_map[curr_index] = path_len;
        path.push(curr_index);
        path_len += 1;

        if curr_index == end_index {
            return path;
        }

        let curr_pos = map.index_to_coord(curr_index);

        for direction in DIRECTIONS {
            let next_pos = curr_pos + direction;

            if !map.is_inside(next_pos) {
                continue;
            }

            let next_index = map.coord_to_index(next_pos);

            if map.get_from_coord(next_pos) == b'#' || path_map[next_index] != i32::MAX {
                continue;
            }

            curr_index = map.coord_to_index(next_pos);

            break;
        }
    }
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

pub fn part_one(input: &str) -> i32 {
    let width = input.find('\n').unwrap();
    let input = input.split('\n').collect::<String>();

    let start_index = input.find('S').unwrap();
    let end_index = input.find('E').unwrap();

    let map = Map {
        data: input.as_bytes(),
        width,
    };

    let start_pos = map.index_to_coord(start_index);
    let end_pos = map.index_to_coord(end_index);

    let mut path_map = vec![i32::MAX; map.width * map.width];

    let path = find_path(&map, &mut path_map, start_pos, end_pos);
    let path_len = path.len() as i32;

    let mut faster_paths_count = 0;

    let mut cheat_set = HashSet::with_capacity_and_hasher(20000, FxBuildHasher);

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

                    if cheat_set.insert((cheat_start_index as u16, cheat_end_index as u16)) {
                        let new_path_len = i as i32 + (path_len - path_map[cheat_end_index]) + 2;
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
    // let width = input.find('\n').unwrap();
    // let input = input.split('\n').collect::<String>();

    // let start_index = input.find('S').unwrap();
    // let end_index = input.find('E').unwrap();

    // let map = Map {
    //     data: input.as_bytes(),
    //     width,
    // };

    // let start_pos = map.index_to_coord(start_index);
    // let end_pos = map.index_to_coord(end_index);

    // let path = find_path(&map, start_pos, end_pos);

    // let path_len = path.len();

    // let mut faster_paths_count = 0;

    // let mut correct_path = vec![0i32; map.width * map.width];

    // for (i, index) in path.iter().enumerate() {
    //     correct_path[*index] = (path.len() - i) as i32;
    // }

    // let mut cheat_set = HashSet::with_hasher(FxBuildHasher);
    // for (i, index) in path.iter().enumerate() {
    //     let coord = map.index_to_coord(*index);

    //     for direction in DIRECTIONS {
    //         let next_pos = coord + direction;

    //         if !map.is_inside(next_pos) {
    //             continue;
    //         }

    //         if map.get_from_coord(next_pos) == b'#' {
    //             let after_wall_coord = next_pos + direction;

    //             if !map.is_inside(after_wall_coord) {
    //                 continue;
    //             }

    //             if map.get_from_coord(after_wall_coord) != b'#' {
    //                 let cheat_start_index = map.coord_to_index(next_pos);
    //                 let cheat_end_index = map.coord_to_index(after_wall_coord);

    //                 if cheat_set.insert((cheat_start_index, cheat_end_index)) {
    //                     let new_path_len = i as i32 + correct_path[cheat_end_index] + 2;
    //                     let saved_picoseconds = path_len as i32 - new_path_len;

    //                     if saved_picoseconds >= 100 {
    //                         faster_paths_count += 1;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    // faster_paths_count

    0
}

#[test]
fn part_1_input() {
    let input = include_str!("../input.txt");
    let out = part_one(input);

    assert_eq!(out, 1317);
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
