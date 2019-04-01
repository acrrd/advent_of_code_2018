use itertools::Itertools;
use std::cmp::{max, min};

type Coord = (i32, i32);

struct BoundingBox {
    min: Coord,
    max: Coord,
}

fn parse_coord(line: &str) -> Coord {
    line.split(",")
        .map(str::trim)
        .map(|n| n.parse::<i32>().expect("Cannot parse number"))
        .tuples()
        .next()
        .expect("Cannot find a point with 2 coordinates")
}

fn parse_coords(input: &str) -> Vec<Coord> {
    input.lines().map(parse_coord).collect()
}

fn get_bounding_box<'a>(coords: impl Iterator<Item = &'a Coord>) -> BoundingBox {
    const MIN_COORD: Coord = (0, 0);
    const MAX_COORD: Coord = (std::i32::MAX, std::i32::MAX);
    let min_c = |a: &Coord, b: &Coord| (min(a.0, b.0), min(a.1, b.1));
    let max_c = |a: &Coord, b: &Coord| (max(a.0, b.0), max(a.1, b.1));

    coords.fold(
        BoundingBox {
            min: MAX_COORD,
            max: MIN_COORD,
        },
        |bb, c| BoundingBox {
            min: min_c(&bb.min, &c),
            max: max_c(&bb.max, &c),
        },
    )
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::{get_bounding_box, parse_coord, parse_coords};

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

    /*
    A**
    *B*
    **C
    */
    #[test]
    fn test_get_bounding_box_simple() {
        let cs = vec![(0, 0), (1, 1), (2, 2)];
        let bb = get_bounding_box(cs.iter());
        assert_eq!(bb.min, (0, 0));
        assert_eq!(bb.max, (2, 2));
    }

    /*
    *A*
    B*C
    *D*
    */
    #[test]
    fn test_get_bounding_box_complex() {
        let cs = vec![(1, 0), (0, 1), (2, 1), (1, 2)];
        let bb = get_bounding_box(cs.iter());
        assert_eq!(bb.min, (0, 0));
        assert_eq!(bb.max, (2, 2));
    }
}
