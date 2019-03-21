use std::io::{self, Read};
use std::iter::Iterator;

fn get_nums(input: &String) -> Result<Vec<i32>, std::num::ParseIntError> {
    input.lines().map(|line| line.parse::<i32>()).collect()
}

fn compute_freq(input: &String) -> Result<i32, std::num::ParseIntError> {
    get_nums(&input).and_then(|nums: Vec<i32>| Ok(nums.iter().sum()))
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let input = input;

    match compute_freq(&input) {
        Ok(freq) => println!("{}", freq),
        Err(err) => println!("Error: {}", err),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::compute_freq;

    #[test]
    fn test_compute_freq() {
        let tests: Vec<(String, i32)> = vec![
            ("+1\n-2\n+3\n+1".to_string(), 3),
            ("+1\n+1\n+1".to_string(), 3),
            ("+1\n+1\n-2".to_string(), 0),
            ("-1\n-2\n-3".to_string(), -6),
        ];

        tests
            .iter()
            .for_each(|(input, result)| assert_eq!(compute_freq(input).unwrap(), *result));
    }

}
