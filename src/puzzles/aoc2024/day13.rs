/*
** src/puzzles/aoc2024/day13.rs
*/

use super::Solution;
use crate::types::Point;
use crate::utils;

use nalgebra::{Matrix2, Vector2};

#[derive(Clone, Copy)]
struct ButtonConfiguration {
    x: i64,
    y: i64,
}

impl From<&str> for ButtonConfiguration {
    fn from(value: &str) -> Self {
        let diffs = utils::split_tail(value, ": ").unwrap();
        let (x_diff, y_diff) = utils::split(diffs, ", ").unwrap();
        let dx = utils::split_tail(x_diff, "+").unwrap();
        let dy = utils::split_tail(y_diff, "+").unwrap();
        Self {
            x: dx.parse().unwrap(),
            y: dy.parse().unwrap(),
        }
    }
}

struct Game {
    button_a_config: ButtonConfiguration,
    button_b_config: ButtonConfiguration,
    prize_location: Point,
}

impl Game {
    const GAMMA: f64 = 0.0001;

    fn new(
        button_a_config: ButtonConfiguration,
        button_b_config: ButtonConfiguration,
        prize_location: Point,
    ) -> Self {
        Self {
            button_a_config,
            button_b_config,
            prize_location,
        }
    }

    fn valid_fractional(a: f64) -> bool {
        let f = a.fract();
        !(Self::GAMMA..=(1.0 - Self::GAMMA)).contains(&f)
    }

    fn minimum_token_cost(&self) -> i64 {
        let a = Matrix2::new(
            self.button_a_config.x as f64,
            self.button_b_config.x as f64,
            self.button_a_config.y as f64,
            self.button_b_config.y as f64,
        );
        let b = Vector2::new(self.prize_location.x as f64, self.prize_location.y as f64);

        if let Some(a_inv) = a.try_inverse() {
            let solution = a_inv * b;
            if solution.x >= 0.0
                && solution.y >= 0.0
                && Self::valid_fractional(solution.x)
                && Self::valid_fractional(solution.y)
            {
                (solution.x.round() as i64 * 3) + solution.y.round() as i64
            } else {
                0
            }
        } else {
            0
        }
    }
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        match value.split('\n').collect::<Vec<_>>().as_slice() {
            &[button_a_line, button_b_line, prize_line] => {
                let button_a_config = ButtonConfiguration::from(button_a_line);
                let button_b_config = ButtonConfiguration::from(button_b_line);

                let prize_str = utils::split_tail(prize_line, ": ").unwrap();
                let (x_loc, y_loc) = utils::split(prize_str, ", ").unwrap();
                let x = utils::split_tail(x_loc, "=").unwrap();
                let y = utils::split_tail(y_loc, "=").unwrap();
                let prize_location =
                    Point::new(x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap());

                Self::new(button_a_config, button_b_config, prize_location)
            }
            _ => unreachable!(),
        }
    }
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // The claw machines here are a little unusual. Instead of a joystick or directional buttons to
    // control the claw, these machines have two buttons labeled A and B. Worse, you can't just put
    // in a token and play; it costs 3 tokens to push the A button and 1 token to push the B
    // button. You estimate that each button would need to be pressed no more than 100 times to win
    // a prize. How else would someone be expected to play?
    let games = input.split("\n\n").map(Game::from).collect::<Vec<_>>();

    // Part A: Figure out how to win as many prizes as possible. What is the fewest tokens you
    // would have to spend to win all possible prizes?
    let tokens = games
        .iter()
        .map(|game| game.minimum_token_cost())
        .sum::<i64>();
    solution.set_part_a(tokens);

    // Part B: Due to a unit conversion error in your measurements, the position of every prize is
    // actually 10000000000000 higher on both the X and Y axis! Using the corrected prize
    // coordinates, figure out how to win as many prizes as possible. What is the fewest tokens you
    // would have to spend to win all possible prizes?
    let delta = 10000000000000;
    let games_corrected = games
        .iter()
        .map(|game| {
            Game::new(
                game.button_a_config,
                game.button_b_config,
                Point::new(game.prize_location.x + delta, game.prize_location.y + delta),
            )
        })
        .collect::<Vec<_>>();
    let tokens_corrected = games_corrected
        .iter()
        .map(|game| game.minimum_token_cost())
        .sum::<i64>();
    solution.set_part_b(tokens_corrected);

    solution
}
