/*
** src/puzzles/aoc2024/day17.rs
*/

use super::Solution;
use crate::utils;

use log::debug;

#[derive(Clone, Copy)]
#[repr(u8)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
struct Computer {
    registers: [u64; 3],
    program: Vec<Opcode>,
    instruction_pointer: usize,
    outputs: Vec<u64>,
}

impl Computer {
    fn combo_operand(&self, operand: u8) -> u64 {
        match operand {
            0..=3 => operand as u64,
            4..=6 => self.registers[operand as usize - 4],
            7 => unreachable!(),
            _ => unreachable!(),
        }
    }

    fn next(&mut self) -> bool {
        if self.instruction_pointer >= self.program.len() {
            return true;
        }
        let instruction = self.program[self.instruction_pointer];
        let operand = self.program[self.instruction_pointer + 1] as u8;
        let mut jump = false;
        match instruction {
            Opcode::Adv => {
                let numerator = self.registers[0];
                let denominator = 2u64.pow(self.combo_operand(operand) as u32);
                self.registers[0] = numerator / denominator;
                debug!("A /= {} = {}", denominator, self.registers[0]);
            }
            Opcode::Bxl => {
                self.registers[1] ^= operand as u64;
                debug!("B ^= {} = {}", operand, self.registers[1]);
            }
            Opcode::Bst => {
                self.registers[1] = self.combo_operand(operand) % 8;
                debug!(
                    "B = {} % 8 = {}",
                    self.combo_operand(operand),
                    self.registers[1]
                );
            }
            Opcode::Jnz => {
                if self.registers[0] > 0 {
                    self.instruction_pointer = operand as usize;
                    jump = true;
                    debug!("A = {}: JUMP to {}", self.registers[0], operand);
                } else {
                    debug!("A = {}: NO JUMP", self.registers[0]);
                }
            }
            Opcode::Bxc => {
                self.registers[1] ^= self.registers[2];
                debug!("B ^= {} = {}", self.registers[2], self.registers[1]);
            }
            Opcode::Out => {
                let output = self.combo_operand(operand) % 8;
                self.outputs.push(output);
                debug!("OUTPUT {} % 8 = {}", self.combo_operand(operand), output);
            }
            Opcode::Bdv => {
                let numerator = self.registers[0];
                let denominator = 2u64.pow(self.combo_operand(operand) as u32);
                self.registers[1] = numerator / denominator;
                debug!(
                    "B = {} / {} = {}",
                    numerator, denominator, self.registers[1]
                );
            }
            Opcode::Cdv => {
                let numerator = self.registers[0];
                let denominator = 2u64.pow(self.combo_operand(operand) as u32);
                self.registers[2] = numerator / denominator;
                debug!(
                    "C = {} / {} = {}",
                    numerator, denominator, self.registers[2]
                );
            }
        }
        if !jump {
            self.instruction_pointer += 2;
            debug!("IP = {}", self.instruction_pointer);
        }
        false
    }
}

impl From<String> for Computer {
    fn from(value: String) -> Self {
        let (register_list, program_str) = utils::split(&value, "\n\n").unwrap();
        let registers = register_list
            .split('\n')
            .map(|line| utils::split_tail(line, ": ").unwrap().parse().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let program_list = utils::split_tail(program_str, ": ").unwrap();
        let program = program_list
            .split(',')
            .map(|instruction| instruction.parse::<u8>().unwrap().into())
            .collect();
        Self {
            registers,
            program,
            instruction_pointer: 0,
            outputs: Vec::new(),
        }
    }
}

fn output(a: u64) -> u64 {
    // see below for decompilation
    let mut b = (a & 0x7) ^ 7;
    let c = a >> b;
    b ^= c;
    b ^= 7;
    b & 0x7
}

fn find_quine_rec(computer: &Computer, register_a: u64, program_index: usize) -> Option<u64> {
    let expected = computer.program[program_index] as u64;

    let mut options = Vec::new();
    for a in register_a..(register_a + 8) {
        if output(a) == expected {
            options.push(a);
        }
    }
    if program_index == 0 {
        options.into_iter().min()
    } else {
        options
            .into_iter()
            .filter_map(|a| find_quine_rec(computer, a << 3, program_index - 1))
            .min()
    }
}

fn find_quine(computer: &Computer) -> Option<u64> {
    // From hand de-compiling the program:
    // IP=0:  B = A % 8 (= A & 0x7)
    // IP=2:  B = B ^ 7
    // IP=4:  C = A / (2**B) (= A >> B)
    // IP=6:  A = A / (2**3) (= A >> 3)
    // IP=8:  B = B ^ C
    // IP=10: B = B ^ 7
    // IP=12: OUTPUT B % 8 (= B & 0x7)
    // IP=14: JUMP if nonzero to IP=0
    //
    // So the program is a loop which runs until A is 0 and divides A with truncation once each
    // loop iteration, outputting B % 8 after performing calculations across each register. This
    // gives us a minimum value for A and the series of calculations simplifies to:
    // 0: B = (A & 0x7) ^ 7
    // 1: C = A >> B
    // 2: B = (B ^ C) ^ 7
    // 3: A >>= 3
    // 4: OUTPUT B & 0x7
    // 5: JUMP to start if A != 0
    //
    // By replacing modulo with & 0x7 we see that each loop computes the output using only the
    // value of A (B and C are derived exclusively from A) and that each iteration shifts off the
    // 3 least-significant bits of A. This means that we can work backwards and construct the input
    // A value recursively:
    find_quine_rec(computer, 0, computer.program.len() - 1)
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // The small handheld device unfolds into an entire computer! One of the Historians tosses it
    // to you. This seems to be a 3-bit computer: its program is a list of 3-bit numbers. The
    // computer also has three registers named A, B, and C, but these registers can hold any size
    // integer.
    let computer = Computer::from(input);

    // Part A: Using the information provided by the debugger, initialize the registers to the
    // given values, then run the program. Once it halts, what do you get if you use commas to join
    // the values it output into a single string?
    let mut computer_a = computer.clone();
    let mut halt = false;
    while !halt {
        halt = computer_a.next();
    }
    let output = computer_a
        .outputs
        .iter()
        .map(|o| o.to_string())
        .collect::<Vec<_>>()
        .join(",");
    solution.set_part_a(output);

    // Part B: What is the lowest positive initial value for register A that causes the program to
    // output a copy of itself?
    let register = find_quine(&computer);
    solution.maybe_set_part_b(register);

    solution
}
