use advent_2020_common::{find_complements, Error};

pub fn first(input: &Vec<u32>, prelude_size: usize) -> Result<usize, Error> {
    for (index, num) in input.iter().enumerate() {
        if index < prelude_size {
            continue;
        }

        let range: &[u32] = &input.as_slice()[(index - prelude_size)..index];
        match find_complements(range, *num, false) {
            Some(_) => (),
            _ => return Ok(*num as usize),
        }
    }

    Error::new("Not found!")
}

#[cfg(test)]
mod tests{
    use super::*;
    use advent_2020_common::input_to_nums;

    fn example() -> Vec<String> {
        vec!(
            String::from("35"),
            String::from("20"),
            String::from("15"),
            String::from("25"),
            String::from("47"),
            String::from("40"),
            String::from("62"),
            String::from("55"),
            String::from("65"),
            String::from("95"),
            String::from("102"),
            String::from("117"),
            String::from("150"),
            String::from("182"),
            String::from("127"),
            String::from("219"),
            String::from("299"),
            String::from("277"),
            String::from("309"),
            String::from("576"),
        )
    }

    #[test]
    fn test_first() {
        let result = first(&input_to_nums(&example()), 5).unwrap();
        assert_eq!(result, 127);
    }

    #[test]
    fn test_parse() {
        let nums = input_to_nums(&example());
        assert_eq!(nums[0], 35);
        assert_eq!(nums[10], 102);
        assert_eq!(nums[19], 576);
    }
}