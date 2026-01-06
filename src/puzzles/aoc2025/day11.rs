/*
** src/puzzles/aoc2025/day11.rs
*/

use super::Solution;
use crate::utils;

use std::collections::HashMap;

const INPUT: u16 = u16::MIN;
const OUTPUT: u16 = u16::MAX;

fn char_id(c: char) -> u16 {
    ((c as u8) - b'a') as u16
}

fn device_id(device: &str) -> u16 {
    if device == "you" {
        INPUT
    } else if device == "out" {
        OUTPUT
    } else {
        let chars = device.chars().collect::<Vec<_>>();
        assert_eq!(chars.len(), 3);
        (1 << 15) | (char_id(chars[0]) << 10) | (char_id(chars[1]) << 5) | char_id(chars[2])
    }
}

fn build_connection_map(input: String) -> HashMap<u16, Vec<u16>> {
    let mut map = HashMap::new();
    for line in input.split('\n') {
        let (input, outputs) = utils::split(line, ": ").unwrap();
        let output_ids = outputs.split(' ').map(device_id).collect();
        map.insert(device_id(input), output_ids);
    }
    map
}

fn find_paths_rec(
    map: &HashMap<u16, Vec<u16>>,
    current: u16,
    target: u16,
    memo: &mut HashMap<u16, usize>,
) -> usize {
    if current == target {
        1
    } else if memo.contains_key(&current) {
        *memo.get(&current).unwrap()
    } else {
        let paths = map
            .get(&current)
            .unwrap_or(&Vec::new())
            .iter()
            .map(|device| find_paths_rec(map, *device, target, memo))
            .sum();
        memo.insert(current, paths);
        paths
    }
}

fn find_paths(map: &HashMap<u16, Vec<u16>>, from: u16, to: u16) -> usize {
    let mut memo = HashMap::new();
    find_paths_rec(map, from, to, &mut memo)
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // You glance around the room and see a tangle of cables and devices running from the server
    // rack to the reactor. The elf rushes off, returning a moment later with a list of the devices
    // and their outputs.
    let connections = build_connection_map(input);

    // Part A: How many different paths lead from you to out?
    let paths = find_paths(&connections, INPUT, OUTPUT);
    solution.set_part_a(paths);

    // Part B: Find all of the paths that lead from svr to out. How many of those paths visit both
    // dac and fft?
    let svr_to_dac = find_paths(&connections, device_id("svr"), device_id("dac"));
    let svr_to_fft = find_paths(&connections, device_id("svr"), device_id("fft"));
    let dac_to_fft = find_paths(&connections, device_id("dac"), device_id("fft"));
    let fft_to_dac = find_paths(&connections, device_id("fft"), device_id("dac"));
    let dac_to_out = find_paths(&connections, device_id("dac"), OUTPUT);
    let fft_to_out = find_paths(&connections, device_id("fft"), OUTPUT);
    let route_a = svr_to_dac * dac_to_fft * fft_to_out;
    let route_b = svr_to_fft * fft_to_dac * dac_to_out;
    solution.set_part_b(route_a + route_b);

    solution
}
