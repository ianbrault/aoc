/*
** src/puzzles/aoc2020/day4.rs
*/

use super::Solution;
use crate::utils;

use std::collections::HashMap;

#[derive(Clone, Copy)]
enum Height {
    Centimeters,
    Inches,
}

impl Height {
    fn parse(value: &str) -> Option<Self> {
        let i = value.find(|c: char| !c.is_ascii_digit())?;
        let n = value[..i].parse::<u64>().ok()?;
        match &value[i..] {
            "cm" => {
                if (150..=193).contains(&n) {
                    return Some(Self::Centimeters);
                }
            }
            "in" => {
                if (59..=76).contains(&n) {
                    return Some(Self::Inches);
                }
            }
            _ => unreachable!(),
        }
        None
    }
}

#[derive(Clone, Copy)]
pub enum EyeColor {
    Amber,
    Blue,
    Brown,
    Gray,
    Green,
    Hazel,
    Other,
}

impl EyeColor {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "amb" => Some(Self::Amber),
            "blu" => Some(Self::Blue),
            "brn" => Some(Self::Brown),
            "gry" => Some(Self::Gray),
            "grn" => Some(Self::Green),
            "hzl" => Some(Self::Hazel),
            "oth" => Some(Self::Other),
            _ => None,
        }
    }
}

// passports have the following fields:
// byr: birth year
// iyr: issue year
// eyr: expiration year
// hgt: height
// hcl: hair color
// ecl: eye color
// pid: passport ID
// cid: country ID (optional)
struct Passport;

impl Passport {
    pub fn has_fields(batch: &str) -> bool {
        // exclude optional cid key
        let mut keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .map(|&k| (k, false))
            .collect::<HashMap<&str, bool>>();
        for entry in batch.split_whitespace() {
            let (key, _) = utils::split(entry, ":").unwrap();
            let entry = keys.entry(key).or_insert(false);
            *entry = true;
        }
        keys.into_iter().all(|(_, v)| v)
    }

    fn parse_year(s: &str, min: u16, max: u16) -> Option<u16> {
        let year = s.parse().ok()?;
        if year >= min && year <= max {
            Some(year)
        } else {
            None
        }
    }

    fn parse_hex(s: &str) -> Option<&str> {
        if !s.starts_with('#') {
            return None;
        }
        let non_hex_digits = s[1..].chars().filter(|c| !c.is_ascii_hexdigit()).count();
        if non_hex_digits == 0 {
            Some(s)
        } else {
            None
        }
    }

    fn parse_pid(s: &str) -> Option<u32> {
        if s.len() != 9 {
            None
        } else {
            s.parse().ok()
        }
    }

    fn parse(batch: &str) -> Option<Self> {
        let mut builder = PassportBuilder::default();
        for entry in batch.split_whitespace() {
            let (key, value) = utils::split(entry, ":")?;
            builder.set(key, value);
        }
        builder.build()
    }
}

// used to construct passports one field at a time
#[derive(Default)]
struct PassportBuilder<'a> {
    byr: Option<u16>,
    iyr: Option<u16>,
    eyr: Option<u16>,
    hgt: Option<Height>,
    hcl: Option<&'a str>,
    ecl: Option<EyeColor>,
    pid: Option<u32>,
    cid: Option<&'a str>,
}

impl<'a> PassportBuilder<'a> {
    fn set(&mut self, key: &str, value: &'a str) {
        match key {
            "byr" => {
                self.byr = Passport::parse_year(value, 1920, 2002);
            }
            "iyr" => {
                self.iyr = Passport::parse_year(value, 2010, 2020);
            }
            "eyr" => {
                self.eyr = Passport::parse_year(value, 2020, 2030);
            }
            "hgt" => {
                self.hgt = Height::parse(value);
            }
            "hcl" => {
                self.hcl = Passport::parse_hex(value);
            }
            "ecl" => {
                self.ecl = EyeColor::parse(value);
            }
            "pid" => {
                self.pid = Passport::parse_pid(value);
            }
            "cid" => {
                self.cid = Some(value);
            }
            _ => unreachable!(),
        };
    }

    fn build(&self) -> Option<Passport> {
        if self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
        {
            Some(Passport)
        } else {
            None
        }
    }
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let batches = input.split("\n\n").collect::<Vec<_>>();

    // Part A: In your batch file, how many passports are valid, without field validation?
    let valid = batches
        .iter()
        .map(|batch| Passport::has_fields(batch))
        .filter(|&b| b)
        .count();
    solution.set_part_a(valid);

    // Part B: In your batch file, how many passports are valid, with field validation?
    let valid = batches
        .iter()
        .filter_map(|batch| Passport::parse(batch))
        .count();
    solution.set_part_b(valid);

    solution
}
