/*
** src/puzzles/aoc2023/day3.rs
*/

use crate::types::{Point, Solution};

struct EngineNumber {
    id: u32,
    start: Point,
    end: Point,
}

impl EngineNumber {
    fn new(id: u32, y: usize, x0: usize, x1: usize) -> Self {
        let start = Point::new(x0 as i64, y as i64);
        let end = Point::new(x1 as i64, y as i64);
        Self { id, start, end }
    }
}

struct EngineSymbol {
    symbol: char,
    point: Point,
}

impl EngineSymbol {
    fn new(symbol: char, x: usize, y: usize) -> Self {
        let point = Point::new(x as i64, y as i64);
        Self { symbol, point }
    }
}

struct Engine {
    numbers: Vec<EngineNumber>,
    symbols: Vec<EngineSymbol>,
}

impl Engine {
    fn adjacent(number: &EngineNumber, symbol: &EngineSymbol) -> bool {
        symbol.point.x >= (number.start.x - 1)
            && symbol.point.x <= (number.end.x + 1)
            && symbol.point.y >= (number.start.y - 1)
            && symbol.point.y <= (number.end.y + 1)
    }

    fn is_part_number(&self, number: &EngineNumber) -> bool {
        for symbol in self.symbols.iter() {
            if Self::adjacent(number, symbol) {
                return true;
            }
        }
        false
    }

    fn gear_ratio(&self, symbol: &EngineSymbol) -> u32 {
        if symbol.symbol != '*' {
            return 0;
        }
        let mut ratio = 1;
        let mut n_adjacent = 0;
        for number in self.numbers.iter() {
            if Self::adjacent(number, symbol) {
                ratio *= number.id;
                n_adjacent += 1;
            }
        }
        if n_adjacent == 2 {
            ratio
        } else {
            0
        }
    }
}

impl From<String> for Engine {
    fn from(value: String) -> Self {
        let mut numbers = Vec::new();
        let mut symbols = Vec::new();
        for (y, line) in value.split("\n").enumerate() {
            let mut number_start = None;
            for (x, c) in line.chars().enumerate() {
                if c == '.' {
                    // terminating a number
                    if let Some(x0) = number_start {
                        let id = line[x0..x].parse().unwrap();
                        numbers.push(EngineNumber::new(id, y, x0, x - 1));
                        number_start = None;
                    }
                } else if c.is_numeric() {
                    // starting a new number
                    if number_start.is_none() {
                        number_start = Some(x);
                    }
                } else {
                    // terminating a number
                    if let Some(x0) = number_start {
                        let id = line[x0..x].parse().unwrap();
                        numbers.push(EngineNumber::new(id, y, x0, x - 1));
                        number_start = None;
                    }
                    symbols.push(EngineSymbol::new(c, x, y));
                }
            }
            // a number was being parsed when the line ended
            if let Some(x0) = number_start {
                let id = line[x0..].parse().unwrap();
                numbers.push(EngineNumber::new(id, y, x0, line.len() - 1));
            }
        }

        Engine { numbers, symbols }
    }
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // The engine schematic (your puzzle input) consists of a visual representation of the engine.
    // There are lots of numbers and symbols you don't really understand, but apparently any number
    // adjacent to a symbol, even diagonally, is a part number and should be included in your sum.
    let engine = Engine::from(input);

    // Part A: What is the sum of all of the part numbers in the engine schematic?
    let part_number_sum = engine
        .numbers
        .iter()
        .filter(|&number| engine.is_part_number(number))
        .map(|number| number.id)
        .sum::<u32>();
    solution.set_part_a(part_number_sum);

    // Part B: A gear is any * symbol that is adjacent to exactly two part numbers. Its gear ratio
    // is the result of multiplying those two numbers together. What is the sum of all of the gear
    // ratios in your engine schematic?
    let gear_ratio_sum = engine
        .symbols
        .iter()
        .map(|symbol| engine.gear_ratio(symbol))
        .sum::<u32>();
    solution.set_part_b(gear_ratio_sum);

    solution
}
