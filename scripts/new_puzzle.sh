#!/usr/bin/env bash

YEAR=$1
DAY=$2

mkdir -p "input/$YEAR/full"
mkdir -p "input/$YEAR/sample"
mkdir -p "src/puzzles/aoc$YEAR"
touch "input/$YEAR/full/$DAY.txt"
touch "input/$YEAR/sample/$DAY.txt"
touch "src/puzzles/aoc$YEAR/day$DAY.rs"

echo "/*
** src/puzzles/aoc$YEAR/day$DAY.rs
*/

use super::Solution;

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();

    // Part A: ???

    // Part B: ???

    solution
}" > "src/puzzles/aoc$YEAR/day$DAY.rs"
