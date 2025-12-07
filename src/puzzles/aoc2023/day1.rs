/*
** src/puzzles/aoc2023/day1.rs
*/

use super::Solution;

use log::debug;

const DIGIT_STRINGS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn first_digit(string: &str, matching_digit_strings: bool) -> Option<u32> {
    for (i, c) in string.char_indices() {
        if c.is_numeric() {
            return Some(c.to_digit(10).unwrap());
        } else if matching_digit_strings {
            let substring = &string[i..];
            for digit in DIGIT_STRINGS {
                if substring.starts_with(digit) {
                    let index = DIGIT_STRINGS.iter().position(|&s| s == digit).unwrap();
                    return Some(index as u32 + 1);
                }
            }
        }
    }
    None
}

fn last_digit(string: &str, matching_digit_strings: bool) -> Option<u32> {
    for (i, c) in string.chars().rev().enumerate() {
        if c.is_numeric() {
            return Some(c.to_digit(10).unwrap());
        } else if matching_digit_strings {
            let j = string.len() - i - 1;
            let substring = &string[j..];
            for digit in DIGIT_STRINGS {
                if substring.starts_with(digit) {
                    let index = DIGIT_STRINGS.iter().position(|&s| s == digit).unwrap();
                    return Some(index as u32 + 1);
                }
            }
        }
    }
    None
}

fn calibration_value(line: &str, matching_digit_strings: bool) -> u32 {
    let a = first_digit(line, matching_digit_strings).unwrap();
    let b = last_digit(line, matching_digit_strings).unwrap();
    debug!("Input: {}: first: {}: last: {}", line, a, b);
    (a * 10) + b
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();

    // Part A: On each line, the calibration value can be found by combining the first digit and
    // the last digit (in that order) to form a single two-digit number. Consider your entire
    // calibration document. What is the sum of all of the calibration values?
    let calibration_values = input
        .split('\n')
        .map(|line| calibration_value(line, false))
        .sum::<u32>();
    solution.set_part_a(calibration_values);

    // Part B: Your calculation isn't quite right. It looks like some of the digits are actually
    // spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also
    // count as valid "digits". Equipped with this new information, what is the sum of all of the
    // calibration values?
    let calibration_values_replacing_digits = input
        .split('\n')
        .map(|line| calibration_value(line, true))
        .sum::<u32>();
    solution.set_part_b(calibration_values_replacing_digits);

    solution
}
