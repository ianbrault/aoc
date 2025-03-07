/*
** src/puzzles/aoc2020/day13.rs
*/

use super::Solution;
use crate::utils;

// an adaptation of Bézout's identity (using the extended Euclidean algorithm) for modular integers
// see: https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Modular_integers
fn inverse(a: i64, n: i64) -> i64 {
    let (mut t, mut new_t) = (0, 1);
    let (mut r, mut new_r) = (n, a);

    while new_r != 0 {
        let q = r / new_r;

        let tmp = new_t;
        new_t = t - q * new_t;
        t = tmp;

        let tmp = new_r;
        new_r = r - q * new_r;
        r = tmp;
    }

    if r > 1 {
        unreachable!()
    } else if t < 0 {
        t + n
    } else {
        t
    }
}

fn find_earliest_timestamp(buses: &[i64]) -> i64 {
    // the non-brute-force solution uses the Chinese Remainder Theorem, with the existence direct
    // construction: by inspection, note that all bus IDs are prime numbers, therefore all possible
    // pairs are coprime; the IDs and offsets form a system of congruences where the solution S is
    // such that S % n_i = a_i for each ID n_i, offset a_i

    // filter for non-zero bus IDs and get the set of offsets
    // note: a-terms are NOT the offsets, they are the IDs with the offsets subtracted out
    let (a, ids): (Vec<_>, Vec<_>) = buses
        .iter()
        .enumerate()
        .filter(|(_, &b)| b > 0)
        .map(|(offset, &id)| (id - offset as i64, id))
        .unzip();

    // get the product n of all moduli (i.e. bus IDs)
    let n = ids.iter().product::<i64>();
    // get the n_i terms, the product of all moduli except n_i, for each i
    let n_i = ids.iter().map(|&i| n / i).collect::<Vec<_>>();
    // find the Bézout coefficients of the terms
    // note: the solution only requires the first coefficient
    let m = n_i
        .iter()
        .zip(ids.iter())
        .map(|(&a, &b)| inverse(a, b))
        .collect::<Vec<_>>();
    // calculate the solution
    let x = (0..ids.len()).map(|i| a[i] * m[i] * n_i[i]).sum::<i64>();
    x % n
}

fn parse_input(input: String) -> (i64, Vec<i64>) {
    let (earliest, bus_list) = utils::split(input.as_str(), "\n").unwrap();
    let earliest_departure = earliest.parse().unwrap();
    let buses = bus_list
        .split(',')
        .map(|s| {
            if s == "x" {
                // leave placeholder values for out-of-service buses
                0
            } else {
                s.parse().unwrap()
            }
        })
        .collect::<Vec<_>>();
    (earliest_departure, buses)
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let (earliest_departure, buses) = parse_input(input);

    // Part A: What is the ID of the earliest bus you can take to the airport multiplied by the
    // number of minutes you'll need to wait for that bus?
    let (id, delay) = buses
        .iter()
        .filter(|&&b| b > 0)
        .map(|b| (b, b - (earliest_departure % b)))
        .min_by_key(|(_, delay)| *delay)
        .unwrap();
    solution.set_part_a(id * delay);

    // Part B: What is the earliest timestamp such that all of the listed bus IDs depart at offsets
    // matching their positions in the list?
    let timestamp = find_earliest_timestamp(&buses);
    solution.set_part_b(timestamp);

    solution
}
