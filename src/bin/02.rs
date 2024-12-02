use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
1 3 2 4 5
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
8 6 4 4 1
1 3 6 7 9
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut answer = 0;
        for line in reader.lines() {
            let line = line?;
            let numbers: Vec<i64> = line
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect();
            let mut bad = false;
            let mut prev_diff = 0;
            for i in 1..numbers.len() {
                let diff = numbers[i] - numbers[i - 1];
                if (diff.abs() < 1 || diff.abs() > 3) || (diff * prev_diff < 0) {
                    bad = true;
                    break;
                }
                prev_diff = diff;
            }
            if bad == false {
                answer += 1;
            }
        }
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

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
            let numbers: Vec<i64> = line
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect();
            let mut bad = false;
            let mut prev_diff = 0;
            for i in 1..numbers.len() {
                let diff = numbers[i] - numbers[i - 1];
                if (diff.abs() < 1 || diff.abs() > 3) || (diff * prev_diff < 0) {
                    bad = true;
                    break;
                }
                prev_diff = diff;
            }
            if bad {
                for skip in 0..numbers.len() {
                    bad = false;
                    let mut prev_diff = 0;
                    let start = if skip == 0 { 2 } else { 1 };
                    for i in start..numbers.len() {
                        if i == skip {
                            continue;
                        }
                        let prev = if i - 1 == skip { i - 2 } else { i - 1 };
                        let diff = numbers[i] - numbers[prev];
                        if (diff.abs() < 1 || diff.abs() > 3) || (diff * prev_diff < 0) {
                            bad = true;
                            break;
                        }
                        prev_diff = diff;
                    }
                    if !bad {
                        break;
                    }
                }
            }
            if !bad {
                answer += 1;
            }
        }
        Ok(answer)
    }

    let res = part2(BufReader::new(TEST.as_bytes()))?;

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
