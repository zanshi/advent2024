const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

struct Map<'a> {
    input: &'a [u8],
    width: usize,
}

impl Map<'_> {
    fn coord_to_index(&self, (x, y): (i32, i32)) -> usize {
        (y * self.width as i32 + x) as usize
    }

    fn index_to_coord(&self, index: usize) -> (i32, i32) {
        let x = index % self.width;
        let y = index / self.width;

        (x as i32, y as i32)
    }

    fn get(&self, index: usize) -> u8 {
        self.input[index] - 48
    }

    fn trail_head_score(
        &self,
        (x, y): (i32, i32),
        prev_height: i32,
        visited_heads: &mut Vec<usize>,
    ) -> i32 {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.width as i32 {
            return 0;
        }

        let index = self.coord_to_index((x, y));
        let height = self.get(index) as i32;

        if height == (prev_height + 1) {
            if height == 9 {
                if !visited_heads.contains(&index) {
                    visited_heads.push(index);
                    return 1;
                } else {
                    return 0;
                }
            }

            return self.trail_head_score(
                (x + DIRECTIONS[0].0, y + DIRECTIONS[0].1),
                height,
                visited_heads,
            ) + self.trail_head_score(
                (x + DIRECTIONS[1].0, y + DIRECTIONS[1].1),
                height,
                visited_heads,
            ) + self.trail_head_score(
                (x + DIRECTIONS[2].0, y + DIRECTIONS[2].1),
                height,
                visited_heads,
            ) + self.trail_head_score(
                (x + DIRECTIONS[3].0, y + DIRECTIONS[3].1),
                height,
                visited_heads,
            );
        }

        0
    }

    fn trail_head_rating(&self, (x, y): (i32, i32), prev_height: i32) -> i32 {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.width as i32 {
            return 0;
        }

        let index = self.coord_to_index((x, y));
        let height = self.get(index) as i32;

        if height == (prev_height + 1) {
            if height == 9 {
                return 1;
            }

            return self.trail_head_rating((x + DIRECTIONS[0].0, y + DIRECTIONS[0].1), height)
                + self.trail_head_rating((x + DIRECTIONS[1].0, y + DIRECTIONS[1].1), height)
                + self.trail_head_rating((x + DIRECTIONS[2].0, y + DIRECTIONS[2].1), height)
                + self.trail_head_rating((x + DIRECTIONS[3].0, y + DIRECTIONS[3].1), height);
        }

        0
    }

    // fn viz(&self, pos: (i32, i32)) {
    //     println!();
    //     for y in 0..self.width {
    //         println!();
    //         for x in 0..self.width {
    //             if pos.0 == x as i32 && pos.1 == y as i32 {
    //                 print!("*");
    //             } else {
    //                 print!("{}", self.get(self.coord_to_index((x as i32, y as i32))));
    //             }
    //         }
    //     }
    //     println!();
    //     println!();
    // }
}

pub fn part_one() -> i64 {
    let input = include_str!("../input.txt");
    let width = input.find('\n').unwrap();
    let input = input.split('\n').collect::<String>();
    let input = input.as_bytes();

    let map = Map { input, width };

    let mut total_trail_head_score = 0;

    let mut visited_heads = Vec::new();

    for (i, start) in input.iter().enumerate() {
        let start = start - 48;
        if start == 0 {
            let coord = map.index_to_coord(i);

            visited_heads.clear();

            let score = map.trail_head_score(coord, -1, &mut visited_heads);

            total_trail_head_score += score;
        }
    }

    total_trail_head_score as i64
}

pub fn part_two() -> i64 {
    let input = include_str!("../input.txt");
    let width = input.find('\n').unwrap();
    let input = input.split('\n').collect::<String>();
    let input = input.as_bytes();

    let map = Map { input, width };

    let mut total_trail_head_score = 0;

    for (i, start) in input.iter().enumerate() {
        let start = start - 48;
        if start == 0 {
            let coord = map.index_to_coord(i);

            let score = map.trail_head_rating(coord, -1);

            total_trail_head_score += score;
        }
    }

    total_trail_head_score as i64
}
