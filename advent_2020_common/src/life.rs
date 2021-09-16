use std::collections::{HashMap, VecDeque};
use super::Error;
use std::marker::PhantomData;

#[derive(Debug, PartialEq, Hash, Clone, Copy, Eq)]
pub enum TwoD {
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
        };
        Ok(ok_val)
    }
}


#[derive(Debug, PartialEq, Hash, Clone, Copy, Eq)]
pub enum ThreeD {
    X = 0, 
    Y,
    Z
}

impl std::convert::TryFrom<usize> for ThreeD {
    type Error = Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let ok_val = match value {
            0 => ThreeD::X,
            1 => ThreeD::Y,
            2 => ThreeD::Z,
            _ => return Error::new("bad dim")
        };
        Ok(ok_val)
    }
}

impl std::convert::TryInto<usize> for ThreeD {
    type Error = Error;

    fn try_into(self) -> Result<usize, Self::Error> {
        let ok_val = match self {
            ThreeD::X => 0,
            ThreeD::Y => 1,
            ThreeD::Z => 2,
        };
        Ok(ok_val)
    }
}


#[derive(Debug, PartialEq, Hash, Clone, Copy, Eq)]
pub enum FourD {
    X = 0, 
    Y,
    Z,
    T,
}

impl std::convert::TryFrom<usize> for FourD {
    type Error = Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let ok_val = match value {
            0 => FourD::X,
            1 => FourD::Y,
            2 => FourD::Z,
            3 => FourD::T,
            _ => return Error::new("bad dim")
        };
        Ok(ok_val)
    }
}

impl std::convert::TryInto<usize> for FourD {
    type Error = Error;

    fn try_into(self) -> Result<usize, Self::Error> {
        let ok_val = match self {
            FourD::X => 0,
            FourD::Y => 1,
            FourD::Z => 2,
            FourD::T => 3,
        };
        Ok(ok_val)
    }
}

pub struct Assigner<DimensionType> where DimensionType : std::convert::TryInto<usize> {
    pub spots: HashMap<Coord<DimensionType>, LifeOption>,
}

