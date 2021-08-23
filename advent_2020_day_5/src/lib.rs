use advent_2020_common::Error;

struct SeatId(usize);

pub fn first(input: &[String]) -> Result<usize, Error> {
    let mut max = 0;
    for input in input {
        let seat_id = SeatId::new(input)?;
        if seat_id.0 > max { 
            max = seat_id.0;
        }
    }
    Ok(max)
}

pub fn second(input: &[String]) -> Result<usize, Error> {
    let mut sorted: Vec<usize> = input.iter()
        .map(|s| SeatId::new(s))
        .flatten()
        .map(|s| s.0)
        .collect();
    sorted.sort();

    let mut last_i: Option<usize> = None;
    for seat_id in sorted {
        if last_i.map(|last_i| seat_id - last_i).filter(|diff| *diff == 2).is_some() {
            return Ok(seat_id - 1);
        }

        last_i = Some(seat_id);
    }

    Error::new("Not found")
}

impl SeatId {
    fn new(input: &String) -> Result<SeatId, Error> {
        if input.len() != 10 {
            return Error::new("wrong sized input");
        }

        let mut cs = input.chars();
        let row = binary_from_chars(&mut cs, 7, 'F', 'B')?;
        let col = binary_from_chars(&mut cs, 3, 'L', 'R')?;
        let seat_id = (row << 3) | col;
        Ok(SeatId(seat_id))
    }
}

fn binary_from_chars(input: &mut dyn Iterator<Item = char>, count: usize, front_char: char, back_char: char) -> Result<usize, Error> {
    let row_part = input.take(count);
    let mut accum = 0;
    for (i, c) in row_part.enumerate() {
        match c {
            c if c == back_char => accum = accum + (1 << (count-i - 1)),
            c if c == front_char => (),
            _ => return Error::from_string(format!("Unknown F/B character: {}", c))
        }
    }
    Ok(accum)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn examples() -> Vec<(String, usize)> {
        vec!(
            (String::from("FBFBBFFRLR"), 357),
            (String::from("BFFFBBFRRR"), 567),
            (String::from("FFFBBBFRRR"), 119),
            (String::from("BBFFBBFRLL"), 820),
        )
    }

    #[test]
    fn test_first() { 
        let input: Vec<String> = examples().iter().map(|(e, _)| e.clone()).collect();
        let result = first(&input).unwrap();
        let expected = 820;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_seat_id() {
        for (input, expected) in examples().iter() {
            let seat_id = SeatId::new(&input).unwrap();
            assert_eq!(seat_id.0, *expected);
        }
    }

    #[test]
    fn test_binary_from_chars() {
        let mut input = "FBFBBFF".chars();
        let result = binary_from_chars(&mut input, 7, 'F', 'B').unwrap();
        let expected = 44;
        assert_eq!(result, expected);
    }
}