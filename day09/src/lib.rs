pub fn part_one() -> i64 {
    // - read file and free space blocks
    // - defrag
    // - calculate checksum
    //

    let input = include_str!("../input.txt");
    let input = input.as_bytes();

    // 6604945869790 - wrong too low

    let mut filesystem: Vec<i32> = Vec::with_capacity(100000);

    let mut id = 0;
    for (i, digit) in input.iter().enumerate().map(|(i, digit)| (i, digit - 48)) {
        let entry = if i % 2 == 0 {
            let entry = id;
            id += 1;
            entry
        } else {
            255 // mark free space as 255
        };

        filesystem.resize(filesystem.len() + digit as usize, entry);
    }

    let mut free_space_cursor = filesystem.iter().position(|x| *x == 255).unwrap();
    let mut file_block_cursor =
        (filesystem.len() - 1) - filesystem.iter().rev().position(|x| *x != 255).unwrap();

    while free_space_cursor < file_block_cursor {
        if filesystem[file_block_cursor] != 255 {
            filesystem[free_space_cursor] = filesystem[file_block_cursor];
            filesystem[file_block_cursor] = 255;
        }

        while filesystem[free_space_cursor] != 255 && free_space_cursor < file_block_cursor {
            free_space_cursor += 1;
        }

        while filesystem[file_block_cursor] == 255 && free_space_cursor < file_block_cursor {
            file_block_cursor -= 1;
        }

        // println!("cursors: {}, {}", file_block_cursor, free_space_cursor);
        // println!();
        // for (i, entry) in filesystem.iter().enumerate() {
        //     if *entry == 255 {
        //         if i == free_space_cursor {
        //             print!("(.)");
        //         } else {
        //             print!(".");
        //         }
        //     } else {
        //         if i == file_block_cursor {
        //             print!("({})", entry);
        //         } else {
        //             print!("{}", entry);
        //         }
        //     }
        // }
        // println!();
    }

    let checksum = filesystem
        .iter()
        .enumerate()
        .filter(|(_, x)| **x != 255)
        .fold(0, |acc, (i, x)| acc + (i * (*x as usize)));

    checksum as i64
}

pub fn part_two() -> i64 {
    let input = include_str!("../input.txt");
    0
}
