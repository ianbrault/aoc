/*
** src/puzzles/aoc2024/day8.rs
*/

use super::Solution;
use crate::types::{Grid, Point};
use crate::utils;

use log::debug;

use std::collections::HashSet;

struct Antenna {
    point: Point,
    frequency: char,
}

impl Antenna {
    fn new(i: usize, j: usize, frequency: char) -> Self {
        Self {
            point: Point::new(i as i64, j as i64),
            frequency,
        }
    }
}

fn antinode_locations(
    map: &Grid<char>,
    antennae: &[Antenna],
    frequencies: &[char],
) -> HashSet<Point> {
    let mut locations = HashSet::new();
    let height = map.height as i64;
    let width = map.width as i64;

    for &frequency in frequencies {
        debug!("frequency: {}", frequency);
        let matches = antennae
            .iter()
            .filter(|antenna| antenna.frequency == frequency)
            .collect::<Vec<_>>();
        for i in 0..(matches.len() - 1) {
            for j in (i + 1)..matches.len() {
                debug!("antennae: {} and {}", matches[i].point, matches[j].point);
                // sort antennae by j-coordinate for easier comparison
                let (p0, p1) = utils::min_max_by_key(&matches[i].point, &matches[j].point, |p| p.y);
                let (dx, dy) = (p1.x - p0.x, p1.y - p0.y);
                let a0 = Point::new(p1.x + dx, p1.y + dy);
                if a0.x >= 0 && a0.y >= 0 && a0.x < height && a0.y < width {
                    locations.insert(a0);
                }
                let a1 = Point::new(p0.x - dx, p0.y - dy);
                if a1.x >= 0 && a1.y >= 0 && a1.x < height && a1.y < width {
                    locations.insert(a1);
                }
            }
        }
    }

    locations
}

fn antinode_locations_resonant(
    map: &Grid<char>,
    antennae: &[Antenna],
    frequencies: &[char],
) -> HashSet<Point> {
    let mut locations = HashSet::new();
    let height = map.height as i64;
    let width = map.width as i64;

    for &frequency in frequencies {
        debug!("frequency: {}", frequency);
        let matches = antennae
            .iter()
            .filter(|antenna| antenna.frequency == frequency)
            .collect::<Vec<_>>();
        for i in 0..(matches.len() - 1) {
            for j in (i + 1)..matches.len() {
                debug!("antennae: {} and {}", matches[i].point, matches[j].point);
                locations.insert(matches[i].point);
                locations.insert(matches[j].point);
                // sort antennae by j-coordinate for easier comparison
                let (p0, p1) = utils::min_max_by_key(&matches[i].point, &matches[j].point, |p| p.y);
                let (dx, dy) = (p1.x - p0.x, p1.y - p0.y);
                let mut a0 = Point::new(p1.x + dx, p1.y + dy);
                while a0.x >= 0 && a0.y >= 0 && a0.x < height && a0.y < width {
                    locations.insert(a0);
                    a0.x += dx;
                    a0.y += dy;
                }
                let mut a1 = Point::new(p0.x - dx, p0.y - dy);
                while a1.x >= 0 && a1.y >= 0 && a1.x < height && a1.y < width {
                    locations.insert(a1);
                    a1.x -= dx;
                    a1.y -= dy;
                }
            }
        }
    }

    locations
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // Each antenna is tuned to a specific frequency indicated by a single lowercase letter,
    // uppercase letter, or digit. You create a map of these antennas.
    let map = Grid::from(input);
    let antennae = map
        .iter_grid()
        .filter_map(|(i, j, &c)| {
            if c != '.' {
                Some(Antenna::new(i, j, c))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let frequencies = antennae
        .iter()
        .map(|antenna| antenna.frequency)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    // Part A: Calculate the impact of the signal. How many unique locations within the bounds of
    // the map contain an antinode?
    let antinodes = antinode_locations(&map, &antennae, &frequencies);
    solution.set_part_a(antinodes.len());

    // Part B: Calculate the impact of the signal using this updated model. How many unique
    // locations within the bounds of the map contain an antinode?
    let antinodes_resonant = antinode_locations_resonant(&map, &antennae, &frequencies);
    solution.set_part_b(antinodes_resonant.len());

    solution
}
