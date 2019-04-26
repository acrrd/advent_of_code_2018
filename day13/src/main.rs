use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Debug, PartialEq, Clone)]
enum Axe {
    X,
    Y,
}

// Direction Up means that the coordinate will increase
// while Down means that the coordinate will descrease
// > and v are Up
// < and ^ are Down
#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Up,
    Down,
}

// Turn(Up) means that the direction is the same after the turn
// >\ after the turn the direction is still Up
//  v
// Turn(Down) means that the direction will flip
// /< after the turn the direction change from Down to Up Up
// v
//
#[derive(Debug, PartialEq)]
enum TrackPiece {
    Straight(Axe),
    Turn(Direction),
    Intersection,
}

type Coord = (usize, usize);
type Track = HashMap<Coord, TrackPiece>;

#[derive(Debug, Clone)]
struct Cart {
    coord: Coord,
    axe: Axe,
    direction: Direction,
    next_intersection_move: usize,
}

enum IntersectionMove {
    Straight,
    TurnLeft,
    TurnRight,
}

const INTERSECTION_MOVE_ORDER: [IntersectionMove; 3] = [
    IntersectionMove::TurnLeft,
    IntersectionMove::Straight,
    IntersectionMove::TurnRight,
];

impl Cart {
    fn new(coord: Coord, axe: Axe, direction: Direction) -> Cart {
        Cart {
            coord,
            axe,
            direction,
            next_intersection_move: 0,
        }
    }
}

fn move_cart(track: &Track, cart: &mut Cart) {
    let change = |n: usize| match cart.direction {
        Direction::Up => n + 1,
        Direction::Down => n - 1,
    };

    let invert_axe = |axe: &Axe| match *axe {
        Axe::X => Axe::Y,
        Axe::Y => Axe::X,
    };

    let (old_x, old_y) = cart.coord;
    cart.coord = match cart.axe {
        Axe::X => (change(old_x), old_y),
        Axe::Y => (old_x, change(old_y)),
    };

    let mut tile = track.get(&cart.coord).expect("Malformed track");
    let intersection_tile: TrackPiece;

    if let TrackPiece::Intersection = tile {
        intersection_tile = match INTERSECTION_MOVE_ORDER[cart.next_intersection_move] {
            // we dont care about the axe of the straight
            IntersectionMove::Straight => TrackPiece::Straight(Axe::X),
            IntersectionMove::TurnLeft => TrackPiece::Turn(match cart.axe {
                Axe::X => Direction::Down,
                Axe::Y => Direction::Up,
            }),
            IntersectionMove::TurnRight => TrackPiece::Turn(match cart.axe {
                Axe::X => Direction::Up,
                Axe::Y => Direction::Down,
            }),
        };
        tile = &intersection_tile;

        cart.next_intersection_move += 1;
        cart.next_intersection_move %= 3;
    };

    let tile = tile;

    if let TrackPiece::Turn(direction) = tile {
        cart.axe = invert_axe(&cart.axe);

        if let Direction::Down = direction {
            cart.direction = match cart.direction {
                Direction::Up => Direction::Down,
                Direction::Down => Direction::Up,
            }
        }
    }
}

fn parse_track(input: &str) -> (Track, Vec<Cart>) {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != ' ')
                .map(move |(x, c)| ((x, y), c))
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
                        track.insert(coord, TrackPiece::Turn(Direction::Down));
                    }
                    '\\' => {
                        track.insert(coord, TrackPiece::Turn(Direction::Up));
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

fn simulate_till_crash(track: &Track, mut carts: Vec<Cart>) -> Coord {
    let mut coords_to_idx =
        carts
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut coords_to_idx, (idx, cart)| {
                coords_to_idx.insert(cart.coord, idx);
                coords_to_idx
            });

    loop {
        carts.sort_unstable_by_key(|cart| cart.coord);

        for (idx, mut cart) in carts.iter_mut().enumerate() {
            coords_to_idx.remove(&cart.coord);

            move_cart(&track, &mut cart);
            // detect crash
            if coords_to_idx.contains_key(&cart.coord) {
                return cart.coord;
            }

            coords_to_idx.insert(cart.coord, idx);
        }
    }
}

