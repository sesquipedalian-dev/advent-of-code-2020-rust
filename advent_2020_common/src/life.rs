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
        self.spots.insert(Coord::new(row, column, 0), value);
    }

    pub fn assign_3d(&mut self, row: usize, column: usize, z: usize, value: LifeOption) {
        self.spots.insert(Coord::new(row, column, z), value);
    }
    
    pub fn commit(&mut self, other: &mut LifeSpace) {
        for (coord, value) in self.spots.iter() {
            other.spots.insert(coord.clone(), *value);
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

#[derive(Debug, PartialEq, Hash, Eq)]
pub struct Coord {
    dim: Vec<usize>,
}

impl Coord {
    pub fn new(row: usize, column: usize, z: usize) -> Coord {
        Coord{dim: vec!(row, column, z)}
    }

    pub fn row(&self) -> usize {
        self.dim[0]
    }

    pub fn column(&self) -> usize {
        self.dim[1]
    }

    pub fn z(&self) -> usize {
        self.dim[2]
    }
}

impl Clone for Coord {
    fn clone(&self) -> Coord {
        Coord::new(self.row(), self.column(), self.z())
    }
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
                result.spots.insert(Coord::new(row, column, 0), new_opt);
                column += 1;
            }
            row += 1;
        }
        
        Ok(result)
    }

    pub fn at(&self, row: usize, column: usize, z: usize) -> Option<&LifeOption> {
        self.spots.get(&Coord::new(row, column, z))
    }

    pub fn neighbors(&self, row: usize, column: usize) -> NeighborIterator {
        NeighborIterator{spots: &self.spots, row, column, z: 0, count: 0, skip_chars: None, direction_count: 1, use_z: false}
    }

    pub fn neighbors_3d(&self, row: usize, column: usize, z: usize) -> NeighborIterator {
        NeighborIterator{spots: &self.spots, row, column, z, count: 0, skip_chars: None, direction_count: 1, use_z: true}
    }

    pub fn neighbors_skip_floor(&self, row: usize, column: usize) -> NeighborIterator {
        NeighborIterator{spots: &self.spots, row, column, z: 0, count: 0, skip_chars: Some(LifeOption::Floor), direction_count: 1, use_z: false}
    }

    pub fn to_string(&self) -> String {
        let mut max_row = 0;
        let mut max_col = 0;
        let mut max_z = 0;
        for (coord, value) in self.spots.iter() {
            if coord.row() > max_row {
                max_row = coord.row();
            }
            if coord.column() > max_col { 
                max_col = coord.column();
            }
            if coord.z() > max_z {
                max_z = coord.z();
            }
        }

        let mut accum = String::new();
        for row in 0..=max_row {
            for column in 0..=max_col {
                for z in 0..=max_z {
                    let next_char = match self.at(row, column, z) {
                        Some(LifeOption::Occupied) => '#',
                        Some(LifeOption::Unoccupied) => 'L',
                        _ => '.',
                    };
                    accum.push(next_char);
                }
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
    z: usize,
    count: usize,
    skip_chars: Option<LifeOption>,
    direction_count: isize,
    use_z: bool,
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
        let (row_diff, column_diff, z_diff) = match self.count {
            // z = 0
            0 => (0-self.direction_count, 0-self.direction_count, 0),
            1 => (0-self.direction_count, 0, 0),
            2 => (0-self.direction_count, self.direction_count, 0),
            3 => (0, 0-self.direction_count, 0),
            4 => (0, self.direction_count, 0),
            5 => (self.direction_count, 0-self.direction_count, 0),
            6 => (self.direction_count, 0, 0),
            7 => (self.direction_count, self.direction_count, 0),
            // z = -1
            8 if self.use_z => (0-self.direction_count, 0-self.direction_count, -1),
            9 if self.use_z => (0-self.direction_count, 0, -1),
            10 if self.use_z => (0-self.direction_count, self.direction_count, -1),
            11 if self.use_z => (0, 0-self.direction_count, -1),
            12 if self.use_z => (0, self.direction_count, -1),
            13 if self.use_z => (self.direction_count, 0-self.direction_count, -1),
            14 if self.use_z => (self.direction_count, 0, -1),
            15 if self.use_z => (self.direction_count, self.direction_count, -1),
            16 if self.use_z => (0, 0, -1),
            // z = +1
            17 if self.use_z => (0-self.direction_count, 0-self.direction_count, 1),
            18 if self.use_z => (0-self.direction_count, 0, 1),
            19 if self.use_z => (0-self.direction_count, self.direction_count, 1),
            20 if self.use_z => (0, 0-self.direction_count, 1),
            21 if self.use_z => (0, self.direction_count, 1),
            22 if self.use_z => (self.direction_count, 0-self.direction_count, 1),
            23 if self.use_z => (self.direction_count, 0, 1),
            24 if self.use_z => (self.direction_count, self.direction_count, 1),
            25 if self.use_z => (0, 0, 1),
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

        let z = match (self.z as isize) + z_diff {
            z if z < 0 => { 
                self.next_direction();
                return self.next()
            }
            z => z as usize
        };
        
        let value = self.spots.get(&Coord::new(row, column, z))
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

    // next up: let's add some 3d tests? I guess for neighbor iter

    #[test]
    fn test_parse() {
        let mut result = LifeSpace::new(&example()).unwrap();
        assert_eq!(result.at(0, 0, 0), Some(&LifeOption::Unoccupied));
        assert_eq!(result.at(0, 1, 0), Some(&LifeOption::Floor));

        let mut assigner = Assigner::new();
        assigner.assign(5, 5, LifeOption::Occupied);
        assigner.commit(&mut result);
        assert_eq!(result.at(5, 5, 0), Some(&LifeOption::Occupied));
    }

    #[test]
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

    #[test]
    fn test_neighbors_limits() {
        let mut input = LifeSpace::new(&example()).unwrap();
        let iter = input.neighbors(9, 0);
        let result: Vec<LifeOption> = iter.collect();
        assert_eq!(result, vec!(
            LifeOption::Unoccupied, LifeOption::Floor, LifeOption::Floor,
        ));
    }

    #[test]
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

    #[test]
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

    #[test]
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

    #[test]
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