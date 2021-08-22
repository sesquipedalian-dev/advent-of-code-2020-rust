use advent_2020_common::Error;

fn first(input: &[String]) -> u32 {
    0
}

#[derive(PartialEq, Debug)]
pub struct PasswordRule {
    first_num: u32,
    second_num: u32,
    letter: char,
    password_to_test: String
}

impl PasswordRule {
    /// Take a string representation like so:
    /// "1-3 a: abcde"
    /// and parse it into this struct's fields.
    /// Examples:
    /// ```
    /// use advent_2020_week_2::PasswordRule;
    /// let input = String::from("1-3 a: abcde");
    /// let pw_rule = PasswordRule::from_string(&input).unwrap();
    /// ```
    /// 
    pub fn from_string(input: &String) -> Result<PasswordRule, Error> {
        let mut first_num = 0;
        let mut second_num = 0;
        let mut letter = 'a';
        let mut password_to_test = String::from("");

        let mut found = false;

        for (i, part) in input.split(' ').enumerate() {
            found = true;

            match i {
                0 => {
                    let parts: Vec<&str> = part.split('-').collect();
                    if parts.len() < 2 {
                        return Error::new("Couldn't parse, int part missing");
                    }

                    first_num = (&parts[0]).parse().or(Error::new("not an int"))?;
                    second_num = (&parts[1]).parse().or(Error::new("not an int"))?;
                }
                1 => {
                    letter = part.chars().nth(0).unwrap();
                }
                2 => {
                    password_to_test = String::from(part);
                }
                _ => return Error::new("couldn't parse, too many parts!?")
            }
        }

        if !found {
            Error::new("NYI")
        } else {
            Ok(PasswordRule{first_num, second_num, letter, password_to_test})
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        let input: Vec<&str> = vec!("1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc");
        input.iter()
            .map(|s: &&str| { String::from(*s) })
            .collect()
    }

    #[test]
    fn first() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_new_rule() {
        let input = String::from("1-3 a: abcdef");
        let rule = PasswordRule::from_string(&input).unwrap();
        let expected = PasswordRule{
            first_num: 1,
            second_num: 3,
            letter: 'a',
            password_to_test: String::from("abcdef")
        };
        assert_eq!(rule, expected);
    }
}