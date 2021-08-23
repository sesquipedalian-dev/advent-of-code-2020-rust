#[macro_use]
extern crate lazy_static;

use std::collections::HashSet;
use advent_2020_common::Error;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref FIELD_PAIR_REGEX: Regex = Regex::new("([^: ]+):([^: ]+)").unwrap();
}

lazy_static! {
    static ref EYE_COLORS: HashSet<&'static str> = {
        let mut m = HashSet::new();
        m.insert("amb");
        m.insert("blu");
        m.insert("brn");
        m.insert("gry");
        m.insert("grn");
        m.insert("hzl");
        m.insert("oth");
        m
    };
}

pub fn first(input: &[String]) -> Result<u32, Error> {
    count_valid_passports(input, false)
}

fn count_valid_passports(input: &[String], validate: bool) -> Result<u32, Error> {
    let mut valid_count = 0;
    let mut current_start = 0;
    
    for (i, input_line) in input.iter().enumerate() {
        if input_line.len() == 0 {
            if let Ok(_) = PassportFields::new(&input[current_start .. i], validate) {
                valid_count = valid_count + 1;
            }
            current_start = i;
        }
    }

    Ok(valid_count)
}

#[derive(Debug, PartialEq)]
enum Height {
    Inches(u32),  
    Centimeters(u32),
    Missing,
}


#[derive(Debug, PartialEq)]
struct PassportFields<'a> {
    birth_year: u32,
    issue_year: u32, 
    expiration_year: u32, 
    height: Height,
    hair_color: &'a str,
    eye_color: &'static str,
    passport_id: &'a str,
}

impl PassportFields<'_> {
    fn new(input: &[String], validate: bool) -> Result<PassportFields, Error> {
        let mut birth_year = 0;
        let mut issue_year = 0;
        let mut expiration_year = 0;
        let mut height = Height::Missing;
        let mut hair_color = "";
        let mut eye_color = "";
        let mut passport_id = "";
        let mut found_fields = HashSet::<String>::new();

        for line in input.iter() {
            for captures in FIELD_PAIR_REGEX.captures_iter(line) {
                let field_name = &captures[1];
                
                let field_value = &captures[2];
                
                match field_name {
                    "byr" => {
                        found_fields.insert(String::from(field_name));
                        if validate {
                            println!("nop");
                        }
                    },
                    "iyr" =>  {
                        found_fields.insert(String::from(field_name));
                        if validate {
                            println!("nop");
                        }
                    },
                    "eyr" =>  {
                        found_fields.insert(String::from(field_name));
                        if validate {
                            println!("nop");
                        }
                    },
                    "hgt" => {
                        found_fields.insert(String::from(field_name));
                        if validate {
                            println!("nop");
                        }
                    },
                    "hcl" => {
                        found_fields.insert(String::from(field_name));
                        if validate {
                            println!("nop");
                        }
                    },
                    "ecl" => {
                        found_fields.insert(String::from(field_name));
                        if validate {
                            println!("nop");
                        }
                    },
                    "pid" =>  {
                        found_fields.insert(String::from(field_name));
                        if validate {
                            println!("nop");
                        }
                    },
                    _ =>  {}, // ignored
                }
            }
        }

        return if found_fields.len() != 7 {
            Error::from_string(format!("Not all fields present! {}", found_fields.len()))
        } else {
            Ok(PassportFields{
                birth_year, issue_year, expiration_year, height, hair_color, eye_color, passport_id, 
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec!(
            String::from("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd"),
            String::from("byr:1937 iyr:2017 cid:147 hgt:183cm"),
            String::from(""),
            String::from("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884"),
            String::from("hcl:#cfa07d byr:1929"),
            String::from(""),
            String::from("hcl:#ae17e1 iyr:2013"),
            String::from("eyr:2024"),
            String::from("ecl:brn pid:760753108 byr:1931"),
            String::from("hgt:179cm"),
            String::from(""),
            String::from("hcl:#cfa07d eyr:2025 pid:166559648"),
            String::from("iyr:2011 ecl:brn hgt:59in"),
        )
    }

    fn example_invalid() -> Vec<String> {
        vec!(
            String::from("ecl:aaa pid:860033327 eyr:2020 hcl:#fffffd"),
            String::from("byr:1937 iyr:2017 cid:147 hgt:183cm"),
            String::from(""),
        )
    }

    #[test]
    fn test_first() {
        let result = first(&example()).unwrap();
        let expected = 2;
        assert_eq!(result, expected)
    }

    #[test]
    fn test_new_passport_fields_no_validate() {
        let input: Vec<String> = example().iter().take(2).map(|s| s.clone()).collect();
        let result = PassportFields::new(&input, false).unwrap();
        let expected = PassportFields{
            birth_year: 0,
            eye_color: "",
            passport_id: "",
            expiration_year: 0,
            hair_color: "",
            issue_year: 0,
            height: Height::Missing,
        };
        assert_eq!(result, expected);
    }

    // #[test]
    fn test_new_passport_fields_validate_ok() {
        let input: Vec<String> = example().iter().take(2).map(|s| s.clone()).collect();
        let result = PassportFields::new(&input, true).unwrap();
        let expected = PassportFields{
            birth_year: 1937,
            eye_color: "gry",
            passport_id: "860033327",
            expiration_year: 2020,
            hair_color: "#fffffd",
            issue_year: 2017,
            height: Height::Centimeters(183),
        };
        assert_eq!(result, expected);
    }

    
    // #[test]
    fn test_new_passport_fields_validate_invalid() {
        let input: Vec<String> = example_invalid();
        let result = PassportFields::new(&input, true);
        assert!(result.is_err());
    }
}
