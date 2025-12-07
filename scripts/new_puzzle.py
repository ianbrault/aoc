#!/usr/bin/env python3
# scripts/new_puzzle.py

import argparse
import pathlib
import re
import sys
import textwrap

template = """\
/*
** src/puzzles/aoc{year}/day{day}.rs
*/

use super::Solution;

pub fn solve(input: String) -> Solution {{
    let mut solution = Solution::new();

    // Part A: ???

    // Part B: ???

    solution
}}
"""

parser = argparse.ArgumentParser(description="Create files for new AOC puzzles.")
parser.add_argument("year", type=int)
parser.add_argument("day", type=int)
args = parser.parse_args()

project_directory = pathlib.Path(__file__).resolve().parent.parent

# Create input directories and files
input_directory = (
    pathlib.Path(__file__).resolve().parent.parent / "input" / str(args.year)
)
# Full input
input_full = input_directory / "full"
input_full.mkdir(parents=True, exist_ok=True)
(input_full / f"{args.day}.txt").touch()
# Sample input
input_sample = input_directory / "sample"
input_sample.mkdir(parents=True, exist_ok=True)
(input_sample / f"{args.day}.txt").touch()

# Create source directories and files
source_directory = project_directory / "src" / "puzzles" / f"aoc{args.year}"
source_directory.mkdir(parents=True, exist_ok=True)
source_file = source_directory / f"day{args.day}.rs"
source_file.write_text(template.format(year=args.year, day=args.day))

# Add the new source file to the mod.rs file
mod_file = source_directory / "mod.rs"
mod_file_contents = mod_file.read_text()
match = re.search(r"crate::puzzle_set\!\(\s*((?:day\d+,?\s*)+)\);", mod_file_contents)
if not match:
    sys.exit(f"ERROR: failed to parse {mod_file}")
module_match = match.group(1).strip()
days = [int(day[3:]) for day in re.split(r", *", module_match)] + [args.day]
module_string = ", ".join(f"day{d}" for d in days)
module_string_wrapped = "\n".join(
    textwrap.wrap(module_string, width=100, subsequent_indent=" " * 4)
)
mod_file.write_text(mod_file_contents.replace(module_match, module_string_wrapped))
