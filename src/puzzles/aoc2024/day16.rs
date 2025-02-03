/*
** src/puzzles/aoc2024/day16.rs
*/

use super::Solution;
use crate::types::{Direction, Grid};

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Clone, Eq, PartialEq)]
struct SearchState {
    position: (usize, usize),
    direction: Direction,
    cost: usize,
}

impl SearchState {
    fn new(position: (usize, usize), direction: Direction, cost: usize) -> Self {
        Self {
            position,
            direction,
            cost,
        }
    }
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        // reverse the ordering on costs for priority queue
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct DistanceMatrix(HashMap<((usize, usize), Direction), usize>);

impl DistanceMatrix {
    fn get(&self, point: (usize, usize)) -> Option<usize> {
        Direction::cardinal().filter_map(|direction| self.0.get(&(point, direction)).copied()).min()
    }
}

fn maze_distance_matrix(grid: &Grid<char>, initial: Vec<((usize, usize), Direction)>) -> DistanceMatrix {
    let mut distances = HashMap::new();
    let mut heap = BinaryHeap::new();

    for (point, direction) in initial {
        distances.insert((point, direction), 0);
        heap.push(SearchState::new(point, direction, 0));
    }

    while let Some(SearchState {
        position,
        direction,
        cost,
    }) = heap.pop()
    {
        if distances[&(position, direction)] < cost {
            continue;
        }
        for next_direction in Direction::cardinal() {
            if next_direction == direction {
                continue;
            }
            let next_distance = distances.entry((position, next_direction)).or_insert(usize::MAX);
            if *next_distance > cost + 1000 {
                *next_distance = cost + 1000;
                heap.push(SearchState::new(position, next_direction, cost + 1000));
            }
        }
        if let Some(next) = grid.neighbor(position.0, position.1, direction) {
            let next_distance = distances.entry((next, direction)).or_insert(usize::MAX);
            if grid.get(next.0, next.1) != &'#' && *next_distance > cost + 1 {
                *next_distance = cost + 1;
                heap.push(SearchState::new(next, direction, cost + 1));
            }
        }
    }
    DistanceMatrix(distances)
}

fn maze_best_paths(grid: &Grid<char>, distances_from_start: DistanceMatrix) -> Option<usize> {
    let end = grid.find(&'E')?;
    let distance_to_end = distances_from_start.get(end)?;

    let initial = Direction::cardinal().map(|direction| (end, direction)).collect::<Vec<_>>();
    let distances_from_end = maze_distance_matrix(grid, initial);

    let mut points = HashSet::new();
    for (i, j, _) in grid.iter_grid() {
        let point = (i, j);
        for direction in Direction::cardinal() {
            let flip_direction = direction.turn_90_clockwise().turn_90_clockwise();
            let distance_from_start = distances_from_start.0.get(&(point, direction)).cloned().unwrap_or(usize::MAX);
            let distance_from_end = distances_from_end.0.get(&(point, flip_direction)).cloned().unwrap_or(usize::MAX);
            if distance_from_start < usize::MAX && distance_from_end < usize::MAX && distance_from_start + distance_from_end == distance_to_end {
                points.insert(point);
            }
        }
    }
    Some(points.len())
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // The Reindeer start on the Start Tile facing East and need to reach the End Tile. They can
    // move forward one tile at a time (increasing their score by 1), but never into a wall. They
    // can also rotate clockwise or counterclockwise 90 degrees at a time (increasing their score
    // by 1000). To figure out the best place to sit, you grab a map from a nearby kiosk.
    let grid = Grid::from(input);
    let start = grid.find(&'S').unwrap();
    let end = grid.find(&'E').unwrap();
    let initial = vec![(start, Direction::East)];

    // Part A: Analyze your map carefully. What is the lowest score a Reindeer could possibly get?
    let distances = maze_distance_matrix(&grid, initial);
    solution.maybe_set_part_a(distances.get(end));

    // Part B: Analyze your map further. How many tiles are part of at least one of the best paths
    // through the maze?
    // let tiles = maze_best_paths(&grid, distances);
    let tiles = maze_best_paths(&grid, distances);
    solution.maybe_set_part_b(tiles);

    solution
}
