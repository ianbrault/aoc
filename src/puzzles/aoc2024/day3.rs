/*
** src/puzzles/aoc2024/day3.rs
*/

use super::Solution;

use log::debug;

struct Mul(u64, u64);

impl Mul {
    fn product(&self) -> u64 {
        self.0 * self.1
    }
}

enum MulParseState {
    Start,
    U,
    L,
    OpenParen,
    OpenParenToggleOn,
    OpenParenToggleOff,
    CloseParenToggleOff,
    OperandA,
    OperandB,
    O,
    NOrParen,
    Apostrophe,
    T,
}

struct MulParser {
    state: MulParseState,
    operand_a: u64,
    operand_b: u64,
    instructions: Vec<Mul>,
    with_toggle: bool,
    toggled: bool,
}

impl MulParser {
    fn new(with_toggle: bool) -> Self {
        Self {
            state: MulParseState::Start,
            operand_a: 0,
            operand_b: 0,
            instructions: Vec::new(),
            with_toggle,
            toggled: true,
        }
    }

    fn next_state(&mut self, character: char) {
        match self.state {
            MulParseState::Start => {
                if character == 'm' {
                    self.state = MulParseState::U;
                } else if character == 'd' && self.with_toggle {
                    self.state = MulParseState::O;
                }
            }
            MulParseState::U => {
                if character == 'u' {
                    self.state = MulParseState::L;
                } else {
                    self.state = MulParseState::Start;
                }
            }
            MulParseState::L => {
                if character == 'l' {
                    self.state = MulParseState::OpenParen;
                } else {
                    self.state = MulParseState::Start;
                }
            }
            MulParseState::OpenParen => {
                if character == '(' {
                    self.state = MulParseState::OperandA;
                    self.operand_a = 0;
                } else {
                    self.state = MulParseState::Start;
                }
            }
            MulParseState::OpenParenToggleOn => {
                if character == ')' {
                    debug!("toggling on");
                    self.toggled = true;
                }
                self.state = MulParseState::Start;
            }
            MulParseState::OpenParenToggleOff => {
                if character == '(' {
                    self.state = MulParseState::CloseParenToggleOff;
                } else {
                    self.state = MulParseState::Start;
                }
            }
            MulParseState::CloseParenToggleOff => {
                if character == ')' {
                    debug!("toggling off");
                    self.toggled = false;
                }
                self.state = MulParseState::Start;
            }
            MulParseState::OperandA => {
                if character == ',' {
                    self.state = MulParseState::OperandB;
                    self.operand_b = 0;
                } else if character.is_numeric() && self.operand_a < 1000 {
                    self.operand_a *= 10;
                    self.operand_a += character.to_digit(10).unwrap() as u64;
                } else {
                    self.state = MulParseState::Start;
                }
            }
            MulParseState::OperandB => {
                if character == ')' {
                    if !self.with_toggle || self.toggled {
                        debug!(
                            "new instruction: mul({},{})",
                            self.operand_a, self.operand_b
                        );
                        self.instructions.push(Mul(self.operand_a, self.operand_b));
                    } else {
                        debug!("ignoring new instruction, toggled off");
                    }
                    self.operand_a = 0;
                    self.operand_b = 0;
                    self.state = MulParseState::Start;
                } else if character.is_numeric() && self.operand_b < 1000 {
                    self.operand_b *= 10;
                    self.operand_b += character.to_digit(10).unwrap() as u64;
                } else {
                    self.state = MulParseState::Start;
                }
            }
            MulParseState::O => {
                if character == 'o' {
                    self.state = MulParseState::NOrParen;
                } else {
                    self.state = MulParseState::Start;
                }
            }
            MulParseState::NOrParen => {
                if character == 'n' {
                    self.state = MulParseState::Apostrophe;
                } else if character == '(' {
                    self.state = MulParseState::OpenParenToggleOn;
                } else {
                    self.state = MulParseState::Start;
                }
            }
            MulParseState::Apostrophe => {
                if character == '\'' {
                    self.state = MulParseState::T;
                } else {
                    self.state = MulParseState::Start;
                }
            }
            MulParseState::T => {
                if character == 't' {
                    self.state = MulParseState::OpenParenToggleOff;
                } else {
                    self.state = MulParseState::Start;
                }
            }
        }
    }

    fn extract_instructions(&mut self) -> Vec<Mul> {
        std::mem::take(&mut self.instructions)
    }
}

fn parse_instructions(input: &str, with_toggle: bool) -> Vec<Mul> {
    let mut parser = MulParser::new(with_toggle);
    for character in input.chars() {
        parser.next_state(character);
    }
    parser.extract_instructions()
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // The computer appears to be trying to run a program, but its memory is corrupted. All of the
    // instructions have been jumbled up! There are many invalid characters that should be ignored,
    // even if they look like part of a mul instruction.

    // Part A: Scan the corrupted memory for uncorrupted mul instructions. What do you get if you
    // add up all of the results of the multiplications?
    let program = parse_instructions(&input, false);
    let sum = program.iter().map(|i| i.product()).sum::<u64>();
    solution.set_part_a(sum);

    // Part B: There are two new instructions you'll need to handle: the do() instruction enables
    // future mul instructions and the don't() instruction disables future mul instructions. What
    // do you get if you add up all of the results of just the enabled multiplications?
    let program_toggled = parse_instructions(&input, true);
    let sum_toggled = program_toggled.iter().map(|i| i.product()).sum::<u64>();
    solution.set_part_b(sum_toggled);

    solution
}
