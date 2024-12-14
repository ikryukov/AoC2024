use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use core::num;
use ndarray::{array, Array2};
use ndarray_linalg::Solve;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::{read, File};
use std::io::{BufRead, BufReader};
use std::str::FromStr;

const DAY: &str = "14"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut answer = 0;

        let mut u_l = 0;
        let mut u_r = 0;
        let mut b_l = 0;
        let mut b_r = 0;

        for line in reader.lines() {
            let line = line?;
            let re = Regex::new(r"p=([-\d]+),([-\d]+) v=([-\d]+),([-\d]+)").unwrap();

            if let Some(captures) = re.captures(&line) {
                // Extract values for p and v
                let px: i32 = captures[1].parse().unwrap();
                let py: i32 = captures[2].parse().unwrap();
                let vx: i32 = captures[3].parse().unwrap();
                let vy: i32 = captures[4].parse().unwrap();

                // Print the extracted values
                // println!("Position: ({}, {}), Velocity: ({}, {})", px, py, vx, vy);

                let width = 101;
                let height = 103;

                for step in 0..100000 {}

                let mut pos_x = (px + 100 * vx) % width;
                if pos_x < 0 {
                    pos_x = width + pos_x;
                }
                let mut pos_y = (py + 100 * vy) % height;
                if pos_y < 0 {
                    pos_y = height + pos_y;
                }

                if pos_x < width / 2 {
                    if pos_y < height / 2 {
                        // upper left
                        u_l += 1;
                    } else if pos_y > height / 2 {
                        // bottom left
                        b_l += 1;
                    }
                } else if pos_x > width / 2 {
                    if pos_y < height / 2 {
                        // upper right
                        u_r += 1;
                    } else if pos_y > height / 2 {
                        // bottom right
                        b_r += 1;
                    }
                }
            }
        }
        answer = u_l * u_r * b_l * b_r;
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    // assert_eq!(12, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut answer = 0;

        let mut u_l = 0;
        let mut u_r = 0;
        let mut b_l = 0;
        let mut b_r = 0;

        let mut robots_pos: Vec<(i32, i32)> = vec![];
        let mut robots_vel: Vec<(i32, i32)> = vec![];

        for line in reader.lines() {
            let line = line?;
            let re = Regex::new(r"p=([-\d]+),([-\d]+) v=([-\d]+),([-\d]+)").unwrap();

            if let Some(captures) = re.captures(&line) {
                // Extract values for p and v
                let px: i32 = captures[1].parse().unwrap();
                let py: i32 = captures[2].parse().unwrap();
                let vx: i32 = captures[3].parse().unwrap();
                let vy: i32 = captures[4].parse().unwrap();

                // Print the extracted values
                // println!("Position: ({}, {}), Velocity: ({}, {})", px, py, vx, vy);
                robots_pos.push((px, py));
                robots_vel.push((vx, vy));
            }
        }

        let width = 101;
        let height = 103;
        let mut best = 0;
        for step in 0..7859 {
            for (i, robot_pos) in robots_pos.iter_mut().enumerate() {
                let robot_vel = robots_vel[i];
                let mut pos_x = (robot_pos.0 + robot_vel.0) % width;
                if pos_x < 0 {
                    pos_x = width + pos_x;
                }
                let mut pos_y = (robot_pos.1 + robot_vel.1) % height;
                if pos_y < 0 {
                    pos_y = height + pos_y;
                }

                robot_pos.0 = pos_x;
                robot_pos.1 = pos_y;
            }

            for axis in 2..width {
                let mut left_part: HashSet<(i32, i32)> = HashSet::new();
                let mut right_part: HashSet<(i32, i32)> = HashSet::new();

                for (pos_x, pos_y) in &robots_pos {
                    if *pos_x < axis {
                        left_part.insert((*pos_x, *pos_y));
                    } else if *pos_x > axis {
                        right_part.insert((*pos_x, *pos_y));
                    }
                }

                let mut count = 0;
                for r in &left_part {
                    let reflected = (axis + (axis - r.0), r.1);
                    if right_part.contains(&reflected) {
                        count += 1;
                    }
                }
                if count > best {
                    best = count;
                    println!(
                        "Found best = {} on iter = {} axis = {}",
                        best,
                        step + 1,
                        axis
                    );
                    break;
                }
            }
        }

        Ok(answer)
    }

    // part2(BufReader::new(TEST.as_bytes()))?;

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}
