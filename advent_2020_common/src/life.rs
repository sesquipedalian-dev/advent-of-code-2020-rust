use std::collections::{HashMap, VecDeque};
use super::Error;
use std::marker::PhantomData;

#[derive(Debug, PartialEq, Hash, Clone, Copy, Eq)]
enum TwoD {
    X = 0, 
    Y
}

impl std::convert::TryFrom<usize> for TwoD {
    type Error = Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let ok_val = match value {
            0 => TwoD::X,
            1 => TwoD::Y,
            _ => return Error::new("bad dim")
        };
        Ok(ok_val)
    }
}

impl std::convert::TryInto<usize> for TwoD {
    type Error = Error;

    fn try_into(self) -> Result<usize, Self::Error> {
        let ok_val = match self {
            TwoD::X => 0,
            TwoD::Y => 1,
            _ => return Error::new("bad dim")
        };
        Ok(ok_val)
    }
}

pub struct Assigner<DimensionType> where DimensionType : std::convert::TryInto<usize> {
    spots: HashMap<Coord<DimensionType>, LifeOption>,
}

impl<DimensionType> Assigner<DimensionType> where 
    DimensionType : std::convert::TryInto<usize>,
    Coord<DimensionType> : Eq + std::hash::Hash,
{
    pub fn new() -> Self {
        Assigner::<DimensionType>{spots: HashMap::new()}
    }

    pub fn assign(&mut self, row: usize, column: usize, value: LifeOption) {
        self.spots.insert(Coord::<DimensionType>::new_2d(row, column), value);
    }

    pub fn assign_3d(&mut self, row: usize, column: usize, z: usize, value: LifeOption) {
        self.spots.insert(Coord::<DimensionType>::new_3d(row, column, z), value);
    }
    
    pub fn commit(&mut self, other: &mut LifeSpace<DimensionType>) {
        for (coord, value) in self.spots.iter() {
            other.spots.insert(coord.clone(), *value);
        }
        self.spots.clear();
    }

    pub fn empty(&self) -> bool {
        self.spots.len() == 0
    }
}

#[derive(std::cmp::Eq, std::hash::Hash, Debug, PartialEq, Clone, Copy)]
pub enum LifeOption {
    Occupied,
    Unoccupied,
    Floor,
}

#[derive(std::cmp::Eq, Debug, PartialEq, Hash)]
pub struct Coord<DimensionType> where DimensionType : std::convert::TryInto<usize> {
    dim: Vec<usize>,
    pd: PhantomData<DimensionType>,
}

// impl<DimensionType, ValueType: Eq> Eq for Coord<DimensionType, ValueType> {
//     fn eq(&self, other: &Coord<DimensionType, ValueType>) -> bool {
//         self.dim == other.dim
//     }
// }

impl<DimensionType> Coord<DimensionType> where 
    DimensionType: std::convert::TryInto<usize>
{
    pub fn new_1d(v: usize) -> Self {
        Coord::<DimensionType>{dim: vec!(v), pd: PhantomData}
    }

    pub fn new_2d(u: usize, v: usize) -> Self {
        Coord::<DimensionType>{dim: vec!(u, v), pd: PhantomData}
    }

    pub fn new_3d(t: usize, u: usize, v: usize) -> Self {
        Coord::<DimensionType>{dim: vec!(t, u, v), pd: PhantomData}
    }

    pub fn new_4d(s: usize, t: usize, u: usize, v: usize) -> Self {
        Coord::<DimensionType>{dim: vec!(s, t, u, v), pd: PhantomData}
    }

    pub fn at(&self, dimension: DimensionType) -> Result<usize, DimensionType::Error> {
        Ok(self.dim[dimension.try_into()?])
    }

    pub fn set(&mut self, dimension: DimensionType, value: usize) -> Result<(), DimensionType::Error> {
        Ok(self.dim[dimension.try_into()?] = value)
    }
}

impl<DimensionType> Clone for Coord<DimensionType> where DimensionType: std::convert::TryInto<usize> {
    fn clone(&self) -> Self {
        let mut new_vec: Vec<usize> = Vec::new();
        for i in self.dim.iter() {
            new_vec.push(*i)
        }
        Coord::<DimensionType>{dim: new_vec, pd: PhantomData}
    }
}

