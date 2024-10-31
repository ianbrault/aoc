/*
** src/puzzles/aoc2023/day12.rs
*/

use super::Solution;
use crate::utils;

use std::collections::HashMap;

type Cache<'a> = HashMap<(&'a str, &'a [usize], usize), usize>;

struct Record {
    states: String,
    springs: Vec<usize>,
}

impl Record {
    fn arrangements_rec<'a>(cache: &mut Cache<'a>, states: &'a str, springs: &'a [usize], done_in_group: usize) -> usize {
        // check for cached solution
        if let Some(&answer) = cache.get(&(states, springs, done_in_group)) {
            return answer;
        }
        if states.is_empty() {
            // is this a valid solution?
            let solutions = if springs.is_empty() && done_in_group == 0 {
                1
            } else {
                0
            };
            cache.insert((states, springs, done_in_group), solutions);
            return solutions;
        }

        let mut solutions = 0;
        let next_state = states.chars().nth(0).unwrap();
        // branch on unknown state
        let options = if next_state == '?' {
            vec!['.', '#']
        } else {
            vec![next_state]
        };
        for state in options {
            if state == '#' {
                // extend the current group
                solutions += Self::arrangements_rec(cache, &states[1..], springs, done_in_group + 1);
            } else if done_in_group > 0 {
                // if in a group and it can be closed, close it
                if !springs.is_empty() && springs[0] == done_in_group {
                    solutions += Self::arrangements_rec(cache, &states[1..], &springs[1..], 0);
                }
            } else {
                // not in a group, move on to the next symbol
                solutions += Self::arrangements_rec(cache, &states[1..], springs, 0);
            }
        }
        cache.insert((states, springs, done_in_group), solutions);
        solutions
    }

    fn arrangements(&self) -> usize {
        let mut cache = Cache::new();
        Self::arrangements_rec(&mut cache, &self.states, &self.springs, 0)
    }

    fn unfold(&self) -> Self {
        // note the EOF that was added to the states
        let mut states = String::with_capacity(((self.states.len() - 1) * 5) + 1);
        let mut springs = Vec::with_capacity(self.springs.len() * 5);
        for i in 0..5 {
            states += &self.states[..(self.states.len() - 1)];
            if i < 4 {
                states += "?";
            }
            for &spring in self.springs.iter() {
                springs.push(spring);
            }
        }
        states += ".";

        Self { states, springs }
    }
}

impl From<&str> for Record {
    fn from(value: &str) -> Self {
        let (states_str, springs_str) = utils::split(value, " ").unwrap();
        let mut states = states_str.into();
        // add an additional operational spring to the end to serve as an EOF
        states += ".";
        let springs = springs_str.split(',').map(|s| s.parse().unwrap()).collect();
        Self { states, springs }
    }
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // Many of the springs have fallen into disrepair, so they're not actually sure which springs
    // would even be safe to use! Worse yet, their condition records of which springs are damaged
    // are also damaged! You'll need to help them repair the damaged records.
    let records = input.split('\n').map(Record::from).collect::<Vec<_>>();

    // Part A: For each row, count all of the different arrangements of operational and broken
    // springs that meet the given criteria. What is the sum of those counts?
    let arrangements = records.iter().map(|record| record.arrangements()).sum::<usize>();
    solution.set_part_a(arrangements);

    // Part B: When you examine the records, you discover that they were actually folded up this
    // whole time! To unfold the records, on each row, replace the list of spring conditions with
    // five copies of itself (separated by ?) and replace the list of contiguous groups of damaged
    // springs with five copies of itself. Unfold your condition records; what is the new sum of
    // possible arrangement counts?
    let records_unfolded = records.iter().map(|record| record.unfold());
    let arrangements_unfolded = records_unfolded.map(|record| record.arrangements()).sum::<usize>();
    solution.set_part_b(arrangements_unfolded);

    solution
}
