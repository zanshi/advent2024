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

    // vertical
    {
        // up + down
        for col in 0..140 {
            for row in 0..(140 - 4) {
                if (input.as_bytes()[col * 140 + row] == b'X'
                    && input.as_bytes()[col * 140 + row + 1] == b'M'
                    && input.as_bytes()[col * 140 + row + 2] == b'A'
                    && input.as_bytes()[col * 140 + row + 3] == b'S')
                    || (input.as_bytes()[col * 140 + row] == b'S'
                        && input.as_bytes()[col * 140 + row + 1] == b'A'
                        && input.as_bytes()[col * 140 + row + 2] == b'M'
                        && input.as_bytes()[col * 140 + row + 3] == b'X')
                {
                    count += 1;
                }
            }
        }
    }

    // diagonal
    {
        for col in 0..(140 - 4) {
            for row in 0..(140 - 4) {
                if (input.as_bytes()[col * 140 + row] == b'X'
                    && input.as_bytes()[(col + 1) * 140 + row + 1] == b'M'
                    && input.as_bytes()[(col + 2) * 140 + row + 2] == b'A'
                    && input.as_bytes()[(col + 3) * 140 + row + 3] == b'S')
                    || (input.as_bytes()[col * 140 + row] == b'S'
                        && input.as_bytes()[(col + 1) * 140 + row + 1] == b'A'
                        && input.as_bytes()[(col + 2) * 140 + row + 2] == b'M'
                        && input.as_bytes()[(col + 3) * 140 + row + 3] == b'X')
                {
                    count += 1;
                }
            }
        }

        for col in 4..140 {
            for row in 0..(140 - 4) {
                if (input.as_bytes()[col * 140 + row] == b'X'
                    && input.as_bytes()[(col - 1) * 140 + row + 1] == b'M'
                    && input.as_bytes()[(col - 2) * 140 + row + 2] == b'A'
                    && input.as_bytes()[(col - 3) * 140 + row + 3] == b'S')
                    || (input.as_bytes()[col * 140 + row] == b'S'
                        && input.as_bytes()[(col - 1) * 140 + row + 1] == b'A'
                        && input.as_bytes()[(col - 2) * 140 + row + 2] == b'M'
                        && input.as_bytes()[(col - 3) * 140 + row + 3] == b'X')
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
