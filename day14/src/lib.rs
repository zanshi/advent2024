use glam::IVec2;

#[derive(Debug)]
struct Particle {
    p: IVec2,
    v: IVec2,
}

#[derive(Debug)]
struct Quadrant {
    upper_left: IVec2,
    lower_right: IVec2,
}

fn parse_input(input: &str) -> Vec<Particle> {
    let input = input.split('\n');

    let mut particles = Vec::new();

    for line in input {
        let (p, v) = line.split_once(' ').unwrap();

        let p = p
            .strip_prefix("p=")
            .unwrap()
            .split_once(',')
            .map(|(x, y)| IVec2 {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            })
            .unwrap();

        let v = v
            .strip_prefix("v=")
            .unwrap()
            .split_once(',')
            .map(|(x, y)| IVec2 {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            })
            .unwrap();

        particles.push(Particle { p, v });
    }

    particles
}

fn tick(particles: &mut Vec<Particle>, grid: IVec2) {
    for particle in particles {
        particle.p += particle.v;

        if particle.p.x >= grid.x {
            particle.p.x -= grid.x;
        }

        if particle.p.x < 0 {
            particle.p.x += grid.x;
        }

        if particle.p.y >= grid.y {
            particle.p.y -= grid.y;
        }

        if particle.p.y < 0 {
            particle.p.y += grid.y;
        }
    }
}

fn visualize_particles_on_grid(particles: &[Particle], grid: IVec2) {
    println!();
    for y in 0..grid.y {
        for x in 0..grid.x {
            let mut robot_count = 0;
            for particle in particles {
                if particle.p.x == x && particle.p.y == y {
                    robot_count += 1;
                }
            }
            if robot_count > 0 {
                print!("{}", robot_count);
            } else {
                print!(".");
            }
        }
        println!();
    }

    println!();
}

fn calculate_safety_factor(particles: &[Particle], quadrants: &[Quadrant; 4]) -> i64 {
    let mut quadrants_score = [0i64; 4];
    for particle in particles {
        for (i, quadrant) in quadrants.iter().enumerate() {
            if particle.p.x >= quadrant.upper_left.x
                && particle.p.y >= quadrant.upper_left.y
                && particle.p.x < quadrant.lower_right.x
                && particle.p.y < quadrant.lower_right.y
            {
                quadrants_score[i] += 1;
                break;
            }
        }
    }

    quadrants_score.into_iter().product()
}

pub fn part_one(input: &str) -> i64 {
    let mut particles = parse_input(input);

    let grid = IVec2::new(101, 103);

    let quadrants = [
        Quadrant {
            upper_left: IVec2::ZERO,
            lower_right: IVec2::new(grid.x / 2, grid.y / 2),
        },
        Quadrant {
            upper_left: IVec2::new(grid.x / 2 + 1, 0),
            lower_right: IVec2::new(grid.x, grid.y / 2),
        },
        Quadrant {
            upper_left: IVec2::new(0, grid.y / 2 + 1),
            lower_right: IVec2::new(grid.x / 2, grid.y),
        },
        Quadrant {
            upper_left: IVec2::new(grid.x / 2 + 1, grid.y / 2 + 1),
            lower_right: IVec2::new(grid.x, grid.y),
        },
    ];

    // visualize_particles_on_grid(&particles, grid);

    for _ in 0..100 {
        tick(&mut particles, grid);
        // visualize_particles_on_grid(&particles, grid);
    }

    // visualize_particles_on_grid(&particles, grid);

    calculate_safety_factor(&particles, &quadrants)
}

fn find_consecutive_ones(particles: &[Particle], grid: IVec2, grid_cells: &mut [u8]) -> bool {
    grid_cells.fill(0);

    for particle in particles {
        let index = (particle.p.y * grid.x + particle.p.x) as usize;
        grid_cells[index] += 1;
    }

    const SEARCH_PATTERN: &[u8; 8] = &[1u8; 8];

    for window in grid_cells.windows(8) {
        if window == SEARCH_PATTERN {
            return true;
        }
    }

    false
}

pub fn part_two(input: &str) -> i64 {
    let mut particles = parse_input(input);

    let grid = IVec2::new(101, 103);

    let mut grid_cells = vec![0u8; (grid.x * grid.y) as usize];

    for i in 0..10000 {
        tick(&mut particles, grid);

        let consecutive_ones = find_consecutive_ones(&particles, grid, &mut grid_cells);

        if consecutive_ones {
            // visualize_particles_on_grid(&particles, grid);
            return i + 1;
        }

        // code used to find it semi-manually...
        // let safety_factor = calculate_safety_factor(&particles, &quadrants);

        // if safety_factor < 200000000 {
        //     println!("Iteration: {}", i);
        //     visualize_particles_on_grid(&particles, grid);
        // }
    }

    0
}

#[test]
fn part_1_input() {
    let input = include_str!("../input.txt");
    let out = part_one(input);

    assert_eq!(out, 233709840);
}

#[test]
fn part_2_input() {
    let input = include_str!("../input.txt");
    let out = part_two(input);

    assert_eq!(out, 6620);
}
