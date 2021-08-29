use advent_2020_common::Error;
use regex::Regex;
use std::convert::TryFrom;

#[derive(Clone, Debug, PartialEq, Copy)]
enum Directions {
    East = 0,
    South,
    West,
    North
}

impl TryFrom<u32> for Directions {
    type Error = advent_2020_common::Error;

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            x if x == Directions::East as u32 => Ok(Directions::East),
            x if x == Directions::South as u32 => Ok(Directions::South),
            x if x == Directions::West as u32 => Ok(Directions::West),
            x if x == Directions::North as u32 => Ok(Directions::North),
            x => Error::from_string(format!("convert invalid u32 to Directions {}", x)),
        }
    }
}

pub fn first(input: &[String]) -> Result<u32, Error> {
    let instruction_regex = Regex::new("^(N|S|E|W|L|R|F)(\\d+)$").or(Error::new("Couldn't compile regex!"))?;
    let mut direction_mods: Vec<u32> = vec!(0, 0, 0, 0);
    let mut current_forward = Directions::East;

    for line in input {
        let caps = match instruction_regex.captures(line) {
            Some(captures) => captures,
            _ => return Error::from_string(format!("invalid line {}", line)),
        };

        let amount: u32 = caps[2].parse().or(Error::new("line missing number"))?;
        let direction = match &caps[1] {
            "N" => Directions::North,
            "S" => Directions::South,
            "E" => Directions::East,
            "W" => Directions::West,
            "F" => current_forward,
            "L" => {
                let new_dir_i32 = ((current_forward as i32) - ((amount / 90) as i32)) % 4;
                let new_dir_i32 = if new_dir_i32 < 0 {
                    new_dir_i32 + 4
                } else {
                    new_dir_i32
                };
                current_forward = Directions::try_from(new_dir_i32 as u32)?;
                continue
            },
            "R" => {
                current_forward = Directions::try_from(((current_forward as u32) + (amount / 90)) % 4)?;
                continue
            },
            x => return Error::from_string(format!("invalid instruction! {}", x)),
        };

        direction_mods[direction as usize] += amount;
    }
    
    Ok(
        (direction_mods[Directions::North as usize] as isize - direction_mods[Directions::South as usize] as isize).abs() as u32 +
        (direction_mods[Directions::East as usize] as isize - direction_mods[Directions::West as usize] as isize).abs() as u32
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec!(
            String::from("F10"),
            String::from("N3"),
            String::from("F7"),
            String::from("R90"),
            String::from("F11"),
        )
    }

    #[test]
    fn test_first() {
        let result = first(&example()).unwrap();
        assert_eq!(result, 25);
    }
}