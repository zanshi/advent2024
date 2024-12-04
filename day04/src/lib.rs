pub fn part_one() -> i32 {
    let input = include_str!("../input.txt");

    // word search

    // horizontal - left to right (right to left)
    // vertical - top down (down top)
    // diagonal - up right + down right (down left + up left)

    // backwards - reverse previous searches

    // overlapping

    // find all instances of XMAS

    let input = input.split('\n').collect::<String>();
    let input = input.as_bytes();

    let mut count = 0;

    // horizontal
    {
        for y in 0..140 {
            let line = &input[y * 140..(y * 140 + 140)];

            for window in line.windows(4) {
                if (window[0] == b'X'
                    && window[1] == b'M'
                    && window[2] == b'A'
                    && window[3] == b'S')
                    || (window[0] == b'S'
                        && window[1] == b'A'
                        && window[2] == b'M'
                        && window[3] == b'X')
                {
                    count += 1;
                }
            }
        }
    }

    // vertical
    {
        // up + down
        for y in 0..(140 - 3) {
            for x in 0..140 {
                if (input[y * 140 + x] == b'X'
                    && input[(y + 1) * 140 + x] == b'M'
                    && input[(y + 2) * 140 + x] == b'A'
                    && input[(y + 3) * 140 + x] == b'S')
                    || (input[y * 140 + x] == b'S'
                        && input[(y + 1) * 140 + x] == b'A'
                        && input[(y + 2) * 140 + x] == b'M'
                        && input[(y + 3) * 140 + x] == b'X')
                {
                    count += 1;
                }
            }
        }
    }

    // diagonal
    {
        for y in 0..(140 - 3) {
            for x in 0..(140 - 3) {
                if (input[y * 140 + x] == b'X'
                    && input[(y + 1) * 140 + x + 1] == b'M'
                    && input[(y + 2) * 140 + x + 2] == b'A'
                    && input[(y + 3) * 140 + x + 3] == b'S')
                    || (input[y * 140 + x] == b'S'
                        && input[(y + 1) * 140 + x + 1] == b'A'
                        && input[(y + 2) * 140 + x + 2] == b'M'
                        && input[(y + 3) * 140 + x + 3] == b'X')
                {
                    count += 1;
                }
            }
        }

        for y in 0..(140 - 3) {
            for x in 3..140 {
                if (input[y * 140 + x] == b'X'
                    && input[(y + 1) * 140 + x - 1] == b'M'
                    && input[(y + 2) * 140 + x - 2] == b'A'
                    && input[(y + 3) * 140 + x - 3] == b'S')
                    || (input[y * 140 + x] == b'S'
                        && input[(y + 1) * 140 + x - 1] == b'A'
                        && input[(y + 2) * 140 + x - 2] == b'M'
                        && input[(y + 3) * 140 + x - 3] == b'X')
                {
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn part_two() -> i32 {
    let input = include_str!("../input.txt").split('\n').collect::<String>();
    let input = input.as_bytes();

    let mut count = 0;

    for y in 0..(140 - 2) {
        for x in 0..(140 - 2) {
            if input[(y + 1) * 140 + x + 1] != b'A' {
                continue;
            }

            // M . M
            // . A .
            // S . S
            if input[y * 140 + x] == b'M'
                && input[(y + 1) * 140 + x + 1] == b'A'
                && input[(y + 2) * 140 + x + 2] == b'S'
                && input[y * 140 + x + 2] == b'M'
                && input[(y + 2) * 140 + x] == b'S'
            {
                count += 1;
            }

            // S . S
            // . A .
            // M . M
            if input[y * 140 + x] == b'S'
                && input[(y + 1) * 140 + x + 1] == b'A'
                && input[(y + 2) * 140 + x + 2] == b'M'
                && input[y * 140 + x + 2] == b'S'
                && input[(y + 2) * 140 + x] == b'M'
            {
                count += 1;
            }

            // M . S
            // . A .
            // M . S
            if input[y * 140 + x] == b'M'
                && input[(y + 1) * 140 + x + 1] == b'A'
                && input[(y + 2) * 140 + x + 2] == b'S'
                && input[y * 140 + x + 2] == b'S'
                && input[(y + 2) * 140 + x] == b'M'
            {
                count += 1;
            }

            // S . M
            // . A .
            // S . M
            if input[y * 140 + x] == b'S'
                && input[(y + 1) * 140 + x + 1] == b'A'
                && input[(y + 2) * 140 + x + 2] == b'M'
                && input[y * 140 + x + 2] == b'M'
                && input[(y + 2) * 140 + x] == b'S'
            {
                count += 1;
            }
        }
    }

    count
}
