use advent_2020_common::Error;
use advent_2020_common::life::*;

pub fn first(input: &mut LifeSpace) -> Result<usize, Error> {   
    loop {
        let mut assigner = Assigner::new();

        for (coord, value) in input.spots.iter() {
            let occupied_count = input.neighbors(coord.row, coord.column)
                .filter(|v| *v == LifeOption::Occupied)
                .count();
            match value {
                LifeOption::Floor => continue,
                LifeOption::Occupied if occupied_count >= 4 => {
                    assigner.assign(coord.row, coord.column, LifeOption::Unoccupied)
                },
                LifeOption::Unoccupied if occupied_count == 0 => {
                    assigner.assign(coord.row, coord.column, LifeOption::Occupied)
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
            let occupied_count = input.neighbors_skip_floor(coord.row, coord.column)
                .filter(|v| *v == LifeOption::Occupied)
                .count();
            match value {
                LifeOption::Floor => continue,
                LifeOption::Occupied if occupied_count >= 5 => {
                    assigner.assign(coord.row, coord.column, LifeOption::Unoccupied)
                },
                LifeOption::Unoccupied if occupied_count == 0 => {
                    assigner.assign(coord.row, coord.column, LifeOption::Occupied)
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

    //#[test]
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

    //#[test]
    fn test_parse() {
        let mut result = LifeSpace::new(&example()).unwrap();
        assert_eq!(result.at(0, 0), Some(&LifeOption::Unoccupied));
        assert_eq!(result.at(0, 1), Some(&LifeOption::Floor));

        let mut assigner = Assigner::new();
        assigner.assign(5, 5, LifeOption::Occupied);
        assigner.commit(&mut result);
        assert_eq!(result.at(5, 5), Some(&LifeOption::Occupied));
    }

    //#[test]
    fn test_neighbors() {
        let mut input = LifeSpace::new(&example()).unwrap();
        let iter = input.neighbors(1, 1);
        let result: Vec<LifeOption> = iter.collect();
        assert_eq!(result, vec!(
            LifeOption::Unoccupied, LifeOption::Floor, LifeOption::Unoccupied,
            LifeOption::Unoccupied,                    LifeOption::Unoccupied,
            LifeOption::Unoccupied, LifeOption::Floor, LifeOption::Unoccupied,
        ));
    }

    //#[test]
    fn test_neighbors_limits() {
        let mut input = LifeSpace::new(&example()).unwrap();
        let iter = input.neighbors(9, 0);
        let result: Vec<LifeOption> = iter.collect();
        assert_eq!(result, vec!(
            LifeOption::Unoccupied, LifeOption::Floor, LifeOption::Floor,
        ));
    }

    //#[test]
    fn test_neighbors_right_limit() {
        let mut input = LifeSpace::new(&example()).unwrap();
        let iter = input.neighbors(7, 9);
        let result: Vec<LifeOption> = iter.collect();
        assert_eq!(result, vec!(
            LifeOption::Floor, LifeOption::Floor, 
            LifeOption::Unoccupied, 
            LifeOption::Floor, LifeOption::Unoccupied,
        ));
    }

    //#[test]
    fn test_neigbors_skip_floor_lots() {
        let example = vec!(
            String::from(".......#."),
            String::from("...#....."),
            String::from(".#......."),
            String::from("........."),
            String::from("..#L....#"),
            String::from("....#...."),
            String::from("........."),
            String::from("#........"),
            String::from("...#....."),
        );
        let input = LifeSpace::new(&example).unwrap();
        let iter = input.neighbors_skip_floor(4, 3);
        let result: Vec<LifeOption> = iter.collect();
        assert_eq!(result, vec!(
            LifeOption::Occupied, LifeOption::Occupied, LifeOption::Occupied,
            LifeOption::Occupied,                       LifeOption::Occupied,
            LifeOption::Occupied, LifeOption::Occupied, LifeOption::Occupied,
        ));
    }

    //#[test]
    fn test_neighbors_skip_floor_one() {
        let example = vec!(
            String::from("............."),
            String::from(".L.L.#.#.#.#."),
            String::from("............."),
        );
        let input = LifeSpace::new(&example).unwrap();
        let iter = input.neighbors_skip_floor(1, 1);
        let result: Vec<LifeOption> = iter.collect();
        assert_eq!(result, vec!(
            LifeOption::Unoccupied,
        ));
    }

    //#[test]
    fn test_neighbors_skip_none() {
        let example = vec!(
            String::from(".##.##."),
            String::from("#.#.#.#"),
            String::from("##...##"),
            String::from("...L..."),
            String::from("##...##"),
            String::from("#.#.#.#"),
            String::from(".##.##."),
        );
        let input = LifeSpace::new(&example).unwrap();
        let iter = input.neighbors_skip_floor(3, 3);
        let result: Vec<LifeOption> = iter.collect();
        assert_eq!(result, vec!());
    }
}
