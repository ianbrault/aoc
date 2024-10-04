/*
** src/puzzles/aoc2023/day4.rs
*/

use super::Solution;
use crate::utils;

use std::collections::HashSet;

struct Card {
    winning_numbers: HashSet<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn winning_number_count(&self) -> usize {
        self.numbers
            .iter()
            .filter(|number| self.winning_numbers.contains(number))
            .count()
    }

    fn points(&self) -> u32 {
        let winning_count = self.winning_number_count();
        if winning_count > 0 {
            2_u32.pow(winning_count as u32 - 1)
        } else {
            0
        }
    }
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let card_list = utils::split_tail(value, ":").unwrap();
        let (winning_number_string, number_string) = utils::split(card_list, "|").unwrap();

        let winning_numbers = utils::split_and_parse(winning_number_string).collect();
        let numbers = utils::split_and_parse(number_string).collect();

        Self {
            winning_numbers,
            numbers,
        }
    }
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // Each card has two lists of numbers separated by a vertical bar: a list
    // of winning numbers and then a list of numbers you have.
    let cards = input.split('\n').map(Card::from).collect::<Vec<_>>();

    // Part A: Take a seat in the large pile of colorful cards. How many points are they worth
    // in total?
    let point_total = cards.iter().map(Card::points).sum::<u32>();
    solution.set_part_a(point_total);

    // Part B: Scratchcards cause you to win more scratchcards equal to the number of winning
    // numbers you have. Specifically, you win copies of the scratchcards below the winning card
    // equal to the number of matches. Process all of the original and copied scratchcards until no
    // more scratchcards are won. Including the original set of scratchcards, how many total
    // scratchcards do you end up with?
    let mut copies = vec![1; cards.len()];
    for (i, card) in cards.iter().enumerate() {
        for j in 1..=card.winning_number_count() {
            if i + j < cards.len() {
                copies[i + j] += copies[i];
            }
        }
    }
    let total_cards = copies.into_iter().sum::<u32>();
    solution.set_part_b(total_cards);

    solution
}
