#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec!(
            String::from(".#."),
            String::from("..#"),
            String::from("###"),
        )
    }

    #[test]
    fn test_first() { 
        assert_eq!(1, 1);
    }
}