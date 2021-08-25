const SUM_LOOKED_FOR: u32 = 2020;
use advent_2020_common::{find_complements, input_to_nums};

pub fn first(input: &[String]) -> u32 {
    let input = input_to_nums(input);
    find_complements(input.as_slice(), SUM_LOOKED_FOR, true).unwrap()
}



pub fn second(input: &[String]) -> u32 {
    let input = input_to_nums(input);
    let input_len = input.len();

    for i in 0 .. input_len {
        let complement = SUM_LOOKED_FOR - input[i];
        let rest: &[u32] = &[&input[..i], &input[(i+1)..]].concat();
        if let Some(partial_product) = find_complements(rest, complement, true) {
            return partial_product * input[i];
        }
    }
    
    panic!("Solution not found!")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        let input: Vec<&str> = vec!("1721", "979", "366", "299", "675", "1456");
        input.iter()
            .map(|s: &&str| { String::from(*s) })
            .collect()
    }
    #[test]
    fn first_test() {
        let input = example();
        let result = first(&input);
        assert_eq!(result, 514579);
    }

    #[test]
    fn second_test() { 
        let input = example();
        let result = second(&input);
        assert_eq!(result, 241861950);
    }
}