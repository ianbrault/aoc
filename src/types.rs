/*
** src/types.rs
*/

use super::utils;

use anyhow::Error;
use case_iterable::CaseIterable;
use nalgebra::{Matrix2, Vector2};

use std::cmp::{self, Ordering};
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;
use std::ops::Sub;

#[derive(CaseIterable, Clone, Copy, Debug, Eq, Hash, PartialEq)]
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
    pub fn cardinal() -> impl Iterator<Item = Self> {
        vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
        .into_iter()
    }

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

    pub fn turn_90_counterclockwise(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
            Self::East => Self::North,
            Self::NorthEast => Self::NorthWest,
            Self::NorthWest => Self::SouthWest,
            Self::SouthWest => Self::SouthEast,
            Self::SouthEast => Self::NorthEast,
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

    pub fn origin() -> Self {
        Self::new(0, 0)
    }

    pub fn new_for_grid(i: usize, j: usize) -> Self {
        Self {
            x: i as i64,
            y: j as i64,
        }
    }

    pub fn reflect_x(&self, x: i64) -> Self {
        let dx = self.x - x;
        Self::new(x - dx, self.y)
    }

    pub fn reflect_y(&self, y: i64) -> Self {
        let dy = self.y - y;
        Self::new(self.x, y - dy)
    }

    pub fn manhattan_distance(a: Self, b: Self) -> i64 {
        let dx = a.x - b.x;
        let dy = a.y - b.y;
        dx.abs() + dy.abs()
    }

    pub fn ccw(a: &Point, b: &Point, c: &Point) -> bool {
        // Are the 3 points listed in counter-clockwise order?
        // If the slope of the line AB is less than the slope of the line AC
        (c.y - a.y) * (b.x - a.x) > (b.y - a.y) * (c.x - a.x)
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::fmt::Display for Point {
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

#[derive(Clone, PartialEq)]
pub struct Line {
    pub p0: Point,
    pub p1: Point,
    pub slope: Option<i64>,
    pub y_intercept: Option<i64>,
}

impl Line {
    pub fn new(p0: Point, p1: Point) -> Self {
        let slope = if p0.x == p1.x {
            None
        } else {
            let lp = cmp::min_by_key(p0, p1, |p| p.x);
            let rp = if lp == p0 { p1 } else { p0 };
            Some((rp.y - lp.y) / (rp.x - lp.x))
        };
        let y_intercept = if p0.x == p1.x {
            None
        } else {
            Some(p0.y - (p0.x * slope.unwrap()))
        };
        Self {
            p0,
            p1,
            slope,
            y_intercept,
        }
    }

    pub fn is_horizontal(&self) -> bool {
        self.p0.y == self.p1.y
    }

    pub fn is_vertical(&self) -> bool {
        self.p0.x == self.p1.x
    }

    pub fn x_min(&self) -> i64 {
        cmp::min(self.p0.x, self.p1.x)
    }

    pub fn x_max(&self) -> i64 {
        cmp::max(self.p0.x, self.p1.x)
    }

    pub fn y_min(&self) -> i64 {
        cmp::min(self.p0.y, self.p1.y)
    }

    pub fn y_max(&self) -> i64 {
        cmp::max(self.p0.y, self.p1.y)
    }

    pub fn contains_point(&self, p: &Point) -> bool {
        if self.is_vertical() {
            p.x == self.p0.x && (self.y_min()..=self.y_max()).contains(&p.y)
        } else {
            p.y == (self.slope.unwrap() * p.x) + self.y_intercept.unwrap()
                && (self.x_min()..=self.x_max()).contains(&p.x)
                && (self.y_min()..=self.y_max()).contains(&p.y)
        }
    }

    fn has_intersection(a: &Self, b: &Self) -> bool {
        // Note: the below does not cover scenarios when an endpoint is the intersection
        a.contains_point(&b.p0) || a.contains_point(&b.p1)
            || b.contains_point(&a.p0) || b.contains_point(&a.p1)
        // See https://bryceboe.com/2006/10/23/line-segment-intersection-algorithm/
        // Lines A and B intersect if and only if points A0 and A1 are separated by segment B0-B1
        // and points B0 and B1 are separated by segment A0-A1 then: if A0 and A1 are separated by
        // segment B0-B1 then A0-B0-B1 and A1-B0-B1 should have opposite orientation; i.e. either
        // A0-B0-B1 or A1-B0-B1 is counter-clockwise but NOT both
            || Point::ccw(&a.p0, &b.p0, &b.p1)
            != Point::ccw(&a.p1, &b.p0, &b.p1)
            && Point::ccw(&a.p0, &a.p1, &b.p0)
                != Point::ccw(&a.p0, &a.p1, &b.p1)
    }

    pub fn intersection(a: &Self, b: &Self) -> Option<Point> {
        if Self::has_intersection(a, b) {
            // Solve the system of equations
            let ma = a.slope? as f64;
            let mb = b.slope? as f64;
            let mat = Matrix2::new(-ma, 1.0, -mb, 1.0);
            let vec = Vector2::new(a.y_intercept? as f64, b.y_intercept? as f64);
            let sol = mat.try_inverse()? * vec;
            let x = sol[0];
            let y = sol[1];
            if x.fract() == 0.0 && y.fract() == 0.0 {
                return Some(Point::new(x.round() as i64, y.round() as i64));
            }
        }
        None
    }
}

impl TryFrom<&str> for Line {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Error> {
        if let Some((p0_str, p1_str)) = utils::split(value, " -> ") {
            let p0 = Point::try_from(p0_str)?;
            let p1 = Point::try_from(p1_str)?;
            Result::Ok(Self::new(p0, p1))
        } else {
            Result::Err(Error::msg("missing separator"))
        }
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}->{}", self.p0, self.p1))
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

    pub fn distance(from: (usize, usize), to: (usize, usize)) -> usize {
        let (ai, aj) = from;
        let (bi, bj) = to;
        let di = (bi as i64 - ai as i64).unsigned_abs();
        let dj = (bj as i64 - aj as i64).unsigned_abs();
        (di + dj) as usize
    }

    pub fn get(&self, i: usize, j: usize) -> &T {
        let row = self.inner.get(i).unwrap_or_else(|| {
            panic!(
                "Grid::get: index out of bounds: i: {}: size: {}x{}",
                i, self.width, self.height
            )
        });
        row.get(j).unwrap_or_else(|| {
            panic!(
                "Grid::get: index out of bounds: j: {}: size: {}x{}",
                j, self.width, self.height
            )
        })
    }

    pub fn get_mut(&mut self, i: usize, j: usize) -> &mut T {
        let row = self.inner.get_mut(i).unwrap_or_else(|| {
            panic!(
                "Grid::get_mut: index out of bounds: i: {}: size: {}x{}",
                i, self.width, self.height
            )
        });
        row.get_mut(j).unwrap_or_else(|| {
            panic!(
                "Grid::get_mut: index out of bounds: j: {}: size: {}x{}",
                j, self.width, self.height
            )
        })
    }

    pub fn set(&mut self, i: usize, j: usize, value: T) {
        let row = self.inner.get_mut(i).unwrap_or_else(|| {
            panic!(
                "Grid::set: index out of bounds: i: {}: size: {}x{}",
                i, self.width, self.height
            )
        });
        let col = row.get_mut(j).unwrap_or_else(|| {
            panic!(
                "Grid::set: index out of bounds: j: {}: size: {}x{}",
                j, self.width, self.height
            )
        });
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

    pub fn neighbors(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        Direction::cardinal()
            .filter_map(|direction| self.neighbor(i, j, direction))
            .collect::<Vec<_>>()
    }

    pub fn neighbors_with_diagonal(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        Direction::all_cases()
            .filter_map(|direction| self.neighbor(i, j, direction))
            .collect::<Vec<_>>()
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

    pub fn iter_grid_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut T)> {
        self.inner.iter_mut().enumerate().flat_map(|(i, row)| {
            row.iter_mut()
                .enumerate()
                .map(move |(j, item)| (i, j, item))
        })
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

    pub fn find_with<P>(&self, predicate: P) -> Option<(usize, usize)>
    where
        P: Fn(&T) -> bool,
    {
        for (i, j, x) in self.iter_grid() {
            if predicate(x) {
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

impl<I, T> FromIterator<I> for Grid<T>
where
    I: Iterator<Item = T>,
    T: Clone,
{
    fn from_iter<J>(iter: J) -> Self
    where
        J: IntoIterator<Item = I>,
    {
        let inner = iter
            .into_iter()
            .map(|inner| inner.collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let height = inner.len();
        let width = inner[0].len();
        Self {
            width,
            height,
            inner,
        }
    }
}

impl<T> From<Vec<Vec<T>>> for Grid<T>
where
    T: Clone,
{
    fn from(value: Vec<Vec<T>>) -> Self {
        let height = value.len();
        let width = value[0].len();
        Self {
            width,
            height,
            inner: value,
        }
    }
}

impl From<String> for Grid<char> {
    fn from(value: String) -> Self {
        value.split('\n').map(|line| line.chars()).collect()
    }
}

impl std::fmt::Display for Grid<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, j, c) in self.iter_grid() {
            if j == 0 && i > 0 {
                writeln!(f)?;
            }
            write!(f, "{}", c)?;
        }
        writeln!(f)
    }
}

#[derive(Clone, Debug)]
pub struct RangeInclusive<T>
where
    T: Clone + Copy + Ord + PartialOrd + Sub<Output = T>,
{
    pub start: T,
    pub end: T,
}

impl<T> RangeInclusive<T>
where
    T: Clone + Copy + Ord + PartialOrd + Sub<Output = T>,
{
    pub fn new(start: T, end: T) -> Self {
        Self { start, end }
    }

    pub fn contains(&self, value: &T) -> bool {
        value >= &self.start && value <= &self.end
    }

    pub fn size(&self) -> T {
        self.end - self.start
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        (other.start >= self.start && other.start <= self.end)
            || (other.end >= self.start && other.end <= self.end)
    }

    fn try_combine(&self, other: &Self) -> (Self, Option<Self>) {
        if self.overlaps(other) {
            let min = cmp::min(self.start, other.start);
            let max = cmp::max(self.end, other.end);
            (Self::new(min, max), None)
        } else {
            (self.clone(), Some(other.clone()))
        }
    }

    fn reduce_recursive(input: Vec<Self>, current: Self, index: usize, output: &mut Vec<Self>) {
        match &input[index..] {
            [] => {
                output.push(current);
            }
            [next, ..] => {
                let combination = current.try_combine(next);
                if let (range_a, Some(range_b)) = combination {
                    // Unsuccessful combination
                    output.push(range_a);
                    Self::reduce_recursive(input, range_b, index + 1, output)
                } else if let (range_a, None) = combination {
                    // Successful combination
                    Self::reduce_recursive(input, range_a, index + 1, output)
                } else {
                    unreachable!()
                }
            }
        }
    }

    pub fn reduce(mut ranges: Vec<Self>) -> Vec<Self> {
        let mut output = Vec::with_capacity(ranges.len());
        // Sort the ranges to start
        ranges.sort_by(|a, b| a.start.cmp(&b.start));
        if !ranges.is_empty() {
            let first = ranges[0].clone();
            Self::reduce_recursive(ranges, first, 1, &mut output);
        }
        output
    }
}

pub type Maze = Grid<char>;

impl Maze {
    pub fn create(width: usize, height: usize) -> Self {
        let inner = vec![vec!['.'; width]; height];
        Self {
            width,
            height,
            inner,
        }
    }

    pub fn is_wall(&self, i: usize, j: usize) -> bool {
        self.get(i, j) == &'#'
    }

    pub fn distance_matrix(&self, start: (usize, usize)) -> Grid<usize> {
        let mut distances = Grid::from(vec![vec![usize::MAX; self.width]; self.height]);
        let mut heap = BinaryHeap::new();

        distances.set(start.0, start.1, 0);
        heap.push(SearchState {
            position: start,
            cost: 0,
        });

        while let Some(SearchState {
            position: (i, j),
            cost,
        }) = heap.pop()
        {
            let distance = distances.get(i, j);
            if *distance < cost {
                continue;
            }
            for (ii, jj) in self.neighbors(i, j) {
                if self.is_wall(ii, jj) {
                    continue;
                }
                let next_distance = distances.get_mut(ii, jj);
                if *next_distance > cost + 1 {
                    *next_distance = cost + 1;
                    heap.push(SearchState {
                        position: (ii, jj),
                        cost: cost + 1,
                    });
                }
            }
        }
        distances
    }

    #[allow(unused)]
    pub fn to_csv(&self) -> String {
        let mut rows = Vec::with_capacity(self.height);
        for i in 0..self.height {
            let row = self
                .iter_row(i)
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(",");
            rows.push(row);
        }
        rows.join("\n")
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct SearchState {
    position: (usize, usize),
    cost: usize,
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

    pub fn extend<I>(&mut self, iter: I)
    where
        I: Iterator<Item = T>,
    {
        for item in iter {
            self.add(item);
        }
    }

    pub fn remove(&mut self, element: T) -> Option<usize> {
        self.counts.remove(&element)
    }

    pub fn sorted(&self) -> impl Iterator<Item = (&T, usize)> {
        let mut paired_list = self.counts.iter().map(|(k, &v)| (k, v)).collect::<Vec<_>>();
        paired_list.sort_by_key(|&(_, v)| v);
        paired_list.into_iter().rev()
    }

    pub fn min(&self) -> usize {
        let sorted = self.sorted().collect::<Vec<_>>();
        sorted.last().map_or(0, |&(_, count)| count)
    }

    pub fn max(&self) -> usize {
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
