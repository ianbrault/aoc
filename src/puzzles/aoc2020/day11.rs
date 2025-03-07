/*
** src/puzzles/aoc2020/day11.rs
*/

use super::Solution;
use crate::types::{Direction, Grid};

#[derive(Clone, Copy, Default)]
enum State {
    #[default]
    Floor,
    Empty,
    Occupied,
}

impl State {
    fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    fn is_occupied(&self) -> bool {
        matches!(self, Self::Occupied)
    }

    fn occupied(&self) -> usize {
        match self {
            Self::Occupied => 1,
            _ => 0,
        }
    }
}

impl From<char> for State {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Floor,
            'L' => Self::Empty,
            '#' => Self::Occupied,
            _ => unreachable!(),
        }
    }
}

enum Visibility {
    Adjacent,
    LineOfSight,
    NotSet,
}

// the ferry seating is a cellular automaton
//
// the rule is:
// if a seat is empty and there are no occupied seats in the neighborhood, the
// seat becomes occupied; if a seat is occupied and 4 or more seats in the
// neighborhood are also occupied, the seat becomes empty; otherwise, no change
struct FerryAutomaton {
    grid: Grid<State>,
    // rule configuration
    visibility: Visibility,
    occupied_threshold: usize,
}

impl FerryAutomaton {
    fn with(mut self, visibility: Visibility, occupied_threshold: usize) -> Self {
        self.visibility = visibility;
        self.occupied_threshold = occupied_threshold;
        self
    }

    fn occupied_adjacent(&self, i: usize, j: usize) -> usize {
        self.grid
            .neighbors_with_diagonal(i, j)
            .into_iter()
            .map(|(i, j)| self.grid.get(i, j).occupied())
            .sum()
    }

    fn check_line_of_sight(&self, mut i: usize, mut j: usize, direction: Direction) -> bool {
        while let Some((ii, jj)) = self.grid.neighbor(i, j, direction) {
            match self.grid.get(ii, jj) {
                State::Occupied => return true,
                State::Empty => return false,
                _ => {
                    i = ii;
                    j = jj;
                }
            }
        }
        false
    }

    fn occupied_line_of_sight(&self, i: usize, j: usize) -> usize {
        Direction::all_cases()
            .map(|direction| self.check_line_of_sight(i, j, direction))
            .filter(|&found| found)
            .count()
    }

    fn visible_occupied(&self, row: usize, col: usize) -> usize {
        match self.visibility {
            Visibility::Adjacent => self.occupied_adjacent(row, col),
            Visibility::LineOfSight => self.occupied_line_of_sight(row, col),
            _ => unreachable!(),
        }
    }

    // creates the next generation of the automaton by applying the rule to the
    // current generation; returns the number of cells that changed state
    fn run(&mut self) -> usize {
        let mut next = self.grid.clone();
        let mut changed = 0;
        // iterate thru the grid, accounting for the padding along the borders
        for (i, j, state) in self.grid.iter_grid() {
            // note: save a bit by not checking adjacencies for the floor
            let next_state = if state.is_empty() && self.visible_occupied(i, j) == 0 {
                changed += 1;
                State::Occupied
            } else if state.is_occupied() && self.visible_occupied(i, j) >= self.occupied_threshold
            {
                changed += 1;
                State::Empty
            } else {
                *state
            };
            next.set(i, j, next_state);
        }
        self.grid = next;
        changed
    }

    fn run_to_completion(&mut self) {
        let mut changed = usize::MAX;
        while changed > 0 {
            changed = self.run();
        }
    }

    fn occupied_seats(&self) -> u64 {
        self.grid
            .iter_grid()
            .filter(|(_, _, cell)| cell.is_occupied())
            .count() as u64
    }
}

impl From<&str> for FerryAutomaton {
    fn from(value: &str) -> Self {
        let grid = value
            .split('\n')
            .map(|line| line.chars().map(State::from))
            .collect::<Grid<_>>();
        Self {
            grid,
            // default options, call Self::with() afterwards
            visibility: Visibility::NotSet,
            occupied_threshold: 0,
        }
    }
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();

    // Part A: Simulate your seating area by applying the seating rules repeatedly until no seats
    // change state. How many seats end up occupied?
    let mut automaton = FerryAutomaton::from(input.as_str()).with(Visibility::Adjacent, 4);
    automaton.run_to_completion();
    solution.set_part_a(automaton.occupied_seats());

    // Part B: Given the new visibility method and the rule change for occupied seats becoming
    // empty, once equilibrium is reached, how many seats end up occupied?
    let mut automaton = FerryAutomaton::from(input.as_str()).with(Visibility::LineOfSight, 5);
    automaton.run_to_completion();
    solution.set_part_b(automaton.occupied_seats());

    solution
}
