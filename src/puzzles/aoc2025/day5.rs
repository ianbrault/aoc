/*
** src/puzzles/aoc2025/day5.rs
*/

use super::Solution;
use crate::types::RangeInclusive;
use crate::utils;

struct Database {
    fresh_ingredients: Vec<RangeInclusive<u64>>,
    available_ingredients: Vec<u64>,
}

impl Database {
    fn new(fresh_ingredients: Vec<RangeInclusive<u64>>, available_ingredients: Vec<u64>) -> Self {
        Self {
            fresh_ingredients: RangeInclusive::reduce(fresh_ingredients),
            available_ingredients,
        }
    }

    fn ingredient_is_fresh(&self, ingredient: u64) -> bool {
        for range in self.fresh_ingredients.iter() {
            if range.contains(&ingredient) {
                return true;
            }
        }
        false
    }
}

impl From<String> for Database {
    fn from(value: String) -> Self {
        let (fresh_ingredient_lines, available_ingredient_lines) =
            utils::split(&value, "\n\n").unwrap();
        let fresh_ingredients = fresh_ingredient_lines
            .split('\n')
            .map(|line| {
                let (start, end) = utils::split(line, "-").unwrap();
                RangeInclusive::new(start.parse().unwrap(), end.parse().unwrap())
            })
            .collect();
        let available_ingredients = available_ingredient_lines
            .split('\n')
            .map(|line| line.parse().unwrap())
            .collect();
        Self::new(fresh_ingredients, available_ingredients)
    }
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // The database operates on ingredient IDs. It consists of a list of fresh ingredient ID
    // ranges, a blank line, and a list of available ingredient IDs.
    let database = Database::from(input);

    // Part A: Process the database file from the new inventory management system. How many of the
    // available ingredient IDs are fresh?
    let fresh_ingredients = database
        .available_ingredients
        .iter()
        .filter(|&&ingredient| database.ingredient_is_fresh(ingredient))
        .count();
    solution.set_part_a(fresh_ingredients);

    // Part B: Process the database file again. How many ingredient IDs are considered to be fresh
    // according to the fresh ingredient ID ranges?
    let ingredients = database
        .fresh_ingredients
        .iter()
        .map(|range| range.size() + 1)
        .sum::<u64>();
    solution.set_part_b(ingredients);

    solution
}
