/*
** src/puzzles/aoc2025/day10.rs
*/

use super::Solution;

use std::cmp;
use std::collections::VecDeque;

const JOLTAGE_COUNT: usize = 10;

#[derive(Clone)]
struct JoltageList([u16; JOLTAGE_COUNT + 1]);

impl JoltageList {
    fn get(&self, i: usize) -> u16 {
        self.0[i + 1]
    }

    fn decrement(&mut self, i: usize, by: u16) {
        self.0[i + 1] -= by;
    }

    fn iter(&self) -> impl Iterator<Item = &u16> {
        self.0.iter().skip(1).take(self.0[0] as usize)
    }
}

impl From<Vec<u16>> for JoltageList {
    fn from(value: Vec<u16>) -> Self {
        assert!(value.len() <= JOLTAGE_COUNT);
        let mut inner = [0; JOLTAGE_COUNT + 1];
        inner[0] = value.len() as u16;
        for (i, v) in value.into_iter().enumerate() {
            inner[i + 1] = v;
        }
        Self(inner)
    }
}

struct ButtonPressCombinations {
    combinations: Vec<u16>,
    maxima: Vec<u16>,
}

impl ButtonPressCombinations {
    fn distribute_presses(mut m: u16, maxima: &[u16], combinations: &mut Vec<u16>) {
        for (i, slot) in combinations.iter_mut().enumerate().rev() {
            let value = cmp::min(m, maxima[i]);
            *slot = value;
            m -= value;
            if m == 0 {
                break;
            }
        }
        if m != 0 {
            combinations.clear();
        }
    }

    fn new(n: usize, m: u16, maxima: Vec<u16>) -> Self {
        let mut combinations = vec![0; n];
        Self::distribute_presses(m, &maxima, &mut combinations);
        Self {
            combinations,
            maxima,
        }
    }
}

impl Iterator for ButtonPressCombinations {
    type Item = Vec<u16>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.combinations.is_empty() {
            return None;
        }
        let combo = self.combinations.clone();

        let mut i = self.combinations.iter().rposition(|v| *v != 0).unwrap();
        let mut distribution = 0;
        loop {
            if i == 0 {
                self.combinations.clear();
                break;
            }
            distribution += self.combinations[i] - 1;
            self.combinations[i - 1] += 1;
            self.combinations[i] = 0;
            i -= 1;
            if self.combinations[i] <= self.maxima[i] {
                break;
            }
        }
        Self::distribute_presses(distribution, &self.maxima, &mut self.combinations);

        Some(combo)
    }
}

struct Machine {
    indicator_lights: u16,
    button_wiring: Vec<Vec<u8>>,
    joltage_requirements: JoltageList,
}

impl Machine {
    fn press_button_lights(&self, index: usize, state: u16) -> u16 {
        let mut next = state;
        for &i in self.button_wiring[index].iter() {
            next ^= 1 << i;
        }
        next
    }

    fn fewest_button_presses_lights(&self) -> usize {
        let mut queue = VecDeque::new();
        queue.push_back((0, 0));

        while let Some((state, pushes)) = queue.pop_front() {
            if state == self.indicator_lights {
                return pushes;
            }
            for index in 0..self.button_wiring.len() {
                let next_state = self.press_button_lights(index, state);
                if next_state != 0 {
                    queue.push_back((next_state, pushes + 1));
                }
            }
        }
        unreachable!()
    }

    fn press_button_joltages(&self, button_index: usize, count: u16, state: &mut JoltageList) {
        for i in self.button_wiring[button_index].iter() {
            state.decrement(*i as usize, count);
        }
    }

    fn wiring_count(&self, joltage_index: usize, button_mask: u16) -> usize {
        self.button_wiring
            .iter()
            .enumerate()
            .map(|(i, wiring)| {
                if button_mask & (1 << i) != 0 {
                    wiring
                        .iter()
                        .filter(|&&i| i as usize == joltage_index)
                        .count()
                } else {
                    0
                }
            })
            .sum()
    }

