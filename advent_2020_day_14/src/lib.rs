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

struct MaskedAddressUpdateIteratorBuilder {
    and_mask: u64,
    add_mask: u64,
    floating_bits: Vec<u64>,
    is_null: bool,
}

impl MaskedAddressUpdateIteratorBuilder {
    fn null() -> MaskedAddressUpdateIteratorBuilder {
        MaskedAddressUpdateIteratorBuilder{
            and_mask: 0,
            add_mask: 0,
            floating_bits: Vec::new(),
            is_null: true,
        }
    }

    fn build(&self, mem_address: u64) -> MaskedAddressUpdateIterator {
        if self.is_null {
            panic!("Call to null builder");
        }

        let max_count: u64 = 1 << self.floating_bits.iter().len();
        let partially_masked_address = (mem_address & self.and_mask) + self.add_mask;

        MaskedAddressUpdateIterator{
            count: 0, 
            max_count: max_count,
            // and_mask: self.and_mask,
            // add_mask: self.add_mask,
            floating_bits: &self.floating_bits,
            mem_address: partially_masked_address,
        }
    }
}

struct MaskedAddressUpdateIterator<'a> {
    count: u64,
    max_count: u64,
    // and_mask: u64,
    // add_mask: u64,
    floating_bits: &'a Vec<u64>,
    mem_address: u64,
}

impl Iterator for MaskedAddressUpdateIterator<'_> {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.count == self.max_count {
            return None;
        }

        let extra_add_mask = self.floating_bits.iter().enumerate().fold(0, |accum, (vec_index,  bit_index)| {
            let is_bit_on = ((1 << vec_index) & self.count) > 0;
            if is_bit_on {
                 accum + (1 << bit_index)
            } else {
                accum
            }
        });
        
        self.count += 1;
        Some(self.mem_address + extra_add_mask)
    }
}

fn parse_mask_part_2(input: &str) -> Result<MaskedAddressUpdateIteratorBuilder, Error> {
    let mut and_mask: u64 = !0;
    let mut add_mask: u64 = 0;
    let mut floating_bits: Vec<u64> = Vec::new();

    for (i, c) in input.chars().enumerate() {
        match c {
            'X' => {
                and_mask = and_mask ^ (1 << (35-i));
                floating_bits.push((35-i) as  u64);
            },
            '1' => {
                add_mask += 1 << (35-i);
                and_mask = and_mask ^ (1 << (35-i));
            },
            '0' => continue, // ignored
            _ => return Error::new("Unknown char in bitmask")
        }
    }

    Ok(MaskedAddressUpdateIteratorBuilder{and_mask, add_mask, floating_bits, is_null: false})
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

pub fn second(input: &[String]) -> Result<u64, Error> {
    let mut mask_builder = MaskedAddressUpdateIteratorBuilder::null();
    let mut mem: HashMap<u64, u64> = HashMap::new();

    for line in input {
        if let Some(captures) = MASK_INSTRUCTION_REGEX.captures(line) {
            let parsed = parse_mask_part_2(&captures[1])?;
            mask_builder = parsed;
        } else if let Some(captures) = UPDATE_INSTRUCTION_REGEX.captures(line) {
            let mem_loc: u64 = captures[1].parse().or(Error::new("can't parse mem loc"))?;
            let set_value: u64 = captures[2].parse().or(Error::new("can't parse new value"))?;
            let mem_loc_iter = mask_builder.build(mem_loc);
            for mem_loc in mem_loc_iter {
                mem.insert(mem_loc, set_value);
            }
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

    fn example2() -> Vec<String> { 
        vec!(
            String::from("mask = 000000000000000000000000000000X1001X"),
            String::from("mem[42] = 100"),
            String::from("mask = 00000000000000000000000000000000X0XX"),
            String::from("mem[26] = 1"),
        )
    }

    #[test]
    fn test_first() {
        let result = first(&example()).unwrap();
        assert_eq!(result, 165);
    }

    #[test]
    fn test_second() {
        let result = second(&example2()).unwrap();
        assert_eq!(result, 208);
    }
    #[test]
    fn test_parse_mask() {
        let input = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        let (and_mask, add_mask) = parse_mask(&input).unwrap();
        assert_eq!(and_mask, 0xFF_FF_FF_FF_FF_FF_FF_BD);
        assert_eq!(add_mask, 0b1000000);
    }

    #[test]
    fn test_parse_mask_part_2() {
        let input = "000000000000000000000000000000X1001X";
        let builder = parse_mask_part_2(input).unwrap();
        let result: Vec<u64> = builder.build(42).collect();
        assert_eq!(result, vec!(26, 58, 27, 59));
    }

    #[test]
    fn test_parse_mask_part_2_case_2() {
        let input = "00000000000000000000000000000000X0XX";
        let builder = parse_mask_part_2(input).unwrap();
        let result: Vec<u64> = builder.build(26).collect();
        assert_eq!(result, vec!(16, 24, 18, 26, 17, 25, 19, 27));
    }
}