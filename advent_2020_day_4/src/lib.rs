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
    static ref HEIGHT_REGEX: Regex = Regex::new("(\\d+)(cm|in)").unwrap();
}

lazy_static! { 
    static ref HAIR_COLOR_REGEX: Regex = Regex::new("^#([0-9a-f]{6})$").unwrap();
}

lazy_static! { 
    static ref PASSPORT_ID_REGEX: Regex = Regex::new("^[0-9]{9}$").unwrap();
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

pub fn second(input: &[String]) -> Result<u32, Error> {
    count_valid_passports(input, true)
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
struct PassportFields {
    birth_year: u32,
    issue_year: u32, 
    expiration_year: u32, 
    height: Height,
    hair_color: String,
    eye_color: String,
    passport_id: String,
}

fn extract_number(input: &str, field_name: &str, min: u32, max: u32, required_digits: Option<usize>) -> Result<u32, Error> {
    match input {
        v if required_digits.is_some() && v.len() != required_digits.unwrap() => {
            Error::from_string(format!("{} not {} digits: ({})", field_name, required_digits.unwrap(), v))
        },
        v => match v.parse::<u32>() {
            Err(_) => Error::from_string(format!("{} not a number", field_name)),
            Ok(v) if v < min || max < v => Error::from_string(format!("{} not in range {} - {}", field_name, min, max)),
            Ok(v) => Ok(v)
        },
    }
}

impl PassportFields {
    fn new(input: &[String], validate: bool) -> Result<PassportFields, Error> {
        let mut birth_year = 0;
        let mut issue_year = 0;
        let mut expiration_year = 0;
        let mut height = Height::Missing;
        let mut hair_color = String::new();
        let mut eye_color = String::new();
        let mut passport_id = String::new();
        let mut found_fields = HashSet::<String>::new();

        for line in input.iter() {
            for captures in FIELD_PAIR_REGEX.captures_iter(line) {
                let field_name = &captures[1];
                
                let field_value = String::from(&captures[2]);
                
                match field_name {
                    "byr" => {
                        found_fields.insert(String::from(field_name));
                        if validate {
                            birth_year = extract_number(&field_value, field_name, 1920, 2002, Some(4))?;
                        }
                    },
                    "iyr" =>  {
                        found_fields.insert(String::from(field_name));
                        if validate {
                            issue_year = extract_number(&field_value, field_name, 2010, 2020, Some(4))?;
                        }
                    },
                    "eyr" =>  {
                        found_fields.insert(String::from(field_name));
                        if validate {
                            expiration_year = extract_number(&field_value, field_name, 2020, 2030, Some(4))?;
                        }
                    },
                    "hgt" => {
                        found_fields.insert(String::from(field_name));
                        if validate {
                            match HEIGHT_REGEX.captures(&field_value) {
                                Some(captures) => {
                                    let measurement = &captures[1];
                                    let units = &captures[2];
                                    height = match units {
                                        "in" => {
                                            let measurement = extract_number(measurement, field_name, 59, 76, None)?;
                                            Height::Inches(measurement)
                                        },
                                        "cm" => {
                                            let measurement = extract_number(measurement, field_name, 150, 193, None)?;
                                            Height::Centimeters(measurement)
                                        },
                                        _ => return Error::new("height not in inches nor centimeres"),
                                    };
                                },
                                _ => return Error::new("height not formatted like (num)(units)")
                            }
                        }
                    },
                    "hcl" => {
                        found_fields.insert(String::from(field_name));
                        if validate {
                            if !HAIR_COLOR_REGEX.is_match(&field_value) {
                                return Error::new("hair color doesn't match format #[a-f0-9]{6}");
                            }
                            hair_color = field_value;
                        }
                    },
                    "ecl" => {
                        found_fields.insert(String::from(field_name));
                        if validate {
                            if !EYE_COLORS.contains(&field_value[..]) {
                                let valid_colors: Vec<&str> = EYE_COLORS.iter().map(|s| *s).collect();
                                return Error::from_string(format!("eye color must be one of {:?}", valid_colors));
                            }
                            eye_color = field_value;
                        }
                    },
                    "pid" =>  {
                        found_fields.insert(String::from(field_name));
                        if validate {
                            if !PASSPORT_ID_REGEX.is_match(&field_value) { 
                                return Error::new("passport_id doesn't match format [0-9]{6}");
                            }
                            passport_id = field_value;
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
                birth_year, issue_year, expiration_year, height,
                hair_color: String::from(hair_color),
                passport_id: passport_id.to_owned(),
                eye_color: eye_color.to_owned(),
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

    fn example_mixed() -> Vec<String> {
        vec!(
            String::from("eyr:1972 cid:100"),
            String::from("hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"),
            String::from(""),
            String::from("iyr:2019"),
            String::from("hcl:#602927 eyr:1967 hgt:170cm"),
            String::from("ecl:grn pid:012533040 byr:1946"),
            String::from(""),
            String::from("hcl:dab227 iyr:2012"),
            String::from("ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"),
            String::from(""),
            String::from("hgt:59cm ecl:zzz"),
            String::from("eyr:2038 hcl:74454a iyr:2023"),
            String::from("pid:3556412378 byr:2007"),
            String::from(""),
            String::from("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980"),
            String::from("hcl:#623a2f"),
            String::from(""),
            String::from("eyr:2029 ecl:blu cid:129 byr:1989"),
            String::from("iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"),
            String::from(""),
            String::from("hcl:#888785"),
            String::from("hgt:164cm byr:2001 iyr:2015 cid:88"),
            String::from("pid:545766238 ecl:hzl"),
            String::from("eyr:2022"),
            String::from(""),
            String::from("iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"),
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
    fn test_second() { 
        let result = second(&example_mixed()).unwrap();
        let expected = 4;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_new_passport_fields_no_validate() {
        let input: Vec<String> = example().iter().take(2).map(|s| s.clone()).collect();
        let result = PassportFields::new(&input, false).unwrap();
        let expected = PassportFields{
            birth_year: 0,
            eye_color: String::from(""),
            passport_id: String::from(""),
            expiration_year: 0,
            hair_color: String::from(""),
            issue_year: 0,
            height: Height::Missing,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_new_passport_fields_validate_ok() {
        let input: Vec<String> = example().iter().take(2).map(|s| s.clone()).collect();
        let result = PassportFields::new(&input, true).unwrap();
        let expected = PassportFields{
            birth_year: 1937,
            eye_color: String::from("gry"),
            passport_id: String::from("860033327"),
            expiration_year: 2020,
            hair_color: String::from("#fffffd"),
            issue_year: 2017,
            height: Height::Centimeters(183),
        };
        assert_eq!(result, expected);
    }

    
    #[test]
    fn test_new_passport_fields_validate_invalid() {
        let input: Vec<String> = example_invalid();
        let result = PassportFields::new(&input, true);
        assert!(result.is_err());
    }
}
