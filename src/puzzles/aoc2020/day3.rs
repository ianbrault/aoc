/*
** src/puzzles/aoc2020/day3.rs
*/

use super::Solution;
use crate::types::Maze;

fn traverse(map: &Maze, dx: usize, dy: usize) -> usize {
    let mut trees = 0;
    let mut i = 0;
    let mut j = 0;
    while i < map.height {
        if *map.get(i, j) == '#' {
            trees += 1;
        }
        i += dy;
        j = (j + dx) % map.width;
    }
    trees
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let map = Maze::from(input);

    // Part A: Starting at the top-left corner of your map and following a slope of right 3 and
    // down 1, how many trees would you encounter?
    let trees = traverse(&map, 3, 1);
    solution.set_part_a(trees);

    // Part B: What do you get if you multiply together the number of trees encountered on each of
    // the listed slopes?
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let trees = slopes
        .into_iter()
        .map(|(dx, dy)| traverse(&map, dx, dy))
        .product::<usize>();
    solution.set_part_b(trees);

    solution
}