pub struct LifeSpace<DimensionType> where DimensionType: std::convert::TryInto<usize> {
    pub spots: HashMap<Coord<DimensionType>, LifeOption>,
    pub tentative_spots: HashMap<Coord<DimensionType>, LifeOption>,
}

impl<DimensionType> LifeSpace<DimensionType> where 
    DimensionType: std::convert::TryInto<usize>,
    DimensionType: std::convert::TryFrom<usize>,
    DimensionType: Copy,
    DimensionType: std::fmt::Debug,
    Coord<DimensionType> : Eq + std::hash::Hash,
{
    pub fn new(input: &[String], dimensions: usize) -> Result<Self, Error> {
        let mut row = 0;
        let mut column = 0;
        let mut result = LifeSpace::<DimensionType>{spots: HashMap::new(), tentative_spots: HashMap::new()};

        for line in input.iter(){
            column = 0;
            for spot in line.chars() {
                let new_opt: LifeOption = match spot {
                    '#' => LifeOption::Occupied,
                    '.' => LifeOption::Floor,
                    'L' => LifeOption::Unoccupied,
                    x => return Error::from_string(format!("unknown char {}", x)),
                };
                let new_coord = match dimensions {
                    2 => Coord::<DimensionType>::new_2d(row, column),
                    3 => Coord::<DimensionType>::new_3d(row, column, 0),
                    4 => Coord::<DimensionType>::new_4d(row, column, 0, 0),
                    _ => return Error::new("unknown dimensionality requested"),
                };
                
                result.spots.insert(new_coord, new_opt);
                column += 1;
            }
            row += 1;
        }
        
        Ok(result)
    }

    pub fn at_2d(&self, row: usize, column: usize) -> Option<&LifeOption> {
        self.spots.get(&Coord::<DimensionType>::new_2d(row, column))
    }

    pub fn at_3d(&self, row: usize, column: usize, z: usize) -> Option<&LifeOption> {
        self.spots.get(&Coord::<DimensionType>::new_3d(row, column, z))
    }

    pub fn at_4d(&self, row: usize, column: usize, z: usize, t: usize) -> Option<&LifeOption> {
        self.spots.get(&Coord::<DimensionType>::new_4d(row, column, z, t))
    }

    pub fn neighbors(&self, row: usize, column: usize) -> NeighborIterator<DimensionType> {
        NeighborIterator::<DimensionType>{spots: &self.spots, row, column, z: 0, count: 0, skip_chars: None, direction_count: 1, use_z: false}
    }

    pub fn neighbors_3d(&self, row: usize, column: usize, z: usize) -> NeighborIterator<DimensionType> {
        NeighborIterator::<DimensionType>{spots: &self.spots, row, column, z, count: 0, skip_chars: None, direction_count: 1, use_z: true}
    }

    pub fn neighbors_skip_floor(&self, row: usize, column: usize) -> NeighborIterator<DimensionType> {
        NeighborIterator::<DimensionType>{spots: &self.spots, row, column, z: 0, count: 0, skip_chars: Some(LifeOption::Floor), direction_count: 1, use_z: false}
    }

    pub fn to_string(&self) -> Result<String, Error> {
        let mut accum = String::new();

        let mut maxes: Vec<usize> = Vec::new();
        for (coord, value) in self.spots.iter() {
            if maxes.len() == 0 { 
                maxes = coord.dim.iter().map(|_| 0).collect();
            }

            for (i, dimension_max) in maxes.iter_mut().enumerate() {
                if coord.dim[i] > *dimension_max {
                    *dimension_max = coord.dim[i];
                }
            }
        }

        let dummy: VecDeque<usize> = VecDeque::new();
        self.to_string_rec(&maxes, maxes.len() - 1, &dummy, &mut accum)?;
        Ok(accum)
    }

    pub fn to_string_rec(&self, max_dims: &Vec<usize>, dim: usize, prev_dim_coords: &VecDeque<usize>, accum: &mut String) -> Result<(), Error> {
        let current_dimension = DimensionType::try_from(dim).or(Error::new("wrong dim bro"))?;
        
        for i in 0..=max_dims[dim] {
            let mut this_coord = prev_dim_coords.clone();
            this_coord.push_front(i);

            if dim == 0 {
                let this_coord = Coord::<DimensionType>{dim: this_coord.iter().map(|s| *s).rev().collect(), pd: PhantomData};
                if let Some(v) = self.spots.get(&this_coord) {
                    println!("Iteration {:?} {:?}", this_coord, v);
                    let next_char = match v {
                        LifeOption::Occupied => '#',
                        LifeOption::Unoccupied => 'L',
                        _ =>  '.',
                    };
                    accum.push(next_char);
                }
            } else {
                self.to_string_rec(&max_dims, dim - 1, &this_coord, accum)?;
                if dim == 1 {
                    accum.push('\n');
                }
            }
        }

        Ok(())
    }
    // pub fn to_string(&self) -> String {
    //     // for dims 1 + 2 print a square
    //     // for dims 3 + 4, say z / t == <num, num> and print the square
        
    //     let mut maxes: Vec<usize> = Vec::new();
    //     for (coord, value) in self.spots.iter() {
    //         if maxes.len() == 0 { 
    //             maxes = coord.dim.iter().map(|_| 0).collect();
    //         }

    //         for (i, dimension_max) in maxes.iter().enumerate() {
    //             if coord.dim[i] > *dimension_max {
    //                 maxes[i] = coord.dim[i];
    //             }
    //         }
    //     }

    //     let max_dim = maxes.len() - 1;
    //     let max_dim_max = maxes[max_dim];
    //     let mut counters: Vec<(usize, usize)> = Vec::new(); // coordinate to be processed in dimension
    //     for i in 0.. max_dim_max {
    //         counters.push((max_dim, i));
    //     }
    //     let accum = String::new();
    //     let current_higher_order_dims: Vec<usize> = Vec::new();
    //     loop {
    //         let (current_dim, counter) = counters.pop().unwrap();

    //         match current_dim {
    //             0 => {                    
    //                 let new_coord_dim: Vec<usize> = counters.iter().map(|n| *n).collect();
    //                 let next_char = self.spots.get(&Coord::<DimensionType>{dim: new_coord_dim});
    //                 let next_char = match next_char {
    //                     Some(LifeOption::Occupied) => '#',
    //                     Some(LifeOption::Unoccupied) => 'L',
    //                     _ => '.',
    //                 };
    //                 accum.push(next_char);
    //             }
    //             1 => {
    //                 accum.push('\n');
    //             }
    //         }

    //         counters[current_dim] += 1;
    //         // if we haven't processed all the numbers in this dimension, add back this dimension's counter + 1
    //         if counters[current_dim] < maxes[current_dim] {
    //             counters.push(counter + 1)
    //         } else if counters.len() > 1 {
    //             // else we've hit the end of a dim; if this isn't the final dimension then increment in the next dim
    //             let counter = counters.pop().unwrap();
    //             counters.push(counter + 1)
    //         } else {
    //             // else last dimension is done, we can break 
    //             break
    //         }
    //     }

    //     accum
    // }
}


