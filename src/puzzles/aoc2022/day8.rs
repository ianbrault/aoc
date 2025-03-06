/*
** src/puzzles/aoc2022/day8.rs
*/

use super::Solution;
use crate::types::Grid;

use log::debug;

fn is_exterior(heights: &Grid<u32>, i: usize, j: usize) -> bool {
    i == 0 || j == 0 || i == heights.height - 1 || j == heights.width - 1
}

fn is_visible_up(heights: &Grid<u32>, i: usize, j: usize) -> bool {
    let height = heights.get(i, j);
    heights
        .iter_col(j)
        .filter(|&&ii| ii < i as u32)
        .all(|ii| ii < height)
}

fn is_visible_down(heights: &Grid<u32>, i: usize, j: usize) -> bool {
    let height = heights.get(i, j);
    heights
        .iter_col(j)
        .filter(|&&ii| ii > i as u32)
        .all(|ii| ii < height)
}

fn is_visible_left(heights: &Grid<u32>, i: usize, j: usize) -> bool {
    let height = heights.get(i, j);
    heights
        .iter_row(i)
        .filter(|&&jj| jj < j as u32)
        .all(|jj| jj < height)
}

fn is_visible_right(heights: &Grid<u32>, i: usize, j: usize) -> bool {
    let height = heights.get(i, j);
    heights
        .iter_row(i)
        .filter(|&&jj| jj > j as u32)
        .all(|jj| jj < height)
}

fn is_visible(heights: &Grid<u32>, i: usize, j: usize) -> bool {
    // check left/right first for better cache performance
    is_exterior(heights, i, j)
        || is_visible_left(heights, i, j)
        || is_visible_right(heights, i, j)
        || is_visible_up(heights, i, j)
        || is_visible_down(heights, i, j)
}

fn viewing_distance_up(heights: &Grid<u32>, i: usize, j: usize) -> u64 {
    let height = heights.get(i, j);
    let mut dist = 1;
    let mut ii = i as i64 - 1;
    while ii > 0 && heights.get(ii as usize, j) < height {
        dist += 1;
        ii -= 1;
    }
    dist
}

fn viewing_distance_down(heights: &Grid<u32>, i: usize, j: usize) -> u64 {
    let height = heights.get(i, j);
    let mut dist = 1;
    let mut ii = i as i64 + 1;
    while (ii as usize) < heights.height - 1 && heights.get(ii as usize, j) < height {
        dist += 1;
        ii += 1;
    }
    dist
}

fn viewing_distance_left(heights: &Grid<u32>, i: usize, j: usize) -> u64 {
    let height = heights.get(i, j);
    let mut dist = 1;
    let mut jj = j as i64 - 1;
    while jj > 0 && heights.get(i, jj as usize) < height {
        dist += 1;
        jj -= 1;
    }
    dist
}

fn viewing_distance_right(heights: &Grid<u32>, i: usize, j: usize) -> u64 {
    let height = heights.get(i, j);
    let mut dist = 1;
    let mut jj = j as i64 + 1;
    while (jj as usize) < heights.width - 1 && heights.get(i, jj as usize) < height {
        dist += 1;
        jj += 1;
    }
    dist
}

fn scenic_score(heights: &Grid<u32>, i: usize, j: usize) -> u64 {
    if is_exterior(heights, i, j) {
        debug!("tree ({},{}) is exterior with scenic score 0", i, j);
        0
    } else {
        // check left/right first for better cache performance
        let left = viewing_distance_left(heights, i, j);
        debug!("tree ({},{}) has left viewing distance {}", i, j, left);
        let right = viewing_distance_right(heights, i, j);
        debug!("tree ({},{}) has right viewing distance {}", i, j, right);
        let up = viewing_distance_up(heights, i, j);
        debug!("tree ({},{}) has up viewing distance {}", i, j, up);
        let down = viewing_distance_down(heights, i, j);
        debug!("tree ({},{}) has down viewing distance {}", i, j, down);
        left * right * up * down
    }
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let tree_heights = input
        .split('\n')
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()))
        .collect::<Grid<_>>();

    // Part A: Consider your map; how many trees are visible from outside the grid?
    let visible = tree_heights
        .iter_grid()
        .filter(|&(i, j, _)| is_visible(&tree_heights, i, j))
        .count();
    solution.set_part_a(visible);

    // Part B: Consider each tree on your map. What is the highest scenic score possible for
    // any tree?
    let score = tree_heights
        .iter_grid()
        .map(|(i, j, _)| scenic_score(&tree_heights, i, j))
        .max();
    solution.maybe_set_part_b(score);

    solution
}
