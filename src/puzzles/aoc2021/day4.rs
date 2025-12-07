/*
** src/puzzles/aoc2021/day4.rs
*/

use super::Solution;

use std::collections::{HashMap, HashSet};

const BINGO_SIZE: usize = 5;

#[derive(Clone, Debug)]
struct BingoBoard {
    // stores the numbers on the card
    numbers: HashSet<u8>,
    // stores the positions of the numbers
    positions: HashMap<u8, usize>,
    // stores marked number positions
    marked: HashSet<usize>,
}

impl BingoBoard {
    fn mark(&mut self, number: u8) {
        if self.numbers.contains(&number) {
            self.marked.insert(*self.positions.get(&number).unwrap());
        }
    }

    fn contains_row(&self, pos: usize) -> bool {
        pos.is_multiple_of(5)
            && self.marked.contains(&(pos + 1))
            && self.marked.contains(&(pos + 2))
            && self.marked.contains(&(pos + 3))
            && self.marked.contains(&(pos + 4))
    }

    fn contains_col(&self, pos: usize) -> bool {
        self.marked.contains(&(pos + BINGO_SIZE))
            && self.marked.contains(&(pos + (BINGO_SIZE * 2)))
            && self.marked.contains(&(pos + (BINGO_SIZE * 3)))
            && self.marked.contains(&(pos + (BINGO_SIZE * 4)))
    }

    fn is_complete(&self) -> bool {
        // check rows/columns with marked positions
        for &pos in self.marked.iter() {
            if self.contains_row(pos) || self.contains_col(pos) {
                return true;
            }
        }
        false
    }

    fn score(&self, final_number: u8) -> u64 {
        let mut sum: u64 = 0;
        for number in self.numbers.iter() {
            let pos = self.positions.get(number).unwrap();
            if !self.marked.contains(pos) {
                sum += *number as u64;
            }
        }
        sum * final_number as u64
    }
}

impl From<&str> for BingoBoard {
    fn from(value: &str) -> Self {
        let mut numbers = HashSet::new();
        let mut positions = HashMap::new();
        for (pos, num_str) in value
            .split_whitespace()
            .filter(|ss| !ss.is_empty())
            .enumerate()
        {
            let num = num_str.parse().unwrap();
            numbers.insert(num);
            positions.insert(num, pos);
        }

        Self {
            numbers,
            positions,
            marked: HashSet::new(),
        }
    }
}

fn find_winning_board(mut boards: Vec<BingoBoard>, numbers: &[u8]) -> Option<u64> {
    for &number in numbers {
        // mark each board
        for board in boards.iter_mut() {
            board.mark(number);
        }
        // check if any are complete
        for board in boards.iter() {
            if board.is_complete() {
                let score = board.score(number);
                return Some(score);
            }
        }
    }
    None
}

fn find_final_winning_board(mut boards: Vec<BingoBoard>, numbers: &[u8]) -> Option<u64> {
    let mut complete_boards = HashSet::new();
    let mut last_board = None;
    for &number in numbers {
        // mark each board
        for board in boards.iter_mut() {
            board.mark(number);
        }
        // check if any are complete
        for (i, board) in boards.iter().enumerate() {
            if board.is_complete() && !complete_boards.contains(&i) {
                complete_boards.insert(i);
                let score = board.score(number);
                last_board = Some(score);
            }
        }
    }
    last_board
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let numbers = parts[0]
        .split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .collect::<Vec<_>>();
    let bingo_boards = parts
        .iter()
        .skip(1)
        .map(|&s| BingoBoard::from(s))
        .collect::<Vec<_>>();

    // Part A: Figure out which board will win first. What will your final score be if you choose
    // that board?
    let score = find_winning_board(bingo_boards.clone(), &numbers);
    solution.maybe_set_part_a(score);

    // Part B: Figure out which board will win last. Once it wins, what would its final score be?
    let score = find_final_winning_board(bingo_boards.clone(), &numbers);
    solution.maybe_set_part_b(score);

    solution
}
