use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

const DAY: &str = "05"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut answer = 0;
        let mut is_rule = true;
        let mut hm: HashSet<(i32, i32)> = Default::default();

        for line in reader.lines() {
            let line = line?;

            if line.is_empty() {
                is_rule = false;
                continue;
            }
            if is_rule {
                let rule: Vec<i32> = line.split('|').map(|x| i32::from_str(x).unwrap()).collect();
                let a = rule[0];
                let b = rule[1];
                println!("{} {}", a, b);
                // a should be before b
                hm.insert((a, b));
            } else {
                let updates: Vec<i32> =
                    line.split(',').map(|x| i32::from_str(x).unwrap()).collect();
                println!("updates: {:?}", updates);
                let mut ok = true;
                for i in 1..updates.len() {
                    if ok {
                        let curr = updates[i];
                        for j in 0..i {
                            if hm.contains(&(curr, updates[j])) {
                                ok = false;
                                break;
                            }
                        }
                    } else {
                        break;
                    }
                }
                if ok {
                    answer += updates[updates.len() / 2] as usize;
                }
            }
        }
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        let mut is_rule = true;
        let mut hm: HashSet<(i32, i32)> = Default::default();

        for line in reader.lines() {
            let line = line?;

            if line.is_empty() {
                is_rule = false;
                continue;
            }
            if is_rule {
                let rule: Vec<i32> = line.split('|').map(|x| i32::from_str(x).unwrap()).collect();
                let a = rule[0];
                let b = rule[1];
                println!("{} {}", a, b);
                // a should be before b
                hm.insert((a, b));
            } else {
                let mut updates: Vec<i32> =
                    line.split(',').map(|x| i32::from_str(x).unwrap()).collect();
                println!("updates: {:?}", updates);
                let mut ok = true;
                for i in 1..updates.len() {
                    if ok {
                        let curr = updates[i];
                        for j in 0..i {
                            if hm.contains(&(curr, updates[j])) {
                                ok = false;
                                break;
                            }
                        }
                    } else {
                        break;
                    }
                }
                if !ok {
                    while !ok {
                        ok = true;
                        for i in 1..updates.len() {
                            if ok {
                                let curr = updates[i];
                                for j in 0..i {
                                    if hm.contains(&(curr, updates[j])) {
                                        updates.swap(i, j);
                                        ok = false;
                                        break;
                                    }
                                }
                            } else {
                                break;
                            }
                        }
                    }
                    answer += updates[updates.len() / 2] as usize;
                }
            }
        }
        Ok(answer)
    }

    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
