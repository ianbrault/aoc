/*
** src/puzzles/aoc2022/day12.rs
*/

use super::Solution;
use crate::types::Grid;

use log::debug;

use std::cmp;
use std::collections::HashSet;

const MAX_HEIGHT: i64 = 25;

fn elevation(c: char) -> i64 {
    let base = 'a' as i64;
    match c {
        'S' => 0,
        'E' => MAX_HEIGHT,
        // a is 0, z is 25
        _ => c as i64 - base,
    }
}

fn search_is_done(
    destination: (usize, usize),
    distances: &Grid<i64>,
    unvisited_set: &HashSet<(usize, usize)>,
) -> bool {
    // iterate until the top has been visited or the smallest tentative
    // distance in the unvisited set is infinity
    // also terminate if the unvisited set is empty
    unvisited_set.is_empty()
        || !unvisited_set.contains(&destination)
        || unvisited_set
            .iter()
            .map(|&(i, j)| *distances.get(i, j))
            .min()
            .unwrap_or(i64::MAX)
            == i64::MAX
}

fn is_reachable(
    heightmap: &Grid<i64>,
    current: (usize, usize),
    destination: (usize, usize),
) -> bool {
    let height_curr = heightmap.get(current.0, current.1);
    let height_dest = heightmap.get(destination.0, destination.1);
    height_curr - height_dest <= 1
}

fn dijkstra(heightmap: &Grid<i64>) -> Grid<i64> {
    let bottom = heightmap.find(&0).unwrap();
    let mut unvisited_set = HashSet::new();
    for (i, j, _) in heightmap.iter_grid() {
        unvisited_set.insert((i, j));
    }

    // set all tentative distances to infinity and set the top to 0
    let mut current = heightmap.find(&MAX_HEIGHT).unwrap();
    let mut distances = Grid::from(vec![vec![i64::MAX; heightmap.width]; heightmap.height]);
    distances.set(current.0, current.1, 0);

    // iterate until the bottom has been visited or the smallest tentative
    // distance in the unvisited set is infinity
    while !search_is_done(bottom, &distances, &unvisited_set) {
        debug!("visiting node {:?}", current);
        let distance = *distances.get(current.0, current.1);
        // consider all unvisited neighbors
        for node in heightmap
            .neighbors(current.0, current.1)
            .into_iter()
            .filter(|n| is_reachable(heightmap, current, *n) && unvisited_set.contains(n))
        {
            let (ii, jj) = node;
            // calculate their tentative distance thru the current node
            let node_distance = *distances.get(ii, jj);
            let new_distance = distance + 1;
            distances.set(ii, jj, cmp::min(node_distance, new_distance));
        }
        // remove the current node from the unvisited set
        unvisited_set.remove(&current);
        // select the unvisited node with the smallest tentative distance
        if let Some(node) = next_node(&unvisited_set, &distances) {
            current = node;
        }
    }

    distances
}

fn next_node(
    unvisited_set: &HashSet<(usize, usize)>,
    distances: &Grid<i64>,
) -> Option<(usize, usize)> {
    // select the unvisited node with the smallest tentative distance
    if let Some((point, _)) = unvisited_set
        .iter()
        .map(|&(i, j)| ((i, j), distances.get(i, j)))
        .min_by(|(_, da), (_, db)| da.cmp(db))
    {
        Some(point)
    } else {
        None
    }
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let heightmap = input
        .split('\n')
        .map(|line| line.chars().map(elevation))
        .collect::<Grid<_>>();
    // calculate the distances to the top
    let distances = dijkstra(&heightmap);

    // Part A: What is the fewest steps required to move from your current position to the location
    // that should get the best signal?
    let (i, j) = heightmap.find(&0).unwrap();
    let best_path_from_start = distances.get(i, j);
    solution.set_part_a(best_path_from_start);

    // Part B: What is the fewest steps required to move starting from any square with elevation a
    // to the location that should get the best signal?
    let best_path_from_bottom = heightmap
        .iter_grid()
        .filter(|&(_, _, &h)| h == 0)
        .map(|(i, j, _)| distances.get(i, j))
        .min()
        .unwrap();
    solution.set_part_b(best_path_from_bottom);

    solution
}
