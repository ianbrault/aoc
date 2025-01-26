/*
** src/puzzles/aoc2024/day14.rs
*/

use super::Solution;
use crate::types::Point;
use crate::utils;

use log::debug;

use std::collections::HashSet;

const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;

#[derive(Clone)]
struct Robot {
    position: Point,
    velocity: Point,
}

impl Robot {
    fn move_once(&mut self) {
        let mut position = self.position + self.velocity;
        if position.x >= WIDTH {
            position.x %= WIDTH;
        } else if position.x < 0 {
            position.x += WIDTH;
        }
        if position.y >= HEIGHT {
            position.y %= HEIGHT;
        } else if position.y < 0 {
            position.y += HEIGHT;
        }
        self.position = position;
    }

    fn move_over_time(&mut self, time: usize) {
        debug!("robot: p={} v={}", self.position, self.velocity);
        for _ in 0..time {
            self.move_once();
            debug!("moved to {}", self.position);
        }
    }
}

impl From<&str> for Robot {
    fn from(value: &str) -> Self {
        let (pos_str_full, vel_str_full) = utils::split(value, " ").unwrap();
        let pos_str = utils::split_tail(pos_str_full, "=").unwrap();
        let vel_str = utils::split_tail(vel_str_full, "=").unwrap();
        let position = Point::try_from(pos_str).unwrap();
        let velocity = Point::try_from(vel_str).unwrap();
        Self { position, velocity }
    }
}

fn safety_factor(robots: &[Robot]) -> usize {
    let quad_a = robots
        .iter()
        .filter(|robot| robot.position.x < WIDTH / 2 && robot.position.y < HEIGHT / 2)
        .count();
    let quad_b = robots
        .iter()
        .filter(|robot| robot.position.x > WIDTH / 2 && robot.position.y < HEIGHT / 2)
        .count();
    let quad_c = robots
        .iter()
        .filter(|robot| robot.position.x < WIDTH / 2 && robot.position.y > HEIGHT / 2)
        .count();
    let quad_d = robots
        .iter()
        .filter(|robot| robot.position.x > WIDTH / 2 && robot.position.y > HEIGHT / 2)
        .count();
    quad_a * quad_b * quad_c * quad_d
}

fn robots_in_unique_positions(robots: &[Robot]) -> bool {
    let mut positions = HashSet::new();
    for robot in robots {
        if positions.contains(&robot.position) {
            return false;
        }
        positions.insert(robot.position);
    }
    true
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // The area outside the bathroom is swarming with robots! You make a list of all of the robots'
    // current positions and velocities, one robot per line.
    let mut robots = input.split('\n').map(Robot::from).collect::<Vec<_>>();

    // Part A: Predict the motion of the robots in your list within a space which is 101 tiles wide
    // and 103 tiles tall. What will the safety factor be after exactly 100 seconds have elapsed?
    for robot in robots.iter_mut() {
        robot.move_over_time(100);
    }
    let safety = safety_factor(&robots);
    solution.set_part_a(safety);

    // Part B: What is the fewest number of seconds that must elapse for the robots to display the
    // Easter egg?
    // move until the robots are all in unique positions
    let mut elapsed = 100;
    while !robots_in_unique_positions(&robots) {
        for robot in robots.iter_mut() {
            robot.move_once();
        }
        elapsed += 1;
    }
    solution.set_part_b(elapsed);

    solution
}
