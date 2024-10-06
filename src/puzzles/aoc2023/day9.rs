/*
** src/puzzles/aoc2023/day9.rs
*/

use super::Solution;
use crate::utils;

fn all_zeros(sequence: &[i64]) -> bool {
    sequence.iter().all(|&x| x == 0)
}

fn sequence_derivative(sequence: &[i64]) -> Vec<i64> {
    let mut derivative = Vec::new();
    for i in 1..sequence.len() {
        derivative.push(sequence[i] - sequence[i - 1]);
    }
    derivative
}

fn sequence_history(sequence: &[i64]) -> Vec<Vec<i64>> {
    let mut history = vec![sequence.to_vec()];
    // add derivatives until we reach steady state
    while !all_zeros(&history[history.len() - 1]) {
        history.push(sequence_derivative(&history[history.len() - 1]));
    }
    history
}

fn extrapolate_value_back(history: &[Vec<i64>]) -> i64 {
    let mut last = *history.last().unwrap().last().unwrap();
    for sequence in history.iter().rev().skip(1) {
        last += sequence.last().unwrap();
    }
    last
}

fn extrapolate_value_front(history: &[Vec<i64>]) -> i64 {
    let mut first = *history.last().unwrap().first().unwrap();
    for sequence in history.iter().rev().skip(1) {
        first = sequence.first().unwrap() - first;
    }
    first
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // You pull out your handy Oasis And Sand Instability Sensor and analyze your surroundings. The
    // OASIS produces a report of many values and how they are changing over time (your puzzle
    // input). Each line in the report contains the history of a single value.
    let sequences = input.split('\n').map(|line| utils::split_and_parse::<i64>(line).collect::<Vec<_>>()).collect::<Vec<_>>();
    let history = sequences.iter().map(|sequence| sequence_history(sequence)).collect::<Vec<_>>();

    // Part A: Analyze your OASIS report and extrapolate the next value for each history. What is
    // the sum of these extrapolated values?
    let history_sum_back = history.iter().map(|hist| extrapolate_value_back(hist)).sum::<i64>();
    solution.set_part_a(history_sum_back);

    // Part B: Analyze your OASIS report again, this time extrapolating the previous value for each
    // history. What is the sum of these extrapolated values?
    let history_sum_front = history.iter().map(|hist| extrapolate_value_front(hist)).sum::<i64>();
    solution.set_part_b(history_sum_front);

    solution
}
