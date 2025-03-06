/*
** src/puzzles/aoc2021/day3.rs
*/

use super::Solution;

const N_BITS: usize = 12;

#[derive(Clone)]
struct Binary {
    digits: [u8; N_BITS],
}

impl Binary {
    fn bit(&self, i: usize) -> u8 {
        self.digits[N_BITS - i - 1]
    }
}

impl From<&str> for Binary {
    fn from(s: &str) -> Self {
        let mut digits = [0; N_BITS];
        for (i, c) in s.chars().enumerate() {
            digits[i] = c.to_digit(10).unwrap() as u8;
        }
        Self { digits }
    }
}

impl From<&Binary> for u32 {
    fn from(value: &Binary) -> Self {
        let mut n = 0;
        for (i, &x) in value.digits.iter().rev().enumerate() {
            n |= (x as u32) << i;
        }
        n
    }
}

fn count_bits(numbers: &[Binary]) -> [u64; N_BITS] {
    let mut bit_count = [0; N_BITS];

    for number in numbers.iter() {
        for (i, &bit) in number.digits.iter().enumerate() {
            if bit == 1 {
                bit_count[i] += 1;
            }
        }
    }

    bit_count
}

fn most_common_bit(bit_counts: &[u64; N_BITS], n_numbers: usize, bit: usize) -> u8 {
    let pos = N_BITS - bit - 1;
    if bit_counts[pos] >= n_numbers as u64 / 2 {
        1
    } else {
        0
    }
}

fn least_common_bit(bit_counts: &[u64; N_BITS], n_numbers: usize, bit: usize) -> u8 {
    let pos = N_BITS - bit - 1;
    if bit_counts[pos] >= n_numbers as u64 / 2 {
        0
    } else {
        1
    }
}

fn power_consumption(numbers: &[Binary], bit_counts: &[u64; 12]) -> u64 {
    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..N_BITS {
        match most_common_bit(bit_counts, numbers.len(), i) {
            0 => epsilon |= 1 << i,
            1 => gamma |= 1 << i,
            _ => unreachable!(),
        };
    }

    gamma * epsilon
}

fn life_support_rating(numbers: &[Binary]) -> u32 {
    // determine oxygen generator rating
    let mut oxygen_numbers = numbers.to_vec();
    for i in (0..N_BITS).rev() {
        let bit_counts = count_bits(&oxygen_numbers);
        let bit = most_common_bit(&bit_counts, oxygen_numbers.len(), i);
        oxygen_numbers = oxygen_numbers
            .iter()
            .filter(|n| n.bit(i) == bit)
            .cloned()
            .collect();
        if oxygen_numbers.len() == 1 {
            break;
        }
    }

    // determine CO2 scrubber rating
    let mut co2_numbers = numbers.to_vec();
    for i in (0..N_BITS).rev() {
        let bit_counts = count_bits(&co2_numbers);
        let bit = least_common_bit(&bit_counts, co2_numbers.len(), i);
        co2_numbers = co2_numbers
            .iter()
            .filter(|n| n.bit(i) == bit)
            .cloned()
            .collect();
        if co2_numbers.len() == 1 {
            break;
        }
    }

    let oxygen_rating: u32 = (&oxygen_numbers[0]).into();
    let co2_rating: u32 = (&co2_numbers[0]).into();
    oxygen_rating * co2_rating
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let numbers = input.split('\n').map(Binary::from).collect::<Vec<_>>();
    let bit_counts = count_bits(&numbers);

    // Part A: Use the binary numbers in your diagnostic report to calculate the gamma rate and
    // epsilon rate, then multiply them together. What is the power consumption of the submarine?
    let power = power_consumption(&numbers, &bit_counts);
    solution.set_part_a(power);

    // Part B: Use the binary numbers in your diagnostic report to calculate the oxygen generator
    // rating and CO2 scrubber rating, then multiply them together. What is the life support rating
    // of the submarine?
    let life_support = life_support_rating(&numbers);
    solution.set_part_b(life_support);

    solution
}
