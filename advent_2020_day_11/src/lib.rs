use std::collections::HashMap;
use advent_2020_common::Error;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SeatingAreaOption {
    Occupied,
    Unoccupied,
    Floor,
}

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
pub struct Coord {
    row: usize,
    column: usize,
}

pub struct SeatingArea {
    spots: HashMap<Coord, SeatingAreaOption>,
    tentative_spots: HashMap<Coord, SeatingAreaOption>,
}

impl SeatingArea {
    pub fn new(input: &[String]) -> Result<SeatingArea, Error> {
        let mut row = 0;
        let mut column = 0;
        let mut result = SeatingArea{spots: HashMap::new(), tentative_spots: HashMap::new()};

        for line in input.iter(){
            column = 0;
            for spot in line.chars() {
                let new_opt: SeatingAreaOption = match spot {
                    '#' => SeatingAreaOption::Occupied,
                    '.' => SeatingAreaOption::Floor,
                    'L' => SeatingAreaOption::Unoccupied,
                    x => return Error::from_string(format!("unknown char {}", x)),
                };
                result.spots.insert(Coord{row, column}, new_opt);
                column += 1;
            }
            row += 1;
        }
        
        Ok(result)
    }

    pub fn at(&self, row: usize, column: usize) -> Option<&SeatingAreaOption> {
        self.spots.get(&Coord{row, column})
    }

    pub fn neighbors(&self, row: usize, column: usize) -> NeighborIterator {
        NeighborIterator{spots: &self.spots, row, column, count: 0}
    }

    pub fn to_string(&self) -> String {
        let mut max_row = 0;
        let mut max_col = 0;
        for (coord, value) in self.spots.iter() {
            if coord.row > max_row {
                max_row = coord.row;
            }
            if coord.column > max_col { 
                max_col = coord.column;
            }
        }

        let mut accum = String::new();
        for row in 0..=max_row {
            for column in 0..=max_col {
                let next_char = match self.at(row, column) {
                    Some(SeatingAreaOption::Occupied) => '#',
                    Some(SeatingAreaOption::Unoccupied) => 'L',
                    _ => '.',
                };
                accum.push(next_char);
            }
            accum.push('\n');
        }

        accum
    }
}

pub struct Assigner {
    spots: HashMap<Coord, SeatingAreaOption>,
}

impl Assigner {
    pub fn new() -> Assigner {
        Assigner{spots: HashMap::new()}
    }

    pub fn assign(&mut self, row: usize, column: usize, value: SeatingAreaOption) {
        self.spots.insert(Coord{row, column}, value);
    }
    
    pub fn commit(&mut self, other: &mut SeatingArea) {
        for (coord, value) in self.spots.iter() {
            other.spots.insert(*coord, *value);
        }
        self.spots.clear();
    }

    pub fn empty(&self) -> bool {
        self.spots.len() == 0
    }
}

pub struct NeighborIterator<'a> {
    spots: &'a HashMap<Coord, SeatingAreaOption>,
    row: usize, 
    column: usize,
    count: usize,
}

impl Iterator for NeighborIterator<'_> {
    type Item = SeatingAreaOption;
    fn next(&mut self) -> Option<SeatingAreaOption> {
        let (row_diff, column_diff) = match self.count {
            0 => (-1, -1),
            1 => (-1, 0),
            2 => (-1, 1),
            3 => (0, -1),
            4 => (0, 1),
            5 => (1, -1),
            6 => (1, 0),
            7 => (1, 1),
            _ => return None
        };
        self.count += 1;

        let row = match (self.row as isize) + row_diff {
            x if x < 0 => return self.next(),
            x => x as usize
        };

        let column = match (self.column as isize) + column_diff {
            x if x < 0 => return self.next(),
            x => x as usize
        };
        self.spots.get(&Coord{row, column})
            .map(|s| *s)
            .or_else(|| self.next())
    }   
}

pub fn first(input: &mut SeatingArea) -> Result<usize, Error> {   
    loop {
        let mut assigner = Assigner::new();

        for (coord, value) in input.spots.iter() {
            let occupied_count = input.neighbors(coord.row, coord.column)
                .filter(|v| *v == SeatingAreaOption::Occupied)
                .count();
            match value {
                SeatingAreaOption::Floor => continue,
                SeatingAreaOption::Occupied if occupied_count >= 4 => {
                    assigner.assign(coord.row, coord.column, SeatingAreaOption::Unoccupied)
                },
                SeatingAreaOption::Unoccupied if occupied_count == 0 => {
                    assigner.assign(coord.row, coord.column, SeatingAreaOption::Occupied)
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
    Ok(input.spots.iter().filter(|(_, s)| **s == SeatingAreaOption::Occupied).count())
}

pub fn second(input: &mut SeatingArea) -> Result<usize, Error> {
    Error::new("NYI")
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
        let mut input = SeatingArea::new(&example()).unwrap();
        let result = first(&mut input).unwrap();
        assert_eq!(result, 37);
    }

    // #[test]
    fn test_second() {
        let mut input = SeatingArea::new(&example()).unwrap();
        let result = second(&mut input).unwrap();
        assert_eq!(result, 26);
    }

    #[test]
    fn test_parse() {
        let mut result = SeatingArea::new(&example()).unwrap();
        assert_eq!(result.at(0, 0), Some(&SeatingAreaOption::Unoccupied));
        assert_eq!(result.at(0, 1), Some(&SeatingAreaOption::Floor));

        let mut assigner = Assigner::new();
        assigner.assign(5, 5, SeatingAreaOption::Occupied);
        assigner.commit(&mut result);
        assert_eq!(result.at(5, 5), Some(&SeatingAreaOption::Occupied));
    }

    #[test]
    fn test_neighbors() {
        let mut input = SeatingArea::new(&example()).unwrap();
        let iter = input.neighbors(1, 1);
        let result: Vec<SeatingAreaOption> = iter.collect();
        assert_eq!(result, vec!(
            SeatingAreaOption::Unoccupied, SeatingAreaOption::Floor, SeatingAreaOption::Unoccupied,
            SeatingAreaOption::Unoccupied,                           SeatingAreaOption::Unoccupied,
            SeatingAreaOption::Unoccupied, SeatingAreaOption::Floor, SeatingAreaOption::Unoccupied,
        ));
    }

    #[test]
    fn test_neighbors_limits() {
        let mut input = SeatingArea::new(&example()).unwrap();
        let iter = input.neighbors(9, 0);
        let result: Vec<SeatingAreaOption> = iter.collect();
        assert_eq!(result, vec!(
            SeatingAreaOption::Unoccupied, SeatingAreaOption::Floor, SeatingAreaOption::Floor,
        ));
    }

    #[test]
    fn test_neighbors_right_limit() {
        let mut input = SeatingArea::new(&example()).unwrap();
        let iter = input.neighbors(7, 9);
        let result: Vec<SeatingAreaOption> = iter.collect();
        assert_eq!(result, vec!(
            SeatingAreaOption::Floor, SeatingAreaOption::Floor, 
            SeatingAreaOption::Unoccupied, 
            SeatingAreaOption::Floor, SeatingAreaOption::Unoccupied,
        ));
    }
}
