/*
** src/puzzles/aoc2025/day1.rs
*/

use super::Solution;

fn parse_rotation(value: &str) -> i64 {
    let multiplier = match value.chars().next().unwrap() {
        'L' => -1,
        'R' => 1,
        _ => unreachable!(),
    };
    let distance = value[1..].parse::<i64>().unwrap();
    distance * multiplier
}

fn rotation_count(position: i64, distance: i64) -> i64 {
    let mut count = distance.abs() / 100;
    if position % 100 != 0 {
        let zero_distance = if distance.signum() != position.signum() {
            (position % 100).abs()
        } else {
            100 - (position % 100).abs()
        };
        if (distance % 100).abs() >= zero_distance {
            count += 1;
        }
    }
    count
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // The attached document contains a sequence of rotations, one per line, which tell you how to
    // open the safe.
    let rotations = input.split('\n').map(parse_rotation).collect::<Vec<_>>();

    // Part A: Analyze the rotations in your attached document. What's the actual password to open
    // the door?
    let mut position = 50;
    let mut zero_points = 0;
    for distance in rotations.iter() {
        position += distance;
        if position % 100 == 0 {
            zero_points += 1;
        }
    }
    solution.set_part_a(zero_points);

    // Part B: Using password method 0x434C49434B, what is the password to open the door?
    position = 50;
    let mut zero_passes = 0;
    for &distance in rotations.iter() {
        zero_passes += rotation_count(position, distance);
        position += distance;
    }
    solution.set_part_b(zero_passes);

    solution
}
