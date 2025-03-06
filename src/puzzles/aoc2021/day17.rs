/*
** src/puzzles/aoc2021/day17.rs
*/

use super::Solution;
use crate::utils;

use std::cmp;
use std::ops::Range;

fn launch_probe(x_range: &Range<i64>, y_range: &Range<i64>, vx: i64, vy: i64) -> bool {
    // does the probe, when launched at the given velocity, land within the target area?
    let mut x = 0;
    let mut y = 0;
    let mut vx = vx;
    let mut vy = vy;

    while x <= x_range.end && y >= y_range.end {
        x += vx;
        y += vy;

        if vx > 0 {
            vx -= 1;
        } else {
            // extra check to terminate early
            if x == 0 && !x_range.contains(&x) {
                return false;
            }
        }
        vy -= 1;
    }

    x_range.contains(&x) && y_range.contains(&y)
}

fn max_y(x_range: &Range<i64>, y_range: &Range<i64>, vx: i64, vy: i64) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut vx = vx;
    let mut vy = vy;

    let mut max_y = 0;
    while x <= x_range.end && y >= y_range.end {
        x += vx;
        y += vy;
        max_y = cmp::max(y, max_y);

        if vx > 0 {
            vx -= 1;
        }
        vy -= 1;
    }

    max_y
}

fn find_highest_y(x_range: &Range<i64>, y_range: &Range<i64>) -> i64 {
    // just brute-force it: initial vx and vy must be positive
    let mut y_max = 0;
    for vx in 1..=x_range.end {
        for vy in 1..=1000 {
            if launch_probe(x_range, y_range, vx, vy) {
                y_max = cmp::max(y_max, max_y(x_range, y_range, vx, vy));
            }
        }
    }
    y_max
}

fn possible_initial_velocities(x_range: &Range<i64>, y_range: &Range<i64>) -> usize {
    // just brute-force it: initial vx must be positive
    let mut count = 0;
    for vx in 1..=x_range.end {
        for vy in y_range.start..=1000 {
            if launch_probe(x_range, y_range, vx, vy) {
                count += 1
            }
        }
    }
    count
}

fn parse_range(s: &str) -> Range<i64> {
    let (start, end) = utils::split(&s[2..], "..").unwrap();
    start.parse().unwrap()..(end.parse::<i64>().unwrap() + 1)
}

fn parse_input(input: String) -> (Range<i64>, Range<i64>) {
    let ranges = utils::split_tail(input.as_str(), ": ").unwrap();
    let (x, y) = utils::split(ranges, ", ").unwrap();
    (parse_range(x), parse_range(y))
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let (x_range, y_range) = parse_input(input);

    // Part A: Find the initial velocity that causes the probe to reach the highest y position and
    // still eventually be within the target area after any step. What is the highest y position it
    // reaches on this trajectory?
    let y = find_highest_y(&x_range, &y_range);
    solution.set_part_a(y);

    // Part B: How many distinct initial velocity values cause the probe to be within the target
    // area after any step?
    let velocities = possible_initial_velocities(&x_range, &y_range);
    solution.set_part_b(velocities);

    solution
}
