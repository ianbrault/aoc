/*
** src/puzzles/aoc2024/day11.rs
*/

use super::Solution;
use crate::types::Counter;
use crate::utils;

fn blink(stones: &[u64], count: usize) -> usize {
    let mut stone_counts = stones.iter().copied().collect::<Counter<_>>();

    for _ in 0..count {
        let mut new_counts = Counter::new();
        for (&stone, &stone_count) in stone_counts.iter() {
            if stone == 0 {
                new_counts.add_many(1, stone_count);
            } else {
                let n_digits = utils::num_digits(stone);
                if n_digits.is_multiple_of(2) {
                    let power = 10u64.pow(n_digits / 2);
                    new_counts.add_many(stone / power, stone_count);
                    new_counts.add_many(stone % power, stone_count);
                } else {
                    new_counts.add_many(stone * 2024, stone_count);
                }
            }
        }
        stone_counts = new_counts;
    }

    stone_counts.iter().map(|(_, &count)| count).sum()
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // You notice a strange set of physics-defying stones. Every time you blink, the stones change.
    // Sometimes, the number engraved on a stone changes. Other times, a stone might split in two,
    // causing all the other stones to shift over a bit to make room in their straight line.
    let stones = utils::split_and_parse::<u64>(&input).collect::<Vec<_>>();

    // Part A: Consider the arrangement of stones in front of you. How many stones will you have
    // after blinking 25 times?
    let count = blink(&stones, 25);
    solution.set_part_a(count);

    // Part B: How many stones would you have after blinking a total of 75 times?
    let count = blink(&stones, 75);
    solution.set_part_b(count);

    solution
}
