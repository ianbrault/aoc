/*
** src/puzzles/aoc2021/day2.rs
*/

use super::Solution;
use crate::utils;

enum Direction {
    Forward,
    Up,
    Down,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "forward" => Self::Forward,
            "up" => Self::Up,
            "down" => Self::Down,
            _ => panic!("invalid direction: {}", s),
        }
    }
}

struct Command {
    direction: Direction,
    unit: u64,
}

impl From<&str> for Command {
    fn from(value: &str) -> Self {
        let (dir_str, unit_str) = utils::split(value, " ").unwrap();
        let direction = Direction::from(dir_str);
        let unit = unit_str.parse().unwrap();
        Self { direction, unit }
    }
}

pub struct Navigator {
    position: i64,
    depth: i64,
    aim: i64,
}

impl Navigator {
    fn new() -> Self {
        Self {
            position: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn handle_command(&mut self, command: &Command) {
        match command.direction {
            Direction::Forward => self.position += command.unit as i64,
            Direction::Up => self.depth -= command.unit as i64,
            Direction::Down => self.depth += command.unit as i64,
        }
    }

    fn handle_command_with_aim(&mut self, command: &Command) {
        match command.direction {
            Direction::Forward => {
                self.position += command.unit as i64;
                self.depth += self.aim * command.unit as i64;
            }
            Direction::Up => self.aim -= command.unit as i64,
            Direction::Down => self.aim += command.unit as i64,
        }
    }
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let commands = input.split('\n').map(Command::from).collect::<Vec<_>>();

    // Part A: What do you get if you multiply your final horizontal position by your final depth?
    let mut navigator = Navigator::new();
    for command in commands.iter() {
        navigator.handle_command(command);
    }
    solution.set_part_a(navigator.position * navigator.depth);

    // Part B: Using this new interpretation of the commands, calculate the horizontal position and
    // depth you would have after following the planned course. What do you get if you multiply
    // your final horizontal position by your final depth?
    let mut navigator = Navigator::new();
    for command in commands.iter() {
        navigator.handle_command_with_aim(command);
    }
    solution.set_part_b(navigator.position * navigator.depth);

    solution
}
