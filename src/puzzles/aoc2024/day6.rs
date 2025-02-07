/*
** src/puzzles/aoc2024/day6.rs
*/

use super::Solution;
use crate::types::{Direction, Grid};

use std::collections::HashSet;

fn positions_visited(map: &Grid<char>) -> HashSet<(usize, usize)> {
    let mut position = map.find(&'^').unwrap();
    let mut direction = Direction::North;

    let mut positions = HashSet::new();
    while let Some(new_position) = map.neighbor(position.0, position.1, direction) {
        if *map.get(new_position.0, new_position.1) == '#' {
            direction = direction.turn_90_clockwise();
        } else {
            position = new_position;
            positions.insert(position);
        }
    }
    positions
}

fn obstruction_creates_loop(
    map: &Grid<char>,
    start: (usize, usize),
    obstruction: (usize, usize),
) -> bool {
    let mut map = map.clone();
    map.set(obstruction.0, obstruction.1, '#');

    let mut position = start;
    let mut direction = Direction::North;
    let mut obstructions_hit = HashSet::new();

    while let Some(new_position) = map.neighbor(position.0, position.1, direction) {
        if *map.get(new_position.0, new_position.1) == '#' {
            // barrier hit, check if it has been hit in this direction before
            if obstructions_hit.contains(&(new_position, direction)) {
                return true;
            } else {
                obstructions_hit.insert((new_position, direction));
                direction = direction.turn_90_clockwise();
            }
        } else {
            position = new_position;
        }
    }
    false
}

fn find_loop_obstructions(map: &Grid<char>, path: HashSet<(usize, usize)>) -> usize {
    let mut count = 0;
    let start = map.find(&'^').unwrap();
    for &point in path.iter() {
        if point != start && obstruction_creates_loop(map, start, point) {
            count += 1;
        }
    }
    count
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let map = Grid::from(input);

    // Part A: Predict the path of the guard. How many distinct positions will the guard visit
    // before leaving the mapped area?
    let path = positions_visited(&map);
    solution.set_part_a(path.len());

    // Part B: You need to get the guard stuck in a loop by adding a single new obstruction.
    // How many different positions could you choose for this obstruction?
    let obstructions = find_loop_obstructions(&map, path);
    solution.set_part_b(obstructions);

    solution
}
