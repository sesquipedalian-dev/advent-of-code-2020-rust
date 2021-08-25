use advent_2020_common::{input, input_to_nums, Error};
use advent_2020_day_9::{first};

fn main() -> Result<(), Error> {
    let input = input().or(Error::new("Couldn't read input file"))?;
    println!("Using puzzle input {:?}", input);

    let input = input_to_nums(&input);

    println!("First output: {}", first(&input, 25).unwrap());
    // println!("Second output: {}", second(&input).unwrap());
    Ok(())
}
