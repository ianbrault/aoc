/*
** src/puzzles/aoc2024/day19.rs
*/

use super::Solution;
use crate::utils;

use log::debug;

use std::collections::HashMap;

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (towel_patterns, towel_designs) = utils::split(input, "\n\n").unwrap();
    let patterns = towel_patterns.split(", ").collect();
    let designs = towel_designs.split('\n').collect();
    (patterns, designs)
}

fn design_is_possible_rec(design: &str, patterns: &[&str], index: usize) -> bool {
    if index >= design.len() {
        return true;
    }
    for pattern in patterns.iter() {
        if design[index..].starts_with(pattern)
            && design_is_possible_rec(design, patterns, index + pattern.len())
        {
            return true;
        }
    }
    false
}

fn design_is_possible(design: &str, patterns: &[&str]) -> bool {
    let possible = design_is_possible_rec(design, patterns, 0);
    debug!("design {}: {}", design, possible);
    possible
}

fn design_permutations_rec(
    design: &str,
    patterns: &[&str],
    index: usize,
    memo: &mut HashMap<usize, usize>,
) -> usize {
    if index >= design.len() {
        return 1;
    }
    if memo.contains_key(&index) {
        return memo[&index];
    }

    let mut count = 0;
    for pattern in patterns.iter() {
        if design[index..].starts_with(pattern) {
            count += design_permutations_rec(design, patterns, index + pattern.len(), memo);
        }
    }
    memo.insert(index, count);
    count
}

fn design_permutations(design: &str, patterns: &[&str]) -> usize {
    let mut memo = HashMap::new();
    design_permutations_rec(design, patterns, 0, &mut memo)
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // To start, collect all of the available towel patterns and the list of desired designs.
    let (patterns, designs) = parse_input(&input);

    // Part A: To get into the onsen as soon as possible, consult your list of towel patterns and
    // desired designs carefully. How many designs are possible?
    let possible_designs = designs
        .into_iter()
        .filter(|design| design_is_possible(design, &patterns))
        .collect::<Vec<_>>();
    solution.set_part_a(possible_designs.len());

    // Part B: To avoid an endless cycle of towel rearrangement, maybe you should just give them
    // every possible option. What do you get if you add up the number of different ways you could
    // make each design?
    let total_designs = possible_designs
        .into_iter()
        .map(|design| design_permutations(design, &patterns))
        .sum::<usize>();
    solution.set_part_b(total_designs);

    solution
}
