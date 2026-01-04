/*
** src/puzzles/aoc2025/day9.rs
*/

use super::Solution;
use crate::types::{Line, Point, RangeInclusive};

use std::cmp;

struct Rectangle {
    area: i64,
    x: RangeInclusive<i64>,
    y: RangeInclusive<i64>,
}

impl Rectangle {
    fn new(a: Point, b: Point) -> Self {
        let area = ((b.x - a.x).abs() + 1) * ((b.y - a.y).abs() + 1);
        let x = RangeInclusive::new(cmp::min(a.x, b.x), cmp::max(a.x, b.x));
        let y = RangeInclusive::new(cmp::min(a.y, b.y), cmp::max(a.y, b.y));
        Self { area, x, y }
    }

    fn contains_line(&self, line: &Line) -> bool {
        if line.is_horizontal() {
            line.p0.y > self.y.start
                && line.p0.y < self.y.end
                && line.p0.x <= self.x.end
                && line.p1.x >= self.x.start
        } else if line.is_vertical() {
            line.p0.x > self.x.start
                && line.p0.x < self.x.end
                && line.p0.y <= self.y.end
                && line.p1.y >= self.y.start
        } else {
            unreachable!()
        }
    }
}

fn generate_rectangles(points: &[Point]) -> Vec<Rectangle> {
    let mut rectangles = Vec::with_capacity(points.len() * 2);
    for (i, a) in points[..(points.len() - 1)].iter().enumerate() {
        for b in points[(i + 1)..].iter() {
            rectangles.push(Rectangle::new(*a, *b));
        }
    }
    rectangles
}

fn ordered_line(a: Point, b: Point) -> Line {
    if a.x == b.x {
        let p0 = Point::new(a.x, cmp::min(a.y, b.y));
        let p1 = Point::new(a.x, cmp::max(a.y, b.y));
        Line::new(p0, p1)
    } else if a.y == b.y {
        let p0 = Point::new(cmp::min(a.x, b.x), a.y);
        let p1 = Point::new(cmp::max(a.x, b.x), a.y);
        Line::new(p0, p1)
    } else {
        unreachable!()
    }
}

fn generate_edges(points: &[Point]) -> Vec<Line> {
    let mut edges = Vec::with_capacity(points.len() + 1);
    for (i, a) in points[..(points.len() - 1)].iter().enumerate() {
        edges.push(Line::new(*a, points[i + 1]));
    }
    edges.push(ordered_line(points[points.len() - 1], points[0]));
    edges
}

fn largest_inner_rectangle(rectangles: &[Rectangle], edges: &[Line]) -> Option<i64> {
    'outer: for rectangle in rectangles {
        for edge in edges.iter() {
            if rectangle.contains_line(edge) {
                continue 'outer;
            }
        }
        return Some(rectangle.area);
    }
    None
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // The Elves would like to find the largest rectangle that uses red tiles for two of its
    // opposite corners.
    let red_tiles = input
        .split('\n')
        .map(|line| Point::try_from(line).unwrap())
        .collect::<Vec<_>>();

    // Part A: Using two red tiles as opposite corners, what is the largest area of any rectangle
    // you can make?
    let mut rectangles = generate_rectangles(&red_tiles);
    rectangles.sort_by_key(|rectangle| rectangle.area);
    rectangles.reverse();
    solution.maybe_set_part_a(rectangles.first().map(|rectangle| rectangle.area));

    // Part B: Using two red tiles as opposite corners, what is the largest area of any rectangle
    // you can make using only red and green tiles?
    let edges = generate_edges(&red_tiles);
    let area = largest_inner_rectangle(&rectangles, &edges);
    solution.maybe_set_part_b(area);

    solution
}
