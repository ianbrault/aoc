/*
** src/puzzles/aoc2020/day5.rs
*/

use super::Solution;

struct BoardingPass(u64);

impl BoardingPass {
    fn binary_partition(slice: &str, mut min: u64, mut max: u64, cmin: char, cmax: char) -> u64 {
        for c in slice.chars() {
            let delta = (max - min) / 2;
            if c == cmin {
                max -= delta;
            } else if c == cmax {
                min += delta;
            }
        }
        min
    }
}

impl From<&str> for BoardingPass {
    fn from(value: &str) -> Self {
        let row = Self::binary_partition(&value[..7], 0, 128, 'F', 'B');
        let col = Self::binary_partition(&value[7..10], 0, 8, 'L', 'R');
        Self((row * 8) + col)
    }
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let boarding_passes = input
        .split('\n')
        .map(BoardingPass::from)
        .collect::<Vec<_>>();

    // Part A: What is the highest seat ID on a boarding pass?
    let max = boarding_passes.iter().map(|BoardingPass(n)| n).max();
    solution.maybe_set_part_a(max);

    // Part B: What is the ID of your seat?
    let mut ids = boarding_passes
        .iter()
        .map(|BoardingPass(n)| n)
        .collect::<Vec<_>>();
    ids.sort();
    // find boarding pass IDs which have a gap of 1
    let mut my_id = None;
    for (i, &id) in ids.iter().enumerate().skip(1) {
        if id - ids[i - 1] == 2 {
            my_id = Some(id - 1);
        }
    }
    solution.maybe_set_part_b(my_id);

    solution
}
