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
            _ if bus_id_entry == "x" => continue, //ignored
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
    fn test_parse() {
        let result = parse(&example()).unwrap();
        assert_eq!(result, BusProbInput{earliest_departure_timestamp: 939, in_service_bus_ids: vec!(7,13,59,31,19)});
    }
}