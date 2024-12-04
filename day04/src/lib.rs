pub fn part_one() -> i32 {
    let input = include_str!("../input.txt");

    // word search

    // horizontal - left to right (right to left)
    // vertical - top down (down top)
    // diagonal - up right + down right (down left + up left)

    // backwards - reverse previous searches

    // overlapping

    // find all instances of XMAS

    let mut count = 0;

    // horizontal
    {
        let lines = input.lines();

        for line in lines {
            // forwards + backwards
            for window in line.as_bytes().windows(4) {
                if window == b"XMAS" || window == b"SAMX" {
                    count += 1;
                }
            }
        }
    }

    let input = input.split('\n').collect::<String>();
    let input = &input[..];

    // vertical
    {
        // up + down
        for x in 0..140 {
            for y in 0..(140 - 4) {
                if (input.as_bytes()[y * 140 + x] == b'X'
                    && input.as_bytes()[(y + 1) * 140 + x] == b'M'
                    && input.as_bytes()[(y + 2) * 140 + x] == b'A'
                    && input.as_bytes()[(y + 3) * 140 + x] == b'S')
                    || (input.as_bytes()[y * 140 + x] == b'S'
                        && input.as_bytes()[(y + 1) * 140 + x] == b'A'
                        && input.as_bytes()[(y + 2) * 140 + x] == b'M'
                        && input.as_bytes()[(y + 3) * 140 + x] == b'X')
                {
                    count += 1;
                }
            }
        }
    }

    // diagonal
    {
        for y in 0..(140 - 4) {
            for x in 0..(140 - 4) {
                if (input.as_bytes()[y * 140 + x] == b'X'
                    && input.as_bytes()[(y + 1) * 140 + x + 1] == b'M'
                    && input.as_bytes()[(y + 2) * 140 + x + 2] == b'A'
                    && input.as_bytes()[(y + 3) * 140 + x + 3] == b'S')
                    || (input.as_bytes()[y * 140 + x] == b'S'
                        && input.as_bytes()[(y + 1) * 140 + x + 1] == b'A'
                        && input.as_bytes()[(y + 2) * 140 + x + 2] == b'M'
                        && input.as_bytes()[(y + 3) * 140 + x + 3] == b'X')
                {
                    count += 1;
                }
            }
        }

        for y in 0..(140 - 4) {
            for x in 3..140 {
                if (input.as_bytes()[y * 140 + x] == b'X'
                    && input.as_bytes()[(y + 1) * 140 + x - 1] == b'M'
                    && input.as_bytes()[(y + 2) * 140 + x - 2] == b'A'
                    && input.as_bytes()[(y + 3) * 140 + x - 3] == b'S')
                    || (input.as_bytes()[y * 140 + x] == b'S'
                        && input.as_bytes()[(y + 1) * 140 + x - 1] == b'A'
                        && input.as_bytes()[(y + 2) * 140 + x - 2] == b'M'
                        && input.as_bytes()[(y + 3) * 140 + x - 3] == b'X')
                {
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn part_two() -> i32 {
    let input = include_str!("../input.txt");

    todo!()
}
