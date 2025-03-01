/*
** src/puzzles/aoc2022/day10.rs
*/

use super::Solution;

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i64),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let sep = if let Some(i) = value.find(' ') {
            i
        } else {
            value.len()
        };
        match &value[..sep] {
            "noop" => Self::Noop,
            "addx" => {
                let n = value[(sep + 1)..].parse().unwrap();
                Self::Addx(n)
            }
            _ => unreachable!(),
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
struct CPU {
    register: i64,
    cycle: u64,
    signal_strengths: Vec<i64>,
    image: String,
}

impl CPU {
    fn new() -> Self {
        Self {
            register: 1,
            cycle: 1,
            signal_strengths: Vec::new(),
            // image will always start with "#"
            image: String::from('#'),
        }
    }

    fn draw_pixel(&mut self) {
        // move to the next line of the image on each 40th cycle
        if self.cycle % 40 == 0 {
            self.image.push('\n');
        }
        let pixel_pos = self.cycle as i64 % 40;
        let sprite_start = self.register - 1;
        let sprite_end = self.register + 1;
        let pixel = if pixel_pos >= sprite_start && pixel_pos <= sprite_end {
            '#'
        } else {
            '.'
        };
        self.image.push(pixel);
    }

    fn next_cycle(&mut self) {
        // draw the pixel at the start of the cycle
        self.draw_pixel();
        self.cycle += 1;
        // check if the cycle is notable and log the signal strength if so
        if (self.cycle as i64 - 20) % 40 == 0 {
            self.signal_strengths
                .push(self.register * self.cycle as i64);
        }
    }

    fn process_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Noop => {
                // no operation, increment the cycle and advance to the next
                // instruction
                self.next_cycle();
            }
            Instruction::Addx(n) => {
                // addx takes 2 cycles
                // the first cycle has no effect
                self.next_cycle();
                // the value is added to the register at the end of the second
                // cycle, then advance to the next instruction
                self.register += n;
                self.next_cycle();
            }
        }
    }

    fn run_program(&mut self, instructions: &[Instruction]) {
        for instruction in instructions.iter() {
            self.process_instruction(instruction);
        }
    }
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let instructions = input.split('\n').map(Instruction::from).collect::<Vec<_>>();
    let mut cpu = CPU::new();

    // Part A: Find the signal strength during the 20th, 60th, 100th, 140th, 180th, and 220th
    // cycles. What is the sum of these six signal strengths?
    cpu.run_program(&instructions);
    let signal_strength_sum = cpu.signal_strengths.iter().sum::<i64>();
    solution.set_part_a(signal_strength_sum);

    // Part B: Render the image given by your program. What eight capital letters appear on the CRT?
    let image = format!("\n{}", &cpu.image[..cpu.image.len() - 2]);
    solution.set_part_b(image);

    solution
}
