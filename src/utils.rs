/*
** src/utils.rs
*/

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::Path;

/// reads the contents of a file into a string
pub fn read_file(path: &Path) -> io::Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}