impl<DimensionType> Assigner<DimensionType> where 
    DimensionType : std::convert::TryInto<usize>,
    Coord<DimensionType> : Eq + std::hash::Hash,
{
    pub fn new() -> Self {
        Assigner::<DimensionType>{spots: HashMap::new()}
    }

    pub fn assign(&mut self, row: isize, column: isize, value: LifeOption) {
        self.spots.insert(Coord::<DimensionType>::new_2d(row, column), value);
    }

    pub fn assign_3d(&mut self, row: isize, column: isize, z: isize, value: LifeOption) {
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
    pub dim: Vec<isize>,
    pub pd: PhantomData<DimensionType>,
}

// impl<DimensionType, ValueType: Eq> Eq for Coord<DimensionType, ValueType> {
//     fn eq(&self, other: &Coord<DimensionType, ValueType>) -> bool {
//         self.dim == other.dim
//     }
// }

impl<DimensionType> Coord<DimensionType> where 
    DimensionType: std::convert::TryInto<usize>
{
    pub fn new_1d(v: isize) -> Self {
        Coord::<DimensionType>{dim: vec!(v), pd: PhantomData}
    }

    pub fn new_2d(u: isize, v: isize) -> Self {
        Coord::<DimensionType>{dim: vec!(u, v), pd: PhantomData}
    }

    pub fn new_3d(t: isize, u: isize, v: isize) -> Self {
        Coord::<DimensionType>{dim: vec!(t, u, v), pd: PhantomData}
    }

    pub fn new_4d(s: isize, t: isize, u: isize, v: isize) -> Self {
        Coord::<DimensionType>{dim: vec!(s, t, u, v), pd: PhantomData}
    }

    pub fn at(&self, dimension: DimensionType) -> Result<isize, DimensionType::Error> {
        Ok(self.dim[dimension.try_into()?])
    }

    pub fn set(&mut self, dimension: DimensionType, value: isize) -> Result<(), DimensionType::Error> {
        Ok(self.dim[dimension.try_into()?] = value)
    }
}

impl<DimensionType> Clone for Coord<DimensionType> where DimensionType: std::convert::TryInto<usize> {
    fn clone(&self) -> Self {
        let mut new_vec: Vec<isize> = Vec::new();
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
        let mut result = LifeSpace::<DimensionType>{spots: HashMap::new(), tentative_spots: HashMap::new()};

        for line in input.iter(){
            let mut column = 0;
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

    pub fn at_2d(&self, row: isize, column: isize) -> Option<&LifeOption> {
        self.spots.get(&Coord::<DimensionType>::new_2d(row, column))
    }

    pub fn at_3d(&self, row: isize, column: isize, z: isize) -> Option<&LifeOption> {
        self.spots.get(&Coord::<DimensionType>::new_3d(row, column, z))
    }

    pub fn at_4d(&self, row: isize, column: isize, z: isize, t: isize) -> Option<&LifeOption> {
        self.spots.get(&Coord::<DimensionType>::new_4d(row, column, z, t))
    }

    pub fn neighbors(&self, row: isize, column: isize) -> NeighborIterator<DimensionType> {
        NeighborIterator::<DimensionType>{spots: &self.spots, row, column, count: 0, skip_chars: None, direction_count: 1}
    }

    pub fn neighbors_skip_floor(&self, row: isize, column: isize) -> NeighborIterator<DimensionType> {
        NeighborIterator::<DimensionType>{spots: &self.spots, row, column, count: 0, skip_chars: Some(LifeOption::Floor), direction_count: 1}
    }

    pub fn to_string(&self) -> Result<String, Error> {
        let mut accum = String::new();

        let mut min_maxes: Vec<(isize, isize)> = Vec::new();
        for (coord, _) in self.spots.iter() {
            if min_maxes.len() == 0 { 
                min_maxes = coord.dim.iter().map(|_| (0, 0)).collect();
            }

            for (i, (dim_min, dim_max)) in min_maxes.iter_mut().enumerate() {
                if coord.dim[i] > *dim_max {
                    *dim_max = coord.dim[i];
                }

                if coord.dim[i] < *dim_min {
                    *dim_min = coord.dim[i];
                }
            }
        }

        let dummy: VecDeque<isize> = VecDeque::new();
        // swap x and y min / max
        let min_maxes = min_maxes.iter().take(2).rev()
            .chain(min_maxes.iter().skip(2))
            .map(|s| *s)
            .collect();
        println!("min maxes {:?}", min_maxes);
        self.to_string_rec(&min_maxes, min_maxes.len() - 1, &dummy, &mut accum)?;
        Ok(accum)
    }

    pub fn to_string_rec(&self, min_maxes: &Vec<(isize, isize)>, dim: usize, prev_dim_coords: &VecDeque<isize>, accum: &mut String) -> Result<(), Error> {
        for i in min_maxes[dim].0 ..= min_maxes[dim].1 {
            let mut this_coord = prev_dim_coords.clone();
            this_coord.push_front(i);

            if dim == 0 {
                // we have to reverse the x / y coordinates to print in rows
                let new_coord = 
                    this_coord.iter().take(2).rev()
                    .chain(this_coord.iter().skip(2));
                let this_coord = Coord::<DimensionType>{dim: new_coord.map(|s| *s).collect(), pd: PhantomData};
                
                if let Some(v) = self.spots.get(&this_coord) {
                    let next_char = match v {
                        LifeOption::Occupied => '#',
                        LifeOption::Unoccupied => 'L',
                        _ =>  '.',
                    };
                    accum.push(next_char);
                } else {
                    accum.push('.');
                }
            } else {
                if dim > 1 {
                    accum.push_str(&format!(" {}={} ", dim, i));
                } else if i == 0 {
                    accum.push('\n');
                }
                if dim == 1 {
                    accum.push_str(&format!("x = {} ", i))
                }                
                self.to_string_rec(&min_maxes, dim - 1, &this_coord, accum)?;
                if dim == 1 {
                    accum.push('\n');
                }
            }
        }

        Ok(())
    }
}

pub struct NeighborIterator<'a, DimensionType> where
    DimensionType: std::convert::TryInto<usize>,
    DimensionType: std::convert::TryFrom<usize>
{
    spots: &'a HashMap<Coord<DimensionType>, LifeOption>,
    row: isize, 
    column: isize,
    count: isize,
    skip_chars: Option<LifeOption>,
    direction_count: isize
}

impl<DimensionType> NeighborIterator<'_, DimensionType> where
    DimensionType: std::convert::TryInto<usize>,
    DimensionType: std::convert::TryFrom<usize>
{
    fn next_direction(&mut self) {
        self.direction_count = 1;
        self.count += 1;
    }
}

impl Iterator for NeighborIterator<'_, TwoD> {
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
            x => x as isize
        };

        let column = match (self.column as isize) + column_diff {
            x if x < 0 => {
                self.next_direction();
                return self.next()
            },
            x => x as isize
        };

        let value = self.spots.get(&Coord::<TwoD>::new_2d(row, column))
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

    // #[test]
    // fn test_direction_iter() {
    //     let result = LifeSpace::<TwoD>::new(&example(), 2).unwrap();
    //     let mut iter = DirectionIterator::<TwoD>{
    //         spots: &result.spots,
    //         direction: vec!(1, 1),
    //         original_coord: Coord::new_2d(0, 0),
    //         count: 0,
    //         skip_chars: LifeOption::Unoccupied,
    //     };
    //     assert_eq!(iter.next(), Some(LifeOption::Floor));
    //     assert_eq!(iter.next(), Some(LifeOption::Floor));
    //     assert_eq!(iter.next(), Some(LifeOption::Floor));
    //     assert_eq!(iter.next(), None);
    // }

    #[test]
    fn test_to_string() {
        let result = LifeSpace::<TwoD>::new(&example(), 2).unwrap();
        let mut expected: String = String::from("\n");
        let joined_str: String = example().iter().enumerate().map(|(x, s)| format!("x = {} {}\n", x, s)).collect();
        expected.push_str(joined_str.as_str());
        let result = result.to_string().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_string_3d() { 
        let mut ls = LifeSpace::<ThreeD>::new(&example(), 3).unwrap();
        ls.spots.insert(Coord::<ThreeD>::new_3d(0, 0, 1), LifeOption::Occupied);
        let result = ls.to_string().unwrap();
        let expected = " 2=0 \nx = 0 L.LL.LL.LL\nx = 1 LLLLLLL.LL\nx = 2 L.L.L..L..\nx = 3 LLLL.LL.LL\nx = 4 L.LL.LL.LL\nx = 5 L.LLLLL.LL\nx = 6 ..L.L.....\nx = 7 LLLLLLLLLL\nx = 8 L.LLLLLL.L\nx = 9 L.LLLLL.LL\n 2=1 \nx = 0 #.........\nx = 1 ..........\nx = 2 ..........\nx = 3 ..........\nx = 4 ..........\nx = 5 ..........\nx = 6 ..........\nx = 7 ..........\nx = 8 ..........\nx = 9 ..........\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_neighbors() {
        let mut input = LifeSpace::<TwoD>::new(&example(), 2).unwrap();
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
        let mut input = LifeSpace::<TwoD>::new(&example(), 2).unwrap();
        let iter = input.neighbors(9, 0);
        let result: Vec<LifeOption> = iter.collect();
        assert_eq!(result, vec!(
            LifeOption::Unoccupied, LifeOption::Floor, LifeOption::Floor,
        ));
    }

    #[test]
    fn test_neighbors_right_limit() {
        let mut input = LifeSpace::<TwoD>::new(&example(), 2).unwrap();
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
        let input = LifeSpace::<TwoD>::new(&example, 2).unwrap();
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
        let input = LifeSpace::<TwoD>::new(&example, 2).unwrap();
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
        let input = LifeSpace::<TwoD>::new(&example, 2).unwrap();
        let iter = input.neighbors_skip_floor(3, 3);
        let result: Vec<LifeOption> = iter.collect();
        assert_eq!(result, vec!());
    }
}