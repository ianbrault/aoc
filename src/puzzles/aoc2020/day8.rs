/*
** src/puzzles/aoc2020/day8.rs
*/

use super::Solution;

use std::collections::BTreeSet;

#[derive(Clone, Copy, PartialEq)]
enum Operation {
    Accumulate,
    Jump,
    NoOp,
}

impl From<&str> for Operation {
    fn from(s: &str) -> Self {
        match s {
            "acc" => Self::Accumulate,
            "jmp" => Self::Jump,
            "nop" => Self::NoOp,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy)]
struct Instruction {
    op: Operation,
    n: i64,
}

impl Instruction {
    fn new(op: Operation, n: i64) -> Self {
        Self { op, n }
    }
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        Self {
            op: Operation::from(&s[0..3]),
            n: s[4..s.len()].parse().unwrap(),
        }
    }
}

struct Program {
    acc: i64,
    pc: i64,
    terminated: bool,
}

impl Program {
    fn new() -> Self {
        Self {
            acc: 0,
            pc: 0,
            terminated: false,
        }
    }

    // runs the instructions until the program terminates or an infinite loop
    // is detected and returns the value of the accumulator
    fn run(&mut self, instructions: &[Instruction]) -> i64 {
        // track the past values of the program counter
        let mut pc_hist = BTreeSet::new();

        let mut running = true;
        while running {
            let instr = &instructions[self.pc as usize];
            // store the program counter for the current instruction
            pc_hist.insert(self.pc);

            match instr.op {
                Operation::Accumulate => {
                    self.acc += instr.n;
                    self.pc += 1;
                }
                Operation::Jump => {
                    self.pc += instr.n;
                }
                Operation::NoOp => {
                    self.pc += 1;
                }
            };

            // check if the new program counter value has already been executed
            if pc_hist.contains(&self.pc) {
                // infinite loop detected, stop running the program but do NOT
                // mark the program as terminated
                running = false;
            } else if self.pc as usize == instructions.len() {
                // program terminated nominally
                self.terminated = true;
                running = false;
            }
        }

        self.acc
    }
}

fn fix_program(instructions: &[Instruction]) -> Option<i64> {
    let opposite = |op: &Operation| match op {
        Operation::Jump => Operation::NoOp,
        Operation::NoOp => Operation::Jump,
        _ => unreachable!(),
    };

    // for each jmp/nop instruction, try the program with the opposite
    let mut index = 0;
    while index < instructions.len() {
        // skip any acc instructions
        while instructions[index].op == Operation::Accumulate {
            index += 1;
        }

        let new_instructions = instructions
            .iter()
            .enumerate()
            .map(|(i, &instr)| {
                if i == index {
                    Instruction::new(opposite(&instr.op), instr.n)
                } else {
                    instr
                }
            })
            .collect::<Vec<_>>();

        let mut program = Program::new();
        let rc = program.run(&new_instructions);
        if program.terminated {
            return Some(rc);
        } else {
            index += 1;
        }
    }
    None
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let instructions = input.split('\n').map(Instruction::from).collect::<Vec<_>>();

    // Part A: Immediately before any instruction is executed a second time, what value is in the
    // accumulator?
    let mut program = Program::new();
    let value = program.run(&instructions);
    solution.set_part_a(value);

    // Part B: Fix the program so that it terminates normally by changing exactly one jmp (to nop)
    // or nop (to jmp). What is the value of the accumulator after the program terminates?
    let value = fix_program(&instructions);
    solution.maybe_set_part_b(value);

    solution
}
