/*
** src/puzzles/aoc2020/day16.rs
*/

use super::Solution;
use crate::utils;

use std::collections::HashMap;

struct TicketField<'a> {
    name: &'a str,
    range_1: (u16, u16),
    range_2: (u16, u16),
}

impl<'a> TicketField<'a> {
    fn is_valid(&self, value: u16) -> bool {
        let (a, b) = self.range_1;
        let (c, d) = self.range_2;
        (value >= a && value <= b) || (value >= c && value <= d)
    }

    fn parse_range(value: &str) -> (u16, u16) {
        let (start, end) = utils::split(value, "-").unwrap();
        (start.parse().unwrap(), end.parse().unwrap())
    }
}

impl<'a> From<&'a str> for TicketField<'a> {
    fn from(value: &'a str) -> Self {
        let (name, ranges) = utils::split(value, ": ").unwrap();
        let (range_1_str, range_2_str) = utils::split(ranges, " or ").unwrap();
        let range_1 = Self::parse_range(range_1_str);
        let range_2 = Self::parse_range(range_2_str);
        Self {
            name,
            range_1,
            range_2,
        }
    }
}

struct Ticket {
    fields: Vec<u16>,
}

impl From<&str> for Ticket {
    fn from(s: &str) -> Self {
        let fields = s.split(',').map(|s| s.parse().unwrap()).collect();
        Self { fields }
    }
}

fn valid_for_any_field(fields: &[TicketField<'_>], value: u16) -> bool {
    fields.iter().any(|f| f.is_valid(value))
}

fn derive_field_names<'a>(tickets: &[Ticket], fields: &[TicketField<'a>]) -> Vec<&'a str> {
    // disregard any ticket with invalid fields
    let valid_tickets = tickets
        .iter()
        .filter(|t| t.fields.iter().all(|&f| valid_for_any_field(fields, f)))
        .collect::<Vec<_>>();

    // there is not a clean one-to-one mapping; do an initial pass to assign all possibilities
    let mut field_names = HashMap::new();
    for field in fields.iter() {
        let mut valid = Vec::with_capacity(fields.len());
        for nf in 0..fields.len() {
            if valid_tickets.iter().all(|t| field.is_valid(t.fields[nf])) {
                valid.push(nf);
            }
        }
        field_names.insert(field.name, valid);
    }

    // now we can greedily assign names to the fields: there should be one field with only a single
    // possibility - assign it and remove from all other field possibilities; there should now be
    // another field with only a single possibility, and this chain will continue until all fields
    // have been assigned
    let mut field_names_final = vec![""; fields.len()];
    for _ in 0..fields.len() {
        // find the field with a single possibility
        let (field_name, field_index) = field_names.iter().find(|(_, v)| v.len() == 1).unwrap();
        let (field_name, field_index) = (*field_name, field_index[0]);
        field_names_final[field_index] = field_name;
        // remove as a possibility from other fields
        for (_, possible_fields) in field_names.iter_mut() {
            if possible_fields.contains(&field_index) {
                let i = possible_fields
                    .iter()
                    .position(|&x| x == field_index)
                    .unwrap();
                possible_fields.remove(i);
            }
        }
    }

    field_names_final
}

fn parse_input(input: &str) -> (Vec<TicketField<'_>>, Ticket, Vec<Ticket>) {
    let mut chunks = input
        .split("\n\n")
        .map(|chunk| chunk.split('\n'))
        .collect::<Vec<_>>();
    let fields = chunks[0].clone().map(TicketField::from).collect();
    let my_ticket = Ticket::from(chunks[1].nth(1).unwrap());
    let nearby_tickets = chunks[2].clone().skip(1).map(Ticket::from).collect();
    (fields, my_ticket, nearby_tickets)
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let (fields, my_ticket, nearby_tickets) = parse_input(input.as_str());

    // Part A: Consider the validity of the nearby tickets you scanned. What is your ticket
    // scanning error rate?
    let error_rate = nearby_tickets
        .iter()
        .map(|ticket| {
            ticket
                .fields
                .iter()
                .filter(|&&f| !valid_for_any_field(&fields, f))
                .sum::<u16>() as u64
        })
        .sum::<u64>();
    solution.set_part_a(error_rate);

    // Part B: Once you work out which field is which, look for the six fields on your ticket that
    // start with the word departure. What do you get if you multiply those six values together?
    let field_names = derive_field_names(&nearby_tickets, &fields);
    let answer = my_ticket
        .fields
        .iter()
        .zip(field_names.iter())
        .filter(|(_, fname)| fname.starts_with("departure"))
        .fold(1u64, |acc, (&field, _)| acc * field as u64);
    solution.set_part_b(answer);

    solution
}
