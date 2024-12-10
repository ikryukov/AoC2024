use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::enumerate;
use std::collections::btree_map::Range;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "10"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"; // TODO: Add the test input
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut answer = 0;
        let mut grid: Vec<Vec<char>> = vec![];
        let mut n = 0;
        let mut m = 0;
        for line in reader.lines() {
            let line = line?;
            let char_vec: Vec<char> = line.chars().collect();
            m = char_vec.len();
            grid.push(char_vec);
            n += 1;
        }

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == '0' {
                    let mut score = 0;
                    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
                    let mut was: HashSet<(usize, usize)> = HashSet::new();

                    q.push_back((i, j));
                    was.insert((i, j));

                    while !q.is_empty() {
                        let curr = q.pop_front().unwrap();

                        if grid[curr.0][curr.1] == '9' {
                            score += 1;
                            continue;
                        }

                        let dx = [0, 1, 0, -1];
                        let dy = [1, 0, -1, 0];
                        for dir in 0..4 {
                            let nx: i32 = curr.0 as i32 + dx[dir];
                            let ny: i32 = curr.1 as i32 + dy[dir];
                            if nx >= 0
                                && ny >= 0
                                && nx < n as i32
                                && ny < m as i32
                                && grid[nx as usize][ny as usize]
                                    == (grid[curr.0][curr.1] as u8 + 1) as char
                                && !was.contains(&(nx as usize, ny as usize))
                            {
                                was.insert((nx as usize, ny as usize));
                                q.push_back((nx as usize, ny as usize));
                            }
                        }
                    }
                    // println!("pos ({}, {}) score = {}", i, j, score);
                    answer += score;
                }
            }
        }

        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(36, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        let mut grid: Vec<Vec<char>> = vec![];
        let mut n = 0;
        let mut m = 0;
        for line in reader.lines() {
            let line = line?;
            let char_vec: Vec<char> = line.chars().collect();
            m = char_vec.len();
            grid.push(char_vec);
            n += 1;
        }

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == '0' {
                    let mut score = 0;
                    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
                    let mut was: HashSet<(usize, usize)> = HashSet::new();

                    q.push_back((i, j));
                    was.insert((i, j));

                    while !q.is_empty() {
                        let curr = q.pop_front().unwrap();

                        if grid[curr.0][curr.1] == '9' {
                            score += 1;
                            continue;
                        }

                        let dx = [0, 1, 0, -1];
                        let dy = [1, 0, -1, 0];
                        for dir in 0..4 {
                            let nx: i32 = curr.0 as i32 + dx[dir];
                            let ny: i32 = curr.1 as i32 + dy[dir];
                            if nx >= 0
                                && ny >= 0
                                && nx < n as i32
                                && ny < m as i32
                                && grid[nx as usize][ny as usize]
                                    == (grid[curr.0][curr.1] as u8 + 1) as char
                                && !was.contains(&(nx as usize, ny as usize))
                            {
                                // was.insert((nx as usize, ny as usize));
                                q.push_back((nx as usize, ny as usize));
                            }
                        }
                    }
                    // println!("pos ({}, {}) score = {}", i, j, score);
                    answer += score;
                }
            }
        }

        Ok(answer)
    }

    assert_eq!(81, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}
