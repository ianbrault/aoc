/*
** src/types.rs
*/

use std::fmt;

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

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AOC {} puzzle {}", self.year, self.day + 1)
    }
}

pub enum Answer {
    Int(i64),
    UInt(u64),
    String(String),
}

impl From<i64> for Answer {
    fn from(n: i64) -> Self {
        Self::Int(n)
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

impl From<String> for Answer {
    fn from(n: String) -> Self {
        Self::String(n)
    }
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
}

pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new<T>(x: T, y: T) -> Self
    where
        T: Into<i64>,
    {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}
