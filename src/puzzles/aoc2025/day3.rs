/*
** src/puzzles/aoc2025/day3.rs
*/

use super::Solution;

fn find_digit(bank: &[u64], digits: usize, index: usize, start: usize) -> (u64, usize) {
    // Reserve digits at the end so enough remain for the rest of the joltage
    let end_reserved = digits - index - 1;
    let value = bank[start..(bank.len() - end_reserved)]
        .iter()
        .max()
        .unwrap();
    let value_index = bank
        .iter()
        .enumerate()
        .position(|(i, d)| i >= start && d == value)
        .unwrap();
    (*value, value_index)
}

fn maximum_joltage(bank: &[u64], digits: usize) -> u64 {
    let mut joltage = 0;
    let mut start = 0;
    for index in 0..digits {
        let (value, value_index) = find_digit(bank, digits, index, start);
        joltage += value * 10u64.pow((digits - index - 1) as u32);
        start = value_index + 1;
    }
    joltage
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // The batteries are arranged into banks; each line of digits in your input corresponds to a
    // single bank of batteries.
    let banks = input
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Part A: Find the maximum joltage possible from each bank; what is the total output joltage?
    let joltage_basic = banks
        .iter()
        .map(|bank| maximum_joltage(bank, 2))
        .sum::<u64>();
    solution.set_part_a(joltage_basic);

    // Part B: The joltage output for the bank is still the number formed by the digits of the
    // batteries you've turned on; the only difference is that now there will be 12 digits in each
    // bank's joltage output instead of two. What is the new total output joltage?
    let joltage = banks
        .iter()
        .map(|bank| maximum_joltage(bank, 12))
        .sum::<u64>();
    solution.set_part_b(joltage);

    solution
}
