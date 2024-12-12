use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use core::num;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

const DAY: &str = "12"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut answer = 0;
        let mut grid: Vec<Vec<char>> = vec![];
        let mut was: Vec<Vec<bool>> = vec![];
        let mut n = 0;
        let mut m = 0;
        for line in reader.lines() {
            let line = line?;
            let char_vec: Vec<char> = line.chars().collect();
            m = char_vec.len();
            grid.push(char_vec);
            was.push(vec![false; m]);
            n += 1;
        }

        for i in 0..n {
            for j in 0..m {
                let curr = grid[i][j];
                if curr != '.' && !was[i][j] {
                    let mut q = VecDeque::<(usize, usize)>::new();

                    let mut per = 0;
                    let mut sq = 0;

                    q.push_back((i, j));
                    was[i][j] = true;

                    while !q.is_empty() {
                        let pos = q.pop_front().unwrap();
                        sq += 1;

                        if pos.0 == 0 || grid[pos.0 - 1][pos.1] != curr {
                            per += 1;
                        } else if !was[pos.0 - 1][pos.1] {
                            was[pos.0 - 1][pos.1] = true;
                            q.push_back((pos.0 - 1, pos.1));
                        }
                        if pos.1 == 0 || grid[pos.0][pos.1 - 1] != curr {
                            per += 1;
                        } else if !was[pos.0][pos.1 - 1] {
                            was[pos.0][pos.1 - 1] = true;
                            q.push_back((pos.0, pos.1 - 1));
                        }
                        if pos.0 + 1 == n || grid[pos.0 + 1][pos.1] != curr {
                            per += 1;
                        } else if !was[pos.0 + 1][pos.1] {
                            was[pos.0 + 1][pos.1] = true;
                            q.push_back((pos.0 + 1, pos.1));
                        }
                        if pos.1 + 1 == m || grid[pos.0][pos.1 + 1] != curr {
                            per += 1;
                        } else if !was[pos.0][pos.1 + 1] {
                            was[pos.0][pos.1 + 1] = true;
                            q.push_back((pos.0, pos.1 + 1));
                        }
                    }
                    answer += sq * per;
                }
            }
        }

        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(1930, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        let mut grid: Vec<Vec<char>> = vec![];
        let mut was: Vec<Vec<bool>> = vec![];
        let mut n = 0;
        let mut m = 0;
        for line in reader.lines() {
            let line: String = ".".to_owned() + &line? + ".";
            let char_vec: Vec<char> = line.chars().collect();
            m = char_vec.len();
            if n == 0 {
                grid.push(vec!['.'; m]);
                was.push(vec![false; m]);
            }
            grid.push(char_vec);
            was.push(vec![false; m]);
            n += 1;
        }
        grid.push(vec!['.'; m]);
        was.push(vec![false; m]);
        n += 1;

        for i in 0..n {
            for j in 0..m {
                let curr = grid[i][j];
                if curr != '.' && !was[i][j] {
                    let mut q = VecDeque::<(usize, usize)>::new();

                    let mut per = 0;
                    let mut sq = 0;

                    q.push_back((i, j));
                    was[i][j] = true;

                    let check_corner = |i: usize, j: usize| -> i32 {
                        let mut corner_count = 4;

                        if (grid[i - 1][j - 1] != grid[i][j] && grid[i][j - 1] == grid[i][j])
                            || grid[i - 1][j] == grid[i][j]
                        {
                            corner_count -= 1;
                        }
                        if (grid[i + 1][j + 1] != grid[i][j] && grid[i][j + 1] == grid[i][j])
                            || grid[i + 1][j] == grid[i][j]
                        {
                            corner_count -= 1;
                        }
                        if (grid[i - 1][j + 1] != grid[i][j] && grid[i - 1][j] == grid[i][j])
                            || grid[i][j + 1] == grid[i][j]
                        {
                            corner_count -= 1;
                        }
                        if (grid[i + 1][j - 1] != grid[i][j] && grid[i + 1][j] == grid[i][j])
                            || grid[i][j - 1] == grid[i][j]
                        {
                            corner_count -= 1;
                        }

                        corner_count
                    };
                    let mut corners = 0;
                    while !q.is_empty() {
                        let pos = q.pop_front().unwrap();
                        sq += 1;
                        corners += check_corner(pos.0, pos.1);

                        if pos.0 == 0 || grid[pos.0 - 1][pos.1] != curr {
                            per += 1;
                        } else if !was[pos.0 - 1][pos.1] {
                            was[pos.0 - 1][pos.1] = true;
                            q.push_back((pos.0 - 1, pos.1));
                        }
                        if pos.1 == 0 || grid[pos.0][pos.1 - 1] != curr {
                            per += 1;
                        } else if !was[pos.0][pos.1 - 1] {
                            was[pos.0][pos.1 - 1] = true;
                            q.push_back((pos.0, pos.1 - 1));
                        }
                        if pos.0 + 1 == n || grid[pos.0 + 1][pos.1] != curr {
                            per += 1;
                        } else if !was[pos.0 + 1][pos.1] {
                            was[pos.0 + 1][pos.1] = true;
                            q.push_back((pos.0 + 1, pos.1));
                        }
                        if pos.1 + 1 == m || grid[pos.0][pos.1 + 1] != curr {
                            per += 1;
                        } else if !was[pos.0][pos.1 + 1] {
                            was[pos.0][pos.1 + 1] = true;
                            q.push_back((pos.0, pos.1 + 1));
                        }
                    }
                    println!("{} : {}", grid[i][j], corners);
                    answer += sq * corners as usize;
                }
            }
        }

        Ok(answer)
    }

    assert_eq!(1206, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}
