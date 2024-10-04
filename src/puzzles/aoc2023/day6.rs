/*
** src/puzzles/aoc2023/day6.rs
*/

use super::Solution;
use crate::utils;

fn parse_records(input: &str) -> Vec<(u64, u64)> {
    match input.split('\n').collect::<Vec<_>>().as_slice() {
        &[time_line, distance_line] => {
            let time_list = utils::split_tail(time_line, ": ").unwrap();
            let distance_list = utils::split_tail(distance_line, ": ").unwrap();
            utils::split_and_parse(time_list)
                .zip(utils::split_and_parse(distance_list))
                .collect()
        }
        _ => panic!("parse_records: invalid input: {}", input),
    }
}

fn parse_records_combined(input: &str) -> (u64, u64) {
    match input.split('\n').collect::<Vec<_>>().as_slice() {
        &[time_line, distance_line] => {
            let time_list = utils::split_tail(time_line, ": ").unwrap();
            let distance_list = utils::split_tail(distance_line, ": ").unwrap();

            let time_squashed = time_list
                .split_ascii_whitespace()
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join("");
            let distance_squashed = distance_list
                .split_ascii_whitespace()
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join("");

            let time = time_squashed.parse().unwrap();
            let distance = distance_squashed.parse().unwrap();
            (time, distance)
        }
        _ => panic!("parse_records: invalid input: {}", input),
    }
}

fn quadratic(a: f64, b: f64, c: f64) -> (f64, f64) {
    let root = (b.powf(2.0) - (4.0 * a * c)).sqrt();
    let x = (-b + root) / (2.0 * a);
    let y = (-b - root) / (2.0 * a);
    (x, y)
}

fn solution_count(time_limit: u64, distance_record: u64) -> u64 {
    // distance = velocity * time
    // velocity is time pressed, and the time will be the time limit minus the time pressed
    // and we are interested in clearing the distance record so this becomes:
    // v * t > dR => t * (tL - t) > dR
    // which can be expressed as a quadratic with a variable time pressed
    // -t^2 + tL*t -dR > 0
    // which is an upside-down parabola, so find the solutions and grab all integers between there
    let (mut x, mut y) = quadratic(-1.0, time_limit as f64, -1.0 * distance_record as f64);
    if x.fract() != 0.0 {
        x = x.ceil()
    }
    if y.fract() != 0.0 {
        y = y.ceil()
    } else {
        y -= 1.0;
    }
    y as u64 - x as u64
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // Your puzzle input lists the time allowed for each race and also the best distance ever
    // recorded in that race. To guarantee you win the grand prize, you need to make sure you go
    // farther in each race than the current record holder.
    let records = parse_records(&input);

    // Part A: Determine the number of ways you could beat the record in each race. What do you get
    // if you multiply these numbers together?
    let solution_product = records
        .iter()
        .map(|&(time, distance)| solution_count(time, distance))
        .product::<u64>();
    solution.set_part_a(solution_product);

    // Part B: You realize the piece of paper with race times and record distances you got earlier
    // actually just has very bad kerning. There's really only one race: ignore the spaces between
    // the numbers on each line. How many ways can you beat the record in this one much longer race?
    let (time_limit, distance_record) = parse_records_combined(&input);
    let solution_product_combined = solution_count(time_limit, distance_record);
    solution.set_part_b(solution_product_combined);

    solution
}
