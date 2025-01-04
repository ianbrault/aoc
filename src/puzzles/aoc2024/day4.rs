/*
** src/puzzles/aoc2024/day4.rs
*/

use super::Solution;
use crate::types::Grid;

use log::debug;

fn xmas_count_at(grid: &Grid<char>, i: usize, j: usize) -> usize {
    // pre-condition: the tile at i,j is 'X'
    let mut count = 0;
    // left
    if j >= 3
        && *grid.get(i, j - 1) == 'M'
        && *grid.get(i, j - 2) == 'A'
        && *grid.get(i, j - 3) == 'S'
    {
        debug!("XMAS found left at {},{}", i, j);
        count += 1;
    }
    // diagonal up-left
    if i >= 3
        && j >= 3
        && *grid.get(i - 1, j - 1) == 'M'
        && *grid.get(i - 2, j - 2) == 'A'
        && *grid.get(i - 3, j - 3) == 'S'
    {
        debug!("XMAS found up-left at {},{}", i, j);
        count += 1;
    }
    // up
    if i >= 3
        && *grid.get(i - 1, j) == 'M'
        && *grid.get(i - 2, j) == 'A'
        && *grid.get(i - 3, j) == 'S'
    {
        debug!("XMAS found up at {},{}", i, j);
        count += 1;
    }
    // diagonal up-right
    if i >= 3
        && j < grid.width - 3
        && *grid.get(i - 1, j + 1) == 'M'
        && *grid.get(i - 2, j + 2) == 'A'
        && *grid.get(i - 3, j + 3) == 'S'
    {
        debug!("XMAS found up-right at {},{}", i, j);
        count += 1;
    }
    // right
    if j < grid.width - 3
        && *grid.get(i, j + 1) == 'M'
        && *grid.get(i, j + 2) == 'A'
        && *grid.get(i, j + 3) == 'S'
    {
        debug!("XMAS found right at {},{}", i, j);
        count += 1;
    }
    // diagonal down-right
    if i < grid.height - 3
        && j < grid.width - 3
        && *grid.get(i + 1, j + 1) == 'M'
        && *grid.get(i + 2, j + 2) == 'A'
        && *grid.get(i + 3, j + 3) == 'S'
    {
        debug!("XMAS found down-right at {},{}", i, j);
        count += 1;
    }
    // down
    if i < grid.height - 3
        && *grid.get(i + 1, j) == 'M'
        && *grid.get(i + 2, j) == 'A'
        && *grid.get(i + 3, j) == 'S'
    {
        debug!("XMAS found down at {},{}", i, j);
        count += 1;
    }
    // diagonal down-left
    if i < grid.height - 3
        && j >= 3
        && *grid.get(i + 1, j - 1) == 'M'
        && *grid.get(i + 2, j - 2) == 'A'
        && *grid.get(i + 3, j - 3) == 'S'
    {
        debug!("XMAS found down-left at {},{}", i, j);
        count += 1;
    }
    count
}

fn xmas_count(grid: &Grid<char>) -> usize {
    let mut count = 0;
    for i in 0..grid.height {
        for j in 0..grid.width {
            if *grid.get(i, j) == 'X' {
                count += xmas_count_at(grid, i, j);
            }
        }
    }
    count
}

fn is_x_mas_at(grid: &Grid<char>, i: usize, j: usize) -> bool {
    // pre-condition: the tile at i,j is 'A' and 0 < i < height and 0 < j < width
    let mut mas_count = 0;
    if *grid.get(i - 1, j - 1) == 'M' && *grid.get(i + 1, j + 1) == 'S' {
        mas_count += 1;
    }
    if *grid.get(i - 1, j + 1) == 'M' && *grid.get(i + 1, j - 1) == 'S' {
        mas_count += 1;
    }
    if *grid.get(i + 1, j + 1) == 'M' && *grid.get(i - 1, j - 1) == 'S' {
        mas_count += 1;
    }
    if *grid.get(i + 1, j - 1) == 'M' && *grid.get(i - 1, j + 1) == 'S' {
        mas_count += 1;
    }
    mas_count == 2
}

fn x_mas_count(grid: &Grid<char>) -> usize {
    let mut count = 0;
    for i in 1..(grid.height - 1) {
        for j in 1..(grid.width - 1) {
            if *grid.get(i, j) == 'A' && is_x_mas_at(grid, i, j) {
                count += 1;
            }
        }
    }
    count
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // This word search allows words to be horizontal, vertical, diagonal, written backwards, or
    // even overlapping other words. It's a little unusual, though, as you don't merely need to
    // find one instance of XMAS - you need to find all of them.
    let input_grid = input
        .split('\n')
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let grid = Grid::from(input_grid);

    // Part A: Take a look at the little Elf's word search. How many times does XMAS appear?
    let xmas_words = xmas_count(&grid);
    solution.set_part_a(xmas_words);

    // Part B: You flip over the word search to find that this isn't actually an XMAS puzzle; it's
    // an X-MAS puzzle in which you're supposed to find two MAS in the shape of an X. How many
    // times does an X-MAS appear?
    let x_mas_words = x_mas_count(&grid);
    solution.set_part_b(x_mas_words);

    solution
}
