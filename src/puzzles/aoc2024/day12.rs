/*
** src/puzzles/aoc2024/day12.rs
*/

use super::Solution;
use crate::types::{Direction, Grid, Point};

use std::collections::{HashSet, VecDeque};

fn unvisited_location(map: &Grid<bool>) -> Option<(usize, usize)> {
    let point = map.iter_grid().find(|&(_, _, visited)| !visited);
    point.map(|(i, j, _)| (i, j))
}

fn next_point(point: Point, direction: Direction) -> Point {
    match direction {
        Direction::North => Point::new(point.x - 1, point.y),
        Direction::South => Point::new(point.x + 1, point.y),
        Direction::East => Point::new(point.x, point.y + 1),
        Direction::West => Point::new(point.x, point.y - 1),
        Direction::NorthEast => Point::new(point.x - 1, point.y + 1),
        Direction::NorthWest => Point::new(point.x - 1, point.y - 1),
        Direction::SouthEast => Point::new(point.x + 1, point.y + 1),
        Direction::SouthWest => Point::new(point.x + 1, point.y - 1),
    }
}

fn double_shape(points: HashSet<Point>) -> HashSet<Point> {
    let mut output = HashSet::new();
    for point in points {
        output.insert(Point::new(point.x * 2, point.y * 2));
        output.insert(Point::new(point.x * 2 + 1, point.y * 2));
        output.insert(Point::new(point.x * 2, point.y * 2 + 1));
        output.insert(Point::new(point.x * 2 + 1, point.y * 2 + 1));
    }
    output
}

fn is_exterior_corner(point: Point, points: &HashSet<Point>) -> bool {
    // a point is an exterior corner if exactly 2 non-corner neighbors are outside the shape
    let directions = [
        (Direction::North, Direction::East),
        (Direction::East, Direction::South),
        (Direction::South, Direction::West),
        (Direction::West, Direction::North),
    ];
    for (direction_a, direction_b) in directions {
        let a = next_point(point, direction_a);
        let b = next_point(point, direction_b);
        if !points.contains(&a) && !points.contains(&b) {
            return true;
        }
    }
    false
}

fn is_interior_corner(point: Point, points: &HashSet<Point>) -> bool {
    // a point is an interior corner if exactly 1 corner neighbor is outside the shape
    let directions = [
        (Direction::North, Direction::East, Direction::NorthEast),
        (Direction::East, Direction::South, Direction::SouthEast),
        (Direction::South, Direction::West, Direction::SouthWest),
        (Direction::West, Direction::North, Direction::NorthWest),
    ];
    for (direction_a, direction_b, direction_c) in directions {
        let a = next_point(point, direction_a);
        let b = next_point(point, direction_b);
        let c = next_point(point, direction_c);
        if points.contains(&a) && points.contains(&b) && !points.contains(&c) {
            return true;
        }
    }
    false
}

fn number_of_sides(points: HashSet<Point>) -> usize {
    // double the size of the shape to prevent single-wide arms
    let points = double_shape(points);
    // derive the number of corners in the shape which is equal to the number of edges
    points
        .iter()
        .filter(|&&point| is_exterior_corner(point, &points) || is_interior_corner(point, &points))
        .count()
}

fn fencing_cost(map: &Grid<char>, with_discount: bool) -> usize {
    let mut cost = 0;
    let mut visited = Grid::<bool>::new(map.width, map.height);

    while let Some((i, j)) = unvisited_location(&visited) {
        let plant = map.get(i, j);
        let mut area = 0;
        let mut perimeter = 0;
        let mut points = HashSet::new();

        let mut queue = VecDeque::from(vec![(i, j)]);
        while let Some((i, j)) = queue.pop_front() {
            if *visited.get(i, j) {
                continue;
            }
            area += 1;
            visited.set(i, j, true);
            points.insert(Point::new(i as i64, j as i64));
            // check north of the plant
            if i > 0 && map.get(i - 1, j) == plant {
                queue.push_back((i - 1, j));
            } else {
                perimeter += 1;
            }
            // check west of the plant
            if j > 0 && map.get(i, j - 1) == plant {
                queue.push_back((i, j - 1));
            } else {
                perimeter += 1;
            }
            // check south of the plant
            if i < map.height - 1 && map.get(i + 1, j) == plant {
                queue.push_back((i + 1, j));
            } else {
                perimeter += 1;
            }
            // check east of the plant
            if j < map.width - 1 && map.get(i, j + 1) == plant {
                queue.push_back((i, j + 1));
            } else {
                perimeter += 1;
            }
        }

        if with_discount {
            cost += area * number_of_sides(points);
        } else {
            cost += area * perimeter;
        }
    }

    cost
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // Each garden plot grows only a single type of plant and is indicated by a single letter on
    // your map. When multiple garden plots are growing the same type of plant and are touching,
    // they form a region.
    let map = Grid::from(input);

    // Part A: What is the total price of fencing all regions on your map?
    let price = fencing_cost(&map, false);
    solution.set_part_a(price);

    // Part B: Under the bulk discount, instead of using the perimeter to calculate the price, you
    // need to use the number of sides each region has. What is the new total price of fencing all
    // regions on your map?
    let price_discounted = fencing_cost(&map, true);
    solution.set_part_b(price_discounted);

    solution
}
