#!/usr/bin/env python3

import collections
import os
import re
import subprocess
import sys


__base_dir__ = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def parse_output(proc: subprocess.CompletedProcess, results: collections.defaultdict):
    output = proc.stdout.decode("utf-8")
    year = 0
    puzzle = 0
    for line in output.split("\n"):
        if (match := re.search(r"AOC (\d{4}) puzzle (\d+)", line)):
            year = int(match.group(1))
            puzzle = int(match.group(2))
        elif "Time to solve" in line:
            time = float(line.split()[-1][:-2])
            results[(year, puzzle)].append(time)


def render(results: collections.defaultdict):
    years = set(y for y, _ in results.keys())
    for year in reversed(sorted(years)):
        puzzles = set(p for y, p in results.keys() if y == year)
        icon = "✅" if len(puzzles) == 25 else "📝"
        print(f"### {year} ({len(puzzles)}/25 {icon})\n")
        print("| Puzzle | Time (ms) |\n|:---|---:|")
        for puzzle in sorted(puzzles):
            puzzle_results = results[(year, puzzle)]
            average = sum(puzzle_results) / len(puzzle_results)
            print(f"| {puzzle} | {average:0.3f} |")
        print()


def main():
    os.chdir(__base_dir__)
    results = collections.defaultdict(list)
    for i in range(10):
        proc = subprocess.run(["cargo", "run", "--release"], capture_output=True)
        parse_output(proc, results)
        if i > 0:
            sys.stderr.write(f"{i * 10}% complete...\n")
    render(results)


if __name__ == "__main__":
    try:
        main()
    except Exception as ex:
        sys.exit(f"ERROR: {str(ex)}")
    except KeyboardInterrupt:
        print("Ctrl+C received, terminating...")
        pass
