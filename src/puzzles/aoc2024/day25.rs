/*
** src/puzzles/aoc2024/day25.rs
*/

use super::Solution;

enum SchematicType {
    Lock,
    Key,
}

struct Schematic {
    s_type: SchematicType,
    heights: [usize; 5],
}

impl Schematic {
    const HEIGHT: usize = 7;

    fn fits(lock: &Self, key: &Self) -> bool {
        lock.heights
            .iter()
            .zip(key.heights.iter())
            .all(|(l, k)| l + k <= Self::HEIGHT)
    }
}

impl std::fmt::Debug for Schematic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parts = self
            .heights
            .iter()
            .map(|&h| char::from_digit(h as u32, 10).unwrap().to_string())
            .collect::<Vec<_>>();
        write!(f, "[{}]", parts.join(","))
    }
}

impl From<&str> for Schematic {
    fn from(value: &str) -> Self {
        let mut s_type = SchematicType::Key;
        let mut heights = [0; 5];
        for (i, line) in value.split('\n').enumerate() {
            if i == 0 && line.chars().all(|c| c == '#') {
                s_type = SchematicType::Lock;
            }
            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    heights[j] += 1;
                }
            }
        }
        Self { s_type, heights }
    }
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // Unfortunately, they've lost track of which locks are installed and which keys go with them,
    // so the best they can do is send over schematics of every lock and every key for the floor
    // you're on.
    let schematics = input.split("\n\n").map(Schematic::from).collect::<Vec<_>>();

    // Part A: Analyze your lock and key schematics. How many unique lock/key pairs fit together
    // without overlapping in any column?
    let mut combos = 0;
    for lock in schematics
        .iter()
        .filter(|s| matches!(s.s_type, SchematicType::Lock))
    {
        for key in schematics
            .iter()
            .filter(|s| matches!(s.s_type, SchematicType::Key))
        {
            if Schematic::fits(lock, key) {
                combos += 1;
            }
        }
    }
    solution.set_part_a(combos);

    // Part B: 2024 complete!

    solution
}
