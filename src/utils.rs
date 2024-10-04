/*
** src/utils.rs
*/

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
