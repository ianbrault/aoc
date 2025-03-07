/*
** src/puzzles/aoc2020/day9.rs
*/

use super::Solution;

use std::collections::BTreeSet;

fn find_non_sum_number(numbers: &[u64]) -> Option<u64> {
    let mut preamble = numbers.iter().copied().take(25).collect::<BTreeSet<_>>();
    // iterate thru the remaining numbers to search for the solution
    for (i, &number) in numbers.iter().skip(25).enumerate() {
        // check if the number is the sum of anything in the preamble
        let mut number_is_sum = false;
        for &n in preamble.iter() {
            // first condition is necessary to avoid u64 underflow
            // second condition ensures that the 2 numbers are disjoint
            if (number > n) && (n * 2 != number) && preamble.contains(&(number - n)) {
                number_is_sum = true;
                break;
            }
        }
        if !number_is_sum {
            return Some(number);
        }
        // remove the oldest preamble entry and replace it with the current
        // note: we enumerate after .skip(25) so i starts at 0 and thus
        // tracks the oldest preamble entry
        preamble.remove(&numbers[i]);
        preamble.insert(number);
    }
    None
}

fn find_encryption_weakness(numbers: &[u64], invalid: Option<u64>) -> Option<u64> {
    let target = invalid?;
    // check a sequence of sliding sums
    // bump up the lower end once the sum is greater than the target
    let mut lower = 0;
    let mut upper;
    let mut sum;
    while lower < numbers.len() - 1 {
        upper = lower + 1;
        sum = numbers[lower];
        while sum < target {
            sum += numbers[upper];
            upper += 1;
        }
        if sum == target {
            let min = numbers[lower..upper].iter().copied().min().unwrap();
            let max = numbers[lower..upper].iter().copied().max().unwrap();
            return Some(min + max);
        } else {
            lower += 1;
        }
    }
    None
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let numbers = input
        .split('\n')
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    // Part A: Find the first number in the list (after the preamble) which is not the sum of two
    // of the 25 numbers before it
    let number = find_non_sum_number(&numbers);
    solution.maybe_set_part_a(number);

    // Part B: Find a contiguous set of at least two numbers in your list which sum to the invalid
    // number from step 1. To find the encryption weakness, sum the smallest and largest number in
    // this contiguous range. What is the encryption weakness in your XMAS-encrypted list of
    // numbers?
    let encryption_weakness = find_encryption_weakness(&numbers, number);
    solution.maybe_set_part_b(encryption_weakness);

    solution
}
