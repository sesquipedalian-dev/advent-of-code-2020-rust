use advent_2020_common::Error;
extern crate lazy_static;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

lazy_static! {
    static ref INSTRUCTION_REGEX: Regex = Regex::new("(nop|acc|jmp) ((\\+|-)\\d+)").unwrap();
}

#[derive(Debug, PartialEq, Clone)]
enum Operation {
    None,
    Accumulate,
    Jump
}

#[derive(Debug, PartialEq, Clone)]
struct Instruction {
    op: Operation,
    amount: isize,
}

type Instructions = Vec<Instruction>;

trait MakesInstructions {
    fn from_strings(input: &[String]) ->  Result<Instructions, Error>;
    fn to_strings(&self) -> Vec<String>;
}

impl MakesInstructions for Instructions {
    fn from_strings(input: &[String]) -> Result<Instructions, Error> {
        let mut instructions = Instructions::new();

        for instruction_str in input.iter() {
            let captures = INSTRUCTION_REGEX.captures(instruction_str).ok_or(Error{msg: String::from("didn't match rege")})?;

            let op_str: Option<&str> = captures.get(1).map(|m| m.as_str());
            
            let op = match op_str.as_deref() {
                Some("nop") => Operation::None,
                Some("acc") => Operation::Accumulate,
                Some("jmp") => Operation::Jump,
                _ => return Error::new("missing instruction!"),
            };

            let amount: isize = captures.get(2)
                .map(|m| m.as_str()).ok_or(Error{msg: String::from("missing amount")})?
                .parse().or(Error::new("missing amount"))?;


            instructions.push(Instruction{op, amount});
        }
        
        Ok(instructions)
    }

    fn to_strings(&self) -> Vec<String> {
        self.iter()
            .map(|o| {
                let mut s = String::new();
                s.push_str(match o.op {
                    Operation::None => "nop ",
                    Operation::Accumulate => "acc ",
                    Operation::Jump => "jmp ",
                });
                s.push_str(match o.amount {
                    x if x < 0 => format!("{}", x),
                    x => format!("+{}", x),
                }.as_str());
                s
            })
            .collect()
    }
}

#[derive(Debug, PartialEq)]
enum ProcessResult {
    Terminated(isize),
    InfiniteLoop(isize),
}

fn process_instructions(instructions: &Instructions) -> Result<ProcessResult, Error> {
    let mut accum = 0;
    let mut prog: usize = 0;
    let mut seen: HashSet<usize> = HashSet::new();

    loop {
        if seen.get(&prog).is_some() {
            return Ok(ProcessResult::InfiniteLoop(accum));
        }
        seen.insert(prog);

        let instruction = match instructions.get(prog) {
            Some(p) => p,
            _ => return Ok(ProcessResult::Terminated(accum)),
        };

        match instruction.op {
            Operation::None => prog += 1,
            Operation::Accumulate => {
                accum += instruction.amount;
                prog += 1
            },
            Operation::Jump => {
                let next_prog: isize = (prog as isize) + instruction.amount;
                if next_prog < 0 {
                    return Error::new("program counter went below 0!");
                }
                prog = next_prog as usize;
            }
        }
    }
}

pub fn first(input: &[String]) -> Result<isize, Error> {
    let instructions = Instructions::from_strings(input)?;
    match process_instructions(&instructions) {
        Ok(ProcessResult::Terminated(_)) => Error::new("program counter ran off available instructions"),
        Ok(ProcessResult::InfiniteLoop(accum)) => Ok(accum),
        Err(x) => Err(x),
    }
}

pub fn second(input: &[String]) -> Result<isize, Error> {
    let instructions = Instructions::from_strings(input)?;

    for (index, instruction) in instructions.iter().enumerate() {
        let new_op = match &instruction.op {
            Operation::None => Operation::Jump,
            Operation::Jump => Operation::None,
            x => x.clone(),
        };

        let mut new_instructions = Instructions::from_strings(&instructions.to_strings())?;
        new_instructions[index].op = new_op;
        let result = process_instructions(&new_instructions);
        match result {
            // termianted successfully, this is the one!
            Ok(ProcessResult::Terminated(accum)) => return Ok(accum),
            _ => ()
        }
    }
    
    Error::new("Not found!")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec!(
            String::from("nop +0"),
            String::from("acc +1"),
            String::from("jmp +4"),
            String::from("acc +3"),
            String::from("jmp -3"),
            String::from("acc -99"),
            String::from("acc +1"),
            String::from("jmp -4"),
            String::from("acc +6"),
        )
    }

    #[test]
    fn test_first() {
        let result = first(&example()).unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn test_second() {
        let result = second(&example()).unwrap();
        assert_eq!(result, 8);
    }

    #[test]
    fn test_regex() {
        let captures = INSTRUCTION_REGEX.captures("nop +0").unwrap();
        assert_eq!(&captures[1], "nop");
        assert_eq!(&captures[2], "+0");
        let i: isize = captures[2].parse().unwrap();
        assert_eq!(i, 0);

        let captures = INSTRUCTION_REGEX.captures("jmp -4").unwrap();
        assert_eq!(&captures[1], "jmp");
        assert_eq!(&captures[2], "-4");
        let i: isize = captures[2].parse().unwrap();
        assert_eq!(i, -4);
    }

    #[test]
    fn test_parse() {
        let result = Instructions::from_strings(&example()).unwrap();
        
        assert_eq!(result[0], Instruction{op: Operation::None, amount: 0});
        assert_eq!(result[4], Instruction{op: Operation::Jump, amount: -3});
        assert_eq!(result[8], Instruction{op: Operation::Accumulate, amount: 6});
    }

    #[test]
    fn test_process_instructions() {
        let input = Instructions::from_strings(&example()).unwrap();
        let result = process_instructions(&input).unwrap();

        assert_eq!(result, ProcessResult::InfiniteLoop(5));
    }

    #[test]
    fn test_to_strings() {
        let input = example();
        let orig = Instructions::from_strings(&input).unwrap();
        let str_version = orig.to_strings();
        assert_eq!(input, str_version);
    }
}