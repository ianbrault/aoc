/*
** src/puzzles/aoc2020/day7.rs
*/

use super::Solution;
use crate::utils;

use std::collections::{BTreeSet, HashMap};

struct Rule<'a> {
    bag: &'a str,
    contains: Vec<(u8, &'a str)>,
}

impl<'a> Rule<'a> {
    fn parse_contained_bag(bag: &str) -> (u8, &str) {
        // number of bags is guaranteed to be a single digit
        let n = bag[..1].parse().unwrap();
        let contained_bag = if n == 1 {
            &bag[2..(bag.len() - 4)]
        } else {
            &bag[2..(bag.len() - 5)]
        };
        (n, contained_bag)
    }
}

impl<'a> From<&'a str> for Rule<'a> {
    fn from(value: &'a str) -> Self {
        // ignore the trailing period
        let rule = &value[..(value.len() - 1)];
        let (bag_str, contains_str) = utils::split(rule, " contain ").unwrap();
        let bag = bag_str.strip_suffix(" bags").unwrap();
        // if there are bags contained within, split and parse
        let contains = if contains_str == "no other bags" {
            Vec::new()
        } else {
            contains_str
                .split(", ")
                .map(Self::parse_contained_bag)
                .collect()
        };

        Self { bag, contains }
    }
}

struct BagSolverA<'a> {
    // memoize bags which can contain at least 1 shiny gold bag
    solved_set: BTreeSet<&'a str>,
    // used to store intermediate results i.e. a green bag contains red & blue
    // bags but we do not know if either can contain a gold bag
    // note: this uses a reverse linkage e.g. for the above, the rule would
    // lead to 2 holding cell entries, red->green and blue->green
    // note: the second element is a list since there is a many-to-one
    // containment relationship
    holding_cell: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> BagSolverA<'a> {
    fn new() -> Self {
        Self {
            solved_set: BTreeSet::new(),
            holding_cell: HashMap::new(),
        }
    }

    fn process_rule(&mut self, rule: &Rule<'a>) {
        // check if the bag contains a shiny gold bag
        let contains_gold = rule.contains.iter().any(|(_, b)| *b == "shiny gold");
        // also check if any of the contained bags are in the solved set
        let contains_solved = rule
            .contains
            .iter()
            .any(|(_, b)| self.solved_set.contains(b));

        if contains_gold || contains_solved {
            self.found_solution(rule.bag);
        } else {
            // otherwise, add the rule to the holding cell
            for (_, contained) in rule.contains.iter() {
                let cell = self.holding_cell.entry(contained).or_default();
                cell.push(rule.bag);
            }
        }
    }

    fn found_solution(&mut self, bag: &'a str) {
        // if the bag is or contains a solution, add it to the solved set
        self.solved_set.insert(bag);
        // then check the holding cell for anything which can be solved using
        // our new solution
        if let Some(new_solutions) = self.holding_cell.remove(bag) {
            for new_solution in new_solutions.into_iter() {
                // need to recurse on these new solutions
                self.found_solution(new_solution);
            }
        }
    }
}

struct BagSolverB<'a> {
    // place all rules in a map for quick lookups
    rule_map: HashMap<&'a str, &'a Vec<(u8, &'a str)>>,
}

impl<'a> BagSolverB<'a> {
    fn new(rules: impl Iterator<Item = &'a Rule<'a>>) -> Self {
        let rule_map = rules.map(|r| (r.bag, &r.contains)).collect();

        Self { rule_map }
    }

    fn count_contained_bags(&self, bag: &'a str) -> u64 {
        let contained = self.rule_map.get(bag).unwrap();

        // recurse on any contained bags
        // we could improve by memoizing results, in case different branches of
        // the tree have the same bags, but it is much simpler to map and sum
        // as below, and is still reasonably fast
        contained
            .iter()
            // include 1 for the current bag
            .map(|(n, bag)| (*n as u64) * (1 + self.count_contained_bags(bag)))
            .sum()
    }
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let rules = input.split('\n').map(Rule::from).collect::<Vec<_>>();

    // Part A: How many bag colors can eventually contain at least one shiny gold bag?
    let mut solver = BagSolverA::new();
    for rule in rules.iter() {
        solver.process_rule(rule);
    }
    solution.set_part_a(solver.solved_set.len());

    // Part B: How many individual bags are required inside your single shiny gold bag?
    let solver = BagSolverB::new(rules.iter());
    solution.set_part_b(solver.count_contained_bags("shiny gold"));

    solution
}
