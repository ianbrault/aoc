/*
** src/puzzles/aoc2021/day11.rs
*/

use super::Solution;
use crate::types::Grid;

fn run_step(cells: &mut Grid<u32>) -> u64 {
    let mut flashes = 0;

    // first increment all energy levels by 1
    for (_, _, cell) in cells.iter_grid_mut() {
        *cell += 1;
    }
    // then handle all flashes
    while let Some((i, j)) = cells.find_with(|&cell| cell > 9) {
        flashes += 1;
        // set the energy level to 0
        cells.set(i, j, 0);
        // increment the energy level of all neighboring octopi
        for (ii, jj) in cells.neighbors_with_diagonal(i, j) {
            // do not increment if 0
            let cell = cells.get_mut(ii, jj);
            if *cell != 0 {
                *cell += 1;
            }
        }
    }

    flashes
}

fn run_steps(mut cells: Grid<u32>, n: usize) -> u64 {
    (0..n).map(|_| run_step(&mut cells)).sum()
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let energy_cells = input
        .split('\n')
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()))
        .collect::<Grid<_>>();

    // Part A: Given the starting energy levels of the dumbo octopuses in your cavern, simulate 100
    // steps. How many total flashes are there after 100 steps?
    let flashes = run_steps(energy_cells.clone(), 100);
    solution.set_part_a(flashes);

    // Part B: What is the first step during which all octopuses flash?
    let mut step = None;
    let mut cells = energy_cells.clone();
    let all_flash = (energy_cells.width * energy_cells.height) as u64;
    for i in 0..usize::MAX {
        let n = run_step(&mut cells);
        if n == all_flash {
            step = Some(i + 1);
            break;
        }
    }
    solution.maybe_set_part_b(step);

    solution
}
