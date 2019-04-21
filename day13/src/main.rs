use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Axe {
    X,
    Y,
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
}

#[derive(Debug, PartialEq)]
enum TrackPiece {
    Straight(Axe),
    TurnLeft,
    TurnRight,
    Intersection,
}

type Coord = (usize, usize);
type Track = HashMap<Coord, TrackPiece>;

#[derive(Debug)]
struct Cart {
    coord: Coord,
    axe: Axe,
    direction: Direction,
}

impl Cart {
    fn new(coord: Coord, axe: Axe, direction: Direction) -> Cart {
        Cart {
            coord,
            axe,
            direction,
        }
    }
}

fn parse_track(input: &str) -> (Track, Vec<Cart>) {
    input
        .lines()
        .enumerate()
        .flat_map(|(x, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != ' ')
                .map(move |(y, c)| ((x, y), c))
        })
        .fold(
            (Track::new(), Vec::new()),
            |(mut track, mut carts), (coord, mut tile)| {
                match tile {
                    '>' => {
                        carts.push(Cart::new(coord, Axe::X, Direction::Up));
                        tile = '-';
                    }
                    '<' => {
                        carts.push(Cart::new(coord, Axe::X, Direction::Down));
                        tile = '-';
                    }
                    'v' => {
                        carts.push(Cart::new(coord, Axe::Y, Direction::Up));
                        tile = '|';
                    }
                    '^' => {
                        carts.push(Cart::new(coord, Axe::Y, Direction::Down));
                        tile = '|';
                    }
                    _ => (),
                };

                match tile {
                    '-' => {
                        track.insert(coord, TrackPiece::Straight(Axe::X));
                    }
                    '|' => {
                        track.insert(coord, TrackPiece::Straight(Axe::Y));
                    }
                    '/' => {
                        track.insert(coord, TrackPiece::TurnLeft);
                    }
                    '\\' => {
                        track.insert(coord, TrackPiece::TurnRight);
                    }
                    '+' => {
                        track.insert(coord, TrackPiece::Intersection);
                    }
                    _ => panic!("Unknow tile {}", tile),
                };

                (track, carts)
            },
        )
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::{parse_track, Axe, Direction, TrackPiece};

    #[test]
    fn test_parse_track_pieces() {
        let input = " - | \n / \\\n+";

        let (track, _) = parse_track(input);

        assert!(track.contains_key(&(0, 1)));
        assert_eq!(*track.get(&(0, 1)).unwrap(), TrackPiece::Straight(Axe::X));
        assert!(track.contains_key(&(0, 3)));
        assert_eq!(*track.get(&(0, 3)).unwrap(), TrackPiece::Straight(Axe::Y));
        assert!(track.contains_key(&(1, 1)));
        assert_eq!(*track.get(&(1, 1)).unwrap(), TrackPiece::TurnLeft);
        assert!(track.contains_key(&(1, 3)));
        assert_eq!(*track.get(&(1, 3)).unwrap(), TrackPiece::TurnRight);
        assert!(track.contains_key(&(2, 0)));
        assert_eq!(*track.get(&(2, 0)).unwrap(), TrackPiece::Intersection);
    }

    #[test]
    fn test_parse_track_cart() {
        let input = "><v^";

        let (track, carts) = parse_track(input);

        assert!(track.contains_key(&(0, 0)));
        assert_eq!(*track.get(&(0, 0)).unwrap(), TrackPiece::Straight(Axe::X));
        assert!(track.contains_key(&(0, 1)));
        assert_eq!(*track.get(&(0, 1)).unwrap(), TrackPiece::Straight(Axe::X));
        assert!(track.contains_key(&(0, 2)));
        assert_eq!(*track.get(&(0, 2)).unwrap(), TrackPiece::Straight(Axe::Y));
        assert!(track.contains_key(&(0, 2)));
        assert_eq!(*track.get(&(0, 2)).unwrap(), TrackPiece::Straight(Axe::Y));

        let cart = carts.get(0).expect("> cart");
        assert_eq!(cart.coord, (0, 0));
        assert_eq!(cart.axe, Axe::X);
        assert_eq!(cart.direction, Direction::Up);

        let cart = carts.get(1).expect("< cart");
        assert_eq!(cart.coord, (0, 1));
        assert_eq!(cart.axe, Axe::X);
        assert_eq!(cart.direction, Direction::Down);

        let cart = carts.get(2).expect("v cart");
        assert_eq!(cart.coord, (0, 2));
        assert_eq!(cart.axe, Axe::Y);
        assert_eq!(cart.direction, Direction::Up);

        let cart = carts.get(3).expect("v cart");
        assert_eq!(cart.coord, (0, 3));
        assert_eq!(cart.axe, Axe::Y);
        assert_eq!(cart.direction, Direction::Down);
    }
}
