/*
** src/puzzles/aoc2025/day10.rs
*/

use super::Solution;
use crate::itertools::*;

use std::cmp;
use std::collections::HashMap;

type Button = Vec<usize>;
type JoltageList = Vec<i16>;
type Lights = u16;

struct Machine {
    indicator_lights: Lights,
    button_combinations: HashMap<Lights, Vec<Vec<Button>>>,
    joltage_requirements: JoltageList,
}

impl Machine {
    fn bifurcate(joltages: &JoltageList) -> JoltageList {
        joltages.iter().map(|j| j / 2).collect()
    }

    fn button_press_combinations(buttons: &[Vec<usize>]) -> HashMap<Lights, Vec<Vec<Button>>> {
        let mut button_presses = HashMap::new();

        for count in 0..=buttons.len() {
            for buttons in buttons.iter().combinations(count) {
                let mut lights = 0;
                for button in buttons.iter() {
                    for i in button.iter() {
                        lights ^= 1 << i;
                    }
                }
                let entry = button_presses.entry(lights).or_insert(Vec::new());
                entry.push(buttons.into_iter().cloned().collect());
            }
        }

        button_presses
    }

    fn configure_lights(&self) -> usize {
        self.button_combinations[&self.indicator_lights]
            .iter()
            .map(|presses| presses.len())
            .min()
            .unwrap()
    }

    fn configure_joltages(&self, joltages: JoltageList) -> Option<usize> {
        // Solution cribbed from https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/
        if joltages.iter().all(|j| *j == 0) {
            return Some(0);
        }

        let mut parity = 0;
        for (i, j) in joltages.iter().enumerate() {
            parity |= (*j as u16 & 1) << i;
        }

        let mut solution = None;
        for buttons in self
            .button_combinations
            .get(&parity)
            .unwrap_or(&Vec::new())
            .iter()
        {
            let mut next_joltages = joltages.clone();
            for button in buttons {
                for i in button {
                    next_joltages[*i] -= 1;
                }
            }
            if next_joltages.iter().any(|j| *j < 0) {
                continue;
            }

            // Bifurcate and recurse
            let joltages_halved = Self::bifurcate(&next_joltages);
            if let Some(presses) = self.configure_joltages(joltages_halved) {
                let total_presses = buttons.len() + (2 * presses);
                solution = if solution.is_none() {
                    Some(total_presses)
                } else {
                    solution.map(|p| cmp::min(p, total_presses))
                };
            }
        }
        solution
    }
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let parts = value.split_ascii_whitespace().collect::<Vec<_>>();

        let mut indicator_lights = 0;
        let indicator_diagram = parts.first().unwrap();
        for (i, c) in indicator_diagram[1..(indicator_diagram.len() - 1)]
            .chars()
            .enumerate()
        {
            if c == '#' {
                indicator_lights |= 1 << i;
            }
        }

        let button_wiring = parts[1..(parts.len() - 1)]
            .iter()
            .map(|wiring| {
                wiring[1..(wiring.len() - 1)]
                    .split(',')
                    .map(|n| n.parse().unwrap())
                    .collect()
            })
            .collect::<Vec<_>>();
        // Pre-compute the possible button press combinations
        let mut button_combinations = Self::button_press_combinations(&button_wiring);
        // Ensure the zero-press combination shows up for no lights
        let entry = button_combinations.entry(0).or_insert(Vec::new());
        entry.insert(0, Vec::new());

        let joltage_list = parts.last().unwrap();
        let joltage_requirements = joltage_list[1..(joltage_list.len() - 1)]
            .split(',')
            .map(|joltage| joltage.parse().unwrap())
            .collect();

        Self {
            indicator_lights,
            button_combinations,
            joltage_requirements,
        }
    }
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // The Elves do have the manual for the machines, but the section detailing the initialization
    // procedure was eaten by a Shiba Inu. All that remains of the manual are some indicator light
    // diagrams, button wiring schematics, and joltage requirements for each machine.
    let machines = input.split('\n').map(Machine::from).collect::<Vec<_>>();

    // Part A: Analyze each machine's indicator light diagram and button wiring schematics. What is
    // the fewest button presses required to correctly configure the indicator lights on all of
    // the machines?
    let button_presses = machines
        .iter()
        .map(|machine| machine.configure_lights())
        .sum::<usize>();
    solution.set_part_a(button_presses);

    // Part B: Analyze each machine's joltage requirements and button wiring schematics. What is
    // the fewest button presses required to correctly configure the joltage level counters on all
    // of the machines?
    let joltage_presses = machines
        .iter()
        .filter_map(|machine| machine.configure_joltages(machine.joltage_requirements.clone()))
        .sum::<usize>();
    solution.set_part_b(joltage_presses);

    solution
}
