/*
** src/types.rs
*/

use std::collections::HashMap;
use std::hash::Hash;

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
