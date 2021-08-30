use advent_2020_common::Error;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref MASK_INSTRUCTION_REGEX: Regex = Regex::new("^mask = ((X|1|0){36})$").unwrap();
    static ref UPDATE_INSTRUCTION_REGEX: Regex = Regex::new("^mem\\[(\\d+)\\] = (\\d+)$").unwrap();
}

fn parse_mask(input: &str) -> Result<(u64, u64), Error> {
    let mut new_and_mask: u64 = !0;
    let mut new_add_mask: u64 = 0;

    for (i, c) in input.chars().enumerate() {
        match c {
            'X' => continue, // ignored
            '1' => {
                new_add_mask += 1 << (35-i);
                new_and_mask = new_and_mask ^ (1 << (35-i));
            },
            '0' => new_and_mask = new_and_mask ^ (1 << (35-i)),
            _ => return Error::new("Unknown char in bitmask")
        }
    }

    Ok((new_and_mask, new_add_mask))
}

pub fn first(input: &[String]) -> Result<u64, Error> {
    let mut add_mask: u64 = 0;
    let mut and_mask: u64 = 0;
    let mut mem: HashMap<u64, u64> = HashMap::new();

    for line in input {
        if let Some(captures) = MASK_INSTRUCTION_REGEX.captures(line) {
            let parsed = parse_mask(&captures[1])?;
            add_mask = parsed.1;
            and_mask = parsed.0;
        } else if let Some(captures) = UPDATE_INSTRUCTION_REGEX.captures(line) {
            let mem_loc: u64 = captures[1].parse().or(Error::new("can't parse mem loc"))?;
            let set_value: u64 = captures[2].parse().or(Error::new("can't parse new value"))?;

            mem.insert(mem_loc, (set_value & and_mask) + add_mask);
        }
    }
    
    Ok(mem.iter().fold(0 as u64, |sum, (_, v)| sum + v))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec!(
            String::from("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"),
            String::from("mem[8] = 11"),
            String::from("mem[7] = 101"),
            String::from("mem[8] = 0"),
        )
    }

    #[test]
    fn test_first() {
        let result = first(&example()).unwrap();
        assert_eq!(result, 165);
    }

    #[test]
    fn test_parse_mask() {
        let input = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        let (and_mask, add_mask) = parse_mask(&input).unwrap();
        assert_eq!(and_mask, 0xFF_FF_FF_FF_FF_FF_FF_BD);
        assert_eq!(add_mask, 0b1000000);
    }
}