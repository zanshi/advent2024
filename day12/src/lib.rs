const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

struct Region {
    area: usize,
    perimeter: usize,
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
    fn get(&self, index: usize) -> char {
        unsafe { *self.input.get_unchecked(index) as char }
    }

    fn find_region_cost(
        &self,
        (x, y): (i32, i32),
        last_plant_type: char,
        visited_plots: &mut Vec<bool>,
        region: &mut Region,
    ) {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.width as i32 {
            region.perimeter += 1;
            return;
        }

        let index = self.coord_to_index(x, y);
        let plant_type = self.get(index);

        if plant_type == last_plant_type {
            if visited_plots[index] {
                return;
            } else {
                visited_plots[index] = true;
            }

            region.area += 1;

            self.find_region_cost(
                (x + DIRECTIONS[0].0, y + DIRECTIONS[0].1),
                plant_type,
                visited_plots,
                region,
            );

            self.find_region_cost(
                (x + DIRECTIONS[1].0, y + DIRECTIONS[1].1),
                plant_type,
                visited_plots,
                region,
            );

            self.find_region_cost(
                (x + DIRECTIONS[2].0, y + DIRECTIONS[2].1),
                plant_type,
                visited_plots,
                region,
            );

            self.find_region_cost(
                (x + DIRECTIONS[3].0, y + DIRECTIONS[3].1),
                plant_type,
                visited_plots,
                region,
            );
        } else {
            region.perimeter += 1;
        }
    }

    fn viz(&self, pos: (i32, i32)) {
        println!();
        for y in 0..self.width {
            println!();
            for x in 0..self.width {
                if pos.0 == x as i32 && pos.1 == y as i32 {
                    print!("*");
                } else {
                    print!("{}", self.get(self.coord_to_index(x as i32, y as i32)));
                }
            }
        }
        println!();
        println!();
    }
}

pub fn part_one(input: &str) -> i64 {
    // regions
    // area == number of garden plots
    // perimeter == number of sides in a region that do not touch another plot in the same region
    // price of fence for region = area * perimeter
    // can have regions within regions

    let width = input.find('\n').unwrap();
    let input = input.split('\n').collect::<String>();
    let input = input.as_bytes();

    let mut visited = vec![false; input.len()];

    let map = Map { input, width };

    let mut total_fence_costs = 0;

    for index in 0..map.input.len() {
        if !visited[index] {
            let coord = map.index_to_coord(index);
            let plant_type = map.get(index);
            let mut region = Region {
                area: 0,
                perimeter: 0,
            };

            map.find_region_cost(coord, plant_type, &mut visited, &mut region);

            let region_fence_price = region.area * region.perimeter;

            total_fence_costs += region_fence_price;
        }
    }

    total_fence_costs as i64
}

pub fn part_two() -> i64 {
    0
}

#[test]
fn small_1() {
    let input = include_str!("../input_small_1.txt");
    let out = part_one(input);

    assert_eq!(out, 140);
}

#[test]
fn small_2() {
    let input = include_str!("../input_small_2.txt");
    let out = part_one(input);

    assert_eq!(out, 772);
}

#[test]
fn small_3() {
    let input = include_str!("../input_small_3.txt");
    let out = part_one(input);

    assert_eq!(out, 1930);
}

#[test]
fn input() {
    let input = include_str!("../input.txt");
    let out = part_one(input);

    assert_eq!(out, 1546338);
}
