use std::io::{self, Read};

fn parse_input(input: &str) -> impl Iterator<Item = u32> + '_ {
    input
        .split_whitespace()
        .map(|s| s.parse::<u32>().expect("String is not a number"))
}

fn sum_metadata(input: &mut impl Iterator<Item = u32>) -> u32 {
    let children_n = input.next().expect("Number of children");
    let metadata_n = input.next().expect("Number of metadata") as usize;
    let children_sum: u32 = (0..children_n).map(|_| sum_metadata(input.by_ref())).sum();
    let metadata_sum: u32 = input.take(metadata_n).sum();
    children_sum + metadata_sum
}

fn main() -> io::Result<()> {
 let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let input = input;

    let metadata_sum = sum_metadata(&mut parse_input(&input));
    println!("{}", metadata_sum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{parse_input, sum_metadata};

    #[test]
    fn test_parse_input() {
        let input = "1 2 3 4 5";
        assert_eq!(parse_input(input).collect::<Vec<_>>(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_sum_metadata_zero() {
        let mut input = parse_input("0 0");
        assert_eq!(sum_metadata(&mut input), 0);
    }

    #[test]
    fn test_sum_metadata_metadata_only() {
        let mut input = parse_input("0 2 11 12");
        assert_eq!(sum_metadata(&mut input), 23);
    }

    #[test]
    fn test_sum_metadata_empty_child() {
        let mut input = parse_input("1 2 0 0 11 12");
        assert_eq!(sum_metadata(&mut input), 23);
    }

    #[test]
    fn test_sum_metadata_normal_child() {
        let mut input = parse_input("1 2 0 2 13 14 11 12");
        assert_eq!(sum_metadata(&mut input), 50);
    }

    #[test]
    fn test_sum_metadata_grandchildren() {
        let mut input = parse_input("1 2 2 2 0 1 15 0 1 16 13 14 11 12");
        assert_eq!(sum_metadata(&mut input), 81);
    }

    #[test]
    fn test_sum_metadata_chilrend_and_grandchildren() {
        let mut input = parse_input("2 2 2 2 0 1 15 0 1 16 13 14 2 2 0 1 17 0 1 18 19 20 11 12");
        assert_eq!(sum_metadata(&mut input), 155);
    }

    #[test]
    fn test_sum_metadata_example() {
        let mut input = parse_input("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
        assert_eq!(sum_metadata(&mut input), 138);
    }
}
