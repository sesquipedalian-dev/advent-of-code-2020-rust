use advent_2020_common::Error;
use std::collections::HashSet;

const COL_STEP: usize = 3;

pub fn first(map: &SlopeMap) -> Result<usize, Error> {
    step_through_slope(map, 1, COL_STEP)
}

pub fn second(map: &SlopeMap) -> Result<usize, Error> {
    let steps: Vec<(usize, usize)> = vec!(
        (1, 1), 
        (1, 3), 
        (1, 5), 
        (1, 7), 
        (2, 1),
    );
    Ok(
        steps.iter().fold(
            1, 
            |accum, (row_step, col_step)| {
                let next = step_through_slope(map, *row_step, *col_step).unwrap();
                accum * next
            }
        )
    )
}

fn step_through_slope(map: &SlopeMap, row_step: usize, col_step: usize) -> Result<usize, Error> {
    let range = (map.min_row() + row_step) ..= map.max_row();
    let count = range.step_by(row_step).fold(
        (col_step, 0), 
        |(col, count), row| {
            let new_count = if map.is_tree(row, col) {
                count + 1
            } else { 
                count
            };

            (col + col_step, new_count)
        }
    ).1;

    Ok(count)
}

#[derive(PartialEq, Eq, Hash)]
pub struct Coord(usize, usize);

pub struct SlopeMap {
    trees: HashSet<Coord>,
    max_row: usize, 
    max_col: usize,
}

impl SlopeMap {
    pub fn new(input: &[String]) -> Result<SlopeMap, Error> {
        let mut map = SlopeMap{
            trees: HashSet::<Coord>::new(),
            max_row: input.len() - 1,
            max_col: 0,
        };

        for (row_index, row_str) in input.iter().enumerate() {
            for (col_index, col_char) in row_str.trim().chars().enumerate() {
                // println!("parse {} {} {}", row_index, col_index, col_char);
                if col_index > map.max_col { 
                    map.max_col = col_index;
                }

                if col_char == '#' {
                    map.trees.insert(Coord(row_index, col_index));
                }
            }
        }

        Ok(map)
    }

    pub fn is_tree(&self, row: usize, col: usize) -> bool {
        self.trees.contains(&Coord(row, col % (self.max_col() + 1)))
    }

    pub fn min_row(&self) -> usize { 0 }
    pub fn min_col(&self) -> usize { 0 }

    pub fn max_row(&self) -> usize {
        self.max_row
    }

    pub fn max_col(&self) -> usize {
        self.max_col
    }
}

#[cfg(test)]
mod tests { 
    use super::*;

    fn example() -> Vec<String> {
        vec!(
            String::from("..##......."),
            String::from("#...#...#.."),
            String::from(".#....#..#."),
            String::from("..#.#...#.#"),
            String::from(".#...##..#."),
            String::from("..#.##....."),
            String::from(".#.#.#....#"),
            String::from(".#........#"),
            String::from("#.##...#..."),
            String::from("#...##....#"),
            String::from(".#..#...#.#"),
        )
    }

    #[test]
    fn test_first() { 
        let input = example();
        let map = SlopeMap::new(&input).unwrap();

        let result = first(&map).unwrap();
        let expected = 7;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_second() { 
        let input = example();
        let map = SlopeMap::new(&input).unwrap();

        let result = second(&map).unwrap();
        let expected = 336;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_row_step() { 
        let input = example();
        let map = SlopeMap::new(&input).unwrap();

        let result = step_through_slope(&map, 2, 1).unwrap();
        let expected = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse() {
        let input = example();
        let map = SlopeMap::new(&input).unwrap();
        assert_eq!(map.max_row(), 10);
        assert_eq!(map.max_col(), 10);
        assert!(map.is_tree(0, 2));
        assert!(!map.is_tree(0, 0));
        assert!(map.is_tree(0, map.max_col() + 3)); // loops around
        assert!(map.is_tree(4, 12)); // loop around
        assert!(map.is_tree(2, 6));
    }
}