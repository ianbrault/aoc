/*
** src/puzzles/aoc2022/day3.rs
*/

use super::Solution;
use crate::itertools::*;

use std::collections::BTreeSet;

struct Rucksack {
    compartment_a: BTreeSet<char>,
    compartment_b: BTreeSet<char>,
    full_rucksack: BTreeSet<char>,
}

impl Rucksack {
    fn common_char(&self) -> char {
        *self
            .compartment_a
            .intersection(&self.compartment_b)
            .next()
            .unwrap()
    }

    fn common_char_in_group(elf_a: &Self, elf_b: &Self, elf_c: &Self) -> char {
        let a_b_isect = elf_a
            .full_rucksack
            .intersection(&elf_b.full_rucksack)
            .cloned()
            .collect::<BTreeSet<_>>();
        *a_b_isect.intersection(&elf_c.full_rucksack).next().unwrap()
    }
}

impl From<&str> for Rucksack {
    fn from(value: &str) -> Self {
        let length = value.len();
        let half = length / 2;
        let compartment_a_str = &value[..half];
        let compartment_b_str = &value[half..length];
        let compartment_a = compartment_a_str.chars().collect();
        let compartment_b = compartment_b_str.chars().collect();
        let full_rucksack = value.chars().collect();
        Self {
            compartment_a,
            compartment_b,
            full_rucksack,
        }
    }
}

fn priority(ch: char) -> u64 {
    let cn = ch as u64;
    let base_lower = 'a' as u64;
    let base_upper = 'A' as u64;
    if ch.is_lowercase() {
        cn - base_lower + 1
    } else {
        cn - base_upper + 27
    }
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let rucksacks = input.split('\n').map(Rucksack::from).collect::<Vec<_>>();

    // Part A: Find the item type that appears in both compartments of each rucksack. What is the
    // sum of the priorities of those item types?
    let item_priorities = rucksacks
        .iter()
        .map(|rucksack| rucksack.common_char())
        .map(priority)
        .sum::<u64>();
    solution.set_part_a(item_priorities);

    // Part B: Find the item type that corresponds to the badges of each three-Elf group. What is
    // the sum of the priorities of those item types?
    let elf_groups = rucksacks.iter().triples().collect::<Vec<_>>();
    let group_priorities = elf_groups
        .iter()
        .map(|(a, b, c)| Rucksack::common_char_in_group(a, b, c))
        .map(priority)
        .sum::<u64>();
    solution.set_part_b(group_priorities);

    solution
}
