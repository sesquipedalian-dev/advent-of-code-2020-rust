use advent_2020_common::{input, Error};
use advent_2020_day_11::{first, SeatingArea};

fn main() -> Result<(), Error> {
    let input = input().or(Error::new("Couldn't read input file"))?;
    println!("Using puzzle input {:?}", input);
    let mut input = SeatingArea::new(&input).unwrap();

    println!("First output: {}", first(&mut input).unwrap());
    // println!("Second output: {}", second(&mut input).unwrap());
    Ok(())
}
