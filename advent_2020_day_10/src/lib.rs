use advent_2020_common::Error;
use std::collections::HashMap;
// assume every adapter has a different joltage

// the joltage ratings infer a directed , weighted graph
// the edges are form from one joltage's rated output, to the input of devices that take 1, 2, or 3 more joltage
// there's an implied node with 0 in-degree and a rated joltage of 0
// there's a final with out degree 0 with a rated joltage of the highest other listed one, + 3
// find a spanning tree with DFS / detect cycles
// (cycles are disallowed because you can only use each adapter once)

// count the number of edges with 1 weight and 3 weight to get the answer

// ez mode - sort and count the skips 

pub fn first(input: &mut [u32]) -> Result<u32, Error> {
    input.sort();

    let (mut count_1s, mut count_3s) = (0, 1); // count_3s gets an extra for the final adapter -> device
    let mut previous_joltage = 0;

    for current_joltage in input.iter() {
        match current_joltage - previous_joltage {
            1 => count_1s += 1,
            3 => count_3s += 1,
            _ => (),
        }
        previous_joltage = *current_joltage;
    }

    Ok(count_1s * count_3s)
}

pub fn second(input: &mut [u32]) -> Result<u64, Error> { 
    let mut new_vec: Vec<u64> = Vec::new();
    new_vec.push(0);
    for i in input {
        new_vec.push(*i as u64);
    }
    new_vec.sort();
    let last = new_vec[new_vec.len() - 1];
    new_vec.push(last + 3);

    let mut cache: HashMap<usize, u64> = HashMap::new();

    Ok(paths_to_end(0, &new_vec, &mut cache))
}

fn paths_to_end(i: usize, input: &[u64], cache: &mut HashMap<usize, u64>) -> u64 {
    if let Some(v) = cache.get(&i) {
        return *v;
    }

    // base case: if we're at the end of the array there's only one path
    if i == input.len() - 1 {
        return 1;
    }

    // otherwise, recurse on all the paths from this node to the end and sum
    let result = ((i + 1) ..= (i + 3))
        .filter(|j| (*j < input.len()) && (input[*j] - input[i] <= 3))
        .map(|j| paths_to_end(j, input, cache))
        .fold(0, |accum, next| accum + next);
    cache.insert(i, result);
    result
}

#[cfg(test)]
mod tests {

    use super::*;

    fn example() -> Vec<u32> {
        vec!(
            16,
            10,
            15,
            5,
            1,
            11,
            7,
            19,
            6,
            12,
            4,
        )
        // 0,1,4,5,6,7,10,11,12,15,16,19,22
      }

    fn example2() -> Vec<u32> {
        vec!(
            28,
            33,
            18,
            42,
            31,
            14,
            46,
            20,
            48,
            47,
            24,
            23,
            49,
            45,
            19,
            38,
            39,
            11,
            1,
            32,
            25,
            35,
            8,
            17,
            7,
            9,
            4,
            2,
            34,
            10,
            3,
        )
    }

    fn example3() -> Vec<u32> {
        vec!(
            1, 4, 5, 6, 7
        )
    }

    #[test]
    fn test_first() { 
        let result = first(&mut example()).unwrap();
        assert_eq!(result, 35);

        let result = first(&mut example2()).unwrap();
        assert_eq!(result, 220);
    }

    #[test]
    fn test_second() { 
        let result = second(&mut example()).unwrap();
        assert_eq!(result, 8);

        let result = second(&mut example2()).unwrap();
        assert_eq!(result, 19208);

        let result = second(&mut example3()).unwrap();
        assert_eq!(result, 4);
    }
}