use itertools::Itertools;
use std::borrow::Borrow;
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

fn get_bounding_box<T: Borrow<Coord>>(coords: impl Iterator<Item = T>) -> BoundingBox {
    const MIN_COORD: Coord = (0, 0);
    const MAX_COORD: Coord = (std::i32::MAX, std::i32::MAX);
    let min_c = |a: &Coord, b: &Coord| (min(a.0, b.0), min(a.1, b.1));
    let max_c = |a: &Coord, b: &Coord| (max(a.0, b.0), max(a.1, b.1));

    coords.fold(
        BoundingBox {
            min: MAX_COORD,
            max: MIN_COORD,
        },
        |bb, c| {
            let c = c.borrow();
            BoundingBox {
                min: min_c(&bb.min, c),
                max: max_c(&bb.max, c),
            }
        },
    )
}

fn get_coords(bbox: &BoundingBox) -> Vec<Coord> {
    (bbox.min.0..=bbox.max.0)
        .cartesian_product(bbox.min.1..=bbox.max.1)
        .collect()
}

fn is_on_border(bbox: &BoundingBox, coord: &Coord) -> bool {
    bbox.min.0 == coord.0 || bbox.min.1 == coord.1 || bbox.max.0 == coord.0 || bbox.max.1 == coord.1
}

fn manhattan_distance(a: &Coord, b: &Coord) -> u32 {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as u32
}

fn get_source_area<'a>(
    sources: &Vec<Coord>,
    source_idx: usize,
    get_distance: impl Fn(&Coord, &Coord) -> u32,
    is_border: impl Fn(&Coord) -> bool,
    coords: impl Iterator<Item = &'a Coord>,
) -> u32 {
    let source = sources.get(source_idx).expect("Cannot find source");
    let mut area = 0;
    for coord in coords {
        let source_dist = get_distance(source, coord);
        // if all other sources are further
        if sources
            .iter()
            .all(|s| s == source || get_distance(s, coord) > source_dist)
        {
            if is_border(coord) {
                return 0;
            } else {
                area += 1;
            }
        }
    }
    return area;
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::{
        get_bounding_box, get_coords, get_source_area, is_on_border, manhattan_distance,
        parse_coord, parse_coords, BoundingBox, Coord,
    };

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

    #[test]
    fn test_get_coords() {
        let bbox = BoundingBox {
            min: (0, 0),
            max: (2, 1),
        };

        let mut cs = vec![(0, 0), (1, 0), (0, 1), (1, 1), (2, 0), (2, 1)];
        cs.sort_unstable();

        let mut rr: Vec<Coord> = get_coords(&bbox);
        rr.sort_unstable();
        assert_eq!(cs, rr);
    }

    /*
     ***
     *A*
     ***
     */
    #[test]
    fn test_get_source_area_one_infinity() {
        let bbox = BoundingBox {
            min: (0, 0),
            max: (2, 2),
        };
        let sources = vec![(1, 1)];

        let is_border = |c: &Coord| is_on_border(&bbox, c);
        let area = get_source_area(
            &sources,
            0,
            manhattan_distance,
            is_border,
            get_coords(&bbox).iter(),
        );

        assert_eq!(area, 0);
    }

    /*
    *A*
    BCD
    *E*
    */
    #[test]
    fn test_get_source_area_one_finite() {
        let bbox = BoundingBox {
            min: (0, 0),
            max: (2, 2),
        };
        let sources = vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)];

        let is_border = |c: &Coord| is_on_border(&bbox, c);
        let get_area = |idx: usize| {
            get_source_area(
                &sources,
                idx,
                manhattan_distance,
                is_border,
                get_coords(&bbox).iter(),
            )
        };
        (0..sources.len()).filter(|idx| *idx != 2).for_each(|idx| {
            assert_eq!(get_area(idx), 0);
        });
        assert_eq!(get_area(2), 1);
    }

    /*
    ..........
    .A........
    ..........
    ........C.
    ...D......
    .....E....
    .B........
    ..........
    ..........
    ........F.
    */
    #[test]
    fn test_get_source_area_example() {
        let sources = vec![(1, 1), (1, 6), (8, 3), (3, 4), (5, 5), (8, 9)];
        let bbox = get_bounding_box(sources.iter());

        let is_border = |c: &Coord| is_on_border(&bbox, c);
        let get_area = |idx: usize| {
            get_source_area(
                &sources,
                idx,
                manhattan_distance,
                is_border,
                get_coords(&bbox).iter(),
            )
        };
        (0..3).chain(5..=5).filter(|idx| *idx != 2).for_each(|idx| {
            assert_eq!(get_area(idx), 0);
        });

        assert_eq!(get_area(3), 9);
        assert_eq!(get_area(4), 17);
    }
}
