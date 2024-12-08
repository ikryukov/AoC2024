use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "08"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........
"; // TODO: Add the test input
const TEST1: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"; // TODO: Add the test input
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut answer = 0;
        let mut grid = Vec::new();

        let mut n = 0;
        let mut m = 0;
        let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

        for line in reader.lines() {
            let line = line?;
            let char_vec: Vec<char> = line.chars().collect();
            m = char_vec.len() as i32;

            for (i, &c) in char_vec.iter().enumerate() {
                if c != '.' {
                    antennas.entry(c).or_default().push((n, i as i32));
                }
            }

            grid.push(char_vec);
            n += 1;
        }
        let mut hs: HashSet<(i32, i32)> = HashSet::new();
        for (antenna_type, coordinates) in antennas {
            println!("Antenna Type: {}", antenna_type);

            for (i, &coord1) in coordinates.iter().enumerate() {
                for coord2 in coordinates.iter().skip(i + 1) {
                    let reflected1 = (
                        coord1.0 - (coord2.0 - coord1.0),
                        coord1.1 - (coord2.1 - coord1.1),
                    );
                    let reflected2 = (
                        coord2.0 + (coord2.0 - coord1.0),
                        coord2.1 + (coord2.1 - coord1.1),
                    );
                    println!(
                        "Pair: {:?} <-> {:?} : reflected: {:?} {:?}",
                        coord1, coord2, reflected1, reflected2
                    );
                    let check = |point: (i32, i32)| -> bool {
                        point.0 >= 0 && point.1 >= 0 && point.0 < n && point.1 < m
                    };
                    if check(reflected1) {
                        hs.insert(reflected1);
                    }
                    if check(reflected2) {
                        hs.insert(reflected2);
                    }
                }
            }
        }
        answer = hs.len();
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    // assert_eq!(14, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        let mut grid = Vec::new();

        let mut n = 0;
        let mut m = 0;
        let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

        for line in reader.lines() {
            let line = line?;
            let char_vec: Vec<char> = line.chars().collect();
            m = char_vec.len() as i32;

            for (i, &c) in char_vec.iter().enumerate() {
                if c != '.' {
                    antennas.entry(c).or_default().push((n, i as i32));
                    answer += 1;
                }
            }

            grid.push(char_vec);
            n += 1;
        }
        let mut hs: HashSet<(i32, i32)> = HashSet::new();
        for (antenna_type, coordinates) in antennas {
            println!("Antenna Type: {}", antenna_type);

            for (i, &coord1) in coordinates.iter().enumerate() {
                for coord2 in coordinates.iter().skip(i + 1) {
                    let check = |point: (i32, i32)| -> bool {
                        point.0 >= 0 && point.1 >= 0 && point.0 < n && point.1 < m
                    };

                    let mut reflected1 = (
                        coord1.0 - (coord2.0 - coord1.0),
                        coord1.1 - (coord2.1 - coord1.1),
                    );
                    let mut idx = 1;
                    while check(reflected1) {
                        if grid[reflected1.0 as usize][reflected1.1 as usize] == '.' {
                            hs.insert(reflected1);
                        }
                        idx += 1;
                        reflected1 = (
                            coord1.0 - idx * (coord2.0 - coord1.0),
                            coord1.1 - idx * (coord2.1 - coord1.1),
                        );
                    }
                    let mut reflected2 = (
                        coord2.0 + (coord2.0 - coord1.0),
                        coord2.1 + (coord2.1 - coord1.1),
                    );
                    idx = 1;
                    while check(reflected2) {
                        if grid[reflected2.0 as usize][reflected2.1 as usize] == '.' {
                            hs.insert(reflected2);
                        }
                        idx += 1;
                        reflected2 = (
                            coord2.0 + idx * (coord2.0 - coord1.0),
                            coord2.1 + idx * (coord2.1 - coord1.1),
                        );
                    }
                }
            }
        }
        answer += hs.len();
        Ok(answer)
    }

    assert_eq!(34, part2(BufReader::new(TEST1.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}
