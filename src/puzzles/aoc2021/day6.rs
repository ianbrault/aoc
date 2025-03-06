/*
** src/puzzles/aoc2021/day6.rs
*/

use super::Solution;

const LIFECYCLE: usize = 6;
const INACTIVE_PERIOD: usize = 2;
const ARRAY_SIZE: usize = LIFECYCLE + INACTIVE_PERIOD + 1;

type Lanternfish = [u64; ARRAY_SIZE];

fn parse_lanternfish(input: String) -> Lanternfish {
    let mut lanternfish = [0; ARRAY_SIZE];
    for n in input.split(',') {
        lanternfish[n.parse::<usize>().unwrap()] += 1;
    }
    lanternfish
}

fn simulate_day(lanternfish: Lanternfish) -> Lanternfish {
    let mut new = [0; ARRAY_SIZE];
    for (i, &fish) in lanternfish.iter().enumerate() {
        if i == 0 {
            // fish whose timers have expired are reset
            new[LIFECYCLE] += fish;
            // create new fish, including the inactive period
            new[LIFECYCLE + INACTIVE_PERIOD] += fish;
        } else {
            // decrease the timer for the fish
            new[i - 1] += fish;
        }
    }
    new
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let lanternfish = parse_lanternfish(input);

    // Part A: How many lanternfish would there be after 80 days?
    let mut fish = lanternfish;
    for _ in 0..80 {
        fish = simulate_day(fish);
    }
    let count = fish.into_iter().sum::<u64>();
    solution.set_part_a(count);

    // Part B: How many lanternfish would there be after 256 days?
    let mut fish = lanternfish;
    for _ in 0..256 {
        fish = simulate_day(fish);
    }
    let count = fish.into_iter().sum::<u64>();
    solution.set_part_b(count);

    solution
}
