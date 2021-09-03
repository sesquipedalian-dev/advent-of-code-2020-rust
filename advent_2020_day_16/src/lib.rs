use advent_2020_common::Error;
use lazy_static::lazy_static;
use regex::Regex;
use itertools::{Chunk, Itertools};

lazy_static! {
    static ref FIELD_DEFINITION_REGEX: Regex = Regex::new("^([ \\w]+): (\\d+)-(\\d+) or (\\d+)-(\\d+)$").unwrap();
}

#[derive(Debug)]
enum ParseState {
    FieldDefinitions,
    YourTicket,
    NearbyTickets,
}

fn range_overlap((start_1, end_1): (u32, u32), (in_start_2, end_2): (u32, u32)) -> bool {
    let start_2_i = in_start_2 as isize - 1; // so that ranges like 1-3, 4-6 will combine
    let start_2: u32 = if start_2_i < 0 {
        0
    } else {
        start_2_i as u32
    };

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
                    let range_start_1: u32 = captures[2].parse().or(Error::new("not an int"))?;
                    let range_end_1: u32 = captures[3].parse().or(Error::new("not an int"))?;
                    let range_start_2: u32 = captures[4].parse().or(Error::new("not an int"))?;
                    let range_end_2: u32 = captures[5].parse().or(Error::new("not an int"))?;

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

#[derive(Debug, PartialEq)]
struct FieldDefinition {
    is_departure: bool, 
    range_start_1: u32, 
    range_end_1: u32,
    range_start_2: u32,
    range_end_2: u32,
}

pub fn second(input: &[String]) -> Result<u64, Error> {
    let mut state = ParseState::FieldDefinitions;
    let mut potentially_valid_ranges: Vec<(u32, u32)> = Vec::new();
    let mut your_ticket: Vec<u32> = Vec::new();
    let mut valid_tickets: Vec<Vec<u32>> = Vec::new();
    let mut field_definitions: Vec<FieldDefinition> = Vec::new();

    for line in input.iter() {
        match state {
            ParseState::FieldDefinitions => { 
                if let Some(captures) = FIELD_DEFINITION_REGEX.captures(line) {
                    let field_name: &str = &captures[1];
                    let is_departure = field_name.contains("departure");
                    let range_start_1: u32 = captures[2].parse().or(Error::new("not an int"))?;
                    let range_end_1: u32 = captures[3].parse().or(Error::new("not an int"))?;
                    let range_start_2: u32 = captures[4].parse().or(Error::new("not an int"))?;
                    let range_end_2: u32 = captures[5].parse().or(Error::new("not an int"))?;

                    field_definitions.push(FieldDefinition{is_departure, range_start_1, range_end_1, range_start_2, range_end_2});

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
    
    // for the remaining valid tickets, check which fields are valid for each position on the ticket. 
    // Once the list for all fields is down to size 1, we've identified the rule positions
    let mut possible_fields_by_location: Vec<Vec<usize>> = field_definitions.iter()
        .map(|_| (0..field_definitions.len()).collect()).collect();
    let mut valid_tickets_iter = valid_tickets.iter();
    while {
        possible_fields_by_location.iter().fold(false, |accum, v| accum || (v.len() > 1))
    } {
        if let Some(valid_ticket) = valid_tickets_iter.next() {
            for (position, value) in valid_ticket.iter().enumerate() {
                let mut new_valid_fields: Vec<usize> = Vec::new();
                for field_index in possible_fields_by_location[position].iter() {
                    let FieldDefinition{is_departure: _, range_start_1, range_end_1, range_start_2, range_end_2} = &field_definitions[*field_index];
       
                    if ((range_start_1 <= value) && (value <= range_end_1)) ||
                        ((range_start_2 <= value) && (value <= range_end_2)) {
                        // keep range
                        new_valid_fields.push(*field_index);
                    }
                }
                possible_fields_by_location[position] = new_valid_fields;
            }

            // consolidate possible fields using sudoku rules, e.g. if any position only has one valid option, we can 
            // 'cross it off' from other positions.  Repeat recursively until there are no more changes?
            // remove_known_field_slots(&mut possible_fields_by_location);
            // TODO: this is repetitive because except_arr is re-initialized every loop. Would it be ok to store it?
            let mut except_arr: Vec<usize> = Vec::new();
            loop {
                let mut found_guy: Option<usize> = None;
                for (i, possible_fields) in possible_fields_by_location.iter().enumerate() {
                    if except_arr.contains(&i) { 
                        continue;
                    }
                    if possible_fields.len() == 1 {
                        found_guy = Some(i);
                    }
                }

                if found_guy.is_none() {
                    break;
                }

                let except_index = found_guy.unwrap();
                let remove_index = possible_fields_by_location[except_index][0];
                remove_known_field_slots(&mut possible_fields_by_location, except_index, remove_index);
                except_arr.push(except_index);
            }
        } else {
            return Error::new("Ran out of valid tickets while trying to find field order");
        }
    }

    // ok, now that possible_fields_by_location contains only the valid fields in each location, 
    // check 'your ticket' and multiply together all the values where the field definition is_departure
    let mut result: u64 = 1;
    for (i, value) in your_ticket.iter().enumerate() {
        let field_definition = &field_definitions[possible_fields_by_location[i][0]];
        if field_definition.is_departure {
            result *= *value as u64;
        }
    }

    Ok(result)
}

fn remove_known_field_slots(possible_fields_by_location: &mut Vec<Vec<usize>>, except_index: usize, remove_index: usize) {
    for (i, possible_fields) in possible_fields_by_location.iter_mut().enumerate() {
        if i != except_index {
            let mut without_remove_index: Vec<usize> = possible_fields.iter().map(|s| *s).filter(|i| *i != remove_index).collect();
            possible_fields.clear();
            possible_fields.append(&mut without_remove_index);
        }
    }
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

    fn example2() -> Vec<String> {
        vec!(
            String::from("departure class: 0-1 or 4-19"),
            String::from("row: 0-5 or 8-19"),
            String::from("departure seat: 0-13 or 16-19"),
            String::from(""),
            String::from("your ticket:"),
            String::from("11,12,13"),
            String::from(""),
            String::from("nearby tickets:"),
            String::from("3,9,18"),
            String::from("15,1,5"),
            String::from("5,14,9"),
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
    fn test_second() {
        let result = second(&example2()).unwrap();
        assert_eq!(result, 12 * 13);
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