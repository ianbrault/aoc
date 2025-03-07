/*
** src/puzzles/aoc2020/day10.rs
*/

use super::Solution;

fn adapter_arrangements(joltages: &[u8]) -> usize {
    // the sorted joltages as a DAG, where vertices are connected by an edge if their differences
    // are <= 3; the solution becomes count the number of paths from the first to last vertex
    let n = joltages.len();

    // search in reverse-order and memoize results
    let mut memo = vec![0; n];
    // the end should have a value of 1, a little un-intuitive but it makes
    // the math work out
    memo[n - 1] = 1;
    for i in (0..(n - 1)).rev() {
        // the current item could connect to the next 3 items, depending on
        // their separation (no 2 items are separated by more than 3)
        if i + 1 < n && joltages[i + 1] - joltages[i] <= 3 {
            memo[i] += memo[i + 1];
        }
        if i + 2 < n && joltages[i + 2] - joltages[i] <= 3 {
            memo[i] += memo[i + 2];
        }
        if i + 3 < n && joltages[i + 3] - joltages[i] <= 3 {
            memo[i] += memo[i + 3];
        }
    }

    memo[0]
}

fn parse_joltages(input: String) -> Vec<u8> {
    // include both the charging outlet (0-jolt) and the device's build-in adapter (max-jolt + 3)
    let mut joltages = vec![0];
    joltages.extend(input.split('\n').map(|line| line.parse::<u8>().unwrap()));
    joltages.sort();
    joltages.push(joltages.last().unwrap() + 3);
    joltages
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let joltages = parse_joltages(input);

    // Part A: Find a chain that uses all of your adapters to connect the charging outlet to your
    // device's built-in adapter and count the joltage differences between the charging outlet, the
    // adapters, and your device. What is the number of 1-jolt differences multiplied by the number
    // of 3-jolt differences?
    let mut one_jolts = 0;
    let mut three_jolts = 0;
    // adapter joltages are already sorted, just count the differences of diffs
    for jolt_diff in joltages
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, &j)| j - joltages[i - 1])
    {
        match jolt_diff {
            1 => one_jolts += 1,
            3 => three_jolts += 1,
            _ => unreachable!(),
        }
    }
    solution.set_part_a(one_jolts * three_jolts);

    // Part B: What is the total number of distinct ways you can arrange the adapters to connect
    // the charging outlet to your device?
    let arrangements = adapter_arrangements(&joltages);
    solution.set_part_b(arrangements);

    solution
}
