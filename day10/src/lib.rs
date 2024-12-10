struct Map<'a> {
    input: &'a [u8],
    width: usize,
}

impl<'a> Map<'a> {
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

    fn trail_head_score(&self, (x, y): (i32, i32), height: u8) -> i32 {
        const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

        let mut score = 0;

        for (dir_x, dir_y) in DIRECTIONS {
            let coord = (x + dir_x, y + dir_y);
            if coord.0 >= 0
                && coord.0 < self.width as i32
                && coord.1 >= 0
                && coord.1 < self.width as i32
            {
                let new_height = self.get(self.coord_to_index(coord));
                if new_height == height + 1 {
                    if new_height == 9 {
                        score += 1;
                    } else {
                        score += self.trail_head_score(coord, new_height);
                    }
                }
            }
        }

        score
    }

    fn viz(&self, pos: (i32, i32)) {
        println!();
        for y in 0..self.width {
            println!();
            for x in 0..self.width {
                if pos.0 == x as i32 && pos.1 == y as i32 {
                    print!("*");
                } else {
                    print!("{}", self.get(self.coord_to_index((x as i32, y as i32))));
                }
            }
        }
        println!();
        println!();
    }
}

pub fn part_one() -> i64 {
    let input = include_str!("../input_small.txt");
    let width = input.find('\n').unwrap();
    let input = input.split('\n').collect::<String>();
    let input = input.as_bytes();

    let map = Map { input, width };

    // map.viz();

    let mut total_trail_head_score = 0;

    for i in 0..input.len() {
        let start = input[i] - 48;
        if start == 0 {
            let coord = map.index_to_coord(i);
            // map.viz(coord);
            total_trail_head_score += map.trail_head_score(coord, 0);
        }
    }

    total_trail_head_score as i64
}

pub fn part_two() -> i64 {
    let input = include_str!("../input.txt");
    let input = input.as_bytes();

    0
}
