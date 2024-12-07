use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use core::num;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "07"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"; // TODO: Add the test input

fn go(target: usize, curr: usize, numbers: &Vec<usize>, pos: usize) -> bool {
    let mut res = false;
    if pos == numbers.len() {
        return target == curr;
    }
    if curr + numbers[pos] <= target {
        res |= go(target, curr + numbers[pos], numbers, pos + 1);
    }
    if curr * numbers[pos] <= target {
        res |= go(target, curr * numbers[pos], numbers, pos + 1);
    }
    res
}

fn go2(target: usize, curr: usize, numbers: &Vec<usize>, pos: usize) -> bool {
    let mut res = false;
    if pos == numbers.len() {
        return target == curr;
    }
    if curr + numbers[pos] <= target {
        res |= go2(target, curr + numbers[pos], numbers, pos + 1);
    }
    if curr * numbers[pos] <= target {
        res |= go2(target, curr * numbers[pos], numbers, pos + 1);
    }
    let curr_len = curr.to_string().len();
    let target_len = target.to_string().len();
    let num_len = numbers[pos].to_string().len();
    if curr_len + num_len <= target_len {
        let combo: usize = (curr.to_string() + numbers[pos].to_string().as_str())
            .parse()
            .unwrap();
        if combo <= target {
            res |= go2(target, combo, numbers, pos + 1);
        }
    }
    res
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut answer = 0;

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split(':').collect();
            let target: usize = parts[0].trim().parse().unwrap();
            let numbers: Vec<usize> = parts[1]
                .trim()
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();

            if go(target, 0, &numbers, 0) {
                answer += target;
            }
        }

        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

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
            let parts: Vec<&str> = line.split(':').collect();
            let target: usize = parts[0].trim().parse().unwrap();
            let numbers: Vec<usize> = parts[1]
                .trim()
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();

            if go2(target, 0, &numbers, 0) {
                answer += target;
            }
        }

        Ok(answer)
    }

    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}
