/*
** src/puzzles/aoc2021/day13.rs
*/

use super::Solution;
use crate::types::Point;
use crate::utils;

use std::collections::HashSet;

#[derive(Debug)]
enum Fold {
    X(i64),
    Y(i64),
}

impl Fold {
    fn reflect_point(&self, point: Point) -> Point {
        match self {
            Self::X(x) => point.reflect_x(*x),
            Self::Y(y) => point.reflect_y(*y),
        }
    }
}

impl From<&str> for Fold {
    fn from(value: &str) -> Self {
        let line = value.split(' ').last().unwrap();
        let (axis, point) = utils::split(line, "=").unwrap();
        match axis {
            "x" => Fold::X(point.parse().unwrap()),
            "y" => Fold::Y(point.parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

fn point_eligible_for_fold(point: &Point, fold: &Fold) -> bool {
    match fold {
        Fold::X(x) => point.x > *x,
        Fold::Y(y) => point.y > *y,
    }
}

fn perform_fold(points: HashSet<Point>, fold: &Fold) -> HashSet<Point> {
    let mut new_points = HashSet::new();
    for point in points {
        new_points.insert(if point_eligible_for_fold(&point, fold) {
            fold.reflect_point(point)
        } else {
            point
        });
    }
    new_points
}

fn print_grid(points: HashSet<Point>) -> String {
    let mut grid = vec![String::new()];
    let x_max = points.iter().map(|p| p.x).max().unwrap();
    let y_max = points.iter().map(|p| p.y).max().unwrap();
    for y in 0..=y_max {
        let mut s = String::with_capacity(x_max as usize);
        let px = points
            .iter()
            .filter(|p| p.y == y)
            .map(|p| p.x)
            .collect::<HashSet<_>>();
        for x in 0..=x_max {
            s += if px.contains(&x) { "#" } else { " " };
        }
        grid.push(s);
    }
    grid.join("\n")
}

fn parse_input(input: String) -> (HashSet<Point>, Vec<Fold>) {
    let (point_strings, fold_strings) = utils::split(input.as_str(), "\n\n").unwrap();
    let points = point_strings
        .split('\n')
        .flat_map(Point::try_from)
        .collect();
    let folds = fold_strings.split('\n').map(Fold::from).collect();
    (points, folds)
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let (mut points, folds) = parse_input(input);

    // Part A: How many dots are visible after completing just the first fold instruction on your
    // transparent paper?
    points = perform_fold(points, &folds[0]);
    solution.set_part_a(points.len());

    // Part B: Finish folding the transparent paper according to the instructions. The manual says
    // the code is always eight capital letters. What code do you use to activate the infrared
    // thermal imaging camera system?
    for fold in folds.iter().skip(1) {
        points = perform_fold(points, fold);
    }
    let image = print_grid(points);
    solution.set_part_b(image);

    solution
}
