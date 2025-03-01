/*
** src/puzzles/aoc2022/day4.rs
*/

use super::Solution;
use crate::utils;

struct Pair(u8, u8);

impl From<&str> for Pair {
    fn from(value: &str) -> Self {
        let (a, b) = utils::split(value, "-").unwrap();
        Self(a.parse().unwrap(), b.parse().unwrap())
    }
}

struct AssignmentPair {
    x: Pair,
    y: Pair,
}

impl AssignmentPair {
    fn pair_contains_other(&self) -> bool {
        // x is a smaller pair than y
        self.y.0 <= self.x.0 && self.y.1 >= self.x.1
    }

    fn pairs_overlap(&self) -> bool {
        if self.x.0 < self.y.0 {
            self.y.0 <= self.x.1
        } else {
            self.x.0 <= self.y.1
        }
    }
}

impl From<&str> for AssignmentPair {
    fn from(value: &str) -> Self {
        let (x, y) = utils::split(value, ",").unwrap();
        let a = Pair::from(x);
        let b = Pair::from(y);
        // set the smaller pair as x and the larger as y
        if a.1 - a.0 < b.1 - b.0 {
            Self { x: a, y: b }
        } else {
            Self { x: b, y: a }
        }
    }
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let assignment_pairs = input
        .split('\n')
        .map(AssignmentPair::from)
        .collect::<Vec<_>>();

    // Part A: In how many assignment pairs does one range fully contain the other?
    let contains = assignment_pairs
        .iter()
        .filter(|x| x.pair_contains_other())
        .count();
    solution.set_part_a(contains);

    // Part B: In how many assignment pairs do the ranges overlap?
    let overlaps = assignment_pairs
        .iter()
        .filter(|x| x.pairs_overlap())
        .count();
    solution.set_part_b(overlaps);

    solution
}
