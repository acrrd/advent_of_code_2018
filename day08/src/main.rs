use std::io::{self, Read};

fn parse_input(input: &str) -> impl Iterator<Item = u32> + Clone + '_ {
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

fn node_value(input: &mut impl Iterator<Item = u32>) -> u32 {
    let children_n = input.next().expect("Number of children");
    let metadata_n = input.next().expect("Number of metadata") as usize;
    let childrens_values: Vec<u32> = (0..children_n)
        .map(|_| node_value(input.by_ref()))
        .collect();

    let metadata = input.take(metadata_n);
    if children_n == 0 {
        return metadata.sum();
    }
    metadata
        .filter_map(|idx| {
            let idx = idx as usize;
            if idx > 0 && idx <= children_n as usize {
                if let Some(child_value) = childrens_values.get(idx - 1) {
                    return Some(child_value);
                }
            }
            None
        })
        .sum()
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let input = input;

    let input = parse_input(&input);
    let metadata_sum = sum_metadata(&mut input.clone());
    let root_value = node_value(&mut input.clone());
    println!("{}", metadata_sum);
    println!("{}", root_value);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{node_value, parse_input, sum_metadata};

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

    #[test]
    fn test_node_value_zero() {
        let mut input = parse_input("0 0");
        assert_eq!(node_value(&mut input), 0);
    }

    #[test]
    fn test_node_value_metadata_only() {
        let mut input = parse_input("0 2 11 12");
        assert_eq!(node_value(&mut input), 23);
    }

    #[test]
    fn test_node_value_child() {
        let mut input = parse_input("1 4 0 1 10 0 1 1 2");
        assert_eq!(node_value(&mut input), 20);
    }

    #[test]
    fn test_node_value_example() {
        let mut input = parse_input("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
        assert_eq!(node_value(&mut input), 66);
    }
}
