/*
** src/puzzles/aoc2024/day1.rs
*/

use super::Solution;
use crate::utils;
use crate::types::Counter;

fn parse_lists(input: String) -> (Vec<i64>, Vec<i64>) {
    let mut a = Vec::new();
    let mut b = Vec::new();
    for line in input.split('\n') {
        match utils::split_and_parse::<i64>(line).collect::<Vec<_>>().as_slice() {
            &[sa, sb] => {
                a.push(sa);
                b.push(sb);
            },
            _ => panic!("invalid line: {}", line),
        }
    }
    (a, b)
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // The Elves discover an assortment of notes and lists of historically significant locations.
    let (mut a, mut b) = parse_lists(input);

    // Part A: Pair up the smallest number in the left list with the smallest number in the right
    // list, then the second-smallest left number with the second-smallest right number, and so on.
    // Figure out how far apart the two numbers are in each pair and sum for the total distance.
    a.sort();
    b.sort();
    let distance = a.iter().zip(b.iter()).map(|(aa, bb)| (aa - bb).abs()).sum::<i64>();
    solution.set_part_a(distance);

    // Part B: Calculate a total similarity score by adding up each number in the left list after
    // multiplying it by the number of times that number appears in the right list.
    let counter = b.iter().collect::<Counter<_>>();
    let similarity = a.iter().map(|&aa| aa as usize * counter.get(&aa)).sum::<usize>();
    solution.set_part_b(similarity);

    solution
}
