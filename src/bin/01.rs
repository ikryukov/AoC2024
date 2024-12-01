use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut a: Vec<i64> = Vec::new();
        let mut b: Vec<i64> = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let numbers: Vec<i64> = line
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect();
            // Push the numbers to the respective vectors
            if let [first, second] = numbers[..] {
                a.push(first);
                b.push(second);
            }
        }
        a.sort();
        b.sort();

        let mut answer: usize = 0;
        for i in 0..a.len() {
            answer += (a[i] - b[i]).abs() as usize;
        }
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i64> {
        // TODO: Solve Part 1 of the puzzle
        let mut a: Vec<i64> = Vec::new();
        let mut b: Vec<i64> = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let numbers: Vec<i64> = line
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect();
            // Push the numbers to the respective vectors
            if let [first, second] = numbers[..] {
                a.push(first);
                b.push(second);
            }
        }
        let mut hm = HashMap::<i64, usize>::new();
        for i in b {
            *hm.entry(i).or_insert(0) += 1;
        }

        let mut answer: i64 = 0;
        for i in a {
            let zero: usize = 0;
            answer += i * *hm.get(&i).unwrap_or(&zero) as i64;
        }
        Ok(answer)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
