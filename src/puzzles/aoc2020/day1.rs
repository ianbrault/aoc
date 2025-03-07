/*
** src/puzzles/aoc2020/day1.rs
*/

use super::Solution;

use std::collections::BTreeSet;

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let entries = input
        .split('\n')
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<BTreeSet<_>>();

    // Part A: Find two entries that sum to 2020; what do you get if you multiply them together?
    let mut answer = None;
    for entry in entries.iter() {
        let pair = 2020 - entry;
        if pair > 0 && entries.contains(&pair) {
            answer = Some(entry * pair);
        }
    }
    solution.maybe_set_part_a(answer);

    // Part B: What is the product of the three entries that sum to 2020?
    let mut answer = None;
    for entry_a in entries.iter() {
        for entry_b in entries.iter() {
            let partner = 2020 - entry_a - entry_b;
            if partner > 0 && entries.contains(&partner) {
                answer = Some(entry_a * entry_b * partner);
            }
        }
    }
    solution.maybe_set_part_b(answer);

    solution
}
