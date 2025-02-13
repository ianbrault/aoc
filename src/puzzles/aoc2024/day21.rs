/*
** src/puzzles/aoc2024/day21.rs
*/

use super::Solution;
use crate::hash_map;
use crate::types::Point;

use std::cmp;
use std::collections::{HashMap, VecDeque};
use std::iter;

struct Keypad(HashMap<char, Point>);

impl Keypad {
    fn direction(dx: i64, dy: i64) -> char {
        match (dx, dy) {
            (0, x) if x > 0 => '^',
            (x, 0) if x > 0 => '>',
            (0, x) if x < 0 => 'v',
            (x, 0) if x < 0 => '<',
            _ => unreachable!(),
        }
    }

    fn repeat_sequence(dx: i64, dy: i64) -> Vec<char> {
        iter::repeat(Self::direction(dx, dy))
            .take(cmp::max(dx.unsigned_abs(), dy.unsigned_abs()) as usize)
            .collect()
    }

    fn get(&self, v: char) -> Point {
        self.0[&v]
    }

    fn diagonal_sequences(&self, start: Point, end: Point) -> Vec<Vec<char>> {
        let delta = end - start;
        let x = self.get('X');

        let mut subsequences = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back((start, delta, vec![]));
        while let Some((point, delta, mut sequence)) = queue.pop_front() {
            if point == x {
                continue;
            } else if point == end {
                sequence.push('A');
                subsequences.push(sequence);
            } else {
                // can go left or right, depending on remaining steps
                if delta.x != 0 {
                    let dx = delta.x.signum();
                    let next_point = Point::new(point.x + dx, point.y);
                    let next_delta = Point::new(delta.x - dx, delta.y);
                    let mut next_sequence = sequence.clone();
                    next_sequence.push(Self::direction(dx, 0));
                    queue.push_back((next_point, next_delta, next_sequence));
                }
                if delta.y != 0 {
                    let dy = delta.y.signum();
                    let next_point = Point::new(point.x, point.y + dy);
                    let next_delta = Point::new(delta.x, delta.y - dy);
                    let mut next_sequence = sequence.clone();
                    next_sequence.push(Self::direction(0, dy));
                    queue.push_back((next_point, next_delta, next_sequence));
                }
            }
        }
        subsequences
    }

    fn sequences(&self, from: char, to: char) -> Vec<Vec<char>> {
        let start = self.get(from);
        let end = self.get(to);
        let delta = end - start;

        if start == end {
            // no movement
            vec![vec!['A']]
        } else if delta.x == 0 || delta.y == 0 {
            // horizontal/vertical movement
            let mut sequence = Self::repeat_sequence(delta.x, delta.y);
            sequence.push('A');
            vec![sequence]
        } else {
            self.diagonal_sequences(start, end)
        }
    }
}

struct Compiler {
    numeric_keypad: Keypad,
    directional_keypad: Keypad,
    memo: HashMap<(char, char, usize), usize>,
}

impl Compiler {
    fn new(numeric_keypad: HashMap<char, Point>, directional_keypad: HashMap<char, Point>) -> Self {
        Self {
            numeric_keypad: Keypad(numeric_keypad),
            directional_keypad: Keypad(directional_keypad),
            memo: HashMap::new(),
        }
    }

    fn compile_move(&mut self, from: char, to: char, depth: usize, max_depth: usize) -> usize {
        let key = (from, to, depth);
        if self.memo.contains_key(&key) {
            return self.memo[&key];
        }

        let sequences = if depth == 0 {
            self.numeric_keypad.sequences(from, to)
        } else {
            self.directional_keypad.sequences(from, to)
        };
        let output = if depth == max_depth {
            sequences.into_iter().map(|s| s.len()).min().unwrap()
        } else {
            sequences
                .into_iter()
                .map(|sequence| self.compile_sequence(sequence, depth + 1, max_depth))
                .min()
                .unwrap()
        };
        self.memo.insert(key, output);
        output
    }

    fn compile_sequence(&mut self, sequence: Vec<char>, depth: usize, max_depth: usize) -> usize {
        let mut size = 0;
        let mut current = 'A';
        for next in sequence {
            size += self.compile_move(current, next, depth, max_depth);
            current = next;
        }
        size
    }

    fn compile(&mut self, door_code: &str, max_depth: usize) -> usize {
        self.compile_sequence(door_code.chars().collect(), 0, max_depth)
    }
}

fn complexity(code: &str, sequence_length: usize) -> usize {
    let numeric = code[..(code.len() - 1)].parse::<usize>().unwrap();
    numeric * sequence_length
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // The door to that area is locked, but the computer can't open it; it can only be opened by
    // typing the door codes on the numeric keypad on the door. Unfortunately, the area outside the
    // door is currently depressurized and nobody can go near the door. A robot needs to be sent
    // instead. It has no problem navigating the ship and finding the numeric keypad, but it's not
    // designed for button pushing: it can't be told to push a specific button directly. Instead,
    // it has a robotic arm that can be controlled remotely via a directional keypad.
    let door_codes = input.split('\n').collect::<Vec<_>>();
    let numeric_keypad = hash_map! {
        'X' => Point::new(0, 0),
        '0' => Point::new(1, 0),
        'A' => Point::new(2, 0),
        '1' => Point::new(0, 1),
        '2' => Point::new(1, 1),
        '3' => Point::new(2, 1),
        '4' => Point::new(0, 2),
        '5' => Point::new(1, 2),
        '6' => Point::new(2, 2),
        '7' => Point::new(0, 3),
        '8' => Point::new(1, 3),
        '9' => Point::new(2, 3),
    };
    let directional_keypad = hash_map! {
        '<' => Point::new(0, 0),
        'v' => Point::new(1, 0),
        '>' => Point::new(2, 0),
        'X' => Point::new(0, 1),
        '^' => Point::new(1, 1),
        'A' => Point::new(2, 1),
    };
    let mut compiler = Compiler::new(numeric_keypad, directional_keypad);

    // Part A: Find the fewest number of button presses you'll need to perform in order to cause
    // the robot in front of the door to type each code. What is the sum of the complexities of the
    // five codes on your list?
    let complexities = door_codes
        .iter()
        .map(|code| (code, compiler.compile(code, 2)))
        .map(|(code, length)| complexity(code, length))
        .sum::<usize>();
    solution.set_part_a(complexities);

    // Part B: Just as the missing Historian is released, The Historians realize that a second
    // member of their search party has also been missing this entire time! This time, many more
    // robots are involved. What is the sum of the complexities of the five codes on your list?
    compiler.memo.clear();
    let complexities = door_codes
        .iter()
        .map(|code| (code, compiler.compile(code, 25)))
        .map(|(code, length)| complexity(code, length))
        .sum::<usize>();
    solution.set_part_b(complexities);

    solution
}
