#!/usr/bin/env bash

YEAR=$1
DAY=$2
touch "input/$YEAR/full.$DAY.txt"
touch "input/$YEAR/sample.$DAY.txt"
touch "src/puzzles/aoc$YEAR/day$DAY.rs"
