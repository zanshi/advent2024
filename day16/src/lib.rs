use std::{collections::BinaryHeap, fmt::Display};

use rustc_hash::{FxBuildHasher, FxHashMap as HashMap, FxHashSet as HashSet, FxHasher};

const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

#[derive(PartialEq, Clone, Copy, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl From<Direction> for (i32, i32) {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::North => write!(f, "^"),
            Direction::East => write!(f, ">"),
            Direction::South => write!(f, "v"),
            Direction::West => write!(f, "<"),
        }
    }
}

impl From<(i32, i32)> for Direction {
    fn from(value: (i32, i32)) -> Self {
        match value {
            (0, -1) => Direction::North,
            (1, 0) => Direction::East,
            (0, 1) => Direction::South,
            (-1, 0) => Direction::West,
            _ => unreachable!(),
        }
    }
}

struct Map<'a> {
    input: &'a [u8],
    width: usize,
}

impl Map<'_> {
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

    // fn viz(&self, pos: (i32, i32), dir: Direction) {
    //     for y in 0..self.width {
    //         println!();
    //         for x in 0..self.width {
    //             let c = self.get_from_coord(x as i32, y as i32) as char;

    //             if pos.0 == x as i32 && pos.1 == y as i32 {
    //                 print!("{}", dir);
    //             } else {
    //                 print!("{}", c);
    //             }
    //         }
    //     }
    //     println!();
    //     println!();
    // }
}

fn heuristic_part_one(curr_dir: Direction, next_dir: Direction) -> usize {
    // walk one step: 1 point
    let mut score = 1;

    // turn +-90 degrees: 1000 points
    if next_dir != curr_dir {
        score += 1000;
    }

    score
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Node {
    index: usize,
    parent_index: usize,
    score: i32,
    dir: Direction,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

fn a_star(map: &Map, start: usize, end: usize) -> i32 {
    let mut open_heap: BinaryHeap<(i32, Node)> = BinaryHeap::new();

    let mut open_set = HashSet::with_hasher(FxBuildHasher);

    // let mut closed_set: HashSet<(usize, Direction)> = HashSet::with_hasher(FxBuildHasher);

    let start_node = Node {
        index: start,
        parent_index: usize::MAX,
        score: 0,
        dir: Direction::East,
    };

    open_heap.push((0, start_node));
    open_set.insert(start);

    let mut g_scores = HashMap::with_hasher(FxBuildHasher);

    g_scores.insert((start, Direction::East), 0);

    let mut candidate_scores = Vec::new();

    while !open_heap.is_empty() {
        let (_, curr) = open_heap.pop().unwrap();
        open_set.remove(&curr.index);

        if curr.index == end {
            candidate_scores.push(curr.score);
        }

        // if closed_set.contains(&(curr.index, curr.dir)) {
        //     continue;
        // }

        // closed_set.insert((curr.index, curr.dir));

        let curr_pos = map.index_to_coord(curr.index);

        for dir in DIRECTIONS {
            let next_cell_pos = (curr_pos.0 + dir.0, curr_pos.1 + dir.1);

            // not allowed to turn 180 degrees
            match curr.dir {
                Direction::North => {
                    if next_cell_pos.1 > curr_pos.1 {
                        continue;
                    }
                }
                Direction::East => {
                    if next_cell_pos.0 < curr_pos.0 {
                        continue;
                    }
                }
                Direction::South => {
                    if next_cell_pos.1 < curr_pos.1 {
                        continue;
                    }
                }
                Direction::West => {
                    if next_cell_pos.0 > curr_pos.0 {
                        continue;
                    }
                }
            }

            // outside
            if next_cell_pos.0 >= map.width as i32
                || next_cell_pos.0 < 0
                || next_cell_pos.1 >= map.width as i32
                || next_cell_pos.1 < 0
            {
                continue;
            }

            // wall
            if map.get_from_coord(next_cell_pos.0, next_cell_pos.1) == b'#' {
                continue;
            }

            let next_cell_index = map.coord_to_index(next_cell_pos.0, next_cell_pos.1);
            let next_dir: Direction =
                (next_cell_pos.0 - curr_pos.0, next_cell_pos.1 - curr_pos.1).into();

            // if closed_set.contains(&(next_cell_index, next_dir)) {
            //     continue;
            // }

            let tentative_g =
                g_scores[&(curr.index, curr.dir)] + heuristic_part_one(curr.dir, next_dir);

            let g = g_scores
                .entry((next_cell_index, next_dir))
                .or_insert(usize::MAX);

            if tentative_g < *g {
                let next_node = Node {
                    index: next_cell_index,
                    parent_index: curr.index,
                    score: tentative_g as i32,
                    dir: next_dir,
                };

                *g = tentative_g;

                // map.viz((next_cell_pos.0, next_cell_pos.1), next_dir);

                if !open_set.contains(&next_cell_index) {
                    open_heap.push((next_node.score, next_node));
                    open_set.insert(next_cell_index);
                }
            }
        }
    }

    *candidate_scores.iter().min().unwrap()
}

pub fn part_one(input: &str) -> i32 {
    let width = input.find('\n').unwrap();
    let input = input.split('\n').collect::<String>();
    let input = input.as_bytes();

    let start_index = input.iter().position(|c| *c == b'S').unwrap();
    let end_index = input.iter().position(|c| *c == b'E').unwrap();

    let map = Map { input, width };

    // map.viz(map.index_to_coord(start_index), Direction::East);

    a_star(&map, start_index, end_index)
}

pub fn part_two(input: &str) -> usize {
    0
}

#[test]
fn part_1_small_input() {
    let input = include_str!("../input_small_1.txt");
    let out = part_one(input);

    assert_eq!(out, 7036);
}

#[test]
fn part_1_large_input() {
    let input = include_str!("../input_large_1.txt");
    let out = part_one(input);

    assert_eq!(out, 11048);
}

#[test]
fn part_1_input() {
    let input = include_str!("../input.txt");
    let out = part_one(input);

    assert_eq!(out, 89460);
}

#[test]
fn part_2_large_input() {
    let input = include_str!("../input_large_1.txt");
    let out = part_two(input);

    // assert_eq!(out, 9021);
}

#[test]
fn part_2_input() {
    let input = include_str!("../input.txt");
    let out = part_two(input);

    // assert_eq!(out, 6620);
}