fn simulate_till_one_stand(track: &Track, mut carts: Vec<Cart>) -> Coord {
    loop {
        if carts.len() == 1 {
            return carts[0].coord;
        }

        let mut coords_to_idx =
            carts
                .iter()
                .enumerate()
                .fold(HashMap::new(), |mut coords_to_idx, (idx, cart)| {
                    coords_to_idx.insert(cart.coord, idx);
                    coords_to_idx
                });

        carts.sort_unstable_by_key(|cart| cart.coord);
        let mut to_remove: Vec<usize> = Vec::with_capacity(carts.len());

        for (idx, mut cart) in carts.iter_mut().enumerate() {
            coords_to_idx.remove(&cart.coord);

            move_cart(&track, &mut cart);
            // detect crash
            let crash_cart_idx = coords_to_idx.get(&cart.coord);
            if let Some(&crash_idx) = crash_cart_idx {
                to_remove.push(idx);
                to_remove.push(crash_idx);
                coords_to_idx.remove(&cart.coord);
            } else {
                coords_to_idx.insert(cart.coord, idx);
            }
        }

        to_remove.sort_unstable();
        let mut offset = 0;
        to_remove.iter().for_each(|&idx| {
            carts.remove(idx - offset);
            offset += 1;
        });
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let input = input;

    let (track, carts) = parse_track(&input);
    let crash_coord = simulate_till_crash(&track, carts.clone());

    println!("{:?}", crash_coord);

    let last_cart_coord = simulate_till_one_stand(&track, carts);

    println!("{:?}", last_cart_coord);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        move_cart, parse_track, simulate_till_crash, simulate_till_one_stand, Axe, Direction,
        TrackPiece,
    };

    #[test]
    fn test_parse_track_pieces() {
        let input = " - | \n / \\\n+";

        let (track, _) = parse_track(input);

        assert!(track.contains_key(&(1, 0)));
        assert_eq!(*track.get(&(1, 0)).unwrap(), TrackPiece::Straight(Axe::X));
        assert!(track.contains_key(&(3, 0)));
        assert_eq!(*track.get(&(3, 0)).unwrap(), TrackPiece::Straight(Axe::Y));
        assert!(track.contains_key(&(1, 1)));
        assert_eq!(
            *track.get(&(1, 1)).unwrap(),
            TrackPiece::Turn(Direction::Down)
        );
        assert!(track.contains_key(&(3, 1)));
        assert_eq!(
            *track.get(&(3, 1)).unwrap(),
            TrackPiece::Turn(Direction::Up)
        );
        assert!(track.contains_key(&(0, 2)));
        assert_eq!(*track.get(&(0, 2)).unwrap(), TrackPiece::Intersection);
    }

    #[test]
    fn test_parse_track_cart() {
        let input = "><v^";

        let (track, carts) = parse_track(input);

        assert!(track.contains_key(&(0, 0)));
        assert_eq!(*track.get(&(0, 0)).unwrap(), TrackPiece::Straight(Axe::X));
        assert!(track.contains_key(&(1, 0)));
        assert_eq!(*track.get(&(1, 0)).unwrap(), TrackPiece::Straight(Axe::X));
        assert!(track.contains_key(&(2, 0)));
        assert_eq!(*track.get(&(2, 0)).unwrap(), TrackPiece::Straight(Axe::Y));
        assert!(track.contains_key(&(3, 0)));
        assert_eq!(*track.get(&(3, 0)).unwrap(), TrackPiece::Straight(Axe::Y));

        let cart = carts.get(0).expect("> cart");
        assert_eq!(cart.coord, (0, 0));
        assert_eq!(cart.axe, Axe::X);
        assert_eq!(cart.direction, Direction::Up);

        let cart = carts.get(1).expect("< cart");
        assert_eq!(cart.coord, (1, 0));
        assert_eq!(cart.axe, Axe::X);
        assert_eq!(cart.direction, Direction::Down);

        let cart = carts.get(2).expect("v cart");
        assert_eq!(cart.coord, (2, 0));
        assert_eq!(cart.axe, Axe::Y);
        assert_eq!(cart.direction, Direction::Up);

        let cart = carts.get(3).expect("v cart");
        assert_eq!(cart.coord, (3, 0));
        assert_eq!(cart.axe, Axe::Y);
        assert_eq!(cart.direction, Direction::Down);
    }

    #[test]
    fn test_move_cart_straight() {
        let input = ">-";
        let (track, mut carts) = parse_track(input);
        let cart = &mut carts[0];
        move_cart(&track, cart);
        assert_eq!(cart.coord, (1, 0));
        assert_eq!(cart.axe, Axe::X);
        assert_eq!(cart.direction, Direction::Up);

        let input = "-<";
        let (track, mut carts) = parse_track(input);
        let cart = &mut carts[0];
        move_cart(&track, cart);
        assert_eq!(cart.coord, (0, 0));
        assert_eq!(cart.axe, Axe::X);
        assert_eq!(cart.direction, Direction::Down);

        let input = "v\n|";
        let (track, mut carts) = parse_track(input);
        let cart = &mut carts[0];
        move_cart(&track, cart);
        assert_eq!(cart.coord, (0, 1));
        assert_eq!(cart.axe, Axe::Y);
        assert_eq!(cart.direction, Direction::Up);

        let input = "|\n^";
        let (track, mut carts) = parse_track(input);
        let cart = &mut carts[0];
        move_cart(&track, cart);
        assert_eq!(cart.coord, (0, 0));
        assert_eq!(cart.axe, Axe::Y);
        assert_eq!(cart.direction, Direction::Down);
    }

    #[test]
    fn test_move_cart_turn() {
        let input = ">/";
        let (track, mut carts) = parse_track(input);
        let cart = &mut carts[0];
        move_cart(&track, cart);
        assert_eq!(cart.coord, (1, 0));
        assert_eq!(cart.axe, Axe::Y);
        assert_eq!(cart.direction, Direction::Down);

        let input = "/<";
        let (track, mut carts) = parse_track(input);
        let cart = &mut carts[0];
        move_cart(&track, cart);
        assert_eq!(cart.coord, (0, 0));
        assert_eq!(cart.axe, Axe::Y);
        assert_eq!(cart.direction, Direction::Up);

        let input = ">\\";
        let (track, mut carts) = parse_track(input);
        let cart = &mut carts[0];
        move_cart(&track, cart);
        assert_eq!(cart.coord, (1, 0));
        assert_eq!(cart.axe, Axe::Y);
        assert_eq!(cart.direction, Direction::Up);

        let input = "\\<";
        let (track, mut carts) = parse_track(input);
        let cart = &mut carts[0];
        move_cart(&track, cart);
        assert_eq!(cart.coord, (0, 0));
        assert_eq!(cart.axe, Axe::Y);
        assert_eq!(cart.direction, Direction::Down);

        let input = "v\n/";
        let (track, mut carts) = parse_track(input);
        let cart = &mut carts[0];
        move_cart(&track, cart);
        assert_eq!(cart.coord, (0, 1));
        assert_eq!(cart.axe, Axe::X);
        assert_eq!(cart.direction, Direction::Down);

        let input = "/\n^";
        let (track, mut carts) = parse_track(input);
        let cart = &mut carts[0];
        move_cart(&track, cart);
        assert_eq!(cart.coord, (0, 0));
        assert_eq!(cart.axe, Axe::X);
        assert_eq!(cart.direction, Direction::Up);

        let input = "v\n\\";
        let (track, mut carts) = parse_track(input);
        let cart = &mut carts[0];
        move_cart(&track, cart);
        assert_eq!(cart.coord, (0, 1));
        assert_eq!(cart.axe, Axe::X);
        assert_eq!(cart.direction, Direction::Up);

        let input = "\\\n^";
        let (track, mut carts) = parse_track(input);
        let cart = &mut carts[0];
        move_cart(&track, cart);
        assert_eq!(cart.coord, (0, 0));
        assert_eq!(cart.axe, Axe::X);
        assert_eq!(cart.direction, Direction::Down);
    }

    #[test]
    fn test_move_cart_intersection() {
        let input = ">+";
        let (track, mut carts) = parse_track(input);
        let cart = &mut carts[0];

        move_cart(&track, cart);
        assert_eq!(cart.coord, (1, 0));
        assert_eq!(cart.axe, Axe::Y);
        assert_eq!(cart.direction, Direction::Down);

        let input = "+<";
        let (track, mut carts) = parse_track(input);
        let cart = &mut carts[0];

        move_cart(&track, cart);
        assert_eq!(cart.coord, (0, 0));
        assert_eq!(cart.axe, Axe::Y);
        assert_eq!(cart.direction, Direction::Up);

        let input = "v\n+";
        let (track, mut carts) = parse_track(input);
        let cart = &mut carts[0];

        move_cart(&track, cart);
        assert_eq!(cart.coord, (0, 1));
        assert_eq!(cart.axe, Axe::X);
        assert_eq!(cart.direction, Direction::Up);

        let input = "+\n^";
        let (track, mut carts) = parse_track(input);
        let cart = &mut carts[0];

        move_cart(&track, cart);
        assert_eq!(cart.coord, (0, 0));
        assert_eq!(cart.axe, Axe::X);
        assert_eq!(cart.direction, Direction::Down);

        let input = "v\n+++\n--+";
        let (track, mut carts) = parse_track(input);
        let cart = &mut carts[0];

        move_cart(&track, cart);
        assert_eq!(cart.coord, (0, 1));
        assert_eq!(cart.axe, Axe::X);
        assert_eq!(cart.direction, Direction::Up);

        move_cart(&track, cart);
        assert_eq!(cart.coord, (1, 1));
        assert_eq!(cart.axe, Axe::X);
        assert_eq!(cart.direction, Direction::Up);

        move_cart(&track, cart);
        assert_eq!(cart.coord, (2, 1));
        assert_eq!(cart.axe, Axe::Y);
        assert_eq!(cart.direction, Direction::Up);

        move_cart(&track, cart);
        assert_eq!(cart.coord, (2, 2));
        assert_eq!(cart.axe, Axe::X);
        assert_eq!(cart.direction, Direction::Up);
    }

    #[test]
    fn test_simulate_example_straight() {
        let input = "->---<-";
        let (track, carts) = parse_track(input);
        let crash_coord = simulate_till_crash(&track, carts);
        assert_eq!(crash_coord, (3, 0));

        let input = "|\nv\n|\n|\n|\n^\n|";
        let (track, carts) = parse_track(input);
        let crash_coord = simulate_till_crash(&track, carts);
        assert_eq!(crash_coord, (0, 3));
    }

    #[test]
    fn test_simulate_example_complext() {
        let input = "/->-\\        \n\
                     |   |  /----\\\n\
                     | /-+--+-\\  |\n\
                     | | |  | v  |\n\
                     \\-+-/  \\-+--/\n\
                     \\------/   ";
        let (track, carts) = parse_track(input);
        let crash_coord = simulate_till_crash(&track, carts);
        assert_eq!(crash_coord, (7, 3));
    }

    #[test]
    fn test_simulate_till_one_stand_example() {
        let input = "/>-<\\\n|   |\n| /<+-\\\n| | | v\n\\>+</ |\n  |   ^\n  \\<->/";
        let (track, carts) = parse_track(input);
        let crash_coord = simulate_till_one_stand(&track, carts);
        assert_eq!(crash_coord, (6, 4));
    }
}
