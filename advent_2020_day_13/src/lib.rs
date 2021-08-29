use advent_2020_common::Error;

#[derive(Debug, PartialEq)]
pub struct BusProbInput {
    earliest_departure_timestamp: u32,
    in_service_bus_ids: Vec<u32>,
}

pub fn parse(input: &[String]) -> Result<BusProbInput, Error> {
    let timestamp: u32 = input[0].parse().or(Error::new("Invalid timestamp"))?;
    let mut bus_ids: Vec<u32> = Vec::new();

    for bus_id_entry in input[1].split(',') {
        let num: Result<u32, _> = bus_id_entry.parse();
        match num {
            Ok(num) => bus_ids.push(num),
            _ if bus_id_entry == "x" => bus_ids.push(1), // 0 indicates ignored slot in schedule
            _ => return Error::new("invalid bus id entry"),
        }
    }

    Ok(BusProbInput{earliest_departure_timestamp: timestamp, in_service_bus_ids: bus_ids})
}

pub fn first(input: &BusProbInput) -> Result<u32, Error> {
    let mut max_mod = None;

    for in_service_bus_id in input.in_service_bus_ids.iter() {
        if max_mod.is_none() {
            max_mod = Some((input.earliest_departure_timestamp % in_service_bus_id, in_service_bus_id));
        } else {
            let current_mod = input.earliest_departure_timestamp % in_service_bus_id;
            if current_mod > max_mod.unwrap().0 {
                max_mod = Some((current_mod, in_service_bus_id));
            }
        }
    }

    let (found_mod, found_id) = max_mod.unwrap();
    Ok((found_id - found_mod) * found_id)
}

pub fn second(input: &BusProbInput) -> Result<u64, Error> {
    // OK so, the largest number in the input bus_ids is worth basing our work around
    // since it will have the fewest even multiples to check
    //
    // For each of its multiples, we'll check if the other bus ids match the requirements
    // check if (timestamp - (larger_num_index - target_num_index)) % bus_ids(target_num_index) == 0
    // if they all work out, we've found the target num

    // first identify the largest number in the bus ids
    let (largest_bus_id_index, largest_bus_id) = input.in_service_bus_ids.iter().enumerate().fold(
        (0, 0),
        |(max_index, max_num), (index, num)| {
            if *num > max_num {
                (index, *num)
            } else {
                (max_index, max_num)
            }
        }
    );

    // next iterate through multiples of the largest_bus_id
    let mut multiple: u64 = input.in_service_bus_ids.iter().fold(0, |accum, next| accum * (*next as u64) );
    'outer: loop {
        multiple = multiple + largest_bus_id as u64;

        // for each other index, check if it matches
        for (target_index, target_id) in input.in_service_bus_ids.iter().enumerate() {
            if *target_id == largest_bus_id {
                continue;
            }
            
            // println!("Problematic sub coming up? {} {} {} {}", multiple, largest_bus_id_index, target_index, target_id);
            if ((multiple as i64) - (largest_bus_id_index as i64 - target_index as i64)) as u64 % (*target_id as u64) != 0 {
                continue 'outer;
            }
        }
        // 100000000000000
        // 9223372036854775807
        // 1572270007
        // 4294967295
        // 2147483647
        
        break;
    };

    Ok(multiple - largest_bus_id_index as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> { 
        vec!(
            String::from("939"),
            String::from("7,13,x,x,59,x,31,19"),
        )
    }

    #[test]
    fn test_first(){
        let input = parse(&example()).unwrap();
        let result = first(&input).unwrap();
        assert_eq!(result, 295);
    }

    #[test]
    fn test_second() {
        let input = parse(&example()).unwrap();
        let result = second(&input).unwrap();
        assert_eq!(result, 1068781);
    }

    #[test]
    fn test_second_2() {
        let input = vec!(
            String::from("1"),
            String::from("17,x,13,19"),
        );
        let input = parse(&input).unwrap();
        let result = second(&input).unwrap();
        assert_eq!(result, 3417);
    }

    #[test]
    fn test_second_3() {
        let input = vec!(
            String::from("1"),
            String::from("67,7,59,61"),
        );
        let input = parse(&input).unwrap();
        let result = second(&input).unwrap();
        assert_eq!(result, 754018);
    }

    #[test]
    fn test_second_4() {
        let input = vec!(
            String::from("1"),
            String::from("67,x,7,59,61"),
        );
        let input = parse(&input).unwrap();
        let result = second(&input).unwrap();
        assert_eq!(result, 779210);
    }

    #[test]
    fn test_second_5() {
        let input = vec!(
            String::from("1"),
            String::from("67,7,x,59,61"),
        );
        let input = parse(&input).unwrap();
        let result = second(&input).unwrap();
        assert_eq!(result, 1261476);
    }

    #[test]
    fn test_second_6() {
        let input = vec!(
            String::from("1"),
            String::from("1789,37,47,1889"),
        );
        let input = parse(&input).unwrap();
        let result = second(&input).unwrap();
        assert_eq!(result, 1202161486);
    }

    #[test]
    fn test_parse() {
        let result = parse(&example()).unwrap();
        assert_eq!(result, BusProbInput{earliest_departure_timestamp: 939, in_service_bus_ids: vec!(7,13,1,1,59,1,31,19)});
    }
}