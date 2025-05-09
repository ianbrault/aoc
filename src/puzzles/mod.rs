/*
** src/puzzles/mod.rs
*/

crate::puzzle_modules!(
    2020 => aoc2020,
    2021 => aoc2021,
    2022 => aoc2022,
    2023 => aoc2023,
    2024 => aoc2024
);

#[derive(Clone, Copy)]
pub struct Puzzle {
    pub year: usize,
    pub day: usize,
}

impl Puzzle {
    pub fn new(year: usize, day: usize) -> Self {
        Self { year, day }
    }
}

impl std::fmt::Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AOC {} puzzle {}", self.year, self.day + 1)
    }
}

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
            let years = PuzzleModules::years();
            (years[0], years[years.len() - 1] + 1)
        };
        Self {
            current_year,
            end_year,
            day: day.map(|d| d - 1),
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

pub enum Answer {
    Int(i64),
    UInt(u64),
    String(String),
}

impl From<i32> for Answer {
    fn from(n: i32) -> Self {
        Self::Int(n as i64)
    }
}

impl From<i64> for Answer {
    fn from(n: i64) -> Self {
        Self::Int(n)
    }
}

impl From<&i64> for Answer {
    fn from(n: &i64) -> Self {
        Self::Int(*n)
    }
}

impl From<u32> for Answer {
    fn from(n: u32) -> Self {
        Self::UInt(n as u64)
    }
}

impl From<u64> for Answer {
    fn from(n: u64) -> Self {
        Self::UInt(n)
    }
}

impl From<&u64> for Answer {
    fn from(n: &u64) -> Self {
        Self::UInt(*n)
    }
}

impl From<usize> for Answer {
    fn from(n: usize) -> Self {
        Self::UInt(n as u64)
    }
}

impl From<&usize> for Answer {
    fn from(n: &usize) -> Self {
        Self::UInt(*n as u64)
    }
}

impl From<String> for Answer {
    fn from(n: String) -> Self {
        Self::String(n)
    }
}

impl std::fmt::Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Int(x) => write!(f, "{}", x),
            Self::UInt(x) => write!(f, "{}", x),
            Self::String(x) => write!(f, "{}", x),
        }
    }
}

#[derive(Default)]
pub struct Solution {
    pub part_a: Option<Answer>,
    pub part_b: Option<Answer>,
}

impl Solution {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_part_a<T>(&mut self, answer: T)
    where
        T: Into<Answer>,
    {
        self.part_a = Some(answer.into());
    }

    pub fn set_part_b<T>(&mut self, answer: T)
    where
        T: Into<Answer>,
    {
        self.part_b = Some(answer.into());
    }

    pub fn maybe_set_part_a<T>(&mut self, answer: Option<T>)
    where
        T: Into<Answer>,
    {
        self.part_a = answer.map(|x| x.into());
    }

    #[allow(unused)]
    pub fn maybe_set_part_b<T>(&mut self, answer: Option<T>)
    where
        T: Into<Answer>,
    {
        self.part_b = answer.map(|x| x.into());
    }
}
