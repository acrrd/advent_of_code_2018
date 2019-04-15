use std::collections::HashMap;

struct Pots {
    list: Vec<bool>,
    zero_pos: usize,
}

impl Pots {
    fn new(pots: &[bool]) -> Pots {
        if pots.len() == 0 {
            panic!("There must be at least one pot");
        }
        Pots {
            list: Vec::from(pots),
            zero_pos: 0,
        }
    }
}

// ensure that there are at least 4 empty pots at the begin and at the end
fn ensure_empty_pots(pots: Pots) -> Pots {
    let first_full = pots.list.iter().take(4).position(|p| *p).map(|p| 4 - p);
    let last_full = pots
        .list
        .iter()
        .rev()
        .take(4)
        .position(|p| *p)
        .map(|p| (p as i32 - 4).abs() as usize);

    let to_add_at =
        |pos: Option<usize>| if let Some(p) = pos { (0..p) } else { (0..0) }.map(|_| false);

    let list = to_add_at(first_full)
        .chain(pots.list.into_iter())
        .chain(to_add_at(last_full))
        .collect::<Vec<_>>();

    let zero_pos = pots.zero_pos + first_full.unwrap_or(0);

    Pots { list, zero_pos }
}

type Pattern = [bool; 5];
type Patterns = HashMap<Pattern, bool>;

fn parse_pots(input: &str) -> Vec<bool> {
    input.chars().map(|c| c == '#').collect()
}

fn parse_pattern(line: &str) -> (Pattern, bool) {
    let cs: Vec<_> = line.split(' ').collect();
    let pattern = cs.get(0).expect("Impossible to read pattern");
    let pattern: Vec<_> = parse_pots(pattern);
    let status = cs.get(2).expect("Impossible to read pattern");

    if pattern.len() != 5 {
        panic!("Pattern must have len 5");
    }

    let mut arr = [false; 5];
    arr.copy_from_slice(&pattern);
    (arr, *status == "#")
}

fn parse_patterns(input: &str) -> Patterns {
    input
        .lines()
        .map(parse_pattern)
        .fold(HashMap::new(), |mut patterns, (pattern, value)| {
            patterns.insert(pattern, value);
            patterns
        })
}

