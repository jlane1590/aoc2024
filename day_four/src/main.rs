use std::fs::File;
use std::io::{self, BufRead};
// use std::path::Path;

fn load_file_to_string_vector(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}

fn main() -> io::Result<()> {
    let grid = load_file_to_string_vector("big.txt")?;
    let word = "XMAS";
    let word_len = word.len();
    let first_letter = word.chars().next().unwrap();
    let mut find_cnt: usize = 0;

    for (row_idx, line) in grid.iter().enumerate() {
        for (col_idx, ch) in line.chars().enumerate() {
            if ch != first_letter {
                continue;
            }
            for row_dir in -1..=1 {
                let end_row: i32 = row_idx as i32 + (word_len as i32 - 1) * row_dir;
                if end_row < 0 || end_row >= grid.len() as i32 {
                    continue;
                }
                for col_dir in -1..=1 {
                    let end_col: i32 = col_idx as i32 + (word_len as i32 - 1) * col_dir;
                    if end_col < 0 || end_col >= line.len() as i32 {
                        continue;
                    }
                    let mut found = true;
                    for (w_idx, w_ch) in word.chars().enumerate() {
                        let r_idx = row_idx as i32 + w_idx as i32 * row_dir;
                        let c_idx = col_idx as i32 + w_idx as i32 * col_dir;
                        if grid[r_idx as usize].chars().nth(c_idx as usize).unwrap() != w_ch {
                            found = false;
                            break;
                        }
                    }
                    if found {
                        find_cnt += 1;
                        // println!("Found {} at ({}, {}) in direction ({}, {})", word, row_idx, col_idx, row_dir, col_dir);
                    }
                }
            }
        }
    }
    println!("{:?}", find_cnt);

    find_cnt = 0;
    for (row_idx, line) in grid.iter().enumerate() {
        if row_idx <= 0 || row_idx >= grid.len()-1 {
            continue;
        }
        for (col_idx, ch) in line.chars().enumerate() {
            if col_idx <= 0 || col_idx >= line.len()-1 {
                continue;
            }
            if ch != 'A' {
                continue;
            }
            if ((grid[row_idx-1].chars().nth(col_idx-1).unwrap() == 'M' &&
                 grid[row_idx+1].chars().nth(col_idx+1).unwrap() == 'S') ||
                (grid[row_idx-1].chars().nth(col_idx-1).unwrap() == 'S' &&
                 grid[row_idx+1].chars().nth(col_idx+1).unwrap() == 'M')) &&
               ((grid[row_idx-1].chars().nth(col_idx+1).unwrap() == 'M' &&
                 grid[row_idx+1].chars().nth(col_idx-1).unwrap() == 'S') ||
                (grid[row_idx-1].chars().nth(col_idx+1).unwrap() == 'S' &&
                 grid[row_idx+1].chars().nth(col_idx-1).unwrap() == 'M')) {
                    find_cnt += 1;
               }
        }
    }
    println!("{:?}", find_cnt);

    Ok(())
}
