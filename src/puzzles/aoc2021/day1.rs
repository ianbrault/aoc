/*
** src/puzzles/aoc2021/day1.rs
*/

use super::Solution;

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let sonar_depths = input
        .split('\n')
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    // Part A: How many measurements are larger than the previous measurement?
    let count = sonar_depths
        .iter()
        .enumerate()
        .skip(1)
        .filter(|&(i, depth)| depth > &sonar_depths[i - 1])
        .count();
    solution.set_part_a(count);

    // Part B: Consider sums of a three-measurement sliding window. How many sums are larger than
    // the previous sum?
    let three_sums = sonar_depths
        .iter()
        .enumerate()
        .skip(2)
        .map(|(i, depth)| depth + sonar_depths[i - 1] + sonar_depths[i - 2])
        .collect::<Vec<_>>();
    let count = three_sums
        .iter()
        .enumerate()
        .skip(1)
        .filter(|&(i, sum)| sum > &three_sums[i - 1])
        .count();
    solution.set_part_b(count);

    solution
}
