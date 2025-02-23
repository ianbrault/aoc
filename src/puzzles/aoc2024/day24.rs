/*
** src/puzzles/aoc2024/day24.rs
*/

use super::Solution;
use crate::utils;

use log::debug;

use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Connection<'a> {
    input_a: &'a str,
    input_b: &'a str,
    output: &'a str,
    operation: &'a str,
}

impl<'a> Connection<'a> {
    fn wires(&self) -> Vec<&str> {
        vec![self.input_a, self.input_b, self.output]
    }

    fn is_output(&self) -> bool {
        self.output.starts_with('z')
    }

    fn output_number(&self) -> Option<usize> {
        if self.is_output() {
            self.output[1..].parse().ok()
        } else {
            None
        }
    }
}

impl<'a> From<&'a str> for Connection<'a> {
    fn from(value: &'a str) -> Self {
        let chars = value.chars().collect::<Vec<_>>();
        let op_len = if chars[4] == 'O' { 2 } else { 3 };
        let input_a = &value[..3];
        let operation = &value[4..(4 + op_len)];
        let input_b = &value[(5 + op_len)..(8 + op_len)];
        let output = &value[(value.len() - 3)..];
        Self {
            input_a,
            input_b,
            output,
            operation,
        }
    }
}

impl<'a> fmt::Debug for Connection<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self.operation {
            "AND" => "&",
            "OR" => "|",
            "XOR" => "^",
            _ => unreachable!(),
        };
        write!(
            f,
            "{} {} {} => {}",
            self.input_a, op, self.input_b, self.output
        )
    }
}

struct Device<'a> {
    bits: usize,
    connections: Vec<Connection<'a>>,
    registers: HashMap<&'a str, Option<u8>>,
}

impl<'a> Device<'a> {
    fn new(initial_values: Vec<(&'a str, u8)>, connections: Vec<Connection<'a>>) -> Self {
        let mut registers = HashMap::new();
        for connection in connections.iter() {
            registers.insert(connection.input_a, None);
            registers.insert(connection.input_b, None);
            registers.insert(connection.output, None);
        }
        for &(register, value) in initial_values.iter() {
            registers.insert(register, Some(value));
        }

        let mut bits = 0;
        if let Some(connection) = connections
            .iter()
            .filter(|c| c.output.starts_with('z'))
            .max_by_key(|c| &c.output)
        {
            if let Ok(n) = connection.output[1..].parse::<usize>() {
                bits = n + 1;
            }
        }

        Self {
            bits,
            connections,
            registers,
        }
    }

    fn operation(a: u8, b: u8, op: &str) -> u8 {
        match op {
            "AND" => a & b,
            "OR" => a | b,
            "XOR" => a ^ b,
            _ => unreachable!(),
        }
    }

    fn z(&self) -> Option<u64> {
        let mut number = 0;
        for (register, value) in self.registers.iter().filter(|(k, _)| k.starts_with('z')) {
            let index = register[1..].parse::<usize>().unwrap();
            if let Some(value) = value {
                number |= (*value as u64) << index;
            } else {
                return None;
            }
        }
        Some(number)
    }

    fn register(&self, name: &'a str) -> Option<u8> {
        self.registers.get(&name).cloned()?
    }

    fn complete(&self) -> bool {
        for (name, value) in self.registers.iter() {
            if name.starts_with('z') && value.is_none() {
                return false;
            }
        }
        true
    }

    fn set_register(&mut self, name: &'a str, value: u8) {
        self.registers.insert(name, Some(value));
    }

    fn execute_cycle(&mut self) {
        for i in 0..self.connections.len() {
            let a = self.register(self.connections[i].input_a);
            let b = self.register(self.connections[i].input_b);
            if let (Some(a), Some(b)) = (a, b) {
                let c = Self::operation(a, b, self.connections[i].operation);
                self.set_register(self.connections[i].output, c);
            }
        }
    }
}

impl<'a> From<&'a str> for Device<'a> {
    fn from(value: &'a str) -> Self {
        let (initial_values_str, connections_str) = utils::split(value, "\n\n").unwrap();
        let mut initial_values = Vec::new();
        for line in initial_values_str.split('\n') {
            let input = &line[..3];
            let value = line.chars().last().unwrap().to_digit(10).unwrap() as u8;
            initial_values.push((input, value));
        }
        let connections = connections_str
            .split('\n')
            .map(Connection::from)
            .collect::<Vec<_>>();
        Self::new(initial_values, connections)
    }
}

fn find_output(device: &mut Device) -> u64 {
    while !device.complete() {
        device.execute_cycle();
    }
    device.z().unwrap()
}

fn incorrect_wires<'a>(device: &'a Device) -> Vec<&'a Connection<'a>> {
    let mut incorrect = HashSet::new();

    // this device is a ripple-carry adder so gates between registers must
    // satisfy the following conditions:
    for connection in device.connections.iter() {
        // all connections with Z outputs must be XOR operations, excluding the most-significant
        // bit which has no carry-out signal
        if connection.is_output()
            && connection.operation != "XOR"
            && connection.output_number().unwrap_or(64) + 1 != device.bits
        {
            debug!("condition A: {:?}", connection);
            incorrect.insert(connection);
        }
        // all connections with XOR gates must have input or output wires, no intermediaries
        if connection.operation == "XOR"
            && connection
                .wires()
                .into_iter()
                .all(|w| !w.starts_with('x') && !w.starts_with('y') && !w.starts_with('z'))
        {
            debug!("condition B: {:?}", connection);
            incorrect.insert(connection);
        }
        // connections with AND gates must be connected to OR gates, excluding the input X00
        // connection which has no carry-in signal
        if connection.operation == "AND"
            && connection.input_a != "x00"
            && connection.input_b != "x00"
        {
            for subconnection in device.connections.iter() {
                if (connection.output == subconnection.input_a
                    || connection.output == subconnection.input_b)
                    && subconnection.operation != "OR"
                {
                    debug!("condition C: {:?}", connection);
                    incorrect.insert(connection);
                }
            }
        }
        // connections with XOR gates must not be connected to OR gates
        if connection.operation == "XOR" {
            for subconnection in device.connections.iter() {
                if (connection.output == subconnection.input_a
                    || connection.output == subconnection.input_b)
                    && subconnection.operation == "OR"
                {
                    debug!("condition D: {:?}", connection);
                    incorrect.insert(connection);
                }
            }
        }
    }

    incorrect.into_iter().collect()
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // The device seems to be trying to produce a number through some boolean logic gates. Each
    // gate has two inputs and one output. The gates all operate on values that are either true (1)
    // or false (0).
    let mut device = Device::from(input.as_str());

    // Part A: Simulate the system of gates and wires. What decimal number does it output on the
    // wires starting with z?
    let output = find_output(&mut device);
    solution.set_part_a(output);

    // Part B: Your system of gates and wires has four pairs of gates which need their output wires
    // swapped - eight wires in total. Determine which four pairs of gates need their outputs
    // swapped so that your system correctly performs addition; what do you get if you sort the
    // names of the eight wires involved in a swap and then join those names with commas?
    let connections = incorrect_wires(&device);
    let mut wires = connections
        .into_iter()
        .map(|c| c.output)
        .collect::<Vec<_>>();
    wires.sort();
    solution.set_part_b(wires.join(","));

    solution
}
