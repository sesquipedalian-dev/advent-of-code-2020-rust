use advent_2020_common::{input, Error};
// use advent_2020_week_1::{first, second};

fn main() -> Result<(), Error> {
    let input = input().or(Error::new("Couldn't read input file"))?;
    println!("Using puzzle input {:?}", input);

    // println!("First output: {}", first(&input));
    // println!("Second output: {}", second(&input));
    Ok(())
}
