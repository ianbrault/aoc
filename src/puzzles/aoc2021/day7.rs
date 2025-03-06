/*
** src/puzzles/aoc2021/day7.rs
*/

use super::Solution;

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let positions = input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    // Part A: Determine the horizontal position that the crabs can align to using the least fuel
    // possible. How much fuel must they spend to align to that position?
    let mut numbers = positions.clone();
    numbers.sort_unstable();
    let median = numbers[numbers.len() / 2];
    // determine the fuel used to align all crabs at the median
    let fuel = positions.iter().map(|n| i64::abs(n - median)).sum::<i64>();
    solution.set_part_a(fuel);

    // Part B: As each crab moves, moving further becomes more expensive. How much fuel must they
    // spend to align to that position?
    // the most efficient position is the average of the inputs
    let average = positions.iter().sum::<i64>() as f64 / positions.len() as f64;
    let average_int = average.floor() as i64;
    // determine the fuel used to align all crabs at the median
    let fuel = positions
        .iter()
        .map(|n| i64::abs(n - average_int))
        .map(|n| (0..=n).sum::<i64>())
        .sum::<i64>();
    solution.set_part_b(fuel);

    solution
}
