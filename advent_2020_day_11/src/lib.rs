use advent_2020_common::Error;
use advent_2020_common::life::*;

pub fn first(input: &mut LifeSpace) -> Result<usize, Error> {   
    loop {
        let mut assigner = Assigner::new();

        for (coord, value) in input.spots.iter() {
            let occupied_count = input.neighbors(coord.row(), coord.column())
                .filter(|v| *v == LifeOption::Occupied)
                .count();
            match value {
                LifeOption::Floor => continue,
                LifeOption::Occupied if occupied_count >= 4 => {
                    assigner.assign(coord.row(), coord.column(), LifeOption::Unoccupied)
                },
                LifeOption::Unoccupied if occupied_count == 0 => {
                    assigner.assign(coord.row(), coord.column(), LifeOption::Occupied)
                }
                _ => ()
            };
        }
        
        if assigner.empty() {
            break;
        }

        assigner.commit(input);
    }

    // count the occupied spots
    Ok(input.spots.iter().filter(|(_, s)| **s == LifeOption::Occupied).count())
}

pub fn second(input: &mut LifeSpace) -> Result<usize, Error> {
    loop {
        let mut assigner = Assigner::new();

        for (coord, value) in input.spots.iter() {
            let occupied_count = input.neighbors_skip_floor(coord.row(), coord.column())
                .filter(|v| *v == LifeOption::Occupied)
                .count();
            match value {
                LifeOption::Floor => continue,
                LifeOption::Occupied if occupied_count >= 5 => {
                    assigner.assign(coord.row(), coord.column(), LifeOption::Unoccupied)
                },
                LifeOption::Unoccupied if occupied_count == 0 => {
                    assigner.assign(coord.row(), coord.column(), LifeOption::Occupied)
                }
                _ => ()
            };
        }
        
        if assigner.empty() {
            break;
        }

        assigner.commit(input);
    }

    // count the occupied spots
    Ok(input.spots.iter().filter(|(_, s)| **s == LifeOption::Occupied).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec!(
            String::from("L.LL.LL.LL"),
            String::from("LLLLLLL.LL"),
            String::from("L.L.L..L.."),
            String::from("LLLL.LL.LL"),
            String::from("L.LL.LL.LL"),
            String::from("L.LLLLL.LL"),
            String::from("..L.L....."),
            String::from("LLLLLLLLLL"),
            String::from("L.LLLLLL.L"),
            String::from("L.LLLLL.LL"),
        )
    }

    #[test]
    fn test_first() {
        let mut input = LifeSpace::new(&example()).unwrap();
        let result = first(&mut input).unwrap();
        assert_eq!(result, 37);
    }

    #[test]
    fn test_second() {
        let mut input = LifeSpace::new(&example()).unwrap();
        let result = second(&mut input).unwrap();
        assert_eq!(result, 26);
    }
}