    fn fewest_button_presses_joltages_rec(
        &self,
        state: JoltageList,
        button_mask: u16,
        pushes: usize,
        current_best: &mut usize,
    ) -> Option<usize> {
        if state.iter().all(|&joltage| joltage == 0) {
            *current_best = pushes;
            return Some(pushes);
        }
        if pushes >= *current_best {
            return None;
        }

        // Find the output joltage with the fewest number of buttons wired to it
        let (joltage_index, joltage) = state
            .iter()
            .enumerate()
            .filter(|(_, j)| **j > 0)
            .min_by_key(|(i, j)| (self.wiring_count(*i, button_mask), -(**j as i32)))
            .unwrap();

        let mut button_indices = Vec::new();
        for i in 0..self.button_wiring.len() {
            if button_mask & (1 << i) != 0 && self.button_wiring[i].contains(&(joltage_index as u8))
            {
                button_indices.push(i);
            }
        }
        // Find the maximum number of presses for each button
        let mut maximum_presses = vec![0; button_indices.len()];
        for (i, button_index) in button_indices.iter().enumerate() {
            maximum_presses[i] = u16::MAX;
            for j in self.button_wiring[*button_index].iter() {
                maximum_presses[i] = cmp::min(maximum_presses[i], state.get(*j as usize));
            }
        }

        if button_indices.is_empty() {
            None
        } else {
            // Remove buttons affecting the current joltage from the mask
            let mut next_button_mask = button_mask;
            for button_index in button_indices.iter() {
                next_button_mask ^= 1 << button_index;
            }

            // Recurse over different combinations of button presses
            ButtonPressCombinations::new(button_indices.len(), *joltage, maximum_presses)
                .filter_map(|press_counts| {
                    let mut next_state = state.clone();
                    let mut next_pushes = pushes;
                    for (i, count) in press_counts.into_iter().enumerate() {
                        self.press_button_joltages(button_indices[i], count, &mut next_state);
                        next_pushes += count as usize;
                    }
                    self.fewest_button_presses_joltages_rec(
                        next_state,
                        next_button_mask,
                        next_pushes,
                        current_best,
                    )
                })
                .min()
        }
    }

    fn fewest_button_presses_joltages(&self) -> Option<usize> {
        let mut button_mask = 0;
        for i in 0..self.button_wiring.len() {
            button_mask |= 1 << i;
        }
        let mut current_best = usize::MAX;

        self.fewest_button_presses_joltages_rec(
            self.joltage_requirements.clone(),
            button_mask,
            0,
            &mut current_best,
        )
    }
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let mut parts = value.split_ascii_whitespace().collect::<VecDeque<_>>();

        let indicator_diagram = parts.pop_front().unwrap();
        let mut indicator_lights = 0;
        for (i, c) in indicator_diagram[1..(indicator_diagram.len() - 1)]
            .chars()
            .enumerate()
        {
            if c == '#' {
                indicator_lights |= 1 << i;
            }
        }

        let joltage_list = parts.pop_back().unwrap();
        let joltage_requirements_vec = joltage_list[1..(joltage_list.len() - 1)]
            .split(',')
            .map(|joltage| joltage.parse().unwrap())
            .collect::<Vec<_>>();
        let joltage_requirements = JoltageList::from(joltage_requirements_vec);

        let button_wiring = parts
            .into_iter()
            .map(|wiring| {
                wiring[1..(wiring.len() - 1)]
                    .split(',')
                    .map(|index| index.parse().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Self {
            indicator_lights,
            joltage_requirements,
            button_wiring,
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
    let button_presses_lights = machines
        .iter()
        .map(|machine| machine.fewest_button_presses_lights())
        .sum::<usize>();
    solution.set_part_a(button_presses_lights);

    // Part B: Analyze each machine's joltage requirements and button wiring schematics. What is
    // the fewest button presses required to correctly configure the joltage level counters on all
    // of the machines?
    let button_presses_joltages = machines
        .iter()
        .filter_map(|machine| machine.fewest_button_presses_joltages())
        .sum::<usize>();
    solution.set_part_b(button_presses_joltages);

    solution
}
