use advent_2020_common::{input, Error};
use advent_2020_day_14::{first};

fn main() -> Result<(), Error> {
    let input = input().or(Error::new("Couldn't read input file"))?;
    println!("Using puzzle input {:?}", input);

    // let input = parse(&input).unwrap();

    println!("First output: {}", first(&input).unwrap());
    // println!("Second output: {}", second(&input).unwrap());
    Ok(())
}
