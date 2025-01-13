/*
** src/puzzles/aoc2024/day10.rs
*/

use super::Solution;
use crate::types::Grid;

use std::collections::HashSet;

fn trailhead_score_rec(map: &Grid<u32>, i: usize, j: usize, peaks: &mut HashSet<(usize, usize)>) {
    let height = *map.get(i, j);
    if height == 9 {
        peaks.insert((i, j));
        return;
    }

    if i > 0 && *map.get(i - 1, j) == height + 1 {
        trailhead_score_rec(map, i - 1, j, peaks);
    }
    if j > 0 && *map.get(i, j - 1) == height + 1 {
        trailhead_score_rec(map, i, j - 1, peaks);
    }
    if i < map.height - 1 && *map.get(i + 1, j) == height + 1 {
        trailhead_score_rec(map, i + 1, j, peaks);
    }
    if j < map.width - 1 && *map.get(i, j + 1) == height + 1 {
        trailhead_score_rec(map, i, j + 1, peaks);
    }
}

fn trailhead_score(map: &Grid<u32>, i: usize, j: usize) -> usize {
    let mut peaks = HashSet::new();
    trailhead_score_rec(map, i, j, &mut peaks);
    peaks.len()
}

fn trailhead_rating_rec(
    map: &Grid<u32>,
    i: usize,
    j: usize,
    mut path: Vec<(usize, usize)>,
    all_paths: &mut HashSet<Vec<(usize, usize)>>,
) {
    path.push((i, j));

    let height = *map.get(i, j);
    if height == 9 {
        all_paths.insert(path);
        return;
    }

    if i > 0 && *map.get(i - 1, j) == height + 1 {
        trailhead_rating_rec(map, i - 1, j, path.clone(), all_paths);
    }
    if j > 0 && *map.get(i, j - 1) == height + 1 {
        trailhead_rating_rec(map, i, j - 1, path.clone(), all_paths);
    }
    if i < map.height - 1 && *map.get(i + 1, j) == height + 1 {
        trailhead_rating_rec(map, i + 1, j, path.clone(), all_paths);
    }
    if j < map.width - 1 && *map.get(i, j + 1) == height + 1 {
        trailhead_rating_rec(map, i, j + 1, path.clone(), all_paths);
    }
}

fn trailhead_rating(map: &Grid<u32>, i: usize, j: usize) -> usize {
    let mut paths = HashSet::new();
    trailhead_rating_rec(map, i, j, Vec::new(), &mut paths);
    paths.len()
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // The reindeer brings you a blank topographic map of the surrounding area. The topographic map
    // indicates the height at each position using a scale from 0 (lowest) to 9 (highest).
    let points = input
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let map = Grid::from(points);
    let trailheads = map.find_all(&0);

    // Part A: What is the sum of the scores of all trailheads on your topographic map?
    let scores = trailheads
        .iter()
        .map(|&(i, j)| trailhead_score(&map, i, j))
        .sum::<usize>();
    solution.set_part_a(scores);

    // Part B: The paper describes a second way to measure a trailhead called its rating: the
    // number of distinct hiking trails which begin at that trailhead. What is the sum of the
    // ratings of all trailheads?
    let ratings = trailheads
        .iter()
        .map(|&(i, j)| trailhead_rating(&map, i, j))
        .sum::<usize>();
    solution.set_part_b(ratings);

    solution
}
