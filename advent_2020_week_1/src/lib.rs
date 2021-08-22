const sum_looked_for:u32 = 2020;
use std::collections::HashSet;

pub fn first(input: &[String]) -> u32 {
    let input: Vec<u32> = input
        .iter()
        .map(|s| { s.parse()} )
        .flatten()
        .collect();
       
    let mut seen_complements: HashSet<u32> = HashSet::new();  
    for current in input.iter() {
        let my_complement = sum_looked_for - current;

        if seen_complements.contains(&current) {
            return current * my_complement;
        }

        seen_complements.insert(my_complement);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test() {
        let input: Vec<&str> = vec!("1721", "979", "366", "299", "675", "1456");
        let input: Vec<String> = input.iter()
            .map(|s: &&str| { String::from(*s) })
            .collect();
        let result = first(&input);
        assert_eq!(result, 514579);
    }
}