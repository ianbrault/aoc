/*
** src/puzzles/aoc2025/day2.rs
*/

use super::Solution;
use crate::itertools::*;
use crate::types::RangeInclusive;
use crate::utils;

fn parse_range(value: &str) -> RangeInclusive<u64> {
    let (start, end) = utils::split(value, "-").unwrap();
    RangeInclusive::new(start.parse().unwrap(), end.parse().unwrap())
}

fn extract_digits(number: u64, count: u32, offset: u32) -> u64 {
    let right = 10u64.pow(offset);
    let left = 10u64.pow(offset + count);
    (number % left) / right
}

fn is_valid_id(id: u64) -> bool {
    let digits = utils::num_digits(id);
    if !digits.is_multiple_of(2) {
        return true;
    }
    // Split the number in half
    let power = 10u64.pow(digits / 2);
    let top = id / power;
    let bottom = id % power;
    top != bottom
}

fn is_valid_id_expanded(id: u64) -> bool {
    let digits = utils::num_digits(id);
    for n in 1..=(digits / 2) {
        if digits.is_multiple_of(n)
            && (0..digits)
                .step_by(n as usize)
                .map(|offset| extract_digits(id, n, offset))
                .all_equal()
        {
            return false;
        }
    }
    true
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // One of the younger Elves was playing on a gift shop computer and managed to add a whole
    // bunch of invalid product IDs to their gift shop database!
    let product_ids = input.split(',').map(parse_range).collect::<Vec<_>>();

    // Part A: You can find the invalid IDs by looking for any ID which is made only of some
    // sequence of digits repeated twice. What do you get if you add up all of the invalid IDs?
    let mut sum = 0;
    for range in product_ids.iter() {
        for id in range.start..=range.end {
            if !is_valid_id(id) {
                sum += id;
            }
        }
    }
    solution.set_part_a(sum);

    // Part B: Now, an ID is invalid if it is made only of some sequence of digits repeated at
    // least twice. What do you get if you add up all of the invalid IDs using these new rules?
    sum = 0;
    for range in product_ids.iter() {
        for id in range.start..=range.end {
            if !is_valid_id_expanded(id) {
                sum += id;
            }
        }
    }
    solution.set_part_b(sum);

    solution
}
