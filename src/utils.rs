/*
** src/utils.rs
*/

use std::cmp;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::Path;
use std::str::FromStr;

/// reads the contents of a file into a string
pub fn read_file(path: &Path) -> io::Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

/// splits a string by whitespace and parses the component parts into the given type
pub fn split_and_parse<T>(string: &str) -> impl Iterator<Item = T> + '_
where
    T: FromStr,
{
    string
        .split_ascii_whitespace()
        .filter_map(|x| x.parse::<T>().ok())
}

/// returns the head and tail of the string split at the first instance of the given pattern
pub fn split<'a>(string: &'a str, after: &str) -> Option<(&'a str, &'a str)> {
    if let Some(sep) = string.find(after) {
        let a = &string[..sep];
        let b = &string[(sep + after.len())..];
        Some((a, b))
    } else {
        None
    }
}

/// returns the tail of the string split at the first instance of the given pattern
pub fn split_tail<'a>(string: &'a str, after: &str) -> Option<&'a str> {
    if let Some(sep) = string.find(after) {
        Some(&string[(sep + after.len())..])
    } else {
        None
    }
}

/// greatest-common-divisor of 2 numbers
pub fn gcd(x: u64, y: u64) -> u64 {
    if x == y {
        return x;
    }

    let mut a = cmp::max(x, y);
    let mut b = cmp::min(x, y);
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    a
}

/// least-common-multiple of 2 numbers
pub fn lcm(x: u64, y: u64) -> u64 {
    x * (y / gcd(x, y))
}

/// sort 2 ordered items into a minimum and maximum
pub fn min_max<T>(a: T, b: T) -> (T, T)
where
    T: Clone + Ord,
{
    (
        cmp::min(a.clone(), b.clone()),
        cmp::max(a.clone(), b.clone()),
    )
}

/// sort 2 ordered items into a minimum and maximum, using the given key function
pub fn min_max_by_key<T, F, K>(a: T, b: T, f: F) -> (T, T)
where
    T: Clone,
    F: Fn(&T) -> K,
    K: Ord,
{
    (
        cmp::min_by_key(a.clone(), b.clone(), &f),
        cmp::max_by_key(a.clone(), b.clone(), &f),
    )
}

/// calculates the number of digits in the given integer value
pub fn num_digits(n: u64) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
}
