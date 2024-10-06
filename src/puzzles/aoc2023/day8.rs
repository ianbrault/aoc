/*
** src/puzzles/aoc2023/day8.rs
*/

use super::Solution;
use crate::utils;

use log::debug;

use std::collections::HashMap;

enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Direction::From<char>: invalid character: {}", value),
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Node(String);

impl Node {
    fn is_ghost_start(&self) -> bool {
        self.0.ends_with('A')
    }

    fn is_ghost_end(&self) -> bool {
        self.0.ends_with('Z')
    }

    fn start() -> Self {
        Self("AAA".into())
    }

    fn end() -> Self {
        Self("ZZZ".into())
    }
}

struct Map {
    instructions: Vec<Direction>,
    nodes: HashMap<Node, (Node, Node)>,
}

impl Map {
    fn next_node(&self, from: &Node, direction: &Direction) -> &Node {
        let (a, b) = self.nodes.get(from).unwrap();
        match direction {
            Direction::Left => a,
            Direction::Right => b,
        }
    }

    fn travel(&self, from: &Node, to: &Node) -> usize {
        let mut current = from;
        let mut steps = 0;

        for direction in self.instructions.iter().cycle() {
            current = self.next_node(current, direction);
            steps += 1;
            if current == to {
                break;
            }
        }

        steps
    }

    fn travel_to_end(&self) -> usize {
        self.travel(&Node::start(), &Node::end())
    }

    fn travel_to_end_as_ghost(&self) -> usize {
        // from experimentation: all start nodes will hit an end node and will then continue to
        // loop back to that same end node, so simply find each end node and the overall solution
        // is the least common multiple of each individual solution
        let start_nodes = self.nodes.keys().filter(|node| node.is_ghost_start()).collect::<Vec<_>>();
        let mut steps_to_end_node = Vec::<u64>::with_capacity(start_nodes.len());

        for start in start_nodes.into_iter() {
            let mut node = start;
            let mut steps = 0;
            for direction in self.instructions.iter().cycle() {
                node = self.next_node(node, direction);
                steps += 1;
                if node.is_ghost_end() {
                    steps_to_end_node.push(steps);
                    debug!("start node {} reached end node {} in {} steps", start.0, node.0, steps);
                    break;
                }
            }
        }
        if steps_to_end_node.len() == 1 {
            return steps_to_end_node[0] as usize;
        }

        let mut steps_final = utils::lcm(steps_to_end_node[0], steps_to_end_node[1]);
        for &steps in steps_to_end_node[2..].iter() {
            steps_final = utils::lcm(steps_final, steps);
        }
        steps_final as usize
    }
}

impl From<String> for Map {
    fn from(value: String) -> Self {
        let (instruction_string, node_strings) = utils::split(&value, "\n\n").unwrap();
        let instructions = instruction_string.chars().map(Direction::from).collect();
        let mut nodes = HashMap::new();
        for node_string in node_strings.split('\n') {
            let (source, targets) = utils::split(node_string, " = ").unwrap();
            let (target_a, target_b) = utils::split(&targets[1..(targets.len() - 1)], ", ").unwrap();
            nodes.insert(Node(source.into()), (Node(target_a.into()), Node(target_b.into())));
        }
        Self { instructions, nodes }
    }
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // One of the documents contains a list of left/right instructions, and the rest of the
    // documents seem to describe some kind of network of labeled nodes.
    let map = Map::from(input);

    // Part A: Starting at AAA, follow the left/right instructions. How many steps are required to
    // reach ZZZ?
    let steps = map.travel_to_end();
    solution.set_part_a(steps);

    // Part B: Simultaneously start on every node that ends with A. How many steps does it take
    // before you're only on nodes that end with Z?
    let ghost_steps = map.travel_to_end_as_ghost();
    solution.set_part_b(ghost_steps);

    solution
}
