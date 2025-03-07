/*
** src/puzzles/aoc2020/day6.rs
*/

use super::Solution;
use crate::types::Counter;

use std::collections::BTreeSet;

fn unique_questions_yes(groups: &[&str]) -> usize {
    let mut sum = 0;
    for group in groups {
        let mut unique_answers = BTreeSet::new();
        for line in group.split('\n') {
            for c in line.chars() {
                unique_answers.insert(c);
            }
        }
        sum += unique_answers.len();
    }
    sum
}

fn questions_all_yes(groups: &[&str]) -> usize {
    let mut sum = 0;
    // for each group, track number of respondents and frequency of each answer
    // the number of questions to which everyone answered yes is each entry where the count is
    // the number of respondents
    for group in groups {
        let mut counter = Counter::new();
        let lines = group.split('\n').collect::<Vec<_>>();
        for response in lines.iter() {
            counter.extend(response.chars());
        }
        let all_answered = counter
            .iter()
            .filter(|&(_, count)| *count == lines.len())
            .count();
        sum += all_answered;
    }
    sum
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let groups = input.split("\n\n").collect::<Vec<_>>();

    // Part A: What is the sum of the number of unique questions answered "yes" to in each group?
    let sum = unique_questions_yes(&groups);
    solution.set_part_a(sum);

    // Part B: For each group, count the number of questions to which everyone answered "yes". What
    // is the sum of those counts?
    let sum = questions_all_yes(&groups);
    solution.set_part_b(sum);

    solution
}
