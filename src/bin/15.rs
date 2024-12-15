use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "15"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"; // TODO: Add the test input

const TEST1: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

fn can_move(grid: &Vec<Vec<char>>, box_pos_i: usize, box_pos_j: usize, di: i32) -> bool {
    let neighbor_pos = (box_pos_i as i32 + di) as usize;
    if grid[neighbor_pos][box_pos_j] == '.' && grid[neighbor_pos][box_pos_j + 1] == '.' {
        return true;
    }
    if grid[neighbor_pos][box_pos_j] == '[' {
        return can_move(grid, neighbor_pos, box_pos_j, di);
    }
    if grid[neighbor_pos][box_pos_j] == ']' {
        if grid[neighbor_pos][box_pos_j + 1] == '[' {
            return can_move(grid, neighbor_pos, box_pos_j - 1, di)
                && can_move(grid, neighbor_pos, box_pos_j + 1, di);
        } else if grid[neighbor_pos][box_pos_j + 1] == '.' {
            return can_move(grid, neighbor_pos, box_pos_j - 1, di);
        }
        return false;
    }
    if grid[neighbor_pos][box_pos_j] == '.' && grid[neighbor_pos][box_pos_j + 1] == '[' {
        return can_move(grid, neighbor_pos, box_pos_j + 1, di);
    }
    return false;
}

fn do_move(grid: &mut Vec<Vec<char>>, box_pos_i: usize, box_pos_j: usize, di: i32) {
    let neighbor_pos = (box_pos_i as i32 + di) as usize;
    // 1)
    if grid[neighbor_pos][box_pos_j] == '.' && grid[neighbor_pos][box_pos_j + 1] == '.' {
        grid[neighbor_pos][box_pos_j] = '[';
        grid[neighbor_pos][box_pos_j + 1] = ']';

        grid[box_pos_i][box_pos_j] = '.';
        grid[box_pos_i][box_pos_j + 1] = '.';
        return;
    }
    // 5)
    if grid[neighbor_pos][box_pos_j] == '[' {
        do_move(grid, neighbor_pos, box_pos_j, di);
        grid[neighbor_pos][box_pos_j] = '[';
        grid[neighbor_pos][box_pos_j + 1] = ']';

        grid[box_pos_i][box_pos_j] = '.';
        grid[box_pos_i][box_pos_j + 1] = '.';
        return;
    }
    if grid[neighbor_pos][box_pos_j] == ']' {
        if grid[neighbor_pos][box_pos_j + 1] == '[' {
            // 3)
            do_move(grid, neighbor_pos, box_pos_j + 1, di);
        }
        // 2
        do_move(grid, neighbor_pos, box_pos_j - 1, di);
        grid[neighbor_pos][box_pos_j] = '[';
        grid[neighbor_pos][box_pos_j + 1] = ']';

        grid[box_pos_i][box_pos_j] = '.';
        grid[box_pos_i][box_pos_j + 1] = '.';
        return;
    }
    // 4
    if grid[neighbor_pos][box_pos_j + 1] == '[' {
        do_move(grid, neighbor_pos, box_pos_j + 1, di);
        grid[neighbor_pos][box_pos_j] = '[';
        grid[neighbor_pos][box_pos_j + 1] = ']';

        grid[box_pos_i][box_pos_j] = '.';
        grid[box_pos_i][box_pos_j + 1] = '.';
        return;
    }
}

