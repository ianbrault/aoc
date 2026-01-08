/*
** src/puzzles/aoc2025/day12.rs
*/

use super::Solution;
use crate::utils;

use nalgebra::Matrix3;

type Constraints = (usize, usize, Vec<usize>);

fn parse_input(input: String) -> (Vec<Matrix3<u8>>, Vec<Constraints>) {
    let parts = input.split("\n\n").collect::<Vec<_>>();

    let mut shapes = Vec::new();
    for chunk in parts.iter().take(parts.len() - 1) {
        let shape = chunk
            .split('\n')
            .skip(1)
            .flat_map(|line| {
                line.chars().map(|c| match c {
                    '.' => 0,
                    '#' => 1,
                    _ => unreachable!(),
                })
            })
            .collect::<Vec<u8>>();
        shapes.push(Matrix3::from_row_iterator(shape));
    }

    let mut constraints = Vec::new();
    for line in parts.last().unwrap().split('\n') {
        let (size, shapes) = utils::split(line, ": ").unwrap();
        let (width, height) = utils::split(size, "x").unwrap();
        let shape_list = shapes.split(' ').map(|n| n.parse().unwrap()).collect();
        constraints.push((width.parse().unwrap(), height.parse().unwrap(), shape_list));
    }

    (shapes, constraints)
}

fn tile_count(shape: &Matrix3<u8>) -> usize {
    shape.iter().map(|n| *n as usize).sum()
}

fn check_region(constraint: &Constraints, shapes: &[Matrix3<u8>]) -> bool {
    let (width, height, shape_list) = constraint;

    // Shape is guaranteed to fit if the full bounding boxes all fit together
    let shape_count = shape_list.iter().sum::<usize>();
    if shape_count < (width / 3) * (height / 3) {
        return true;
    }

    // Shape is guaranteed to not fit if the total tiles in the shape are greater than the area
    let tiles = shapes
        .iter()
        .enumerate()
        .map(|(i, shape)| tile_count(shape) * shape_list[i])
        .sum::<usize>();
    if tiles > width * height {
        return false;
    }

    true
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // The Elves have a summary of the situation for you. First, it contains a list of the
    // presents' shapes. Second, it contains the size of the region under each tree and a list of
    // the number of presents of each shape that need to fit into that region.
    let (shapes, constraints) = parse_input(input);

    // Part A: Consider the regions beneath each tree and the presents the Elves would like to fit
    // into each of them. How many of the regions can fit all of the presents listed?
    let regions = constraints
        .iter()
        .filter(|constraint| check_region(constraint, &shapes))
        .count();
    solution.set_part_a(regions);

    // Part B: 2025 complete!

    solution
}
