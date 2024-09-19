/*
** src/puzzles/mod.rs
*/

mod aoc2023;

use crate::puzzle_modules;
use crate::types::Puzzle;

puzzle_modules!(
    2023; aoc2023
);

pub struct PuzzleIterator {
    current_year: usize,
    end_year: usize,
    day: Option<usize>,
    current_day: usize,
}

impl PuzzleIterator {
    pub fn new(year: Option<usize>, day: Option<usize>) -> Self {
        let (current_year, end_year) = if let Some(year) = year {
            (year, year + 1)
        } else {
            (PuzzleModules::START_YEAR, PuzzleModules::END_YEAR)
        };
        Self {
            current_year,
            end_year,
            day,
            current_day: 0,
        }
    }
}

impl Iterator for PuzzleIterator {
    type Item = Puzzle;

    fn next(&mut self) -> Option<Self::Item> {
        if self.day.is_none() && self.current_day == PuzzleModules::puzzle_count(self.current_year)
        {
            self.current_year += 1;
            self.current_day = 0;
        }
        if self.current_year == self.end_year {
            return None;
        }

        let puzzle: Option<Self::Item>;
        if let Some(day) = self.day {
            puzzle = Some(Self::Item::new(self.current_year, day));
            self.current_year += 1;
        } else {
            puzzle = Some(Self::Item::new(self.current_year, self.current_day));
            self.current_day += 1;
        }

        puzzle
    }
}
