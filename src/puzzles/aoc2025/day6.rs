/*
** src/puzzles/aoc2025/day6.rs
*/

use super::Solution;

#[derive(Clone, Copy)]
enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn initial(&self) -> u64 {
        match self {
            Self::Add => 0,
            Self::Multiply => 1,
        }
    }

    fn evaluate(&self, a: u64, b: u64) -> u64 {
        match self {
            Self::Add => a + b,
            Self::Multiply => a * b,
        }
    }
}

impl From<char> for Operation {
    fn from(value: char) -> Self {
        match value {
            '+' => Self::Add,
            '*' => Self::Multiply,
            _ => unreachable!(),
        }
    }
}

struct Problems {
    grid: Vec<Vec<Option<u32>>>,
    operations: Vec<Operation>,
    offsets: Vec<usize>,
    width: usize,
    count: usize,
}

impl Problems {
    fn next_offset(&self, index: usize) -> usize {
        if index + 1 < self.count {
            self.offsets[index + 1]
        } else {
            self.width
        }
    }

    fn solve(&self, index: usize) -> u64 {
        let operation = self.operations[index];
        let offset = self.offsets[index];
        let next = self.next_offset(index);

        let mut solution = operation.initial();
        for row in self.grid.iter() {
            let number = (offset..next)
                .rev()
                .flat_map(|i| row.get(i).into_iter().flatten())
                .enumerate()
                .map(|(i, &n)| n as u64 * 10u64.pow(i as u32))
                .sum::<u64>();
            solution = operation.evaluate(solution, number);
        }
        solution
    }

    fn solve_all(&self) -> u64 {
        (0..self.count).map(|i| self.solve(i)).sum()
    }

    fn solve_cephalopod(&self, index: usize) -> u64 {
        let operation = self.operations[index];
        let offset = self.offsets[index];
        let next = self.next_offset(index);

        let mut solution = operation.initial();
        for column in (offset..next).rev() {
            let mut number = 0;
            let mut power = 0;
            for row in self.grid.iter().rev() {
                if let Some(Some(digit)) = row.get(column) {
                    number += *digit as u64 * 10u64.pow(power);
                    power += 1;
                }
            }
            if number > 0 {
                solution = operation.evaluate(solution, number);
            }
        }
        solution
    }

    fn solve_all_cephalopod(&self) -> u64 {
        (0..self.count).map(|i| self.solve_cephalopod(i)).sum()
    }
}

impl From<String> for Problems {
    fn from(value: String) -> Self {
        let lines = value.split('\n').collect::<Vec<_>>();
        let grid = lines
            .iter()
            .take(lines.len() - 1)
            .map(|line| line.chars().map(|n| n.to_digit(10)).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let width = grid.iter().map(|row| row.len()).max().unwrap();
        let (offsets, operations): (Vec<usize>, Vec<Operation>) = lines
            .last()
            .unwrap()
            .chars()
            .enumerate()
            .filter(|(_, c)| !c.is_ascii_whitespace())
            .map(|(i, c)| (i, Operation::from(c)))
            .unzip();
        let count = offsets.len();
        Problems {
            grid,
            operations,
            offsets,
            width,
            count,
        }
    }
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // Cephalopod math doesn't look that different from normal math. The math worksheet consists of
    // a list of problems; each problem has a group of numbers that need to be either added or
    // multiplied together.
    let problems = Problems::from(input);

    // Part A: Solve the problems on the math worksheet. What is the grand total found by adding
    // together all of the answers to the individual problems?
    let grand_total = problems.solve_all();
    solution.set_part_a(grand_total);

    // Part B: Solve the problems on the math worksheet again. What is the grand total found by
    // adding together all of the answers to the individual problems?
    let grand_total_cephalopod = problems.solve_all_cephalopod();
    solution.set_part_b(grand_total_cephalopod);

    solution
}
