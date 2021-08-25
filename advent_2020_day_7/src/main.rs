use advent_2020_common::{input, Error};
use advent_2020_day_7::{first, second, DirectedBagsGraph};

fn main() -> Result<(), Error> {
    let input = input().or(Error::new("Couldn't read input file"))?;
    println!("Using puzzle input {:?}", input);

    let input1 = DirectedBagsGraph::new(&input, false).unwrap();
    println!("First output: {}", first(&input1, &String::from("shiny gold")).unwrap());
    let input2 = DirectedBagsGraph::new(&input, true).unwrap();
    println!("Second output: {}", second(&input2, &String::from("shiny gold")).unwrap());
    Ok(())
}