pub struct NeighborIterator<'a, DimensionType> where DimensionType: std::convert::TryInto<usize> {
    spots: &'a HashMap<Coord<DimensionType>, LifeOption>,
    row: usize, 
    column: usize,
    z: usize,
    count: usize,
    skip_chars: Option<LifeOption>,
    direction_count: isize,
    use_z: bool,
}

impl<DimensionType> NeighborIterator<'_, DimensionType> where DimensionType: std::convert::TryInto<usize> {
    fn next_direction(&mut self) {
        self.direction_count = 1;
        self.count += 1;
    }
}

impl<DimensionType> Iterator for NeighborIterator<'_, DimensionType> where 
    DimensionType: std::convert::TryInto<usize>,
    Coord<DimensionType> : Eq + std::hash::Hash, 
{
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
        
        let value = self.spots.get(&Coord::<DimensionType>::new_3d(row, column, z))
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
        let mut result = LifeSpace::<TwoD>::new(&example(), 2).unwrap();
        assert_eq!(result.at_2d(0, 0,), Some(&LifeOption::Unoccupied));
        assert_eq!(result.at_2d(0, 1,), Some(&LifeOption::Floor));

        let mut assigner = Assigner::new();
        assigner.assign(5, 5, LifeOption::Occupied);
        assigner.commit(&mut result);
        assert_eq!(result.at_2d(5, 5), Some(&LifeOption::Occupied));
    }

    #[test]
    fn test_to_string() {
        let mut result = LifeSpace::<TwoD>::new(&example(), 2).unwrap();
        let mut expected: String = example().join("\n");
        expected.push('\n');
        let result = result.to_string().unwrap();
        assert_eq!(result, expected);
    }

    // #[test]
    // fn test_neighbors() {
    //     let mut input = LifeSpace::<TwoD>::new(&example(), 2).unwrap();
    //     let iter = input.neighbors(1, 1);
    //     let result: Vec<LifeOption> = iter.collect();
    //     assert_eq!(result, vec!(
    //         LifeOption::Unoccupied, LifeOption::Floor, LifeOption::Unoccupied,
    //         LifeOption::Unoccupied,                    LifeOption::Unoccupied,
    //         LifeOption::Unoccupied, LifeOption::Floor, LifeOption::Unoccupied,
    //     ));
    // }

    // #[test]
    // fn test_neighbors_limits() {
    //     let mut input = LifeSpace::<TwoD>::new(&example(), 2).unwrap();
    //     let iter = input.neighbors(9, 0);
    //     let result: Vec<LifeOption> = iter.collect();
    //     assert_eq!(result, vec!(
    //         LifeOption::Unoccupied, LifeOption::Floor, LifeOption::Floor,
    //     ));
    // }

    // #[test]
    // fn test_neighbors_right_limit() {
    //     let mut input = LifeSpace::<TwoD>::new(&example(), 2).unwrap();
    //     let iter = input.neighbors(7, 9);
    //     let result: Vec<LifeOption> = iter.collect();
    //     assert_eq!(result, vec!(
    //         LifeOption::Floor, LifeOption::Floor, 
    //         LifeOption::Unoccupied, 
    //         LifeOption::Floor, LifeOption::Unoccupied,
    //     ));
    // }

    // #[test]
    // fn test_neigbors_skip_floor_lots() {
    //     let example = vec!(
    //         String::from(".......#."),
    //         String::from("...#....."),
    //         String::from(".#......."),
    //         String::from("........."),
    //         String::from("..#L....#"),
    //         String::from("....#...."),
    //         String::from("........."),
    //         String::from("#........"),
    //         String::from("...#....."),
    //     );
    //     let input = LifeSpace::<TwoD>::new(&example, 2).unwrap();
    //     let iter = input.neighbors_skip_floor(4, 3);
    //     let result: Vec<LifeOption> = iter.collect();
    //     assert_eq!(result, vec!(
    //         LifeOption::Occupied, LifeOption::Occupied, LifeOption::Occupied,
    //         LifeOption::Occupied,                       LifeOption::Occupied,
    //         LifeOption::Occupied, LifeOption::Occupied, LifeOption::Occupied,
    //     ));
    // }

    // #[test]
    // fn test_neighbors_skip_floor_one() {
    //     let example = vec!(
    //         String::from("............."),
    //         String::from(".L.L.#.#.#.#."),
    //         String::from("............."),
    //     );
    //     let input = LifeSpace::<TwoD>::new(&example, 2).unwrap();
    //     let iter = input.neighbors_skip_floor(1, 1);
    //     let result: Vec<LifeOption> = iter.collect();
    //     assert_eq!(result, vec!(
    //         LifeOption::Unoccupied,
    //     ));
    // }

    // #[test]
    // fn test_neighbors_skip_none() {
    //     let example = vec!(
    //         String::from(".##.##."),
    //         String::from("#.#.#.#"),
    //         String::from("##...##"),
    //         String::from("...L..."),
    //         String::from("##...##"),
    //         String::from("#.#.#.#"),
    //         String::from(".##.##."),
    //     );
    //     let input = LifeSpace::<TwoD>::new(&example, 2).unwrap();
    //     let iter = input.neighbors_skip_floor(3, 3);
    //     let result: Vec<LifeOption> = iter.collect();
    //     assert_eq!(result, vec!());
    // }
}