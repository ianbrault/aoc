/*
** src/puzzles/aoc2022/day2.rs
*/

use super::Solution;

#[derive(Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn score(&self) -> u64 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn from_result(opponent_move: &Self, result: &GameResult) -> Self {
        match result {
            GameResult::Draw => opponent_move.clone(),
            GameResult::Win => match opponent_move {
                Self::Rock => Self::Paper,
                Self::Paper => Self::Scissors,
                Self::Scissors => Self::Rock,
            },
            GameResult::Loss => match opponent_move {
                Self::Rock => Self::Scissors,
                Self::Paper => Self::Rock,
                Self::Scissors => Self::Paper,
            },
        }
    }
}

impl From<char> for Move {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => unreachable!(),
        }
    }
}

enum GameResult {
    Win,
    Loss,
    Draw,
}

impl GameResult {
    fn get(opponent_move: &Move, player_move: &Move) -> Self {
        match (opponent_move, player_move) {
            (Move::Rock, Move::Rock) => Self::Draw,
            (Move::Rock, Move::Paper) => Self::Win,
            (Move::Rock, Move::Scissors) => Self::Loss,
            (Move::Paper, Move::Rock) => Self::Loss,
            (Move::Paper, Move::Paper) => Self::Draw,
            (Move::Paper, Move::Scissors) => Self::Win,
            (Move::Scissors, Move::Rock) => Self::Win,
            (Move::Scissors, Move::Paper) => Self::Loss,
            (Move::Scissors, Move::Scissors) => Self::Draw,
        }
    }

    fn score(&self) -> u64 {
        match self {
            Self::Win => 6,
            Self::Loss => 0,
            Self::Draw => 3,
        }
    }
}

impl From<char> for GameResult {
    fn from(c: char) -> Self {
        match c {
            'X' => Self::Loss,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => unreachable!(),
        }
    }
}

struct Game {
    player_move: Move,
    result: GameResult,
}

impl Game {
    fn from_string_with_move(string: &str) -> Self {
        let chars = string.chars().collect::<Vec<_>>();
        let opponent_move = Move::from(chars[0]);
        let player_move = Move::from(chars[2]);
        let result = GameResult::get(&opponent_move, &player_move);
        Self {
            player_move,
            result,
        }
    }

    fn from_string_with_result(string: &str) -> Self {
        let chars = string.chars().collect::<Vec<_>>();
        let opponent_move = Move::from(chars[0]);
        let result = GameResult::from(chars[2]);
        let player_move = Move::from_result(&opponent_move, &result);
        Self {
            player_move,
            result,
        }
    }

    fn score(&self) -> u64 {
        self.player_move.score() + self.result.score()
    }
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();

    // Part A: What would your total score be if everything goes exactly according to your
    // strategy guide?
    let games_with_move = input
        .split('\n')
        .map(Game::from_string_with_move)
        .collect::<Vec<_>>();
    let score = games_with_move.iter().map(|game| game.score()).sum::<u64>();
    solution.set_part_a(score);

    // Part B: Following the Elf's instructions for the second column, what would your total score
    // be if everything goes exactly according to your strategy guide?
    let games_with_result = input
        .split('\n')
        .map(Game::from_string_with_result)
        .collect::<Vec<_>>();
    let score = games_with_result
        .iter()
        .map(|game| game.score())
        .sum::<u64>();
    solution.set_part_b(score);

    solution
}
