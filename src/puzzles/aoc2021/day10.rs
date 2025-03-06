/*
** src/puzzles/aoc2021/day10.rs
*/

use super::Solution;

fn is_opener(c: char) -> bool {
    matches!(c, '(' | '[' | '{' | '<')
}

fn is_closer(c: char) -> bool {
    matches!(c, ')' | ']' | '}' | '>')
}

fn get_closer(opener: char) -> char {
    match opener {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!(),
    }
}

fn opener_matches_closer(opener: char, closer: char) -> bool {
    match opener {
        '(' => closer == ')',
        '[' => closer == ']',
        '{' => closer == '}',
        '<' => closer == '>',
        _ => unreachable!(),
    }
}

fn get_score(c: char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

fn syntax_error_score(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn first_illegal_character(line: &str) -> Option<char> {
    let mut stack = Vec::new();
    for c in line.chars() {
        if is_opener(c) {
            stack.push(c);
        } else if is_closer(c) {
            // ensure that the top of the stack matches
            let top = stack.pop().unwrap();
            if !opener_matches_closer(top, c) {
                return Some(c);
            }
        }
    }
    None
}

fn complete_with_score(line: &str) -> u64 {
    let mut score = 0;
    let mut stack = Vec::new();

    for c in line.chars() {
        if is_opener(c) {
            stack.push(c);
        } else if is_closer(c) {
            let _ = stack.pop().unwrap();
        }
    }

    // match un-closed openers to complete the line
    while let Some(opener) = stack.pop() {
        let closer = get_closer(opener);
        score = (score * 5) + get_score(closer);
    }

    score
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let lines = input.split('\n').collect::<Vec<_>>();

    // Part A: Find the first illegal character in each corrupted line of the navigation subsystem.
    // What is the total syntax error score for those errors?
    let syntax_error_score = lines
        .iter()
        .flat_map(|line| first_illegal_character(line))
        .map(syntax_error_score)
        .sum::<u64>();
    solution.set_part_a(syntax_error_score);

    // Part B: Find the completion string for each incomplete line, score the completion strings,
    // and sort the scores. What is the middle score?
    let mut completion_scores = lines
        .iter()
        .filter(|line| first_illegal_character(line).is_none())
        .map(|line| complete_with_score(line))
        .collect::<Vec<_>>();
    completion_scores.sort();
    let score = completion_scores[completion_scores.len() / 2];
    solution.set_part_b(score);

    solution
}
