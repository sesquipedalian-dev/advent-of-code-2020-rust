use advent_2020_common::Error;
use lazy_static::lazy_static;
use regex::Regex;
use itertools::{Chunk, Itertools};

lazy_static! {
    static ref FIELD_DEFINITION_REGEX: Regex = Regex::new("^\\w+: (\\d+)-(\\d+) or (\\d+)-(\\d+)$").unwrap();
}

#[derive(Debug)]
enum ParseState {
    FieldDefinitions,
    YourTicket,
    NearbyTickets,
}

fn range_overlap((start_1, end_1): (u32, u32), (in_start_2, end_2): (u32, u32)) -> bool {
    let start_2 = in_start_2 - 1; // so that ranges like 1-3, 4-6 will combine

    // if first start or end is within second, then yes
    (start_2 <= start_1 && start_1 <= end_2) ||
    (start_2 <= end_1 && end_1 <= end_2) ||
    // or if first wholy contains the second
    (start_1 <= start_2 && end_2 <= end_1)
}

#[derive(Debug, PartialEq)]
pub struct Input {
    valid_ranges: Vec<(u32, u32)>, // vec of (start_range, end_range)
}

pub fn first(input: &[String]) -> Result<u32, Error> {
    let mut invalid_tally = 0;
    let mut state = ParseState::FieldDefinitions;
    let mut potentially_valid_ranges: Vec<(u32, u32)> = Vec::new();

    for line in input.iter() {
        match state {
            ParseState::FieldDefinitions => { 
                if let Some(captures) = FIELD_DEFINITION_REGEX.captures(line) {
                    let range_start_1: u32 = captures[1].parse().or(Error::new("not an int"))?;
                    let range_end_1: u32 = captures[2].parse().or(Error::new("not an int"))?;
                    let range_start_2: u32 = captures[3].parse().or(Error::new("not an int"))?;
                    let range_end_2: u32 = captures[4].parse().or(Error::new("not an int"))?;

                    for (range_start, range_end) in vec!((range_start_1, range_end_1), (range_start_2, range_end_2)).iter() {
                        // check if the given range intersects an existing one, in which case we can just grow it
                        let mut found = false;
                        for (existing_start, existing_end) in potentially_valid_ranges.iter_mut() {
                            if range_overlap((*existing_start, *existing_end), (*range_start, *range_end)) ||
                                range_overlap((*range_start, *range_end), (*existing_start, *existing_end)) {
                                found = true;
                                *existing_start = if *existing_start < *range_start {
                                    *existing_start
                                } else { 
                                    *range_start
                                };
                                *existing_end = if *existing_end > *range_end {
                                    *existing_end
                                } else {
                                    *range_end
                                };
                                break;
                            }
                        }
                        if !found {
                            potentially_valid_ranges.push((*range_start, *range_end));
                        }
                    }
                } else if line == "your ticket:" {
                    state = ParseState::YourTicket;
                }
                // ignore empty line, invalid input, etc
            },
            ParseState::YourTicket => {
                if line == "nearby tickets:" {
                    state = ParseState::NearbyTickets;
                }
                // ignore 'your ticket' for now
            },
            ParseState::NearbyTickets => { 
                let nums: Vec<u32> = line.split(",").map(|s| s.parse().or(Error::new("not an int"))).flatten().collect();
                for num in nums {
                    let mut found = false;
                    for valid_range in &potentially_valid_ranges {
                        if (valid_range.0 <= num) && (num <= valid_range.1) {
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        invalid_tally += num;
                    }
                }
            },
        }
    }
    
    Ok(invalid_tally)
}


pub fn second(input: &[String]) -> Result<u32, Error> {
    let mut state = ParseState::FieldDefinitions;
    let mut potentially_valid_ranges: Vec<(u32, u32)> = Vec::new();
    let mut your_ticket: Vec<u32> = Vec::new();
    let mut valid_tickets: Vec<Vec<u32>> = Vec::new();

    for line in input.iter() {
        match state {
            ParseState::FieldDefinitions => { 
                if let Some(captures) = FIELD_DEFINITION_REGEX.captures(line) {
                    let range_start_1: u32 = captures[1].parse().or(Error::new("not an int"))?;
                    let range_end_1: u32 = captures[2].parse().or(Error::new("not an int"))?;
                    let range_start_2: u32 = captures[3].parse().or(Error::new("not an int"))?;
                    let range_end_2: u32 = captures[4].parse().or(Error::new("not an int"))?;

                    for (range_start, range_end) in vec!((range_start_1, range_end_1), (range_start_2, range_end_2)).iter() {
                        // check if the given range intersects an existing one, in which case we can just grow it
                        let mut found = false;
                        for (existing_start, existing_end) in potentially_valid_ranges.iter_mut() {
                            if range_overlap((*existing_start, *existing_end), (*range_start, *range_end)) ||
                                range_overlap((*range_start, *range_end), (*existing_start, *existing_end)) {
                                found = true;
                                *existing_start = if *existing_start < *range_start {
                                    *existing_start
                                } else { 
                                    *range_start
                                };
                                *existing_end = if *existing_end > *range_end {
                                    *existing_end
                                } else {
                                    *range_end
                                };
                                break;
                            }
                        }
                        if !found {
                            potentially_valid_ranges.push((*range_start, *range_end));
                        }
                    }
                } else if line == "your ticket:" {
                    state = ParseState::YourTicket;
                }
                // ignore empty line, invalid input, etc
            },
            ParseState::YourTicket => {
                if line == "nearby tickets:" {
                    state = ParseState::NearbyTickets;
                } else if line.len() > 0 {
                    for num in line.split(",").map(|s| s.parse().or(Error::new("not an int"))).flatten() {
                        your_ticket.push(num);
                    }
                }
            },
            ParseState::NearbyTickets => { 
                let nums: Vec<u32> = line.split(",").map(|s| s.parse().or(Error::new("not an int"))).flatten().collect();
                let mut valid = true;
                for num in nums.iter() {
                    let mut found = false;
                    for valid_range in &potentially_valid_ranges {
                        if (valid_range.0 <= *num) && (*num <= valid_range.1) {
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        valid = false;
                    }
                }
                if valid {
                    valid_tickets.push(nums);
                }
            },
        }
    }
    
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> { 
        vec!(
            String::from("class: 1-3 or 5-7"),
            String::from("row: 6-11 or 33-44"),
            String::from("seat: 13-40 or 45-50"),
            String::from("your ticket:"),
            String::from("7,1,14"),
            String::from("nearby tickets:"),
            String::from("7,3,47"),
            String::from("40,4,50"),
            String::from("55,2,20"),
            String::from("38,6,12"),
        )
    }

    #[test]
    fn test_first() {
        let result = first(&example()).unwrap();
        // assert_eq!(result, Input{valid_ranges: vec!(
        //     (1, 3), (5, 11), (13, 50)
        // )});
        assert_eq!(result, 71)
    }

    #[test]
    fn test_range_overlap()  {
        let result = range_overlap((1, 3), (3, 6));
        assert!(result);

        let result = range_overlap((1, 6), (2, 3));
        assert!(result);

        let result = range_overlap((1, 6), (7, 8));
        assert!(result);

        let result = range_overlap((1, 3), (5, 8));
        assert_eq!(result, false);
    }
}