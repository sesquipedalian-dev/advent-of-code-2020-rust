use advent_2020_common::{Error};

#[derive(Debug)]
enum State {
    Initial,
    Add,
    Multiply,
}

pub fn first(input: &[String]) -> Result<u64, Error> {
    let mut accum: u64 = 0;

    for line in input {
        let line = line.replace(')', " ) ").replace('(', " ( ");

        let mut stack_values: Vec<(State, u64)> = vec!((State::Initial, 0));
        // let mut state = State::Initial;
        for token in line.split(' ') {
            let (state, current_value) = stack_values.pop().unwrap();

             match token {
                 x if x.len() < 1 => {
                     stack_values.push((state, current_value));
                     continue
                 }
                "(" => { 
                    stack_values.push((state, current_value));
                    stack_values.push((State::Initial, 0));
                }
                ")" => { 
                    let (prev_state, prev_value) = stack_values.pop().unwrap();
                    let new_value = match prev_state {
                        State::Add => prev_value + current_value,
                        State::Multiply => prev_value * current_value,
                        _ => current_value,
                    };
                    stack_values.push((State::Initial, new_value));
                }
                "+" => { 
                    stack_values.push((State::Add, current_value));
                }
                "*" => {
                    stack_values.push((State::Multiply, current_value));
                }
                // assume any other tokens are a single digit number
                x => {
                    let value: u64 = x.parse().or(Error::from_string(format!("Token not a digit ({})", x)))?;
                    let new_value = match state {
                        State::Add => current_value + value,
                        State::Multiply => current_value * value,
                        _ => value
                    };
                    stack_values.push((State::Initial, new_value));
                }
            }
        }

        let (_, value) = stack_values.pop().unwrap();
        accum += value;
    }
    
    Ok(accum)
}

#[derive(Clone, Copy, Debug)]
struct SecondToken {
    is_mul: bool,
    is_add: bool,
    value: u64,
}

impl SecondToken { 
    fn add() -> SecondToken { 
        SecondToken{is_mul: false, is_add: true, value: 0}
    }

    fn mul() -> SecondToken { 
        SecondToken{is_mul: true, is_add: false, value: 0}
    }

    fn value(v: u64) -> SecondToken {
        SecondToken{is_mul: false, is_add: false, value: v}
    }
}

fn solve_line_second(tokens: Vec<SecondToken>) -> Result<Vec<SecondToken>, Error> {
    // do adds first
    if let Some((index, _)) = tokens.iter().enumerate().find(|(_, t)| t.is_add) {
        let new_v = tokens[index - 1].value + tokens[index + 1].value;
        let mut recur_on_tokens: Vec<SecondToken> = Vec::new(); 
        for (i, next) in tokens.iter().enumerate() { 
            if i < (index - 1) || (index + 1) < i {
                recur_on_tokens.push(*next);
            } else if i == index { 
                recur_on_tokens.push(SecondToken::value(new_v))
            }
        }
        return solve_line_second(recur_on_tokens);
    }

    // then multiplies
    if let Some((index, _)) = tokens.iter().enumerate().find(|(_, t)| t.is_mul) {
        let new_v = tokens[index - 1].value * tokens[index + 1].value;
        let mut recur_on_tokens: Vec<SecondToken> = Vec::new(); 
        for (i, next) in tokens.iter().enumerate() { 
            if i < (index - 1) || (index + 1) < i {
                recur_on_tokens.push(*next);
            } else if i == index { 
                recur_on_tokens.push(SecondToken::value(new_v))
            }
        }
        return solve_line_second(recur_on_tokens);
    }

    // otherwise done
    Ok(tokens)
}

pub fn second(input: &[String]) -> Result<u64, Error> { 
    let mut sum: u64 = 0;

    for line in input {
        let mut this_level_tokens: Vec<SecondToken> = Vec::new();

        let line = line.replace(')', " ) ").replace('(', " ( ");
        let mut token_iter = line.split(' ');

        while let Some(token) = token_iter.next() {
            match token {
                x if x.len() < 1 => {
                    continue
                }
                "(" => {
                    let mut accum = String::new();
                    let mut depth = 0;
                    while let Some(embedded_token) = token_iter.next() {
                        accum.push_str(" ");
                        accum.push_str(embedded_token);
                        accum.push_str(" ");
                        match embedded_token { 
                            ")" if depth > 0 => depth -= 1,
                            ")" => break,
                            "(" => depth += 1,
                            _ => (),
                        }
                    }
                    accum.pop(); // remove trailing ')'
                    accum.pop();
                    let value = second(&[accum])?;
                    this_level_tokens.push(SecondToken::value(value));
                },
                "+" => this_level_tokens.push(SecondToken::add()),
                "*" => this_level_tokens.push(SecondToken::mul()), 
                x => {
                    let value: u64 = x.parse().or(Error::from_string(format!("Token not a digit ({})", x)))?;
                    this_level_tokens.push(SecondToken::value(value));
                }
            }
        }

        let solved_line = solve_line_second(this_level_tokens)?;
        if solved_line.len() != 1 {
            return Error::new("Multiple value tokens on line without op!?");
        }

        sum += solved_line[0].value;
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec!(
            String::from("1 + 2 * 3 + 4 * 5 + 6"),
        )
    }

    fn example2() -> Vec<String> { 
        vec!(
            String::from("1 + (2 * 3) + (4 * (5 + 6))"),
        )
    }

    fn example3() -> Vec<String> { 
        vec!(
            String::from("2 * 3 + (4 * 5)"),
            String::from("5 + (8 * 3 + 9 + 3 * 4 * 3)"),
            String::from("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            String::from("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
        )
    }

    #[test]
    fn test_first_first() {
        let result = first(&example()).unwrap();
        assert_eq!(result, 71)
    }

    #[test]
    fn test_first_second() { 
        let result = first(&example2()).unwrap();
        assert_eq!(result, 51)
    }

    #[test]
    fn test_first_third() { 
        let result = first(&example3()).unwrap();
        assert_eq!(result, 26 + 437 + 12240 + 13632);
    }

    #[test]
    fn test_second_first() { 
        let result = second(&example()).unwrap();
        assert_eq!(result, 231);
    }

    #[test]
    fn test_second_second() { 
        let result = second(&example2()).unwrap();
        assert_eq!(result, 51);
    }

    // #[test]
    fn test_second_third() { 
        let result = second(&example3()).unwrap();
        assert_eq!(result, 46 + 1445 + 669060 + 23340);
    }
}