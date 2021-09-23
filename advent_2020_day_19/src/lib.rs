use advent_2020_common::Error;
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Input {
    rules: HashMap<usize, String>,
    lines: Vec<String>,
}

enum ParseState {
    Rules,
    Lines,
}

pub fn parse(input: &[String]) -> Result<Input, Error> { 
    let mut lines = Vec::new(); 
    let mut rules = HashMap::new(); 

    let mut state = ParseState::Rules;
    for line in input {
        match state {
            _ if line.len() < 1 => state = ParseState::Lines,
            ParseState::Rules => {
                let mut split = line.split(':');
                match split.nth(0) {
                    None => return Error::new("rule without id!?"),
                    Some(id_str) => {
                        let id: usize = id_str.parse().or(Error::new("not an int"))?;
                        match split.nth(0) {
                            None => return Error::new("rule without other part"),
                            Some(rule_str) => {
                                rules.insert(id, String::from(rule_str.trim()));
                            }
                        }
                    }
                }
            },
            ParseState::Lines => { 
                lines.push(line.clone())
            }
        }
    }

    Ok(Input{lines, rules})
}

fn process_rule_recur(input: &Input, rule: &String) -> String {
    match rule {
        r if r.contains("\"") => String::from(r.chars().nth(1).unwrap()),
        r if r.contains("|") => {
            let sub_rules: Vec<String> = r.split('|')
                .map(|s| s.trim())
                .map(|s| process_rule_recur(input, &String::from(s)))
                .collect();
            let mut accum = sub_rules.join(")|(");
            accum.insert(0, '(');
            accum.insert(0, '(');
            accum.push_str("))");
            accum
        },
        r => {
            let sub_rules_processed = r.split(' ')
                .map(|s| s.trim())
                .map(|r_id_str| r_id_str.parse::<usize>().unwrap())
                .map(|r_id| input.rules.get(&r_id).unwrap())
                .map(|r| process_rule_recur(&input, r));
            let mut accum = String::new();
            for sub in sub_rules_processed {
                accum.push_str(sub.as_str());
            }
            accum
        },
    }
}

pub fn first(input: &[String]) -> Result<usize, Error> { 
    let input = parse(&input)?;

    let mut regex_str = process_rule_recur(&input, &input.rules.get(&0).unwrap());
    regex_str.insert(0, '^');
    regex_str.push_str("$");
    let regex = Regex::new(regex_str.as_str()).or(Error::new("invalid regex generated"))?;

    let mut accum = 0;
    for line in input.lines {
        if regex.is_match(line.as_str()) {
            accum += 1;
        }
    }
    
    Ok(accum)
}


pub fn second(input: &[String]) -> Result<usize, Error> { 
    let mut input = parse(&input)?;
    input.rules.insert(8, String::from("42 | 42 8"));
    input.rules.insert(11, String::from("42 31 | 42 11 31"));

    let mut regex_str = process_rule_recur(&input, &input.rules.get(&0).unwrap());
    regex_str.insert(0, '^');
    regex_str.push_str("$");
    let regex = Regex::new(regex_str.as_str()).or(Error::new("invalid regex generated"))?;

    let mut accum = 0;
    for line in input.lines {
        if regex.is_match(line.as_str()) {
            accum += 1;
        }
    }
    
    Ok(accum)
}


