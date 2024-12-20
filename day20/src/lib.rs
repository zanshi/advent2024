use std::collections::VecDeque;

use glam::IVec2;
use rustc_hash::{FxBuildHasher, FxHashSet as HashSet};

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
    fn is_outside(&self, coord: IVec2) -> bool {
        coord.x >= self.width as i32 || coord.x < 0 || coord.y >= self.width as i32 || coord.y < 0
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

            let next_index = map.coord_to_index(next_pos);

            if map.get_from_coord(next_pos) == b'#' || path_map[next_index] != i32::MAX {
                continue;
            }

            curr_index = map.coord_to_index(next_pos);

            break;
        }
    }
}

fn breadth_first_search(
    map: &Map,
    visited: &mut [bool],
    queue: &mut VecDeque<(usize, i32)>,
    cheat_set: &mut HashSet<(i32, i32)>,
    path_map: &[i32],
    start_index: usize,
    path_index: usize,
    path_len: i32,
) -> i32 {
    let cheat_len = 0;
    visited[start_index] = true;
    queue.push_back((start_index, cheat_len));

    let mut valid_cheats = 0;

    while let Some((current_index, cheat_len)) = queue.pop_front() {
        if cheat_len > 20 {
            continue;
        }

        let pos = map.index_to_coord(current_index);

        if map.get_from_coord(pos) != b'#'
            && cheat_set.insert((start_index as i32, current_index as i32))
        {
            let after_start = path_map[current_index] > path_index as i32;

            if after_start {
                let new_path_len =
                    path_index as i32 + (path_len - path_map[current_index]) + cheat_len;

                if let Some(saved_picoseconds) = path_len.checked_sub(new_path_len) {
                    if saved_picoseconds >= 100 {
                        valid_cheats += 1;
                    }
                }
            }
        }

        {
            for direction in DIRECTIONS {
                let next_pos = pos + direction;

                if map.is_outside(next_pos) {
                    continue;
                }

                let next_index = map.coord_to_index(next_pos);

                if !visited[next_index] {
                    visited[next_index] = true;
                    queue.push_back((next_index, cheat_len + 1));
                }
            }
        }
    }

    valid_cheats
}

fn solve_maze(
    map: &Map,
    cheat_map: &mut HashSet<(i32, i32)>,
    was_here: &mut [bool],
    path_map: &[i32],
    cheat_len: i32,
    start_index: usize,
    pos: IVec2,
    valid_cheats: &mut i32,
    path_index: usize,
    path_len: i32,
) -> bool {
    if cheat_len > 20 {
        return false;
    }

    let index = map.coord_to_index(pos);

    if map.is_outside(pos) {
        return false;
    }

    if was_here[index] {
        return false;
    }

    was_here[index] = true;

    // exited wall
    if map.get_from_coord(pos) != b'#' {
        if cheat_map.insert((start_index as i32, index as i32)) {
            let after_start = path_map[index] > path_index as i32;

            if after_start {
                let new_path_len = path_index as i32 + (path_len - path_map[index]) + cheat_len;
                if let Some(saved_picoseconds) = path_len.checked_sub(new_path_len) {
                    if saved_picoseconds >= 50 {
                        *valid_cheats += 1;
                    }
                }
            }

            return true;
        }

        return false;
    }

    for direction in DIRECTIONS {
        let next_pos = pos + direction;

        if map.is_outside(pos) {
            continue;
        }

        solve_maze(
            map,
            cheat_map,
            was_here,
            path_map,
            cheat_len + 1,
            start_index,
            next_pos,
            valid_cheats,
            path_index,
            path_len,
        );
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
    let mut cheat_map = vec![0i32; map.width * map.width];

    let path = find_path(&map, &mut path_map, start_pos, end_pos);
    let path_len = path.len() as i32;

    let mut faster_paths_count = 0;

    for (i, index) in path.iter().enumerate() {
        let coord = map.index_to_coord(*index);

        for (dir_idx, direction) in DIRECTIONS.iter().enumerate() {
            let next_pos = coord + direction;

            if map.get_from_coord(next_pos) == b'#' {
                let after_wall_coord = next_pos + direction;

                if map.is_outside(after_wall_coord) {
                    continue;
                }

                if map.get_from_coord(after_wall_coord) != b'#' {
                    let cheat_start_index = map.coord_to_index(next_pos);
                    let cheat_end_index = map.coord_to_index(after_wall_coord);

                    let cheat = cheat_map[cheat_start_index];

                    let dir_mask = 1 << (dir_idx + 1);

                    if cheat & dir_mask > 0 {
                        continue;
                    }

                    cheat_map[cheat_start_index] |= dir_mask;

                    let new_path_len = i as i32 + (path_len - path_map[cheat_end_index]) + 2;
                    let saved_picoseconds = path_len as i32 - new_path_len;

                    if saved_picoseconds >= 100 {
                        faster_paths_count += 1;
                    }
                }
            }
        }
    }

    faster_paths_count
}

pub fn part_two(input: &str) -> i32 {
    // 451688 -too low
    // 508291- too low
    // 1139964 - too low

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
    let mut cheat_set = HashSet::with_capacity_and_hasher(20000, FxBuildHasher);

    let path = find_path(&map, &mut path_map, start_pos, end_pos);
    let path_len = path.len() as i32;

    let mut faster_paths_count = 0;

    let mut visited = vec![false; map.data.len()];

    let mut queue = VecDeque::with_capacity(1024);

    for (distance_on_path, index) in path.iter().enumerate() {
        let valid_cheats = breadth_first_search(
            &map,
            &mut visited,
            &mut queue,
            &mut cheat_set,
            &path_map,
            *index,
            distance_on_path,
            path_len,
        );

        visited.fill(false);
        queue.clear();

        faster_paths_count += valid_cheats;
    }

    faster_paths_count
}

#[test]
fn part_1_input() {
    let input = include_str!("../input.txt");
    let out = part_one(input);

    assert_eq!(out, 1317);
}

#[test]
fn part_2_input() {
    let input = include_str!("../input.txt");
    let out = part_two(input);

    assert_eq!(out, 982474);
}
