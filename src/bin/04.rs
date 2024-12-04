use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut answer = 0;
        let mut grid: Vec<Vec<char>> = vec![];

        for line in reader.lines() {
            let line = line?;
            let mut tmp: Vec<char> = vec![];
            for c in line.chars() {
                tmp.push(c);
            }
            grid.push(tmp);
        }
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 'X' {
                    for dir in 0..8 {
                        let mut ok = true;
                        for (idx, b) in "XMAS".chars().enumerate() {
                            let dx: Vec<i32> = vec![-1, -1, -1, 0, 1, 1, 1, 0];
                            let dy: Vec<i32> = vec![-1, 0, 1, 1, 1, 0, -1, -1];

                            let nx: i32 = i as i32 + dx[dir] * idx as i32;
                            let ny: i32 = j as i32 + dy[dir] * idx as i32;

                            if nx >= 0
                                && ny >= 0
                                && nx < grid.len() as i32
                                && ny < grid[0].len() as i32
                                && grid[nx as usize][ny as usize] == b
                            {
                            } else {
                                ok = false;
                                break;
                            }
                        }
                        if ok {
                            answer += 1;
                        }
                    }
                }
            }
        }
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        let mut grid: Vec<Vec<char>> = vec![];

        for line in reader.lines() {
            let line = line?;
            let mut tmp: Vec<char> = vec![];
            for c in line.chars() {
                tmp.push(c);
            }
            grid.push(tmp);
        }
        for i in 1..grid.len() - 1 {
            for j in 1..grid[i].len() - 1 {
                if grid[i][j] == 'A'
                    && (grid[i - 1][j - 1] == 'M' && grid[i + 1][j + 1] == 'S'
                        || grid[i - 1][j - 1] == 'S' && grid[i + 1][j + 1] == 'M')
                    && (grid[i - 1][j + 1] == 'M' && grid[i + 1][j - 1] == 'S'
                        || grid[i - 1][j + 1] == 'S' && grid[i + 1][j - 1] == 'M')
                {
                    answer += 1;
                }
            }
        }
        Ok(answer)
    }

    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
