/*
** src/puzzles/aoc2022/day1.rs
*/

use super::Solution;

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // One important consideration is food, in particular, the number of Calories each Elf
    // is carrying.
    let mut calories = input
        .split("\n\n")
        .map(|chunk| {
            chunk
                .split('\n')
                .map(|c| c.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .collect::<Vec<_>>();
    calories.sort();

    // Part A: Find the Elf carrying the most Calories. How many total Calories is that Elf
    // carrying?
    let most_calories = calories.last().cloned();
    solution.maybe_set_part_a(most_calories);

    // Part B: Find the top three Elves carrying the most Calories. How many Calories are those
    // Elves carrying in total?
    let top_3_calories = calories[(calories.len() - 3)..].iter().sum::<u64>();
    solution.set_part_b(top_3_calories);

    solution
}
