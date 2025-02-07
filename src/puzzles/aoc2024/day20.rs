/*
** src/puzzles/aoc2024/day20.rs
*/

use super::Solution;
use crate::types::{Direction, Maze};

use std::cmp;
use std::collections::HashMap;

fn start_direction(maze: &Maze) -> Option<Direction> {
    let (i, j) = maze.find(&'S').unwrap();
    for direction in Direction::cardinal() {
        let (ii, jj) = maze.neighbor(i, j, direction).unwrap();
        if !maze.is_wall(ii, jj) {
            return Some(direction);
        }
    }
    None
}

fn find_path(maze: &Maze) -> Vec<(usize, usize)> {
    let start = maze.find(&'S').unwrap();
    let end = maze.find(&'E').unwrap();
    let mut path = Vec::new();

    let mut current = start;
    let mut direction = start_direction(maze).unwrap();
    while current != end {
        path.push(current);
        let next = maze.neighbor(current.0, current.1, direction).unwrap();
        if maze.is_wall(next.0, next.1) {
            let cw = direction.turn_90_clockwise();
            let cw_next = maze.neighbor(current.0, current.1, cw).unwrap();
            if maze.is_wall(cw_next.0, cw_next.1) {
                direction = direction.turn_90_counterclockwise();
                current = maze.neighbor(current.0, current.1, direction).unwrap();
            } else {
                current = cw_next;
                direction = cw;
            }
        } else {
            current = next;
        }
    }
    path.push(end);

    path
}

fn cheat_candidates(
    maze: &Maze,
    indices: &HashMap<(usize, usize), usize>,
    i: usize,
    j: usize,
    duration: usize,
) -> impl Iterator<Item = usize> {
    let index = indices[&(i, j)];
    let mut candidates = HashMap::new();

    let i_start = i.saturating_sub(duration);
    let j_start = j.saturating_sub(duration);
    let i_end = cmp::min(i + duration + 1, maze.height);
    let j_end = cmp::min(j + duration + 1, maze.height);
    for ii in i_start..i_end {
        for jj in j_start..j_end {
            let point = (ii, jj);
            if point == (i, j) || maze.get(ii, jj) == &'#' {
                continue;
            }
            let distance = Maze::distance((i, j), point);
            if distance > duration {
                continue;
            }
            let cheat_index = indices[&point];
            if cheat_index > index + duration {
                let savings = cheat_index - index - distance;
                let entry = candidates.entry(point).or_insert(savings);
                *entry = cmp::min(*entry, savings);
            }
        }
    }

    candidates.into_values()
}

fn find_cheats(maze: &Maze, path: &[(usize, usize)], duration: usize) -> Vec<usize> {
    let indices = path
        .iter()
        .copied()
        .enumerate()
        .map(|(i, p)| (p, i))
        .collect::<HashMap<_, _>>();

    let mut cheats = Vec::new();
    let end = path[path.len() - 1];
    for &point in path.iter() {
        let (i, j) = point;
        if point == end {
            continue;
        }
        cheats.extend(cheat_candidates(maze, &indices, i, j, duration));
    }
    cheats
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // You've arrived just in time for the frequently-held race condition festival! To make things
    // more interesting, they introduced a new rule to the races: programs are allowed to cheat.
    // The rules for cheating are very strict. Exactly once during a race, a program may disable
    // collision for up to 2 picoseconds. This allows the program to pass through walls as if they
    // were regular track.
    let maze = Maze::from(input);
    let path = find_path(&maze);

    // Part A: You aren't sure what the conditions of the racetrack will be like, so to give
    // yourself as many options as possible, you'll need a list of the best cheats. How many cheats
    // would save you at least 100 picoseconds?
    let cheats = find_cheats(&maze, &path, 2);
    let top_cheats = cheats.iter().filter(|&&cheat| cheat >= 100).count();
    solution.set_part_a(top_cheats);

    // Part B: Apparently, the two-picosecond cheating rule was deprecated several milliseconds
    // ago! The latest version of the cheating rule permits a single cheat that instead lasts at
    // most 20 picoseconds. Find the best cheats again using the updated cheating rules. How many
    // cheats would save you at least 100 picoseconds?
    let cheats = find_cheats(&maze, &path, 20);
    let top_cheats = cheats.iter().filter(|&&cheat| cheat >= 100).count();
    solution.set_part_b(top_cheats);

    solution
}
