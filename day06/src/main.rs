use itertools::Itertools;

type Coord = (u32, u32);

fn parse_coord(line: &str) -> Coord {
    line.split(",")
        .map(str::trim)
        .map(|n| n.parse::<u32>().expect("Cannot parse number"))
        .tuples()
        .next()
        .expect("Cannot find a point with 2 coordinates")
}

fn parse_coords(input: &str) -> Vec<Coord> {
    input.lines().map(parse_coord).collect()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::{parse_coord, parse_coords};

    #[test]
    fn test_parse_coord() {
        assert_eq!((1, 1), parse_coord("1, 1"));
        assert_eq!((11, 1), parse_coord("11, 1"));
        assert_eq!((1, 11), parse_coord("1, 11"));
        assert_eq!((11, 11), parse_coord("11, 11"));
        assert_eq!((111, 1), parse_coord("111, 1"));
        assert_eq!((1, 111), parse_coord("1, 111"));
        assert_eq!((111, 11), parse_coord("111, 11"));
        assert_eq!((11, 111), parse_coord("11, 111"));
        assert_eq!((111, 111), parse_coord("111, 111"));
    }

    #[test]
    fn test_parse_coords() {
        assert_eq!(
            vec![(1, 1), (11, 1), (1, 11), (111, 11), (11, 111), (111, 111)],
            parse_coords(
                "1, 1\n\
                 11, 1\n\
                 1, 11\n\
                 111, 11\n\
                 11, 111\n\
                 111, 111\n"
            )
        )
    }
}
