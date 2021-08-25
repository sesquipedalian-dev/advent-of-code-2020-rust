use advent_2020_common::{input, Error};
use advent_2020_day_7::{first, DirectedBagsGraph};

fn main() -> Result<(), Error> {
    let input = input().or(Error::new("Couldn't read input file"))?;
    println!("Using puzzle input {:?}", input);

    let input = DirectedBagsGraph::new(&input).unwrap();
    println!("First output: {}", first(&input, &String::from("shiny gold")).unwrap());
    // println!("Second output: {}", second(&input).unwrap());
    Ok(())
}
