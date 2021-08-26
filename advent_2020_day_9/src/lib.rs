use advent_2020_common::{find_complements, Error};
use std::cmp::Ordering;

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

pub fn second(input: &Vec<u32>, target_num: u32) -> Result<u32, Error> {
    let mut start_range = 0;
    let mut end_range = 1; // must be at least two numbers in the range
    let mut current_sum = input[0] + input[1];

    loop {
        // catch a case where sum - start_range == target_num
        if end_range == start_range {
            end_range += 1;
            continue;
        }

        let compare = current_sum.cmp(&target_num);
        match compare {
            Ordering::Less => {
                end_range += 1;
                current_sum += input[end_range];
            },
            Ordering::Greater => {
                current_sum -= input[start_range];
                start_range += 1;
            },
            Ordering::Equal => {
                let mut min = u32::MAX;
                let mut max = 0;
                for i in start_range ..= end_range {
                    if input[i] < min {
                        min = input[i];
                    }
                    if input[i] > max {
                        max = input[i];
                    }
                }
                return Ok(min + max);
            }
        }
       
    }
    // TODO: DP problem
    // first: binary search the input vector for our target number.  the numbers
    // _tend_ to increase as you go, and it's more likely that the answer 
    // is close to where you would insert it into the list
    //
    // store sub-solutions in a n * n matrix where
    // 0 35
    // 1 20
    // 2 15
    // 3 25
    // 4 47
    // 5 40
    // 6 62
    // 7 55
    // 8 65
    // 9 95
    // matrix[i, j] = sum(input[i..=(i+1)])
    // matrix[0, j] = input[j]
    // matrix[i, j] = matrix[i-1, j] + input[i+j]
    // stop calculating a column  when > sum looked for
    //  0   1  2  3   4  5  6
    //0           25
    //1        40 72  
    //2
    //3
    //
    // iterate from middle col out, with increasing size of range
    // hmm, wastes some time because e.g. matrix[6, 3] contains matrix[2, 4]
    //
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

    #[test]
    fn test_second() {
        let input = &input_to_nums(&example());
        let result = first(input, 5).unwrap();
        let result = second(input, result as u32).unwrap();
        assert_eq!(result, 62);
    }
}