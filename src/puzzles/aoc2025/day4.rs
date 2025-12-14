/*
** src/puzzles/aoc2025/day4.rs
*/

use super::Solution;
use crate::types::Grid;

#[derive(Clone, PartialEq)]
enum State {
    Empty,
    Paper,
}

impl From<char> for State {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '@' => Self::Paper,
            _ => unreachable!(),
        }
    }
}

fn accessible_rolls(grid: &Grid<State>) -> Vec<(usize, usize)> {
    let mut rolls = Vec::new();
    for (i, j, state) in grid.iter_grid() {
        if state == &State::Paper {
            let adjacent_rolls = grid
                .neighbors_with_diagonal(i, j)
                .into_iter()
                .filter(|&(ii, jj)| grid.get(ii, jj) == &State::Paper)
                .count();
            if adjacent_rolls < 4 {
                rolls.push((i, j));
            }
        }
    }
    rolls
}

fn remove_rolls(grid: &mut Grid<State>, rolls: &[(usize, usize)]) {
    for &(i, j) in rolls {
        grid.set(i, j, State::Empty);
    }
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // The rolls of paper are arranged on a large grid
    let mut grid = input
        .split('\n')
        .map(|line| line.chars().map(State::from))
        .collect::<Grid<_>>();

    // Part A: How many rolls of paper can be accessed by a forklift?
    let rolls = accessible_rolls(&grid);
    solution.set_part_a(rolls.len());

    // Part B: How many rolls of paper in total can be removed by the Elves and their forklifts?
    let mut removed = rolls.len();
    let mut total = removed;
    remove_rolls(&mut grid, &rolls);
    while removed > 0 {
        let rolls = accessible_rolls(&grid);
        removed = rolls.len();
        total += removed;
        remove_rolls(&mut grid, &rolls);
    }
    solution.set_part_b(total);

    solution
}