fn validate(grid: &Vec<Vec<char>>) -> bool {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '[' {
                if (grid[i][j + 1] != ']') {
                    return false;
                }
            }
        }
    }
    return true;
}
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut answer = 0;
        let mut grid: Vec<Vec<char>> = vec![];
        let mut n = 0;
        let mut m = 0;

        let mut robot_pos_i = 0;
        let mut robot_pos_j = 0;
        let mut is_grid_input = true;
        for line in reader.lines() {
            let line = line?;
            if line.is_empty() {
                is_grid_input = false;
                continue;
            }
            if is_grid_input {
                let char_vec: Vec<char> = line.chars().collect();
                m = char_vec.len();
                for i in 0..m {
                    if char_vec[i] == '@' {
                        robot_pos_j = i;
                        robot_pos_i = n;
                    }
                }
                grid.push(char_vec);
                n += 1;
            } else {
                let cmds_vec: Vec<char> = line.chars().collect();
                for cmd in cmds_vec {
                    // println!("{}", cmd);
                    match cmd {
                        '<' => {
                            if grid[robot_pos_i][robot_pos_j - 1] == '#' {
                                continue;
                            }
                            if grid[robot_pos_i][robot_pos_j - 1] == 'O' {
                                let mut box_pos = (robot_pos_i, robot_pos_j - 1);
                                while grid[box_pos.0][box_pos.1] == 'O' {
                                    box_pos.1 -= 1;
                                }
                                if grid[box_pos.0][box_pos.1] == '#' {
                                    // can't move box train
                                    continue;
                                }
                                // move
                                while box_pos.1 < robot_pos_j {
                                    grid[box_pos.0][box_pos.1] = grid[box_pos.0][box_pos.1 + 1];
                                    box_pos.1 += 1;
                                }
                                grid[robot_pos_i][robot_pos_j] = '.';
                            } else if grid[robot_pos_i][robot_pos_j - 1] == '.' {
                                grid[robot_pos_i][robot_pos_j - 1] = '@';
                                grid[robot_pos_i][robot_pos_j] = '.';
                            }
                            robot_pos_j -= 1;
                        }
                        '>' => {
                            if grid[robot_pos_i][robot_pos_j + 1] == '#' {
                                continue;
                            }
                            if grid[robot_pos_i][robot_pos_j + 1] == 'O' {
                                let mut box_pos = (robot_pos_i, robot_pos_j + 1);
                                while grid[box_pos.0][box_pos.1] == 'O' {
                                    box_pos.1 += 1;
                                }
                                if grid[box_pos.0][box_pos.1] == '#' {
                                    // can't move box train
                                    continue;
                                }
                                // move
                                while box_pos.1 > robot_pos_j {
                                    grid[box_pos.0][box_pos.1] = grid[box_pos.0][box_pos.1 - 1];
                                    box_pos.1 -= 1;
                                }
                                grid[robot_pos_i][robot_pos_j] = '.';
                            } else if grid[robot_pos_i][robot_pos_j + 1] == '.' {
                                grid[robot_pos_i][robot_pos_j + 1] = '@';
                                grid[robot_pos_i][robot_pos_j] = '.';
                            }
                            robot_pos_j += 1;
                        }
                        '^' => {
                            if grid[robot_pos_i - 1][robot_pos_j] == '#' {
                                continue;
                            }
                            if grid[robot_pos_i - 1][robot_pos_j] == '['
                                && can_move(&grid, robot_pos_i - 1, robot_pos_j, -1)
                            {
                                do_move(&mut grid, robot_pos_i - 1, robot_pos_j, -1);
                                grid[robot_pos_i][robot_pos_j] = '.';
                                grid[robot_pos_i - 1][robot_pos_j] = '@';
                            } else if grid[robot_pos_i - 1][robot_pos_j] == '.' {
                                grid[robot_pos_i - 1][robot_pos_j] = '@';
                                grid[robot_pos_i][robot_pos_j] = '.';
                            }
                            robot_pos_i -= 1;
                        }
                        'v' => {
                            if grid[robot_pos_i + 1][robot_pos_j] == '#' {
                                continue;
                            }
                            if grid[robot_pos_i + 1][robot_pos_j] == 'O' {
                                let mut box_pos = (robot_pos_i + 1, robot_pos_j);
                                while grid[box_pos.0][box_pos.1] == 'O' {
                                    box_pos.0 += 1;
                                }
                                if grid[box_pos.0][box_pos.1] == '#' {
                                    // can't move box train
                                    continue;
                                }
                                // move
                                while box_pos.0 > robot_pos_i {
                                    grid[box_pos.0][box_pos.1] = grid[box_pos.0 - 1][box_pos.1];
                                    box_pos.0 -= 1;
                                }
                                grid[robot_pos_i][robot_pos_j] = '.';
                            } else if grid[robot_pos_i + 1][robot_pos_j] == '.' {
                                grid[robot_pos_i + 1][robot_pos_j] = '@';
                                grid[robot_pos_i][robot_pos_j] = '.';
                            }
                            robot_pos_i += 1;
                        }
                        _ => {}
                    }

                    // println!("Debug: ");
                    // for i in 0..n
                    // {
                    //     println!("{:?}", grid[i]);
                    // }
                }
            }
        }

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 'O' {
                    answer += i * 100 + j;
                }
            }
        }

        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    // assert_eq!(2028, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut answer = 0;
        let mut grid: Vec<Vec<char>> = vec![];
        let mut n = 0;
        let mut m = 0;

        let mut robot_pos_i = 0;
        let mut robot_pos_j = 0;
        let mut is_grid_input = true;
        let mut cmd_index = 0;
        for line in reader.lines() {
            let line = line?;
            if line.is_empty() {
                is_grid_input = false;

                // println!("Grid:");
                // for row in &grid {
                //     println!("{:?}", row);
                // }
                continue;
            }
            if is_grid_input {
                let char_vec: Vec<char> = line.chars().collect();
                m = char_vec.len();
                let mut tmp: Vec<char> = vec![];
                for i in 0..m {
                    if char_vec[i] == '@' {
                        robot_pos_j = i * 2;
                        robot_pos_i = n;
                        tmp.push('@');
                        tmp.push('.');
                    } else if char_vec[i] == 'O' {
                        tmp.push('[');
                        tmp.push(']');
                    } else {
                        tmp.push(char_vec[i]);
                        tmp.push(char_vec[i]);
                    }
                }
                grid.push(tmp);
                n += 1;
                m *= 2;
            } else {
                let cmds_vec: Vec<char> = line.chars().collect();
                for cmd in cmds_vec {
                    println!("{}", cmd);
                    println!("Grid before:");
                    for row in &grid {
                        for c in row {
                            print!("{}", c);
                        }
                        println!("");
                    }
                    if robot_pos_i == 23 && robot_pos_j == 69 && cmd == 'v' && cmd_index == 12431 {
                        println!("ddd");
                    }
                    match cmd {
                        '<' => {
                            if grid[robot_pos_i][robot_pos_j - 1] == '#' {
                                continue;
                            }
                            if grid[robot_pos_i][robot_pos_j - 1] == ']' {
                                let mut box_pos = (robot_pos_i, robot_pos_j - 1);
                                while grid[box_pos.0][box_pos.1] != '.'
                                    && grid[box_pos.0][box_pos.1] != '#'
                                {
                                    box_pos.1 -= 1;
                                }
                                if grid[box_pos.0][box_pos.1] == '#' {
                                    // can't move box train
                                    continue;
                                }
                                // move
                                while box_pos.1 < robot_pos_j {
                                    grid[box_pos.0][box_pos.1] = grid[box_pos.0][box_pos.1 + 1];
                                    box_pos.1 += 1;
                                }
                                grid[robot_pos_i][robot_pos_j] = '.';
                                robot_pos_j -= 1;
                            } else if grid[robot_pos_i][robot_pos_j - 1] == '.' {
                                grid[robot_pos_i][robot_pos_j - 1] = '@';
                                grid[robot_pos_i][robot_pos_j] = '.';
                                robot_pos_j -= 1;
                            } else {
                                println!("ffff");
                            }
                        }
                        '>' => {
                            if grid[robot_pos_i][robot_pos_j + 1] == '#' {
                                continue;
                            }
                            if grid[robot_pos_i][robot_pos_j + 1] == '[' {
                                let mut box_pos = (robot_pos_i, robot_pos_j + 1);
                                while grid[box_pos.0][box_pos.1] != '.'
                                    && grid[box_pos.0][box_pos.1] != '#'
                                {
                                    box_pos.1 += 1;
                                }
                                if grid[box_pos.0][box_pos.1] == '#' {
                                    // can't move box train
                                    continue;
                                }
                                // move
                                while box_pos.1 > robot_pos_j {
                                    grid[box_pos.0][box_pos.1] = grid[box_pos.0][box_pos.1 - 1];
                                    box_pos.1 -= 1;
                                }
                                grid[robot_pos_i][robot_pos_j] = '.';
                                robot_pos_j += 1;
                            } else if grid[robot_pos_i][robot_pos_j + 1] == '.' {
                                grid[robot_pos_i][robot_pos_j + 1] = '@';
                                grid[robot_pos_i][robot_pos_j] = '.';
                                robot_pos_j += 1;
                            } else {
                                println!("aaaa");
                            }
                        }
                        '^' => {
                            let neighbor = (robot_pos_i - 1, robot_pos_j);
                            if grid[neighbor.0][neighbor.1] == '#' {
                                continue;
                            }
                            if grid[neighbor.0][neighbor.1] == '.' {
                                grid[robot_pos_i][robot_pos_j] = '.';
                                grid[neighbor.0][neighbor.1] = '@';
                                robot_pos_i -= 1;
                            }
                            if grid[neighbor.0][neighbor.1] == ']'
                                && can_move(&grid, neighbor.0, neighbor.1 - 1, -1)
                            {
                                do_move(&mut grid, neighbor.0, neighbor.1 - 1, -1);
                                grid[robot_pos_i][robot_pos_j] = '.';
                                grid[neighbor.0][neighbor.1] = '@';
                                robot_pos_i -= 1;
                            }
                            if grid[neighbor.0][neighbor.1] == '['
                                && can_move(&grid, neighbor.0, neighbor.1, -1)
                            {
                                do_move(&mut grid, neighbor.0, neighbor.1, -1);
                                grid[robot_pos_i][robot_pos_j] = '.';
                                grid[neighbor.0][neighbor.1] = '@';
                                robot_pos_i -= 1;
                            }
                        }
                        'v' => {
                            let neighbor = (robot_pos_i + 1, robot_pos_j);
                            if grid[neighbor.0][neighbor.1] == '#' {
                                continue;
                            }
                            if grid[neighbor.0][neighbor.1] == '.' {
                                grid[robot_pos_i][robot_pos_j] = '.';
                                grid[neighbor.0][neighbor.1] = '@';
                                robot_pos_i += 1;
                            }
                            if grid[neighbor.0][neighbor.1] == ']'
                                && can_move(&grid, neighbor.0, neighbor.1 - 1, 1)
                            {
                                do_move(&mut grid, neighbor.0, neighbor.1 - 1, 1);
                                grid[robot_pos_i][robot_pos_j] = '.';
                                grid[neighbor.0][neighbor.1] = '@';
                                robot_pos_i += 1;
                            }
                            if grid[neighbor.0][neighbor.1] == '['
                                && can_move(&grid, neighbor.0, neighbor.1, 1)
                            {
                                do_move(&mut grid, neighbor.0, neighbor.1, 1);
                                grid[robot_pos_i][robot_pos_j] = '.';
                                grid[neighbor.0][neighbor.1] = '@';
                                robot_pos_i += 1;
                            }
                        }
                        _ => {}
                    }
                    if !validate(&grid) {
                        println!("cmd index: {}", cmd_index);
                    }
                    cmd_index += 1;

                    // for row in &grid {
                    //     for c in row {
                    //         print!("{}", c);
                    //     }
                    //     println!("");
                    // }
                }
            }
        }
        for row in &grid {
            for c in row {
                print!("{}", c);
            }
            println!("");
        }
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == '[' {
                    answer += i * 100 + j;
                }
            }
        }

        Ok(answer)
    }

    let res = part2(BufReader::new(TEST1.as_bytes()))?;
    println!("res = {}", res);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}
