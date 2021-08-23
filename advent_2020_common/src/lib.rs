use std::fs::File;
use std::io::prelude::*;

/// Read the puzzle input file; filename defaults to 'input.txt' 
/// unless specified in the first command line argument
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

/// Convert an iterable of strings into u32s
/// Examples:
/// ```
/// let inputs: Vec<String> = vec!(String::from("20"), String::from("50"));
/// let nums = advent_2020_common::input_to_nums(&inputs);
/// assert_eq!(nums, vec!(20, 50))
/// ```
/// 
pub fn input_to_nums(input: &[String]) -> Vec<u32> {
    input
        .iter()
        .map(|s| { s.parse()} )
        .flatten()
        .collect()
}

#[derive(Debug)]
pub struct Error(String);

impl Error {
    pub fn new<T>(message: &str) -> std::result::Result<T, Error> {
        Err(Error(String::from(message)))
    }
    pub fn from_string<T>(message: String) -> std::result::Result<T, Error> {
        Err(Error(message))
    }
}