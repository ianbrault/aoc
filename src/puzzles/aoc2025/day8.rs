/*
** src/puzzles/aoc2025/day8.rs
*/

use super::Solution;
use crate::types::Point3D;

use std::cmp;
use std::collections::HashSet;

struct Circuits(Vec<HashSet<usize>>);

impl Circuits {
    fn new(junction_boxes: usize) -> Self {
        let inner = (0..junction_boxes)
            .map(|i| {
                let mut circuit = HashSet::new();
                circuit.insert(i);
                circuit
            })
            .collect();
        Self(inner)
    }

    fn connect(&mut self, a: usize, b: usize) {
        let i = self.0.iter().position(|c| c.contains(&a)).unwrap();
        let j = self.0.iter().position(|c| c.contains(&b)).unwrap();
        if i != j {
            let popped = self.0.remove(cmp::max(i, j));
            self.0[cmp::min(i, j)].extend(popped);
        }
    }

    fn sorted(&self) -> Vec<HashSet<usize>> {
        let mut largest = self.0.clone();
        largest.sort_by_key(|circuit| circuit.len());
        largest.reverse();
        largest
    }
}

fn distance(a: &Point3D, b: &Point3D) -> i64 {
    (b.x - a.x).pow(2) + (b.y - a.y).pow(2) + (b.z - a.z).pow(2)
}

fn junction_distances(junction_boxes: &[Point3D]) -> Vec<(usize, usize, i64)> {
    let mut distances = Vec::new();
    for (i, p) in junction_boxes[..(junction_boxes.len() - 1)]
        .iter()
        .enumerate()
    {
        for (j, pp) in junction_boxes[(i + 1)..].iter().enumerate() {
            distances.push((i, i + j + 1, distance(p, pp)));
        }
    }
    distances.sort_by_key(|&(_, _, d)| d);
    distances
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // The Elves are trying to figure out which junction boxes to connect so that electricity can
    // reach every junction box.
    let junction_boxes = input
        .split('\n')
        .map(|line| Point3D::try_from(line).unwrap())
        .collect::<Vec<_>>();
    let distances = junction_distances(&junction_boxes);
    let mut circuits = Circuits::new(junction_boxes.len());

    // Part A: Connect together the 1000 pairs of junction boxes which are closest together.
    // Afterward, what do you get if you multiply together the sizes of the three largest circuits?
    let count = 1000;
    for &(a, b, _) in distances.iter().take(count) {
        circuits.connect(a, b);
    }
    let largest_circuits = circuits.sorted();
    let size = largest_circuits[..3]
        .iter()
        .map(|circuit| circuit.len())
        .product::<usize>();
    solution.set_part_a(size);

    // Part B: Continue connecting the closest unconnected pairs of junction boxes together until
    // they're all in the same circuit. What do you get if you multiply together the X coordinates
    // of the last two junction boxes you need to connect?
    let mut last_connection = None;
    for &(a, b, _) in distances.iter().skip(count) {
        circuits.connect(a, b);
        if circuits.0.len() == 1 {
            last_connection = Some(junction_boxes[a].x * junction_boxes[b].x);
            break;
        }
    }
    solution.maybe_set_part_b(last_connection);

    solution
}
