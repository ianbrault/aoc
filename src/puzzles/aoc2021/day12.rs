/*
** src/puzzles/aoc2021/day12.rs
*/

use super::Solution;
use crate::utils;

use std::collections::{HashMap, HashSet};

type CaveMap<'a> = HashMap<&'a str, Vec<&'a str>>;

fn is_start(cave: &str) -> bool {
    cave == "start"
}

fn is_end(cave: &str) -> bool {
    cave == "end"
}

fn is_small_cave(cave: &str) -> bool {
    cave.chars().all(char::is_lowercase)
}

fn find_paths_small_caves_once_rec<'a>(
    cave_map: &CaveMap<'a>,
    from: &'a str,
    mut visited: HashSet<&'a str>,
) -> Vec<Vec<&'a str>> {
    let mut paths = vec![];
    // add the current cave to the visited caves if it is a small cave
    if is_small_cave(from) {
        visited.insert(from);
    }

    // recurse on un-visited caves
    if let Some(connected_caves) = cave_map.get(from) {
        for cave in connected_caves.iter() {
            if !visited.contains(cave) {
                // base case: end
                if is_end(cave) {
                    paths.push(vec![*cave, from]);
                } else {
                    let paths_rec =
                        find_paths_small_caves_once_rec(cave_map, cave, visited.clone());
                    // add the current cave to the paths and continue
                    for mut path in paths_rec.into_iter() {
                        path.push(from);
                        paths.push(path);
                    }
                }
            }
        }
    }

    paths
}

fn find_paths_small_caves_once<'a>(cave_map: &CaveMap<'a>) -> Vec<Vec<&'a str>> {
    let visited = HashSet::new();
    find_paths_small_caves_once_rec(cave_map, "start", visited)
}

fn find_paths_small_caves_once_or_twice_rec<'a>(
    cave_map: &CaveMap<'a>,
    from: &'a str,
    mut visited: HashSet<&'a str>,
    twice_visited: bool,
) -> Vec<Vec<&'a str>> {
    let mut paths = vec![];
    // add the current cave to the visited caves if it is a small cave
    if is_small_cave(from) {
        visited.insert(from);
    }

    // recurse on un-visited caves
    if let Some(connected_caves) = cave_map.get(from) {
        for cave in connected_caves.iter() {
            // the small cave revisit adds the option for a second branching point
            // if we have already visited a small cave but have not visited any small cave
            // twice, we can (a) skip the cave or (b) continuing on with the cave
            // note: not true for the start cave
            if visited.contains(cave) && !twice_visited && !is_start(cave) {
                // base case: end
                if is_end(cave) {
                    paths.push(vec![*cave, from]);
                } else {
                    let paths_rec = find_paths_small_caves_once_or_twice_rec(
                        cave_map,
                        cave,
                        visited.clone(),
                        true,
                    );
                    // add the current cave to the paths and continue
                    for mut path in paths_rec.into_iter() {
                        path.push(from);
                        paths.push(path);
                    }
                }
            } else if !visited.contains(cave) {
                // base case: end
                if is_end(cave) {
                    paths.push(vec![*cave, from]);
                } else {
                    let paths_rec = find_paths_small_caves_once_or_twice_rec(
                        cave_map,
                        cave,
                        visited.clone(),
                        twice_visited,
                    );
                    // add the current cave to the paths and continue
                    for mut path in paths_rec.into_iter() {
                        path.push(from);
                        paths.push(path);
                    }
                }
            }
        }
    }
    paths
}

fn find_paths_small_caves_once_or_twice<'a>(cave_map: &'a CaveMap) -> Vec<Vec<&'a str>> {
    let visited = HashSet::new();
    find_paths_small_caves_once_or_twice_rec(cave_map, "start", visited, false)
}

fn parse_caves(input: &str) -> CaveMap<'_> {
    let mut caves = HashMap::new();
    for line in input.split('\n') {
        let (from, to) = utils::split(line, "-").unwrap();
        let entry_from = caves.entry(from).or_insert_with(Vec::new);
        entry_from.push(to);
        let entry_to = caves.entry(to).or_insert_with(Vec::new);
        entry_to.push(from);
    }
    caves
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let cave_map = parse_caves(&input);

    // Part A: How many paths through the cave system are there that visit small caves at most once?
    let caves = find_paths_small_caves_once(&cave_map);
    solution.set_part_a(caves.len());

    // Part B: After reviewing the available paths, you realize you might have time to visit a
    // single small cave twice. Given these new rules, how many paths through this cave system are
    // there?
    let caves = find_paths_small_caves_once_or_twice(&cave_map);
    solution.set_part_b(caves.len());

    solution
}
