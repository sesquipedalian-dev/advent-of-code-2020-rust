use advent_2020_common::{input, Error};
use advent_2020_day_11::{first, second, SeatingArea};

fn main() -> Result<(), Error> {
    let input_str = input().or(Error::new("Couldn't read input file"))?;
    println!("Using puzzle input {:?}", input_str);
    let mut input = SeatingArea::new(&input_str).unwrap();

    println!("First output: {}", first(&mut input).unwrap());

    let mut input = SeatingArea::new(&input_str).unwrap();
    println!("Second output: {}", second(&mut input).unwrap());
    Ok(())
}
