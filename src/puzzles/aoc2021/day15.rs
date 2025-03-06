/*
** src/puzzles/aoc2021/day15.rs
*/

use super::Solution;
use crate::types::Grid;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Clone, Copy, Eq, PartialEq)]
struct CoordDistance {
    coord: (usize, usize),
    distance: u64,
}

impl CoordDistance {
    fn new(coord: (usize, usize), distance: u64) -> Self {
        Self { coord, distance }
    }
}

impl Ord for CoordDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| self.coord.cmp(&other.coord))
    }
}

impl PartialOrd for CoordDistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn build_full_cave(cave: &Grid<u32>) -> Grid<u32> {
    let mut full = Grid::new(cave.width * 5, cave.height * 5);

    for row in 0..5 {
        let row_offset = row * cave.height;
        for col in 0..5 {
            let col_offset = col * cave.width;
            for i in 0..cave.height {
                for j in 0..cave.width {
                    let original = cave.get(i, j);
                    let new = original + row as u32 + col as u32;
                    if new > 9 {
                        full.set(row_offset + i, col_offset + j, new % 9);
                    } else {
                        full.set(row_offset + i, col_offset + j, new);
                    }
                }
            }
        }
    }

    full
}

fn lowest_risk_path(cave: &Grid<u32>) -> u64 {
    // implementation of Djikstra's algorithm to find the lowest-risk (i.e. shortest) path between
    // the start and endpoint of the cave
    // assign distance 0 for the origin and infinity for all other nodes
    let mut distances = Grid::from(vec![vec![u64::MAX; cave.width]; cave.height]);
    distances.set(0, 0, 0);

    // easily select the next node
    let mut distance_heap = BinaryHeap::new();
    distance_heap.push(CoordDistance::new((0, 0), 0));

    while let Some(CoordDistance { coord, distance }) = distance_heap.pop() {
        let (i, j) = coord;
        // skip if we have already found a shorter distance to this coordinate
        if distance <= *distances.get(i, j) {
            // consider all neighbors
            for (ii, jj) in cave.neighbors(i, j) {
                let tmp_distance = distance + *cave.get(ii, jj) as u64;
                if tmp_distance < *distances.get(ii, jj) {
                    distance_heap.push(CoordDistance::new((ii, jj), tmp_distance));
                    distances.set(ii, jj, tmp_distance);
                }
            }
        }
    }

    *distances.get(cave.height - 1, cave.width - 1)
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let cave = input
        .split('\n')
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()))
        .collect::<Grid<_>>();
    let full_cave = build_full_cave(&cave);

    // Part A: What is the lowest total risk of any path from the top left to the bottom right?
    let risk = lowest_risk_path(&cave);
    solution.set_part_a(risk);

    // Part B: Using the full map, what is the lowest total risk of any path from the top left to
    // the bottom right?
    let risk = lowest_risk_path(&full_cave);
    solution.set_part_b(risk);

    solution
}
