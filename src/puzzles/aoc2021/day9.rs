/*
** src/puzzles/aoc2021/day9.rs
*/

use super::Solution;
use crate::types::Grid;

use std::collections::{HashSet, VecDeque};

fn is_lowpoint(heightmap: &Grid<u32>, i: usize, j: usize) -> bool {
    let here = heightmap.get(i, j);
    heightmap
        .neighbors(i, j)
        .into_iter()
        .all(|(ii, jj)| heightmap.get(ii, jj) > here)
}

fn basin_size(heightmap: &Grid<u32>, i: usize, j: usize) -> usize {
    // points to be explored
    let mut frontier = VecDeque::new();
    // points already explored
    let mut explored = HashSet::new();

    // start with the given point
    frontier.push_back((i, j));

    while !frontier.is_empty() {
        // pop from the front of the frontier
        let (ii, jj) = frontier.pop_front().unwrap();
        // add unexplored neighbors to the frontier
        // note: exclude neighbors at the maximum height (9)
        for (iii, jjj) in heightmap.neighbors(ii, jj) {
            let v = *heightmap.get(iii, jjj);
            if !explored.contains(&(iii, jjj)) && v < 9 {
                frontier.push_back((iii, jjj));
            }
        }
        // add the current point to the explored set
        explored.insert((ii, jj));
    }

    explored.len()
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let heightmap = input
        .split('\n')
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()))
        .collect::<Grid<_>>();

    // Part A: Find all of the low points on your heightmap. What is the sum of the risk levels of
    // all low points on your heightmap?
    let lowpoints = heightmap
        .iter_grid()
        .filter(|&(i, j, _)| is_lowpoint(&heightmap, i, j))
        .collect::<Vec<_>>();
    let risk = lowpoints
        .iter()
        .map(|&(_, _, height)| height + 1)
        .sum::<u32>();
    solution.set_part_a(risk);

    // Part B: What do you get if you multiply together the sizes of the three largest basins?
    let mut basins = lowpoints
        .iter()
        .map(|&(i, j, _)| basin_size(&heightmap, i, j))
        .collect::<Vec<_>>();
    basins.sort();
    let size = basins.iter().rev().take(3).product::<usize>();
    solution.set_part_b(size);

    solution
}
