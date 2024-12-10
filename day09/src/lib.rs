use std::arch::x86_64::{
    __m256i, _mm256_cmpeq_epi32, _mm256_load_si256, _mm256_movemask_ps, _mm256_set1_epi32,
};

pub fn part_one() -> i64 {
    let input = include_str!("../input.txt");
    let input = input.as_bytes();

    let mut fs = create_fs(input);

    naive_defrag(&mut fs);

    let checksum = calculate_checksum(&fs);

    checksum as i64
}

pub fn part_two() -> i64 {
    let input = include_str!("../input.txt");
    let input = input.as_bytes();

    let mut fs = create_fs(input);

    better_defrag(&mut fs);

    let checksum = calculate_checksum(&fs);

    checksum as i64
}

fn create_fs(input: &[u8]) -> Vec<i32> {
    let mut fs: Vec<i32> = Vec::with_capacity(10000000);

    let mut id = 0;
    for (i, digit) in input.iter().enumerate().map(|(i, digit)| (i, digit - 48)) {
        let entry = if i % 2 == 0 {
            let entry = id;
            id += 1;
            entry
        } else {
            i32::MAX // mark free space as i32::MAX
        };

        fs.resize(fs.len() + digit as usize, entry);
    }

    fs
}

fn naive_defrag(fs: &mut [i32]) {
    let mut free_space_cursor = fs.iter().position(|x| *x == i32::MAX).unwrap();
    let mut file_block_cursor =
        (fs.len() - 1) - fs.iter().rev().position(|x| *x != i32::MAX).unwrap();

    while free_space_cursor < file_block_cursor {
        if fs[file_block_cursor] != i32::MAX {
            fs.swap(file_block_cursor, free_space_cursor);
        }

        while fs[free_space_cursor] != i32::MAX && free_space_cursor < file_block_cursor {
            free_space_cursor += 1;
        }

        while fs[file_block_cursor] == i32::MAX && free_space_cursor < file_block_cursor {
            file_block_cursor -= 1;
        }
    }
}

#[target_feature(enable = "avx2")]
unsafe fn find_first_max_simd(slice: &[i32], mut cursor: usize, end: usize) -> usize {
    let max_vector = _mm256_set1_epi32(i32::MAX);

    while cursor + 8 <= end {
        let data = _mm256_load_si256(slice[cursor..].as_ptr() as *const __m256i);

        let res = _mm256_cmpeq_epi32(max_vector, data);

        let mask = _mm256_movemask_ps(std::mem::transmute::<
            std::arch::x86_64::__m256i,
            std::arch::x86_64::__m256,
        >(res));

        if mask == 0 {
            cursor += 8;
        } else {
            break;
        }
    }

    cursor
}

fn better_defrag(fs: &mut [i32]) {
    let mut free_space_cursor = fs.iter().position(|x| *x == i32::MAX).unwrap();
    let mut file_block_cursor =
        (fs.len() - 1) - fs.iter().rev().position(|x| *x != i32::MAX).unwrap();

    while free_space_cursor < file_block_cursor {
        let file_id = fs[file_block_cursor];

        let file_block_range = {
            let mut extent = 0;
            while free_space_cursor < (file_block_cursor - extent) {
                if fs[file_block_cursor - extent] != file_id {
                    break;
                } else {
                    extent += 1;
                }
            }

            (file_block_cursor - extent + 1)..file_block_cursor + 1
        };

        let mut find_free_space_cursor = free_space_cursor;

        while find_free_space_cursor < file_block_cursor {
            let free_space_block_range = {
                let mut free_space_extent = 0;

                while (find_free_space_cursor + free_space_extent) < file_block_cursor {
                    if fs[find_free_space_cursor + free_space_extent] != i32::MAX {
                        break;
                    } else {
                        free_space_extent += 1;
                    }
                }

                find_free_space_cursor..(find_free_space_cursor + free_space_extent)
            };

            if free_space_block_range.len() >= file_block_range.len() {
                // found valid block
                for (i, file_block_index) in file_block_range.clone().enumerate() {
                    fs.swap(free_space_block_range.start + i, file_block_index);
                }

                break;
            } else {
                find_free_space_cursor += free_space_block_range.len();

                // block too small, try to find another
                find_free_space_cursor =
                    unsafe { find_first_max_simd(fs, find_free_space_cursor, file_block_cursor) };

                while fs[find_free_space_cursor] != i32::MAX
                    && find_free_space_cursor < file_block_cursor
                {
                    find_free_space_cursor += 1;
                }
            }
        }

        file_block_cursor -= file_block_range.len();

        while fs[file_block_cursor] == i32::MAX && free_space_cursor < file_block_cursor {
            file_block_cursor -= 1;
        }

        while fs[free_space_cursor] != i32::MAX && free_space_cursor < file_block_cursor {
            free_space_cursor += 1;
        }
    }
}

fn calculate_checksum(fs: &[i32]) -> usize {
    fs.iter()
        .enumerate()
        .filter(|(_, x)| **x != i32::MAX)
        .fold(0, |acc, (i, x)| acc + (i * (*x as usize)))
}

// fn visualize(filesystem: &[i32]) {
//     println!();
//     for (i, entry) in filesystem.iter().enumerate() {
//         if *entry == i32::MAX {
//             print!(".");
//         } else {
//             print!("{}", entry);
//         }
//     }
//     println!();
// }
