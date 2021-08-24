use std::collections::HashMap;
use advent_2020_common::Error;

pub fn first(input: &[String]) -> Result<usize, Error> {
    count_questions(input, false)
}

pub fn second(input:&[String]) -> Result<usize, Error> {
    count_questions(input, true)
}

fn count_questions(input: &[String], require_all: bool) -> Result<usize, Error> {
    let mut question_count = 0;
    let mut current_start = 0;
    let mut current_question_yesses = HashMap::<char, usize>::new();

    for (i, input_line) in input.iter().enumerate() {
        if input_line.len() == 0 {
            for (_question_name, count) in current_question_yesses.iter() {
                if !require_all || (*count == (i - current_start)) {
                    question_count = question_count + 1;
                }
            }
            current_question_yesses.clear();
            current_start = i + 1;
        } else {
            for c in input_line.chars() {
                let next_count = current_question_yesses.get(&c)
                    .map(|current| current + 1)
                    .unwrap_or(1);
                    
                current_question_yesses.insert(c, next_count);
            }
        }
    }

    Ok(question_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec!(
            String::from("abc"),
            String::from(""),
            String::from("a"),
            String::from("b"),
            String::from("c"),
            String::from(""),
            String::from("ab"),
            String::from("ac"),
            String::from(""),
            String::from("a"),
            String::from("a"),
            String::from("a"),
            String::from("a"),
            String::from(""),
            String::from("b"),
            String::from(""),
        )
    }

    #[test]
    fn test_first() {
        let result = first(&example()).unwrap();
        let expected = 11;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_second() {
        let result = second(&example()).unwrap();
        let expected = 6;
        assert_eq!(result, expected);
    }
}