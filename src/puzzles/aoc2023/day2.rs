/*
** src/puzzles/aoc2023/day2.rs
*/

use log::debug;

use crate::types::Solution;

use std::cmp::max;

struct Game {
    id: u32,
    red_cubes: u32,
    green_cubes: u32,
    blue_cubes: u32,
}

impl Game {
    fn parse_game_id(string: &str) -> u32 {
        let separator = string.find(" ").unwrap();
        string[(separator + 1)..].parse().unwrap()
    }

    fn power(&self) -> u32 {
        self.red_cubes * self.green_cubes * self.blue_cubes
    }
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        // ex: Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let separator = value.find(": ").unwrap();
        let id = Self::parse_game_id(&value[..separator]);
        let cube_list = &value[(separator + 1)..];

        let mut red_cubes = 0;
        let mut green_cubes = 0;
        let mut blue_cubes = 0;
        for cube_set in cube_list.split("; ") {
            let mut red_cubes_pulled = 0;
            let mut green_cubes_pulled = 0;
            let mut blue_cubes_pulled = 0;
            for cubes in cube_set.split(", ") {
                let cubes = cubes.trim();
                let separator = cubes.find(" ").unwrap();
                let count = cubes[..separator].parse::<u32>().unwrap();
                match &cubes[(separator + 1)..] {
                    "red" => red_cubes_pulled = count,
                    "green" => green_cubes_pulled = count,
                    "blue" => blue_cubes_pulled = count,
                    color => panic!("Game::From<String>: invalid color: {}", color),
                }
            }
            red_cubes = max(red_cubes, red_cubes_pulled);
            green_cubes = max(green_cubes, green_cubes_pulled);
            blue_cubes = max(blue_cubes, blue_cubes_pulled);
        }

        debug!(
            "Parsed game {}: red: {}: green: {}: blue: {}",
            id, red_cubes, green_cubes, blue_cubes
        );
        Self {
            id,
            red_cubes,
            green_cubes,
            blue_cubes,
        }
    }
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // Each game is listed with its ID number followed by a semicolon-separated list of subsets of
    // cubes that were revealed from the bag (like 3 red, 5 green, 4 blue)
    let games = input.split("\n").map(Game::from).collect::<Vec<_>>();

    // Part A: Determine which games would have been possible if the bag had been loaded with only
    // 12 red cubes, 13 green cubes, and 14 blue cubes. What is the sum of the IDs of those games?
    let red_cube_count = 12;
    let green_cube_count = 13;
    let blue_cube_count = 14;
    let game_id_sum = games
        .iter()
        .filter(|game| {
            game.red_cubes <= red_cube_count
                && game.green_cubes <= green_cube_count
                && game.blue_cubes <= blue_cube_count
        })
        .map(|game| game.id)
        .sum::<u32>();
    solution.set_part_a(game_id_sum);

    // Part B: The power of a set of cubes is equal to the numbers of red, green, and blue cubes
    // multiplied together. For each game, find the minimum set of cubes that must have been
    // present. What is the sum of the power of these sets?
    let game_power_sum = games.iter().map(|game| game.power()).sum::<u32>();
    solution.set_part_b(game_power_sum);

    solution
}
