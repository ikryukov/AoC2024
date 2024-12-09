use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::enumerate;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "09"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "2333133121414131402"; // TODO: Add the test input
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut answer = 0;

        for line in reader.lines() {
            let line = line?;
            let char_vec: Vec<char> = line.chars().collect();

            let mut file_id = 0;
            let mut mem: Vec<i32> = vec![];
            for (i, &c) in char_vec.iter().enumerate() {
                let count = c as i32 - '0' as i32;
                for _ in 0..count {
                    if i % 2 == 1 {
                        mem.push(-1);
                    } else {
                        mem.push(file_id);
                    }
                }
                if i % 2 == 0 {
                    file_id += 1;
                }
            }
            let mut l = 0;
            let mut r = mem.len() - 1;
            while r > l {
                if mem[l] != -1 {
                    l += 1;
                    continue;
                }
                if mem[r] == -1 {
                    r -= 1;
                    continue;
                }
                mem.swap(l, r);
                l += 1;
                r -= 1;
            }
            for i in 0..r + 1 {
                if mem[i] != -1 {
                    answer += i * (mem[i] as usize);
                }
            }
        }

        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;

        for line in reader.lines() {
            let line = line?;
            let char_vec: Vec<char> = line.chars().collect();

            let mut file_id = 0;
            let mut mem: Vec<i32> = vec![];
            let mut free_spaces: Vec<(usize, i32)> = vec![];
            let mut blocks: Vec<(usize, i32, i32)> = vec![];
            for (i, &c) in char_vec.iter().enumerate() {
                let count = c as i32 - '0' as i32;
                for _x in 0..count {
                    if i % 2 == 1 {
                        mem.push(-1);
                    } else {
                        mem.push(file_id);
                    }
                }
                if i % 2 == 0 {
                    blocks.push((mem.len() - count as usize, count, file_id));
                    file_id += 1;
                } else {
                    free_spaces.push((mem.len() - count as usize, count));
                }
            }
            let mut r = blocks.len() as i32 - 1;
            while r >= 0 {
                for (free_pos, free_size) in &mut free_spaces {
                    let (block_start, block_size, block_id) = blocks[r as usize];

                    if *free_pos >= block_start {
                        break;
                    }

                    if *free_size >= block_size {
                        for i in 0..block_size {
                            mem[*free_pos + i as usize] = block_id;
                            mem[block_start + i as usize] = -1;
                        }

                        *free_size -= block_size;
                        *free_pos += block_size as usize;
                        break;
                    }
                }
                r -= 1;
            }
            for i in 0..mem.len() {
                if mem[i] != -1 {
                    answer += i * (mem[i] as usize);
                }
            }
        }

        Ok(answer)
    }

    assert_eq!(2858, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}
