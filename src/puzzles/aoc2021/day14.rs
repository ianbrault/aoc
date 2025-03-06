/*
** src/puzzles/aoc2021/day14.rs
*/

use super::Solution;
use crate::types::Counter;
use crate::utils;

use std::collections::HashMap;

type PairCounter = Counter<Pair>;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Pair(char, char);

impl Pair {
    fn new(c1: char, c2: char) -> Self {
        Self(c1, c2)
    }
}

impl From<&str> for Pair {
    fn from(value: &str) -> Self {
        Self(value.chars().next().unwrap(), value.chars().nth(1).unwrap())
    }
}

fn pair_matches_rule(pair: &Pair, rules: &HashMap<Pair, char>) -> Option<(Pair, Pair)> {
    if let Some(&c) = rules.get(pair) {
        let pa = Pair::new(pair.0, c);
        let pb = Pair::new(c, pair.1);
        Some((pa, pb))
    } else {
        None
    }
}

fn apply_pair_insertion(rules: &HashMap<Pair, char>, pairs: PairCounter) -> PairCounter {
    let mut output = Counter::new();
    for (pair, &count) in pairs.iter() {
        if let Some((new_pair_a, new_pair_b)) = pair_matches_rule(pair, rules) {
            output.add_many(new_pair_a, count);
            output.add_many(new_pair_b, count);
        } else {
            output.add_many(pair.clone(), count);
        }
    }
    output
}

fn pair_counter_to_char_counter(pair_counts: PairCounter) -> Counter<char> {
    let mut char_counts = Counter::new();
    for (pair, &count) in pair_counts.iter() {
        char_counts.add_many(pair.0, count);
        char_counts.add_many(pair.1, count);
    }

    let mut output = Counter::new();
    for (&c, &count) in char_counts.iter() {
        output.add_many(c, (count + 1) / 2);
    }
    output
}

fn parse_rule(s: &str) -> (Pair, char) {
    let (pair, sub) = utils::split(s, " -> ").unwrap();
    (Pair::from(pair), sub.chars().next().unwrap())
}

fn parse_input(input: &str) -> (&str, HashMap<Pair, char>) {
    let (template, rules_str) = utils::split(input, "\n\n").unwrap();
    let rules = rules_str.split('\n').map(parse_rule).collect();
    (template, rules)
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let (template, rules) = parse_input(input.as_str());
    // initialize the pair counter

    // Part A: Apply 10 steps of pair insertion to the polymer template and find the most and least
    // common elements in the result. What do you get if you take the quantity of the most common
    // element and subtract the quantity of the least common element?
    let mut pairs = template
        .chars()
        .zip(template.chars().skip(1))
        .map(|(a, b)| Pair::new(a, b))
        .collect::<PairCounter>();
    for _ in 0..10 {
        pairs = apply_pair_insertion(&rules, pairs);
    }
    let counts = pair_counter_to_char_counter(pairs);
    solution.set_part_a(counts.max() - counts.min());

    // Part B: Apply 40 steps of pair insertion to the polymer template and find the most and least
    // common elements in the result. What do you get if you take the quantity of the most common
    // element and subtract the quantity of the least common element?
    let mut pairs = template
        .chars()
        .zip(template.chars().skip(1))
        .map(|(a, b)| Pair::new(a, b))
        .collect::<PairCounter>();
    for _ in 0..40 {
        pairs = apply_pair_insertion(&rules, pairs);
    }
    let counts = pair_counter_to_char_counter(pairs);
    solution.set_part_b(counts.max() - counts.min());

    solution
}
