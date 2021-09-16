use advent_2020_common::{life::*, Error};
use std::collections::{HashSet, HashMap};
use std::marker::PhantomData;

struct NeighborCoordIterator<'a, DimensionType> where
    DimensionType: std::convert::TryInto<usize>,
    DimensionType: std::convert::TryFrom<usize>
{
    spots: &'a HashMap<Coord<DimensionType>, LifeOption>,
    original_coord: Coord<DimensionType>,
    count: usize,
}

impl<DimensionType> NeighborCoordIterator<'_, DimensionType> where
    DimensionType: std::convert::TryInto<usize>,
    DimensionType: std::convert::TryFrom<usize>,
{
    fn new_3d<'a>(spots: &'a HashMap<Coord<DimensionType>, LifeOption>, coord: Coord<DimensionType>) -> NeighborCoordIterator<'a, DimensionType> {
        NeighborCoordIterator::<DimensionType>{
            spots,
            original_coord: coord,
            count: 0
        }
    }
}

impl<DimensionType> Iterator for NeighborCoordIterator<'_, DimensionType> where
    DimensionType: std::convert::TryInto<usize>,
    DimensionType: std::convert::TryFrom<usize>
{
    type Item = Coord<DimensionType>;

    fn next(&mut self) -> Option<Coord<DimensionType>> {
        if self.count >= 4usize.pow(self.original_coord.dim.len() as u32) {
            return None
        }

        let mut next_dims: Vec<isize> = Vec::new();
        for (dim, coord) in self.original_coord.dim.iter().enumerate() { 
            let mask = 0b11;
            let diff = match (self.count >> (dim * 2)) & 0b11 {
                0 => -1,
                1 => 0,
                2 => 1,
                _ => {
                    self.count += 1;
                    return self.next()
                }
            };
            next_dims.push(coord + diff);
        }
        // skip current coord
        let all_diffs_0 = next_dims.iter().enumerate().all(|(i, n)| self.original_coord.dim[i] == *n);
        if  all_diffs_0 {
            self.count += 1;
            return self.next();
        }
        let new_coord = Coord::<DimensionType>{dim: next_dims, pd: PhantomData};
        

        self.count += 1;
        Some(new_coord)
    }
}

fn update_coord<DimensionType>(
    assigner: &mut Assigner<DimensionType>, 
    spots: &HashMap<Coord<DimensionType>, LifeOption>, 
    coord: &Coord<DimensionType>,
    value: LifeOption,
    should_recur: bool,
) where
    DimensionType: std::convert::TryInto<usize>,
    DimensionType: std::convert::TryFrom<usize>,
    DimensionType: std::fmt::Debug,
    Coord<DimensionType> : Eq + std::hash::Hash,
{
    if assigner.spots.get(&coord).is_some() { 
        // already handled
        return
    }

    // let mut possible_new_coords = HashSet::<Coord<DimensionType>>::new();
    let neighbors = NeighborCoordIterator::<DimensionType>::new_3d(spots, coord.clone());
    let mut occupied_count = 0;
    for neighbor_coord in neighbors {
        match spots.get(&neighbor_coord) {
            Some(v) if *v == LifeOption::Occupied => {
                occupied_count += 1;
            },
            None if should_recur => {
                update_coord(assigner, spots, &neighbor_coord, LifeOption::Floor, false);
            },
            _ => () // no op
        };
    }

    match value {
        LifeOption::Occupied if !((2 == occupied_count) || (3 == occupied_count)) => {
            assigner.spots.insert(coord.clone(), LifeOption::Floor);
        },
        LifeOption::Floor if occupied_count == 3 => {
            assigner.spots.insert(coord.clone(), LifeOption::Occupied);
        },
        _ => (),
    };
}

pub fn first(input: &[String]) -> Result<usize, Error> {   
    let mut input = LifeSpace::new(input, 3)?;

    for _ in 0..6 {
        let mut assigner = Assigner::new();
       
        for (coord, value) in input.spots.iter() {
            update_coord(&mut assigner, &input.spots, coord, *value, true);
        }
        
        if assigner.empty() {
            break;
        }

        assigner.commit(&mut input);
    }

    // count the occupied spots
    let count: usize = input.spots.iter().filter(|(_, s): &(&advent_2020_common::life::Coord<ThreeD>, &advent_2020_common::life::LifeOption)| **s == LifeOption::Occupied).count();
    Ok(count)
}

pub fn second(input: &[String]) -> Result<usize, Error> {
    let mut input = LifeSpace::new(input, 4)?;

    for _ in 0..6 {
        let mut assigner = Assigner::new();
       
        for (coord, value) in input.spots.iter() {
            update_coord(&mut assigner, &input.spots, coord, *value, true);
        }
        
        if assigner.empty() {
            break;
        }

        assigner.commit(&mut input);
    }

    // count the occupied spots
    let count: usize = input.spots.iter().filter(|(_, s): &(&advent_2020_common::life::Coord<FourD>, &advent_2020_common::life::LifeOption)| **s == LifeOption::Occupied).count();
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec!(
            String::from(".#."),
            String::from("..#"),
            String::from("###"),
        )
    }

    #[test]
    fn test_coord_iterator() {   
        let result = LifeSpace::<TwoD>::new(&example(), 2).unwrap();
        let iter = NeighborCoordIterator::<TwoD>{
            spots: &result.spots,
            original_coord: Coord::new_2d(0, 0),
            count: 0,
        };
        let expected: Vec<Coord<TwoD>> = vec!(
            Coord::new_2d(-1, -1),
            Coord::new_2d(0, -1),
            Coord::new_2d(1, -1),
            Coord::new_2d(-1, 0),
            // Coord::new_2d(0, 0),
            Coord::new_2d(1, 0),
            Coord::new_2d(-1, 1),
            Coord::new_2d(0, 1),
            Coord::new_2d(1, 1),
        );
        let actual: Vec<Coord<TwoD>> = iter.collect();
        assert_eq!(actual, expected);
    }

    
    #[test]
    fn test_coord_iterator_3d() {   
        let result = LifeSpace::<ThreeD>::new(&example(), 3).unwrap();
        let original_coord = Coord::new_3d(0, 1, 0);
        let iter = NeighborCoordIterator::<ThreeD>{
            spots: &result.spots,
            original_coord: original_coord.clone(),
            count: 0,
        };
        let actual: Vec<Coord<ThreeD>> = iter.collect();
        assert_eq!(actual.len(), 26);
        assert!(!actual.contains(&original_coord))
    }

    #[test]
    fn test_first() {
        let result = first(&example()).unwrap();
        assert_eq!(result, 112);
    }

    #[test]
    fn test_second() { 
        let result = second(&example()).unwrap();
        assert_eq!(result, 848);
    }
}