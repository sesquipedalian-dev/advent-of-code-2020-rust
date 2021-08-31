use advent_2020_common::Error;
use std::collections::HashMap;

pub fn first(input: &[u32]) -> Result<u32, Error> {
    spoken_word_game(input, 2020)
}

pub fn second(input: &[u32]) -> Result<u32, Error> {
    spoken_word_game(input, 30000000)
}

fn spoken_word_game(input: &[u32], iterations: usize) -> Result<u32, Error> {
    let mut spoken_words: HashMap<usize, (usize, Option<usize>)> = HashMap::new(); // number -> last turn spoken, preceeding diff between last turns it was spoken

    let mut previous = 0;

    for i in 1 ..= iterations {
        let (spoken_num, previous_turn) = if i <= input.len() {
            (input[i-1] as usize, None)
        } else {
            let new_spoken = match spoken_words.get(&previous) {
                Some((last_turn, Some(turn_before))) => {
                    last_turn - turn_before
                },
                _ => 0,
            };

            let prev_turn_spoken = spoken_words.get(&new_spoken).map(|(last_turn, _)| *last_turn);
            (new_spoken, prev_turn_spoken)
        };

        spoken_words.insert(spoken_num, (i, previous_turn));
        previous = spoken_num;
    }

    Ok(previous as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<u32> {
        vec!(0, 3, 6)
    }

    #[test]
    fn test_first() { 
        let result = first(&example()).unwrap();
        assert_eq!(result, 436);
    }

    #[test]
    fn test_second() {
        let result = second(&example()).unwrap();
        assert_eq!(result, 175594)
    }
}