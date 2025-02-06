/*
** src/puzzles/aoc2024/day18.rs
*/

use super::Solution;
use crate::types::{Grid, Point};

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Clone, Eq, PartialEq)]
struct SearchState {
    position: (usize, usize),
    cost: usize,
}

impl SearchState {
    fn new(position: (usize, usize), cost: usize) -> Self {
        Self { position, cost }
    }
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
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

fn drop_bytes(grid: &mut Grid<bool>, bytes: &[Point]) {
    for point in bytes {
        grid.set(point.y as usize, point.x as usize, true);
    }
}

fn distance_matrix(grid: &Grid<bool>) -> HashMap<(usize, usize), usize> {
    let mut distances = HashMap::new();
    let mut heap = BinaryHeap::new();

    distances.insert((0, 0), 0);
    heap.push(SearchState::new((0, 0), 0));

    while let Some(SearchState { position, cost }) = heap.pop() {
        let distance = distances.entry(position).or_insert(usize::MAX);
        if *distance < cost {
            continue;
        }
        for neighbor in grid.neighbors(position.0, position.1) {
            if *grid.get(neighbor.0, neighbor.1) {
                continue;
            }
            let next_distance = distances.entry(neighbor).or_insert(usize::MAX);
            if *next_distance > cost + 1 {
                *next_distance = cost + 1;
                heap.push(SearchState::new(neighbor, cost + 1));
            }
        }
    }
    distances
}

fn find_blocking_byte(grid: Grid<bool>, bytes: &[Point]) -> Option<String> {
    let exit = (grid.height - 1, grid.width - 1);

    let mut a = 0;
    let mut b = bytes.len() - 1;
    let mut last_blocked = None;

    while a + 1 < b {
        let c = a + ((b - a) / 2);
        let mut c_grid = grid.clone();
        drop_bytes(&mut c_grid, &bytes[..=c]);

        let distances = distance_matrix(&c_grid);
        if distances.get(&exit).unwrap_or(&usize::MAX) == &usize::MAX {
            b = c;
            last_blocked = Some(c);
        } else {
            a = c;
        }
    }

    last_blocked.map(|i| format!("{}", bytes[i]))
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let falling_bytes = input
        .split('\n')
        .map(|line| Point::try_from(line).unwrap())
        .collect::<Vec<_>>();
    let mut grid = Grid::<bool>::new(71, 71);
    let exit = (grid.height - 1, grid.width - 1);

    // Part A: Simulate the first kilobyte (1024 bytes) falling onto your memory space. Afterward,
    // what is the minimum number of steps needed to reach the exit?
    drop_bytes(&mut grid, &falling_bytes[..1024]);
    let distances = distance_matrix(&grid);
    let steps_to_exit = distances.get(&exit);
    solution.maybe_set_part_a(steps_to_exit);

    // Part B: Simulate more of the bytes that are about to corrupt your memory space. What are the
    // coordinates of the first byte that will prevent the exit from being reachable from your
    // starting position?
    let blocker = find_blocking_byte(grid, &falling_bytes[1024..]);
    solution.maybe_set_part_b(blocker);

    let (x, _) = falling_bytes[1024..]
        .iter()
        .enumerate()
        .find(|(_, p)| p.x == 60 && p.y == 21)
        .unwrap();
    dbg!(x);

    solution
}
