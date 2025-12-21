/*
** src/puzzles/aoc2025/day7.rs
*/

use super::Solution;
use crate::types::Point;

use std::collections::{HashMap, HashSet};

struct Manifold {
    start: Point,
    splitters: HashSet<Point>,
    height: i64,
}

impl From<String> for Manifold {
    fn from(value: String) -> Self {
        let mut start = None;
        let mut splitters = HashSet::new();
        let mut height = 0;
        for (i, line) in value.split('\n').enumerate() {
            height += 1;
            for (j, c) in line.chars().enumerate() {
                if c == 'S' {
                    start = Some(Point::new(j as i64, i as i64));
                } else if c == '^' {
                    splitters.insert(Point::new(j as i64, i as i64));
                }
            }
        }
        Self {
            start: start.unwrap(),
            splitters,
            height,
        }
    }
}

fn beam_splits(manifold: &Manifold) -> usize {
    let mut beams = HashSet::new();
    beams.insert(manifold.start);

    let mut splits = 0;
    for _ in manifold.start.y..manifold.height {
        let mut next_beams = HashSet::new();
        for position in beams.into_iter() {
            let next = Point::new(position.x, position.y + 1);
            // Check for a splitter
            if manifold.splitters.contains(&next) {
                splits += 1;
                next_beams.insert(Point::new(next.x - 1, next.y));
                next_beams.insert(Point::new(next.x + 1, next.y));
            } else {
                next_beams.insert(next);
            }
        }
        beams = next_beams;
    }
    splits
}

fn beam_quantum_splits_recursive(
    manifold: &Manifold,
    position: Point,
    memo: &mut HashMap<Point, usize>,
) -> usize {
    if position.y + 1 == manifold.height {
        return 1;
    }
    if memo.contains_key(&position) {
        return memo[&position];
    }
    let next = Point::new(position.x, position.y + 1);
    if manifold.splitters.contains(&next) {
        let left = Point::new(next.x - 1, next.y);
        let left_splits = beam_quantum_splits_recursive(manifold, left, memo);
        memo.insert(left, left_splits);

        let right = Point::new(next.x + 1, next.y);
        let right_splits = beam_quantum_splits_recursive(manifold, right, memo);
        memo.insert(right, right_splits);

        left_splits + right_splits
    } else {
        let splits = beam_quantum_splits_recursive(manifold, next, memo);
        memo.insert(next, splits);
        splits
    }
}

fn beam_quantum_splits(manifold: &Manifold) -> usize {
    let mut memo = HashMap::new();
    beam_quantum_splits_recursive(manifold, manifold.start, &mut memo)
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // You quickly locate a diagram of the tachyon manifold. A tachyon beam enters the manifold and
    // always move downward. Tachyon beams pass freely through empty space but if a tachyon beam
    // encounters a splitter, the beam is stopped; instead, a new tachyon beam continues from the
    // immediate left and right of the splitter.
    let manifold = Manifold::from(input);

    // Part A: How many times will the beam be split?
    let splits = beam_splits(&manifold);
    solution.set_part_a(splits);

    // Part B: Apply the many-worlds interpretation of quantum tachyon splitting to your manifold
    // diagram. In total, how many different timelines would a single tachyon particle end up on?
    let quantum_splits = beam_quantum_splits(&manifold);
    solution.set_part_b(quantum_splits);

    solution
}
