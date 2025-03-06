/*
** src/puzzles/aoc2021/day5.rs
*/

use super::Solution;
use crate::types::{Line, Point};

use std::cmp;
use std::collections::HashSet;

fn vertical_lines_intersect(a: &Line, b: &Line) -> bool {
    let bot = cmp::min_by_key(a, b, |line| line.y_min());
    let top = if bot == a { b } else { a };
    bot.p0.x == top.p0.x && top.y_min() <= bot.y_max()
}

fn intersection_with_vertical(line_a: &Line, line_b: &Line) -> Option<Point> {
    let (vline, other) = if line_a.is_vertical() {
        (line_a, line_b)
    } else {
        (line_b, line_a)
    };

    let vx = vline.p0.x;
    if (other.x_min()..=other.x_max()).contains(&vx) {
        let y = (other.slope.unwrap() * vx) + other.y_intercept.unwrap();
        let p = Point::new(vx, y);
        if vline.contains_point(&p) {
            Some(p)
        } else {
            None
        }
    } else {
        None
    }
}

fn colinear_intersections(line_a: &Line, line_b: &Line, intersections: &mut HashSet<Point>) {
    // special case for vertical intersections
    if line_a.is_vertical() && line_b.is_vertical() {
        if vertical_lines_intersect(line_a, line_b) {
            let x = line_a.p0.x;
            let isect_start = cmp::max(line_a.y_min(), line_b.y_min());
            let isect_end = cmp::min(line_a.y_max(), line_b.y_max());
            for y in isect_start..=isect_end {
                intersections.insert(Point::new(x, y));
            }
        }
    } else {
        let slope = line_a.slope.unwrap();
        // sort the lines by x
        let lline = cmp::min_by_key(line_a, line_b, |line| line.x_min());
        let rline = if lline == line_a { line_b } else { line_a };
        // consider if points on the rightmost line fall along the leftmost
        let lp = cmp::min_by_key(rline.p0, rline.p1, |p| p.x);
        let rp = if lp == rline.p0 { rline.p1 } else { rline.p0 };
        if lline.contains_point(&lp) {
            let mut p = lp;
            while p != rp {
                if lline.contains_point(&p) {
                    intersections.insert(p);
                }
                p.x += 1;
                p.y += slope;
            }
            // check the endpoint
            if lline.contains_point(&p) {
                intersections.insert(p);
            }
        }
    }
}

fn find_intersections(lines: &[Line]) -> HashSet<Point> {
    let n_lines = lines.len();
    let mut intersections = HashSet::new();

    // check line intersections
    for i in 0..(n_lines - 1) {
        for j in (i + 1)..n_lines {
            let line_i = &lines[i];
            let line_j = &lines[j];
            if line_i.slope == line_j.slope {
                colinear_intersections(line_i, line_j, &mut intersections);
            } else if line_i.is_vertical() || line_j.is_vertical() {
                if let Some(p) = intersection_with_vertical(line_i, line_j) {
                    intersections.insert(p);
                }
            } else if let Some(p) = Line::intersection(line_i, line_j) {
                intersections.insert(p);
            }
        }
    }

    intersections
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let vent_lines = input
        .split('\n')
        .map(|line| Line::try_from(line).unwrap())
        .collect::<Vec<_>>();

    // Part A: Consider only horizontal and vertical lines. At how many points do at least two
    // lines overlap?
    let horizontal_vertical_lines = vent_lines
        .iter()
        .filter(|line| line.is_horizontal() || line.is_vertical())
        .cloned()
        .collect::<Vec<_>>();
    let intersections = find_intersections(&horizontal_vertical_lines);
    solution.set_part_a(intersections.len());

    // Part B: Consider all of the lines. At how many points do at least two lines overlap?
    let intersections = find_intersections(&vent_lines);
    solution.set_part_b(intersections.len());

    solution
}
