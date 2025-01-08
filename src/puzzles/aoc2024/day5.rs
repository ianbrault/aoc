/*
** src/puzzles/aoc2024/day5.rs
*/

use super::Solution;
use crate::utils;

use std::collections::{HashMap, HashSet};

struct Rules(HashMap<u8, HashSet<u8>>);

struct Update(Vec<u8>);

impl Update {
    fn middle_number(&self) -> u8 {
        self.0[self.0.len() / 2]
    }

    fn is_valid(&self, rules: &Rules) -> bool {
        for i in 1..self.0.len() {
            if let Some(set) = rules.0.get(&self.0[i]) {
                for nn in self.0[..i].iter() {
                    if set.contains(nn) {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn fix_order_rec(update: &mut Vec<u8>, rules: &Rules) {
        for i in 1..update.len() {
            if let Some(set) = rules.0.get(&update[i]) {
                for j in 0..i {
                    if set.contains(&update[j]) {
                        // j is before i but i should be before j, insert i before j and recurse
                        let n = update.remove(i);
                        update.insert(j, n);
                        Self::fix_order_rec(update, rules);
                        return;
                    }
                }
            }
        }
    }

    fn fix_order(&self, rules: &Rules) -> Self {
        let mut update = self.0.clone();
        Self::fix_order_rec(&mut update, rules);
        Self(update)
    }
}

fn parse_input(input: String) -> (Rules, Vec<Update>) {
    let (rules_str, updates_str) = utils::split(&input, "\n\n").unwrap();

    let mut rules = HashMap::new();
    for rule_line in rules_str.split('\n') {
        let (a, b) = utils::split(rule_line, "|").unwrap();
        let entry = rules.entry(a.parse().unwrap()).or_insert(HashSet::new());
        entry.insert(b.parse().unwrap());
    }
    let mut updates = Vec::new();
    for update_line in updates_str.split('\n') {
        let update = update_line
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<_>>();
        updates.push(Update(update));
    }

    (Rules(rules), updates)
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // The Elf has for you both the page ordering rules and the pages to produce in each update,
    // but can't figure out whether each update has the pages in the right order.
    let (rules, updates) = parse_input(input);

    // Part A: Determine which updates are already in the correct order. What do you get if you add
    // up the middle page number from those correctly-ordered updates?
    let middle_page_sum = updates
        .iter()
        .filter(|update| update.is_valid(&rules))
        .map(|update| update.middle_number() as u32)
        .sum::<u32>();
    solution.set_part_a(middle_page_sum);

    // Part B: Find the updates which are not in the correct order. What do you get if you add up
    // the middle page numbers after correctly ordering just those updates?
    let fixed_middle_page_sum = updates
        .iter()
        .filter(|update| !update.is_valid(&rules))
        .map(|update| update.fix_order(&rules).middle_number() as u32)
        .sum::<u32>();
    solution.set_part_b(fixed_middle_page_sum);

    solution
}
