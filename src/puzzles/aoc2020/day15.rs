/*
** src/puzzles/aoc2020/day15.rs
*/

use super::Solution;

fn run_game(input: &[usize], turns: usize) -> usize {
    let mut previous;
    // stores the last turn when a number was spoken
    // for n_turns=30000000 this is huge (56+ MiB) but the cache misses are amortized by avoiding
    // the hashing and reallocation of HashMap
    let mut numbers = vec![0; turns];

    // the first numbers come directly from the puzzle input
    let mut i = 0;
    while i < input.len() {
        numbers[input[i]] = i + 1;
        i += 1;
    }
    previous = *input.last().unwrap();

    while i < turns {
        // the next number is the number of turns since the previously-spoken number was spoken
        // if it is not tracked, the previous turn was the first time it was spoken
        // note: insert the previous number instead of the current number
        let last_turn = &mut numbers[previous];
        if *last_turn == 0 {
            *last_turn = i;
        }
        previous = i - *last_turn;
        *last_turn = i;
        i += 1;
    }

    previous
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let input = input
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    // Part A: What will be the 2020th number spoken?
    let number = run_game(&input, 2020);
    solution.set_part_a(number);

    // Part B: Given your starting numbers, what will be the 30000000th number spoken?
    let number = run_game(&input, 30000000);
    solution.set_part_b(number);

    solution
}
