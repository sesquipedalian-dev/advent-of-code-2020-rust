use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

pub mod life;

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

/// search for a vector of u32s looking for 2 numbers that sum to the sum_looked_for
/// Some(v) if found, None otherwise.
/// 
pub fn find_complements(input: &[u32], sum_looked_for: u32, needs_multiply: bool) -> Option<u32> {
    let mut seen_complements: HashSet<u32> = HashSet::new();  
    for current in input.iter() {
        if sum_looked_for < *current {
            continue;
        }

        let my_complement = sum_looked_for - current;

        if seen_complements.contains(&current) {
            if needs_multiply {
                return Some(current * my_complement);
            } else {
                return Some(0); // caller doesn't care about value of result
            }
        }

        seen_complements.insert(my_complement);
    }
    None
}

#[derive(Debug)]
pub struct Error{pub msg: String}

impl Error {
    pub fn new<T>(message: &str) -> std::result::Result<T, Error> {
        Err(Error{msg: String::from(message)})
    }
    pub fn from_string<T>(message: String) -> std::result::Result<T, Error> {
        Err(Error{msg:message})
    }
}

