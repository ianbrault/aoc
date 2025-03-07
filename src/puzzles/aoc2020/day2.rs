/*
** src/puzzles/aoc2020/day2.rs
*/

use super::Solution;
use crate::types::Counter;
use crate::utils;

// there are 2 ways to interpret the x and y numbers in the password policy
// (1) range policy: password must contain the given character at least x and
//     at most y times
// (2) position policy: password must contain the given character at exactly
//     one of the positions x and y
enum PasswordPolicyRule {
    RangePolicy,
    PositionPolicy,
}

// defines the validity of a password
// see PasswordPolicyRule for specifics
struct PasswordPolicy {
    character: char,
    x: u8,
    y: u8,
}

impl From<&str> for PasswordPolicy {
    fn from(value: &str) -> Self {
        let (srange, schar) = utils::split(value, " ").unwrap();
        let character = schar.chars().next().unwrap();
        let (sx, sy) = utils::split(srange, "-").unwrap();
        let x = sx.parse().unwrap();
        let y = sy.parse().unwrap();
        Self { character, x, y }
    }
}

// a password
// also stores the frequency of each character in the password string for the
// range-based password policy
struct Password<'a> {
    string: &'a str,
    freq_map: Counter<char>,
}

impl<'a> Password<'a> {
    fn is_valid(&self, policy: &PasswordPolicy, policy_rule: PasswordPolicyRule) -> bool {
        match policy_rule {
            PasswordPolicyRule::RangePolicy => {
                let range = (policy.x)..(policy.y + 1);
                range.contains(&(self.freq_map.get(policy.character) as u8))
            }
            PasswordPolicyRule::PositionPolicy => {
                // note: passwords are NOT zero-indexed
                let x = policy.x - 1;
                let y = policy.y - 1;
                // maybe be less cavalier about unwrapping here?
                let cx = self.string.chars().nth(x as usize).unwrap();
                let cy = self.string.chars().nth(y as usize).unwrap();
                // xor == exactly 1 is equal
                (cx == policy.character) ^ (cy == policy.character)
            }
        }
    }
}

impl<'a> From<&'a str> for Password<'a> {
    fn from(string: &'a str) -> Self {
        let freq_map = string.chars().collect();
        Self { string, freq_map }
    }
}

fn parse_input(input: &str) -> Vec<(Password<'_>, PasswordPolicy)> {
    let mut passwords = Vec::new();
    for line in input.split('\n') {
        let (policy_str, password_str) = utils::split(line, ": ").unwrap();
        let password = Password::from(password_str);
        let policy = PasswordPolicy::from(policy_str);
        passwords.push((password, policy));
    }
    passwords
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let passwords = parse_input(input.as_str());

    // Part A: How many passwords are valid according to the (range-based) corporate policies?
    let valid = passwords
        .iter()
        .filter(|(password, policy)| password.is_valid(policy, PasswordPolicyRule::RangePolicy))
        .count();
    solution.set_part_a(valid);

    // Part B: How many passwords are valid according to the new (position-based) interpretation of
    // the policies?
    let valid = passwords
        .iter()
        .filter(|(password, policy)| password.is_valid(policy, PasswordPolicyRule::PositionPolicy))
        .count();
    solution.set_part_b(valid);

    solution
}
