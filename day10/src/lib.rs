const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

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
    fn get(&self, index: usize) -> u8 {
        unsafe { *self.input.get_unchecked(index) }
    }

    fn trail_head_score(
        &self,
        (x, y): (i32, i32),
        prev_height: i32,
        visited_heads: &mut Vec<usize>,
    ) -> i32 {
        if x < 0 || x >= (self.width - 1) as i32 || y < 0 || y >= (self.width - 1) as i32 {
            return 0;
        }

        let index = self.coord_to_index(x, y);
        let height = self.get(index) as i32;

        if height == (prev_height + 1) {
            if height == 57 {
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

    fn trail_head_rating(&self, x: i32, y: i32, prev_height: u8) -> i32 {
        let mut dir_scores: [i32; 4] = [0i32; 4];

        for ((dir_x, dir_y), score) in DIRECTIONS.iter().zip(dir_scores.iter_mut()) {
            let x = x + dir_x;
            let y = y + dir_y;

            if x < 0 || x >= (self.width - 1) as i32 || y < 0 || y >= (self.width - 1) as i32 {
                continue;
            }

            let height = self.get(self.coord_to_index(x, y));

            if height == (prev_height + 1) {
                if height == 57 {
                    *score += 1;
                } else {
                    *score += self.trail_head_rating(x, y, height);
                }
            }
        }

        dir_scores.into_iter().sum()
    }

    // fn viz(&self, pos: (i32, i32)) {
    //     println!();
    //     for y in 0..WIDTH {
    //         println!();
    //         for x in 0..WIDTH {
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

pub fn part_one(input: &str) -> i32 {
    let width = input.find('\n').unwrap() + 1;
    let input = input.as_bytes();

    let map = Map { input, width };

    let trail_head_indices = input
        .iter()
        .enumerate()
        .filter_map(|(i, x)| (*x == 48).then_some(i));

    trail_head_indices.fold(0, |acc, i| {
        let mut visited_heads = Vec::new();
        let coord = map.index_to_coord(i);
        acc + map.trail_head_score(coord, 47, &mut visited_heads)
    })
}

pub fn part_two(input: &str) -> i64 {
    let width = input.find('\n').unwrap() + 1;
    let input = input.as_bytes();

    let map = Map { input, width };

    let mut total_trail_head_score = 0;

    for (i, start) in input.iter().enumerate() {
        if *start == 48 {
            let (x, y) = map.index_to_coord(i);
            total_trail_head_score += map.trail_head_rating(x, y, 48);
        }
    }

    total_trail_head_score as i64
}
