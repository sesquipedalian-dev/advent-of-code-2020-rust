use advent_2020_common::{input, input_to_nums, Error};
use advent_2020_day_9::{first, second};

fn main() -> Result<(), Error> {
    let input = input().or(Error::new("Couldn't read input file"))?;
    println!("Using puzzle input {:?}", input);

    let input = input_to_nums(&input);

    let first_result = first(&input, 25).unwrap();
    println!("First output: {}", first_result);
    println!("Second output: {}", second(&input, first_result as u32).unwrap());
    Ok(())
}
