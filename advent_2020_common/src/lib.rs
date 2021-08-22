use std::fs::File;
use std::io::prelude::*;

/// Read the puzzle input file; filename defaults to 'input.txt' 
/// unless specified in the first command line argument
/// Examples:
/// ```
/// use advent_2020_common;
/// let input = input().unwrap();
/// println!(Using puzzle input! {:?}", input");
/// ```
/// 
pub fn input() -> Result<Vec<String>, std::io::Error> {
    let mut args = std::env::args();
    let input_filename = args.nth(1).unwrap_or(String::from("input.txt"));

    let mut file = File::open(input_filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let contents: Vec<String> = contents.split("\r\n")
        .map(|s| String::from(s))
        .collect();

    Ok(contents)
}

#[derive(Debug)]
pub struct Error(String);

impl Error {
    pub fn new<T>(message: &str) -> std::result::Result<T, Error> {
        Err(Error(String::from(message)))
    }
}