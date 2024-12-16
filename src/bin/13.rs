use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use core::num;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::{read, File};
use std::io::{BufRead, BufReader};
use std::str::FromStr;

const DAY: &str = "13"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"; // TODO: Add the test input

const TEST1: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=10000000008400, Y=10000000005400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=10000000012748, Y=10000000012176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=10000000007870, Y=10000000006450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=10000000018641, Y=10000000010279"; // TODO: Add the test input

fn extended_gcd(x: i64, y: i64) -> (i64, i64, i64) {
    if y == 0 {
        (x, 1, 0) // gcd, coefficient of x, coefficient of y
    } else {
        let (gcd, a, b) = extended_gcd(y, x % y);
        (gcd, b, a - (x / y) * b)
    }
}

fn solve_minimized(
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    px: i64,
    py: i64,
) -> Option<(i64, i64, i64)> {
    // Solve first equation using Extended Euclidean Algorithm
    let (gcd_x, a1, b1) = extended_gcd(ax, bx);

    // Check if px is divisible by gcd
    if px % gcd_x != 0 {
        return None; // No solution
    }

    // Scale initial solution for px
    let scale = px / gcd_x;
    let a_x = a1 * scale;
    let b_x = b1 * scale;

    // Solve second equation for ay, by
    let (gcd_y, a2, b2) = extended_gcd(ay, by);

    // Check if py is divisible by gcd
    if py % gcd_y != 0 {
        return None; // No solution
    }

    // Scale initial solution for py
    let scale = py / gcd_y;
    let a_y = a2 * scale;
    let b_y = b2 * scale;

    // Adjust coefficients to ensure both constraints are met

    let steps = 1 << 24;
    let dk = if ax < bx { -1 } else { 1 };

    for k in -10000000..=100000 {
        let a = a_x + k * (bx / gcd_x);
        let b = b_x - k * (ax / gcd_x);

        // Ensure it satisfies the second equation
        if a * ay + b * by == py {
            let objective = a * 3 + b;
            return Some((a, b, objective));
        }
        // k += dk;
    }

    None // No integer solution found
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut answer = 0;

        let mut line_idx = 0;
        let mut ax: i64 = 0;
        let mut ay: i64 = 0;
        let mut bx: i64 = 0;
        let mut by: i64 = 0;
        let mut px: i64 = 0;
        let mut py: i64 = 0;

        for line in reader.lines() {
            let line = line?;
            if line.is_empty() {
                continue;
            }
            let parts: Vec<&str> = line.split(|c| c == '+' || c == '=').collect();

            let tmp: Vec<&str> = parts[1].trim().split(',').collect();
            let x = tmp[0].parse::<i64>().unwrap();
            let y = parts[2].trim().parse::<i64>().unwrap();
            println!("x = {} y = {}", x, y);
            if line_idx % 3 == 0 {
                ax = x;
                ay = y;
            } else if line_idx % 3 == 1 {
                bx = x;
                by = y;
            } else {
                px = x;
                py = y;

                match solve_minimized(ax, ay, bx, by, px, py) {
                    Some((a, b, objective)) => {
                        println!("Solution found: a = {}, b = {}, Objective = {}", a, b, objective);
                        answer += objective as usize;
                    }
                    None => {
                        println!("No solution found.");
                    }
                }
            }
            line_idx += 1;
        }
        // a * 3 + b -> min
        // a * (ax, ay) + b * (bx, by) = (px, py)
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(480, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i64> {
        // TODO: Solve Part 1 of the puzzle
        let mut answer = 0;

        let mut line_idx = 0;
        let mut ax: i64 = 0;
        let mut ay: i64 = 0;
        let mut bx: i64 = 0;
        let mut by: i64 = 0;
        let mut px: i64 = 0;
        let mut py: i64 = 0;

        for line in reader.lines() {
            let line = line?;
            if line.is_empty() {
                continue;
            }
            let parts: Vec<&str> = line.split(|c| c == '+' || c == '=').collect();

            let tmp: Vec<&str> = parts[1].trim().split(',').collect();
            let x = tmp[0].parse::<i64>().unwrap();
            let y = parts[2].trim().parse::<i64>().unwrap();
            println!("x = {} y = {}", x, y);
            if line_idx % 3 == 0 {
                ax = x;
                ay = y;
            } else if line_idx % 3 == 1 {
                bx = x;
                by = y;
            } else {
                px = x + 10000000000000;
                py = y + 10000000000000;

                let d = ax * by - ay * bx;
                let z1: i64 = px * by - py * bx;
                let z2 = ax * py - px * ay;
                if z1 % d != 0 || z2 % d != 0 {
                }
                else {
                    answer += 3 * (z1 / d) + (z2 / d);
                }
            }
            line_idx += 1;
        }
        // a * 3 + b -> min
        // a * (ax, ay) + b * (bx, by) = (px, py)
        Ok(answer)
    }

    part2(BufReader::new(TEST.as_bytes()))?;

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}
