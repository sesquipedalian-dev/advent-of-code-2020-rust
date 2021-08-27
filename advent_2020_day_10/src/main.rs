use advent_2020_common::{input, input_to_nums, Error};
use advent_2020_day_10::{first, second};

fn main() -> Result<(), Error> {
    let input = input().or(Error::new("Couldn't read input file"))?;
    let mut input = input_to_nums(&input);
    println!("Using puzzle input {:?}", input);


    println!("First output: {}", first(&mut input).unwrap());
    println!("Second output: {}", second(&mut input).unwrap());
    Ok(())
}
