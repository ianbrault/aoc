/*
** src/puzzles/aoc2024/day7.rs
*/

use super::Solution;
use crate::utils;

struct Equation {
    test_value: u64,
    numbers: Vec<u64>,
}

impl Equation {
    fn concat(x: u64, y: u64) -> u64 {
        (x * 10_u64.pow(utils::num_digits(y))) + y
    }

    fn can_be_solved_rec(&self, total: u64, index: usize) -> bool {
        if index == self.numbers.len() {
            total == self.test_value
        } else if total > self.test_value {
            false
        } else {
            // recurse, checking add and multiply for the next number
            self.can_be_solved_rec(total + self.numbers[index], index + 1)
                || self.can_be_solved_rec(total * self.numbers[index], index + 1)
        }
    }

    fn can_be_solved(&self) -> bool {
        self.can_be_solved_rec(self.numbers[0], 1)
    }

    fn can_be_solved_with_concatenation_rec(&self, total: u64, index: usize) -> bool {
        if index == self.numbers.len() {
            total == self.test_value
        } else if total > self.test_value {
            false
        } else {
            // recurse, checking add and multiply and concatenate for the next number
            self.can_be_solved_with_concatenation_rec(total + self.numbers[index], index + 1)
                || self.can_be_solved_with_concatenation_rec(total * self.numbers[index], index + 1)
                || self.can_be_solved_with_concatenation_rec(
                    Self::concat(total, self.numbers[index]),
                    index + 1,
                )
        }
    }

    fn can_be_solved_with_concatenation(&self) -> bool {
        self.can_be_solved_with_concatenation_rec(self.numbers[0], 1)
    }
}

impl From<&str> for Equation {
    fn from(value: &str) -> Self {
        let (test_value_str, numbers_str) = utils::split(value, ": ").unwrap();
        let test_value = test_value_str.parse().unwrap();
        let numbers = utils::split_and_parse(numbers_str).collect::<Vec<_>>();
        Self {
            test_value,
            numbers,
        }
    }
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // The engineers tell you that the bridge only needs final calibrations, but some young
    // elephants were playing nearby and stole all the operators from their calibration equations!
    // They could finish the calibrations if only you could determine which test values could be
    // produced by placing any combination of operators into their calibration equations.
    let equations = input.split('\n').map(Equation::from).collect::<Vec<_>>();

    // Part A: Determine which equations could possibly be true. What is their total
    // calibration result?
    let calibration_result = equations
        .iter()
        .filter(|eq| eq.can_be_solved())
        .map(|eq| eq.test_value)
        .sum::<u64>();
    solution.set_part_a(calibration_result);

    // Part B: Using your new knowledge of elephant hiding spots, determine which equations could
    // possibly be true. What is their total calibration result?
    let calibration_result_concat = equations
        .iter()
        .filter(|eq| eq.can_be_solved_with_concatenation())
        .map(|eq| eq.test_value)
        .sum::<u64>();
    solution.set_part_b(calibration_result_concat);

    solution
}
