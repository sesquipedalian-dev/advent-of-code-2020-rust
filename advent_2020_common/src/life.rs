use std::collections::HashMap;
use super::Error;

pub struct Assigner {
    spots: HashMap<Coord, LifeOption>,
}

impl Assigner {
    pub fn new() -> Assigner {
        Assigner{spots: HashMap::new()}
    }

    pub fn assign(&mut self, row: usize, column: usize, value: LifeOption) {
        self.spots.insert(Coord{row, column}, value);
    }
    
    pub fn commit(&mut self, other: &mut LifeSpace) {
        for (coord, value) in self.spots.iter() {
            other.spots.insert(*coord, *value);
        }
        self.spots.clear();
    }

    pub fn empty(&self) -> bool {
        self.spots.len() == 0
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LifeOption {
    Occupied,
    Unoccupied,
    Floor,
}

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
pub struct Coord {
    pub row: usize,
    pub column: usize,
}

pub struct LifeSpace {
    pub spots: HashMap<Coord, LifeOption>,
    pub tentative_spots: HashMap<Coord, LifeOption>,
}

impl LifeSpace {
    pub fn new(input: &[String]) -> Result<LifeSpace, Error> {
        let mut row = 0;
        let mut column = 0;
        let mut result = LifeSpace{spots: HashMap::new(), tentative_spots: HashMap::new()};

        for line in input.iter(){
            column = 0;
            for spot in line.chars() {
                let new_opt: LifeOption = match spot {
                    '#' => LifeOption::Occupied,
                    '.' => LifeOption::Floor,
                    'L' => LifeOption::Unoccupied,
                    x => return Error::from_string(format!("unknown char {}", x)),
                };
                result.spots.insert(Coord{row, column}, new_opt);
                column += 1;
            }
            row += 1;
        }
        
        Ok(result)
    }

    pub fn at(&self, row: usize, column: usize) -> Option<&LifeOption> {
        self.spots.get(&Coord{row, column})
    }

    pub fn neighbors(&self, row: usize, column: usize) -> NeighborIterator {
        NeighborIterator{spots: &self.spots, row, column, count: 0, skip_chars: None, direction_count: 1}
    }

    pub fn neighbors_skip_floor(&self, row: usize, column: usize) -> NeighborIterator {
        NeighborIterator{spots: &self.spots, row, column, count: 0, skip_chars: Some(LifeOption::Floor), direction_count: 1}
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
                    Some(LifeOption::Occupied) => '#',
                    Some(LifeOption::Unoccupied) => 'L',
                    _ => '.',
                };
                accum.push(next_char);
            }
            accum.push('\n');
        }

        accum
    }
}


pub struct NeighborIterator<'a> {
    spots: &'a HashMap<Coord, LifeOption>,
    row: usize, 
    column: usize,
    count: usize,
    skip_chars: Option<LifeOption>,
    direction_count: isize
}

impl NeighborIterator<'_> {
    fn next_direction(&mut self) {
        self.direction_count = 1;
        self.count += 1;
    }
}

impl Iterator for NeighborIterator<'_> {
    type Item = LifeOption;

    fn next(&mut self) -> Option<LifeOption> {
        let (row_diff, column_diff) = match self.count {
            0 => (0-self.direction_count, 0-self.direction_count),
            1 => (0-self.direction_count, 0),
            2 => (0-self.direction_count, self.direction_count),
            3 => (0, 0-self.direction_count),
            4 => (0, self.direction_count),
            5 => (self.direction_count, 0-self.direction_count),
            6 => (self.direction_count, 0),
            7 => (self.direction_count, self.direction_count),
            _ => return None
        };

        let row = match (self.row as isize) + row_diff {
            x if x < 0 => {
                self.next_direction();
                return self.next()
            },
            x => x as usize
        };

        let column = match (self.column as isize) + column_diff {
            x if x < 0 => {
                self.next_direction();
                return self.next()
            },
            x => x as usize
        };
        
        let value = self.spots.get(&Coord{row, column})
            .map(|s| *s);

        match value {
            x @ Some(_) if self.skip_chars == x => {
                self.direction_count += 1;
                self.next()
            },
            x @ Some(_) => {
                self.next_direction();
                x
            },
            _ => {
                self.next_direction();
                self.next()
            }
        }
    }   
}