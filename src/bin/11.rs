use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use core::num;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

const DAY: &str = "11"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "125 17"; // TODO: Add the test input

fn apply_rule(a: &Vec<i64>) -> Vec<i64> {
    let mut res = vec![];

    for x in a {
        if *x == 0 {
            res.push(1);
        } else if x.to_string().len() % 2 == 0 {
            let x_str = x.to_string();
            let (left, right) = x_str.split_at(x_str.len() / 2);
            res.push(i64::from_str(left).unwrap());
            res.push(i64::from_str(right).unwrap());
        } else {
            res.push(x * 2024);
        }
    }

    res
}

fn calc(x: i64, blink: i32, cache: &mut HashMap<(i64, i32), i64>) -> i64 {
    if blink == 0 {
        return 1;
    }

    if let Some(&res) = cache.get(&(x, blink)) {
        return res;
    }
    if x == 0 {
        let res = calc(1, blink - 1, cache);
        cache.insert((x, blink), res);
        res
    } else if x.to_string().len() % 2 == 0 {
        let x_str = x.to_string();
        let (left, right) = x_str.split_at(x_str.len() / 2);
        let res = calc(i64::from_str(left).unwrap(), blink - 1, cache)
            + calc(i64::from_str(right).unwrap(), blink - 1, cache);
        cache.insert((x, blink), res);
        res
    } else {
        let res = calc(x * 2024, blink - 1, cache);
        cache.insert((x, blink), res);
        res
    }
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
            let mut numbers: Vec<i64> = line
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect();

            for blink in 0..25 {
                numbers = apply_rule(&numbers);
            }

            answer = numbers.len();
        }

        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(55312, part1(BufReader::new(TEST.as_bytes()))?);

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
            let mut numbers: Vec<i64> = line
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect();

            let mut cache = HashMap::new();

            for x in numbers {
                answer += calc(x, 75, &mut cache);
            }
        }

        Ok(answer as usize)
    }

    // assert_eq!(81, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}
