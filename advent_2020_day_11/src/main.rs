use advent_2020_common::{input, Error, life::LifeSpace};
use advent_2020_day_11::{first, second};

fn main() -> Result<(), Error> {
    let input_str = input().or(Error::new("Couldn't read input file"))?;
    println!("Using puzzle input {:?}", input_str);
    let mut input = LifeSpace::new(&input_str).unwrap();

    println!("First output: {}", first(&mut input).unwrap());

    let mut input = LifeSpace::new(&input_str).unwrap();
    println!("Second output: {}", second(&mut input).unwrap());
    Ok(())
}
