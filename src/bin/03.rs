use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
don't()do()xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))\
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
            let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

            for cap in re.captures_iter(line.as_str()) {
                let num1: usize = cap[1].parse().unwrap();
                let num2: usize = cap[2].parse().unwrap();
                answer += num1 * num2;
                // println!("Found: mul({}, {})", num1, num2);
            }
        }
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    // //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut enable_mul = true;
        let mut answer = 0;
        for line in reader.lines() {
            let line = line?;
            let re = Regex::new(r"(do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\))").unwrap();
            for cap in re.captures_iter(line.as_str()) {
                match &cap[1] {
                    "do()" => enable_mul = true,
                    "don't()" => enable_mul = false,
                    _ if enable_mul => {
                        println!("{} {}", &cap[2], &cap[3]);
                        let num1: usize = cap[2].parse().unwrap();
                        let num2: usize = cap[3].parse().unwrap();
                        answer += num1 * num2;
                    }
                    _ => {}
                }
            }
        }
        Ok(answer)
    }

    assert_eq!(48, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // //endregion

    Ok(())
}