fn next_state(patterns: &Patterns, mut pots: Pots) -> Pots {
    pots.list = pots
        .list
        .windows(5)
        .map(|pattern| *patterns.get(pattern).unwrap_or(&false) )
        .collect::<Vec<_>>();
    ensure_empty_pots(pots)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::{ensure_empty_pots, next_state, parse_pattern, parse_patterns, parse_pots, Pots};

    const T: bool = true;
    const F: bool = false;

    #[test]
    fn test_ensure_empty_posts_one_true() {
        let pots = Pots::new(&[T]);
        let pots = ensure_empty_pots(pots);

        assert_eq!(pots.list, vec![F, F, F, F, T, F, F, F, F]);
        assert_eq!(pots.zero_pos, 4);

        let pots = Pots::new(&[F, T]);
        let pots = ensure_empty_pots(pots);

        assert_eq!(pots.list, vec![F, F, F, F, T, F, F, F, F]);
        assert_eq!(pots.zero_pos, 3);

        let pots = Pots::new(&[F, F, T]);
        let pots = ensure_empty_pots(pots);

        assert_eq!(pots.list, vec![F, F, F, F, T, F, F, F, F]);
        assert_eq!(pots.zero_pos, 2);

        let pots = Pots::new(&[F, F, F, T]);
        let pots = ensure_empty_pots(pots);

        assert_eq!(pots.list, vec![F, F, F, F, T, F, F, F, F]);
        assert_eq!(pots.zero_pos, 1);

        let pots = Pots::new(&[F, F, F, F, T]);
        let pots = ensure_empty_pots(pots);

        assert_eq!(pots.list, vec![F, F, F, F, T, F, F, F, F]);
        assert_eq!(pots.zero_pos, 0);

        let pots = Pots::new(&[F, F, F, F, F, T]);
        let pots = ensure_empty_pots(pots);

        assert_eq!(pots.list, vec![F, F, F, F, F, T, F, F, F, F]);
        assert_eq!(pots.zero_pos, 0);

        let pots = Pots::new(&[T, F]);
        let pots = ensure_empty_pots(pots);

        assert_eq!(pots.list, vec![F, F, F, F, T, F, F, F, F]);
        assert_eq!(pots.zero_pos, 4);

        let pots = Pots::new(&[T, F, F]);
        let pots = ensure_empty_pots(pots);

        assert_eq!(pots.list, vec![F, F, F, F, T, F, F, F, F]);
        assert_eq!(pots.zero_pos, 4);

        let pots = Pots::new(&[T, F, F, F, F]);
        let pots = ensure_empty_pots(pots);

        assert_eq!(pots.list, vec![F, F, F, F, T, F, F, F, F]);
        assert_eq!(pots.zero_pos, 4);
    }

    #[test]
    fn test_parse_pattern() {
        let line = "#.### => .";
        assert_eq!(parse_pattern(line), ([T, F, T, T, T], F));

        let line = "#.### => #";
        assert_eq!(parse_pattern(line), ([T, F, T, T, T], T));

        let line = "..#.. => #";
        assert_eq!(parse_pattern(line), ([F, F, T, F, F], T));
    }

    #[test]
    fn test_parse_patterns() {
        let input = "#.### => .\n\
                     #.#.# => #\n\
                     ..#.. => #";
        let patterns = parse_patterns(input);

        assert!(patterns.contains_key(&[T, F, T, T, T]));
        assert_eq!(*patterns.get(&[T, F, T, T, T]).unwrap(), F);

        assert!(patterns.contains_key(&[T, F, T, F, T]));
        assert_eq!(*patterns.get(&[T, F, T, F, T]).unwrap(), T);

        assert!(patterns.contains_key(&[F, F, T, F, F]));
        assert_eq!(*patterns.get(&[F, F, T, F, F]).unwrap(), T);
    }

    #[test]
    fn test_next_state() {
        let input = "...## => #\n\
                     ..#.. => #\n\
                     .#... => #\n\
                     .#.#. => #\n\
                     .#.## => #\n\
                     .##.. => #\n\
                     .#### => #\n\
                     #.#.# => #\n\
                     #.### => #\n\
                     ##.#. => #\n\
                     ##.## => #\n\
                     ###.. => #\n\
                     ###.# => #\n\
                     ####. => #";
        let patterns = parse_patterns(input);
        let pots = parse_pots("#..#.#..##");
        let pots = ensure_empty_pots(Pots::new(&pots));
        let next = next_state(&patterns, pots);
        assert_eq!(next.list, parse_pots("....#...#....#...."));
        let next = next_state(&patterns, next);
        assert_eq!(next.list, parse_pots("....##..##...##...."));
    }

    #[test]
    fn test_next_state_example() {
        let input = "...## => #\n\
                     ..#.. => #\n\
                     .#... => #\n\
                     .#.#. => #\n\
                     .#.## => #\n\
                     .##.. => #\n\
                     .#### => #\n\
                     #.#.# => #\n\
                     #.### => #\n\
                     ##.#. => #\n\
                     ##.## => #\n\
                     ###.. => #\n\
                     ###.# => #\n\
                     ####. => #";
        let tests = vec![
            "....#...#....#.....#..#..#..#....",
            "....##..##...##....#..#..#..##....",
            "....#.#...#..#.#....#..#..#...#....",
            "....#.#..#...#.#...#..#..##..##....",
            "....#...##...#.#..#..#...#...#....",
            "....##.#.#....#...#..##..##..##....",
            "....#..###.#...##..#...#...#...#....",
            "....#....##.#.#.#..##..##..##..##....",
            "....##..#..#####....#...#...#...#....",
            "....#.#..#...#.##....##..##..##..##....",
            "....#...##...#.#...#.#...#...#...#....",
            "....##.#.#....#.#...#.#..##..##..##....",
            "....#..###.#....#.#...#....#...#...#....",
            "....#....##.#....#.#..##...##..##..##....",
            "....##..#..#.#....#....#..#.#...#...#....",
            "....#.#..#...#.#...##...#...#.#..##..##....",
            "....#...##...#.#.#.#...##...#....#...#....",
            "....##.#.#....#####.#.#.#...##...##..##....",
            "....#..###.#..#.#.#######.#.#.#..#.#...#....",
            "....#....##....#####...#######....#.#..##....",
        ];

        let patterns = parse_patterns(input);
        let pots = parse_pots("#..#.#..##......###...###");
        let pots = ensure_empty_pots(Pots::new(&pots));

        tests.iter().fold(pots, |pots, test| {
            let next = next_state(&patterns, pots);
            assert_eq!(next.list, parse_pots(test));
            next
        });
    }
}
