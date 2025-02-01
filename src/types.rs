/*
** src/types.rs
*/

use super::utils;

use anyhow::Error;

use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::Add;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Direction {
    pub fn grid_delta(&self) -> (i64, i64) {
        match self {
            Self::North => (-1, 0),
            Self::South => (1, 0),
            Self::East => (0, 1),
            Self::West => (0, -1),
            Self::NorthEast => (-1, 1),
            Self::NorthWest => (-1, -1),
            Self::SouthEast => (1, 1),
            Self::SouthWest => (1, -1),
        }
    }

    pub fn turn_90_clockwise(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
            Self::NorthEast => Self::SouthEast,
            Self::NorthWest => Self::NorthEast,
            Self::SouthEast => Self::SouthWest,
            Self::SouthWest => Self::NorthWest,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn new_for_grid(i: usize, j: usize) -> Self {
        Self {
            x: i as i64,
            y: j as i64,
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl TryFrom<&str> for Point {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some((x_str, y_str)) = utils::split(value, ",") {
            let x = x_str.parse::<i64>()?;
            let y = y_str.parse::<i64>()?;
            Result::Ok(Self::new(x, y))
        } else {
            Result::Err(Error::msg("missing comma"))
        }
    }
}

pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    inner: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self
    where
        T: Clone + Default,
    {
        let inner = vec![vec![T::default(); width]; height];
        Self {
            width,
            height,
            inner,
        }
    }

    pub fn get(&self, i: usize, j: usize) -> &T {
        let row = self.inner.get(i).unwrap_or_else(|| panic!(
            "Grid::get: index out of bounds: i: {}: size: {}x{}",
            i, self.width, self.height
        ));
        row.get(j).unwrap_or_else(|| panic!(
            "Grid::get: index out of bounds: j: {}: size: {}x{}",
            j, self.width, self.height
        ))
    }

    pub fn get_mut(&mut self, i: usize, j: usize) -> &mut T {
        let row = self.inner.get_mut(i).unwrap_or_else(|| panic!(
            "Grid::get_mut: index out of bounds: i: {}: size: {}x{}",
            i, self.width, self.height
        ));
        row.get_mut(j).unwrap_or_else(|| panic!(
            "Grid::get_mut: index out of bounds: j: {}: size: {}x{}",
            j, self.width, self.height
        ))
    }

    pub fn set(&mut self, i: usize, j: usize, value: T) {
        let row = self.inner.get_mut(i).unwrap_or_else(|| panic!(
            "Grid::set: index out of bounds: i: {}: size: {}x{}",
            i, self.width, self.height
        ));
        let col = row.get_mut(j).unwrap_or_else(|| panic!(
            "Grid::set: index out of bounds: j: {}: size: {}x{}",
            j, self.width, self.height
        ));
        *col = value;
    }

    pub fn neighbor(&self, i: usize, j: usize, direction: Direction) -> Option<(usize, usize)> {
        let (di, dj) = direction.grid_delta();
        let point = Point::new_for_grid(i, j) + Point::new(di, dj);
        if point.x >= 0
            && point.y >= 0
            && point.x < self.height as i64
            && point.y < self.width as i64
        {
            Some((point.x as usize, point.y as usize))
        } else {
            None
        }
    }

    pub fn iter_row(&self, i: usize) -> impl Iterator<Item = &T> {
        self.inner[i].iter()
    }

    pub fn iter_col(&self, j: usize) -> impl Iterator<Item = &T> {
        (0..self.height).map(move |i| &self.inner[i][j])
    }

    pub fn iter_grid(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.inner
            .iter()
            .enumerate()
            .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, item)| (i, j, item)))
    }

    pub fn find(&self, element: &T) -> Option<(usize, usize)>
    where
        T: PartialEq,
    {
        for (i, j, x) in self.iter_grid() {
            if x == element {
                return Some((i, j));
            }
        }
        None
    }

    pub fn find_all(&self, element: &T) -> Vec<(usize, usize)>
    where
        T: PartialEq,
    {
        let mut indices = Vec::new();
        for (i, j, x) in self.iter_grid() {
            if x == element {
                indices.push((i, j));
            }
        }
        indices
    }

    pub fn find_all_with<P>(&self, predicate: P) -> Vec<(usize, usize)>
    where
        P: Fn(&T) -> bool,
    {
        let mut indices = Vec::new();
        for (i, j, x) in self.iter_grid() {
            if predicate(x) {
                indices.push((i, j));
            }
        }
        indices
    }
}

impl<T> Clone for Grid<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            width: self.width,
            height: self.height,
            inner: self.inner.clone(),
        }
    }
}

impl<T> From<Vec<Vec<T>>> for Grid<T>
where
    T: Clone + Default,
{
    fn from(mut value: Vec<Vec<T>>) -> Self {
        let height = value.len();
        // assert uniform width
        let width = value.iter().map(|row| row.len()).max().unwrap_or(0);
        for row in value.iter_mut().filter(|row| row.len() < width) {
            row.extend_from_slice(vec![T::default(); width - row.len()].as_slice());
        }
        Self {
            width,
            height,
            inner: value,
        }
    }
}

impl From<String> for Grid<char> {
    fn from(value: String) -> Self {
        let array = value
            .split('\n')
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self::from(array)
    }
}

pub struct Counter<T>
where
    T: Hash + Eq,
{
    counts: HashMap<T, usize>,
}

impl<T> Counter<T>
where
    T: Hash + Eq,
{
    pub fn new() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }

    pub fn get(&self, element: T) -> usize {
        *self.counts.get(&element).unwrap_or(&0)
    }

    pub fn add(&mut self, element: T) {
        let entry = self.counts.entry(element).or_insert(0);
        *entry += 1;
    }

    pub fn add_many(&mut self, element: T, count: usize) {
        let entry = self.counts.entry(element).or_insert(0);
        *entry += count;
    }

    pub fn remove(&mut self, element: T) -> Option<usize> {
        self.counts.remove(&element)
    }

    pub fn sorted(&self) -> impl Iterator<Item = (&T, usize)> {
        let mut paired_list = self.counts.iter().map(|(k, &v)| (k, v)).collect::<Vec<_>>();
        paired_list.sort_by_key(|&(_, v)| v);
        paired_list.into_iter().rev()
    }

    pub fn top(&self) -> usize {
        let sorted = self.sorted().collect::<Vec<_>>();
        sorted.first().map_or(0, |&(_, count)| count)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&T, &usize)> {
        self.counts.iter()
    }
}

impl<T> FromIterator<T> for Counter<T>
where
    T: Hash + Eq,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut counter = Self::new();
        for element in iter {
            counter.add(element);
        }
        counter
    }
}