#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> { 
        vec!(
            String::from("0: 1 2"),
            String::from("1: \"a\""),
            String::from("2: 1 3 | 3 1"),
            String::from("3: \"b\""),
        )
    }

    fn example2() -> Vec<String> { 
        vec!(
            String::from("0: 4 1 5"),
            String::from("1: 2 3 | 3 2"),
            String::from("2: 4 4 | 5 5"),
            String::from("3: 4 5 | 5 4"),
            String::from("4: \"a\""),
            String::from("5: \"b\""),
            String::from(""),
            String::from("ababbb"),
            String::from("bababa"),
            String::from("abbbab"),
            String::from("aaabbb"),
            String::from("aaaabbb"),
        )
    }

    fn example3() -> Vec<String> { 
        vec!(
            String::from("42: 9 14 | 10 1"),
            String::from("9: 14 27 | 1 26"),
            String::from("10: 23 14 | 28 1"),
            String::from("1: \"a\""),
            String::from("11: 42 31"),
            String::from("5: 1 14 | 15 1"),
            String::from("19: 14 1 | 14 14"),
            String::from("12: 24 14 | 19 1"),
            String::from("16: 15 1 | 14 14"),
            String::from("31: 14 17 | 1 13"),
            String::from("6: 14 14 | 1 14"),
            String::from("2: 1 24 | 14 4"),
            String::from("0: 8 11"),
            String::from("13: 14 3 | 1 12"),
            String::from("15: 1 | 14"),
            String::from("17: 14 2 | 1 7"),
            String::from("23: 25 1 | 22 14"),
            String::from("28: 16 1"),
            String::from("4: 1 1"),
            String::from("20: 14 14 | 1 15"),
            String::from("3: 5 14 | 16 1"),
            String::from("27: 1 6 | 14 18"),
            String::from("14: \"b\""),
            String::from("21: 14 1 | 1 14"),
            String::from("25: 1 1 | 1 14"),
            String::from("22: 14 14"),
            String::from("8: 42"),
            String::from("26: 14 22 | 1 20"),
            String::from("18: 15 15"),
            String::from("7: 14 5 | 1 21"),
            String::from("24: 14 1"),
            String::from(""),
            String::from("abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa"),
            String::from("bbabbbbaabaabba"),
            String::from("babbbbaabbbbbabbbbbbaabaaabaaa"),
            String::from("aaabbbbbbaaaabaababaabababbabaaabbababababaaa"),
            String::from("bbbbbbbaaaabbbbaaabbabaaa"),
            String::from("bbbababbbbaaaaaaaabbababaaababaabab"),
            String::from("ababaaaaaabaaab"),
            String::from("ababaaaaabbbaba"),
            String::from("baabbaaaabbaaaababbaababb"),
            String::from("abbbbabbbbaaaababbbbbbaaaababb"),
            String::from("aaaaabbaabaaaaababaa"),
            String::from("aaaabbaaaabbaaa"),
            String::from("aaaabbaabbaaaaaaabbbabbbaaabbaabaaa"),
            String::from("babaaabbbaaabaababbaabababaaab"),
            String::from("aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"),
        )
    }

    #[test]
    fn test_first() { 
        let result = first(&example2()).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_example3() {
        let result = first(&example3()).unwrap();
        assert_eq!(result, 3);

        let result = second(&example3()).unwrap();
        assert_eq!(result, 12);
    }

    #[test]
    fn test_process_rule_recur() {
        let input = parse(&example()).unwrap(); 
        let processed = process_rule_recur(&input, &input.rules.get(&0).unwrap());
        assert_eq!(processed, "a((ab)|(ba))");
    }

    #[test]
    fn test_process_rule_recur_2() {
        let input = parse(&example2()).unwrap();
        let processed = process_rule_recur(&input, &input.rules.get(&0).unwrap());
        assert_eq!(processed, "a((((aa)|(bb))((ab)|(ba)))|(((ab)|(ba))((aa)|(bb))))b")
    }

    #[test]
    fn test_parse() { 
        let result = parse(&example()).unwrap();
        let mut rules = HashMap::new();
        rules.insert(0, String::from("1 2"));
        rules.insert(1, String::from("\"a\""));
        rules.insert(2, String::from("1 3 | 3 1"));
        rules.insert(3, String::from("\"b\""));
        let expected = Input{
            rules,
            lines: vec!(),
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_second() { 
        let result = parse(&example2()).unwrap(); 
        let mut rules = HashMap::new();
        rules.insert(0, String::from("4 1 5"));
        rules.insert(1, String::from("2 3 | 3 2"));
        rules.insert(2, String::from("4 4 | 5 5"));
        rules.insert(3, String::from("4 5 | 5 4"));
        rules.insert(4, String::from("\"a\"")); 
        rules.insert(5, String::from("\"b\""));
        let expected = Input{
            rules,
            lines: vec!(
                String::from("ababbb"),
                String::from("bababa"),
                String::from("abbbab"),
                String::from("aaabbb"),
                String::from("aaaabbb"),
            )
        };
        assert_eq!(result, expected);
    }
}