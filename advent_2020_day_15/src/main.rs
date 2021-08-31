use advent_2020_common::{input, Error};
use advent_2020_day_15::{first, second};

fn main() -> Result<(), Error> {
    let input = input().or(Error::new("Couldn't read input file"))?;
    println!("Using puzzle input {:?}", input);

    let input: Vec<u32> = input[0].split(",")
        .map(|i_str| i_str.parse())
        .filter(|i_or_err| i_or_err.is_ok())
        .map(|i_or_err| i_or_err.unwrap())
        .collect();
    println!("First output: {}", first(&input).unwrap());
    println!("Second output: {}", second(&input).unwrap());
    Ok(())
}
