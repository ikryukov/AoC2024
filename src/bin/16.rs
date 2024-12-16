use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "16"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut answer = 0;
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
        let mut dist: Vec<Vec<Vec<usize>>> = vec![];

        let mut n = 0;
        let mut m = 0;

        let mut start_pos = (0, 0);
        let mut end_pos = (0, 0);

        for line in reader.lines() {
            let line = line?;
            let char_vec: Vec<char> = line.chars().collect();
            m = char_vec.len();
            for i in 0..m {
                if char_vec[i] == 'S' {
                    start_pos = (n, i);
                } else if char_vec[i] == 'E' {
                    end_pos = (n, i);
                }
            }
            grid.push(char_vec);
            dist.push(vec![vec![1e7 as usize; 4]; m]);
            n += 1;
        }

        let mut parents: HashMap<((usize, usize), usize), Vec<((usize, usize), usize)>> =
            HashMap::new();

        let mut pq: BinaryHeap<Reverse<(usize, (usize, usize), usize)>> = BinaryHeap::new();
        pq.push(Reverse((0, start_pos, 0)));

        let mut hs: HashSet<(usize, usize)> = HashSet::new();
        let mut min_path_len = usize::MAX;

        while !pq.is_empty() {
            let Reverse((curr_dist, curr_pos, curr_dir)) = pq.pop().unwrap();

            if curr_pos == end_pos {
                if curr_dist < min_path_len {
                    min_path_len = curr_dist;
                    hs.clear();
                } else {
                    continue;
                }
                let mut q: VecDeque<((usize, usize), usize)> = VecDeque::new();
                q.push_back((end_pos, curr_dir));
                while !q.is_empty() {
                    let (pos, dir) = q.pop_front().unwrap();
                    hs.insert(pos);

                    if let Some(parents_vec) = parents.get(&(pos, dir)) {
                        for p in parents_vec {
                            q.push_back(*p);
                        }
                    }
                }
                continue;
            }

            for dir in 0..4 {
                if curr_dir == (dir + 2) % 4 {
                    continue;
                }
                let dx = [0, 1, 0, -1];
                let dy = [1, 0, -1, 0];
                let n_pos = (
                    (curr_pos.0 as i32 + dx[dir]) as usize,
                    (curr_pos.1 as i32 + dy[dir]) as usize,
                );
                if grid[n_pos.0][n_pos.1] == '#' {
                    continue;
                }
                let n_dist = curr_dist + if curr_dir == dir { 1 } else { 1001 };
                if n_dist < dist[n_pos.0][n_pos.1][dir] {
                    pq.push(Reverse((n_dist, n_pos, dir)));
                    dist[n_pos.0][n_pos.1][dir] = n_dist;
                    parents.entry((n_pos, dir)).or_insert(vec![]).clear();
                    parents
                        .get_mut(&(n_pos, dir))
                        .unwrap()
                        .push((curr_pos, curr_dir));
                } else if n_dist == dist[n_pos.0][n_pos.1][dir] {
                    parents
                        .get_mut(&(n_pos, dir))
                        .unwrap()
                        .push((curr_pos, curr_dir));
                }
            }
        }

        // Ok(answer)
        Ok(hs.len())
    }

    let res = part2(BufReader::new(TEST.as_bytes()))?;
    println!("res = {}", res);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}
