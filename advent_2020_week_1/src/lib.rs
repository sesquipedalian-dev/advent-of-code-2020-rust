const SUM_LOOKED_FOR: u32 = 2020;
use std::collections::HashSet;

pub fn first(input: &[String]) -> u32 {
    let input = input_to_nums(input);
    find_complements(input, SUM_LOOKED_FOR).unwrap()
}

fn find_complements(input: Vec<u32>, sum_looked_for: u32) -> Option<u32> {
    let mut seen_complements: HashSet<u32> = HashSet::new();  
    for current in input.iter() {
        if sum_looked_for < *current {
            continue;
        }

        let my_complement = sum_looked_for - current;

        if seen_complements.contains(&current) {
            return Some(current * my_complement);
        }

        seen_complements.insert(my_complement);
    }
    None
}

pub fn second(input: &[String]) -> u32 {
    let input = input_to_nums(input);
    let input_len = input.len();

    for i in 0 .. input_len {
        let complement = SUM_LOOKED_FOR - input[i];
        let rest = [&input[..i], &input[(i+1)..]].concat();
        if let Some(partial_product) = find_complements(rest, complement) {
            return partial_product * input[i];
        }
    }
    
    panic!("Solution not found!")
}

fn input_to_nums(input: &[String]) -> Vec<u32> {
    input
        .iter()
        .map(|s| { s.parse()} )
        .flatten()
        .collect()
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