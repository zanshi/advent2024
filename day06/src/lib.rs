pub fn part_one() -> i32 {
    // 130x130 grid

    // - make grid
    // - trace guard path by moving in grid
    // - count number of visited grid cells

    let input = include_str!("../input.txt").split('\n').collect::<String>();
    let input = input.as_bytes();

    const GRID_SIDE: i32 = 130;

    enum GuardDirection {
        Up,
        Right,
        Down,
        Left,
    }

    struct Grid<'a> {
        data: &'a [u8],
        direction: GuardDirection,
        visited_map: [u8; (GRID_SIDE * GRID_SIDE) as usize],
        x: i32,
        y: i32,
    }

    fn index(x: i32, y: i32) -> usize {
        (x + GRID_SIDE * y) as usize
    }

    impl<'a> Grid<'a> {
        fn new(data: &'a [u8]) -> Self {
            let guard_index = data.iter().position(|x| *x == b'^').unwrap() as i32;

            let x = guard_index % GRID_SIDE;
            let y = guard_index / GRID_SIDE;

            let mut visited_map = [0; (GRID_SIDE * GRID_SIDE) as usize];
            visited_map[index(x, y)] = 1;

            Self {
                data,
                direction: GuardDirection::Up,
                visited_map,
                x,
                y,
            }
        }

        fn count_visited(&self) -> usize {
            self.visited_map.iter().fold(0, |acc, x| acc + *x as usize)
        }

        fn walk(&mut self) -> usize {
            let next_coord = match self.direction {
                GuardDirection::Up => (self.x, self.y - 1),
                GuardDirection::Right => (self.x + 1, self.y),
                GuardDirection::Down => (self.x, self.y + 1),
                GuardDirection::Left => (self.x - 1, self.y),
            };

            // outside
            if next_coord.0 > (GRID_SIDE - 1)
                || next_coord.0 < 0
                || next_coord.1 < 0
                || next_coord.1 > (GRID_SIDE - 1)
            {
                return self.count_visited();
            }

            let idx = index(next_coord.0, next_coord.1);
            let cell = self.data[idx];

            match cell {
                b'.' | b'^' => {
                    self.visited_map[idx] = 1;
                    self.x = next_coord.0;
                    self.y = next_coord.1;
                }
                b'#' => match self.direction {
                    GuardDirection::Up => self.direction = GuardDirection::Right,
                    GuardDirection::Right => self.direction = GuardDirection::Down,
                    GuardDirection::Down => self.direction = GuardDirection::Left,
                    GuardDirection::Left => self.direction = GuardDirection::Up,
                },
                _ => (),
            }

            self.walk()
        }
    }

    let mut grid = Grid::new(input);

    let visited_cells = grid.walk();

    visited_cells as i32
}

pub fn part_two() -> i32 {
    let input = include_str!("../input.txt").split('\n').collect::<String>();
    let input = input.as_bytes();

    const GRID_SIDE: i32 = 130;

    #[derive(Clone, Copy, PartialEq)]
    enum GuardDirection {
        Up,
        Right,
        Down,
        Left,
    }

    struct Grid<'a> {
        data: &'a [u8],
        direction: GuardDirection,
        visited_map: Vec<(u8, Vec<GuardDirection>)>,
        obstruction_index: usize,
        x: i32,
        y: i32,
    }

    fn index(x: i32, y: i32) -> usize {
        (x + GRID_SIDE * y) as usize
    }

    impl<'a> Grid<'a> {
        fn new(data: &'a [u8], obstruction_index: usize) -> Self {
            let guard_index = data.iter().position(|x| *x == b'^').unwrap() as i32;

            let x = guard_index % GRID_SIDE;
            let y = guard_index / GRID_SIDE;

            let mut visited_map = Vec::new();
            visited_map.resize((GRID_SIDE * GRID_SIDE) as usize, (0, Vec::with_capacity(4)));
            visited_map[index(x, y)] = (1, vec![GuardDirection::Up]);

            Self {
                data,
                direction: GuardDirection::Up,
                visited_map,
                x,
                y,
                obstruction_index,
            }
        }

        fn walking_in_loop(&mut self) -> bool {
            let next_coord = match self.direction {
                GuardDirection::Up => (self.x, self.y - 1),
                GuardDirection::Right => (self.x + 1, self.y),
                GuardDirection::Down => (self.x, self.y + 1),
                GuardDirection::Left => (self.x - 1, self.y),
            };

            // outside
            if next_coord.0 > (GRID_SIDE - 1)
                || next_coord.0 < 0
                || next_coord.1 < 0
                || next_coord.1 > (GRID_SIDE - 1)
            {
                return false;
            }

            let idx = index(next_coord.0, next_coord.1);

            if idx == self.obstruction_index {
                match self.direction {
                    GuardDirection::Up => self.direction = GuardDirection::Right,
                    GuardDirection::Right => self.direction = GuardDirection::Down,
                    GuardDirection::Down => self.direction = GuardDirection::Left,
                    GuardDirection::Left => self.direction = GuardDirection::Up,
                }

                return self.walking_in_loop();
            }

            let cell = self.data[idx];

            match cell {
                b'.' | b'^' => {
                    // already visited in this direction, loop
                    if self.visited_map[idx].0 == 1
                        && self.visited_map[idx].1.contains(&self.direction)
                    {
                        return true;
                    }

                    self.visited_map[idx].0 = 1;
                    self.visited_map[idx].1.push(self.direction);

                    self.x = next_coord.0;
                    self.y = next_coord.1;
                }
                b'#' => match self.direction {
                    GuardDirection::Up => self.direction = GuardDirection::Right,
                    GuardDirection::Right => self.direction = GuardDirection::Down,
                    GuardDirection::Down => self.direction = GuardDirection::Left,
                    GuardDirection::Left => self.direction = GuardDirection::Up,
                },
                _ => (),
            }

            self.walking_in_loop()
        }
    }

    let mut obstruction_positions = 0;

    for i in 0..input.len() {
        if input[i] == b'#' || input[i] == b'^' {
            continue;
        }

        let mut grid = Grid::new(input, i);

        if grid.walking_in_loop() {
            obstruction_positions += 1;
        }
    }

    obstruction_positions
}
