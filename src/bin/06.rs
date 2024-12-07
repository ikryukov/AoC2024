use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Cycle;
use std::str::FromStr;

const DAY: &str = "06"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut answer = 1;
        let mut grid = Vec::new();

        let mut n = 0;
        let mut m = 0;
        let mut pos: (i32, i32) = (0, 0);

        for line in reader.lines() {
            let line = line?;
            let char_vec: Vec<char> = line.chars().collect();
            m = char_vec.len() as i32;
            let mut i = 0;
            for c in &char_vec {
                if *c == '^' {
                    pos.0 = n;
                    pos.1 = i;
                }
                i += 1;
            }
            grid.push(char_vec);
            n += 1;
        }

        let dx = vec![-1, 0, 1, 0];
        let dy = vec![0, 1, 0, -1];
        let mut dir = 0;
        grid[pos.0 as usize][pos.1 as usize] = 'X';
        loop {
            // do step
            let nx = pos.0 + dx[dir];
            let ny = pos.1 + dy[dir];
            if nx < 0 || ny < 0 || nx >= n || ny >= m {
                break;
            }
            if grid[nx as usize][ny as usize] == '#' {
                dir = (dir + 1) % 4;
                continue;
            }
            if grid[nx as usize][ny as usize] == '.' {
                answer += 1;
                grid[nx as usize][ny as usize] = 'X';
            }
            pos.0 = nx;
            pos.1 = ny;
        }

        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        let mut grid = Vec::new();

        let mut n: usize = 0;
        let mut m: usize = 0;
        let mut guard_pos: (i32, i32) = (0, 0);

        for line in reader.lines() {
            let line = line?;
            let char_vec: Vec<char> = line.chars().collect();
            m = char_vec.len();
            let mut i = 0;
            for c in &char_vec {
                if *c == '^' {
                    guard_pos.0 = n as i32;
                    guard_pos.1 = i;
                }
                i += 1;
            }
            grid.push(char_vec);
            n += 1;
        }

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == '.' {
                    grid[i][j] = '#';
                    // check
                    let dx = vec![-1, 0, 1, 0];
                    let dy = vec![0, 1, 0, -1];
                    let mut was: HashSet<((i32, i32), i32)> = Default::default();
                    let mut dir = 0;
                    let mut cycle = false;
                    let mut pos = guard_pos;
                    loop {
                        if was.contains(&(pos, dir)) {
                            cycle = true;
                            break;
                        }
                        was.insert((pos, dir));
                        // do step
                        let nx = pos.0 + dx[dir as usize];
                        let ny = pos.1 + dy[dir as usize];
                        if nx < 0 || ny < 0 || nx >= n as i32 || ny >= m as i32 {
                            break;
                        }
                        if grid[nx as usize][ny as usize] == '#' {
                            dir = (dir + 1) % 4;
                            continue;
                        }
                        pos.0 = nx;
                        pos.1 = ny;
                    }
                    if cycle {
                        answer += 1;
                    }
                    grid[i][j] = '.';
                }
            }
        }

        Ok(answer)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}
