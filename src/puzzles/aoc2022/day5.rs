/*
** src/puzzles/aoc2022/day5.rs
*/

use super::Solution;
use crate::utils;

struct Move {
    n_crates: u8,
    from: u8,
    to: u8,
}

impl From<&str> for Move {
    fn from(value: &str) -> Self {
        let words = value.split(' ').collect::<Vec<_>>();
        let n_crates = words[1].parse().unwrap();
        let from = words[3].parse().unwrap();
        let to = words[5].parse().unwrap();
        Self { n_crates, from, to }
    }
}

#[derive(Clone)]
struct Stacks {
    stacks: [Vec<char>; 9],
    buffer: Vec<char>,
}

impl Stacks {
    fn top(&self) -> String {
        self.stacks.iter().map(|s| s[s.len() - 1]).collect()
    }

    fn crate_mover_9000(&mut self, m: &Move) {
        let from = (m.from - 1) as usize;
        let to = (m.to - 1) as usize;
        for _ in 0..m.n_crates {
            let crate_name = self.stacks[from].pop().unwrap();
            self.stacks[to].push(crate_name);
        }
    }

    fn crate_mover_9001(&mut self, m: &Move) {
        let from = (m.from - 1) as usize;
        let to = (m.to - 1) as usize;
        // first load crates into the buffer
        for _ in 0..m.n_crates {
            let crate_name = self.stacks[from].pop().unwrap();
            self.buffer.push(crate_name);
        }
        // then drain from the buffer
        while let Some(crate_name) = self.buffer.pop() {
            self.stacks[to].push(crate_name);
        }
    }
}

impl From<&str> for Stacks {
    fn from(value: &str) -> Self {
        let mut stacks: [Vec<char>; 9] = Default::default();
        let lines = value.split('\n').collect::<Vec<_>>();
        for line in lines[..(lines.len() - 1)].iter().rev() {
            let n_cols = (line.len() + 1) / 4;
            for (col, stack) in stacks.iter_mut().enumerate().take(n_cols) {
                let i = col * 4 + 1;
                let crate_name = line[i..(i + 1)].chars().next().unwrap();
                if crate_name != ' ' {
                    stack.push(crate_name);
                }
            }
        }
        Self {
            stacks,
            buffer: Vec::new(),
        }
    }
}

fn parse_input(input: String) -> (Stacks, Vec<Move>) {
    let (stacks_str, moves_str) = utils::split(&input, "\n\n").unwrap();
    let stacks = Stacks::from(stacks_str);
    let moves = moves_str.split('\n').map(Move::from).collect();
    (stacks, moves)
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let (mut stacks, moves) = parse_input(input);
    let mut stacks_b = stacks.clone();

    // Part A: After the rearrangement procedure completes, what crate ends up on top of each stack?
    for m in moves.iter() {
        stacks.crate_mover_9000(m);
    }
    solution.set_part_a(stacks.top());

    // Part B: Before the rearrangement process finishes, update your simulation so that the Elves
    // know where they should stand to be ready to unload the final supplies. After the
    // rearrangement procedure completes, what crate ends up on top of each stack?
    for m in moves.iter() {
        stacks_b.crate_mover_9001(m);
    }
    solution.set_part_b(stacks_b.top());

    solution
}
