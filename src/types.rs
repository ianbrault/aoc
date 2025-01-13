/*
** src/types.rs
*/

use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
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
        &self.inner[i][j]
    }

    pub fn get_mut(&mut self, i: usize, j: usize) -> &mut T {
        &mut self.inner[i][j]
    }

    pub fn set(&mut self, i: usize, j: usize, value: T) {
        self.inner[i][j] = value;
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

pub struct Counter<'a, T>
where
    T: Hash + Eq,
{
    counts: HashMap<&'a T, usize>,
}

impl<'a, T> Counter<'a, T>
where
    T: Hash + Eq,
{
    pub fn new() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }

    pub fn get(&self, element: &'a T) -> usize {
        *self.counts.get(&element).unwrap_or(&0)
    }

    pub fn add(&mut self, element: &'a T) {
        let entry = self.counts.entry(element).or_insert(0);
        *entry += 1;
    }

    pub fn add_many(&mut self, element: &'a T, count: usize) {
        let entry = self.counts.entry(element).or_insert(0);
        *entry += count;
    }

    pub fn remove(&mut self, element: &'a T) -> Option<usize> {
        self.counts.remove(element)
    }

    pub fn sorted(&'a self) -> impl Iterator<Item = (&'a T, &'a usize)> {
        let mut paired_list = self.counts.iter().map(|(&k, v)| (k, v)).collect::<Vec<_>>();
        paired_list.sort_by_key(|(_, &v)| v);
        paired_list.into_iter().rev()
    }

    pub fn top(&self) -> usize {
        let sorted = self.sorted().collect::<Vec<_>>();
        sorted.first().map_or(0, |(_, &count)| count)
    }
}

impl<'a, T> FromIterator<&'a T> for Counter<'a, T>
where
    T: Hash + Eq,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = &'a T>,
    {
        let mut counter = Self::new();
        for element in iter {
            counter.add(element);
        }
        counter
    }
}
