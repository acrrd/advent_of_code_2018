use std::collections::HashMap;
use std::io::{self, Read};

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
    let at_least = 4;
    let first_full = pots
        .list
        .iter()
        .take(at_least)
        .position(|p| *p)
        .map(|p| at_least - p);
    let last_full = pots
        .list
        .iter()
        .rev()
        .take(at_least)
        .position(|p| *p)
        .map(|p| (p as i32 - at_least as i32).abs() as usize);

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

fn parse_patterns<'a>(lines: impl Iterator<Item = &'a str>) -> Patterns {
    lines
        .map(parse_pattern)
        .fold(HashMap::new(), |mut patterns, (pattern, value)| {
            patterns.insert(pattern, value);
            patterns
        })
}

fn next_state(patterns: &Patterns, mut pots: Pots) -> Pots {
    pots.list = (0..2)
        .map(|_| false)
        .chain(
            pots.list
                .windows(5)
                .map(|pattern| *patterns.get(pattern).unwrap_or(&false)),
        )
        .collect::<Vec<_>>();
    ensure_empty_pots(pots)
}

fn sum_pots_position(pots: &Pots, base: i64) -> i64 {
    pots.list
        .iter()
        .enumerate()
        .filter(|(_, p)| **p)
        .map(|(pos, _)| base + pos as i64 - pots.zero_pos as i64)
        .sum()
}

fn play_game(pots: Pots, patterns: &Patterns, turns: usize) -> Pots {
    (0..turns).fold(pots, |pots, _| next_state(&patterns, pots))
}

fn parse_input(input: &str) -> (Pots, Patterns) {
    let pots = input
        .lines()
        .take(1)
        .map(|line| parse_pots(&line[15..].trim()))
        .nth(0)
        .expect("Initial state");
    let pots = ensure_empty_pots(Pots::new(&pots));

    let patterns = parse_patterns(input.lines().skip(2));
    (pots, patterns)
}

fn find_convergence(mut pots: Pots, patterns: &Patterns) -> (usize, usize, Pots) {
    let reduce = |list: &Vec<bool>| {
        list.iter()
            .skip_while(|v| !**v)
            .cloned()
            .collect::<Vec<bool>>()
    };
    let count_empty_pot_in_front =
        |list: &Vec<bool>| list.iter().position(|p| *p).expect("A full pot");
    let mut turns: usize = 0;
    let mut prev_list = reduce(&pots.list);
    let mut prev_empty_pot_in_front = count_empty_pot_in_front(&pots.list);

    loop {
        turns += 1;
        pots = next_state(&patterns, pots);
        if prev_list == reduce(&pots.list) {
            return (
                turns,
                count_empty_pot_in_front(&pots.list) - prev_empty_pot_in_front,
                pots
            );
        }

        prev_list = reduce(&pots.list);
        prev_empty_pot_in_front = count_empty_pot_in_front(&pots.list);
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let input = input;

    let (pots, patterns) = parse_input(&input);
    let pots = play_game(pots, &patterns, 20);

    println!("{}", sum_pots_position(&pots, 0));

    let info = find_convergence(pots, &patterns);
    let end_turns: usize = 50_000_000_000;
    let remaining_turns = end_turns - 20 - info.0;
    let shift = remaining_turns * info.1;
    println!("{}", sum_pots_position(&info.2, shift as i64));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        ensure_empty_pots, next_state, parse_input, parse_pattern, parse_patterns, parse_pots,
        play_game, sum_pots_position, Pots,
    };

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
        let patterns = parse_patterns(input.lines());

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
        let patterns = parse_patterns(input.lines());
        let pots = parse_pots("#..#.#..##");
        let pots = ensure_empty_pots(Pots::new(&pots));
        let next = next_state(&patterns, pots);
        assert_eq!(next.list, parse_pots("....#...#....#...."));
        assert_eq!(next.zero_pos, 4);
        let next = next_state(&patterns, next);
        assert_eq!(next.list, parse_pots("....##..##...##...."));
        assert_eq!(next.zero_pos, 4);
        let next = next_state(&patterns, next);
        assert_eq!(next.list, parse_pots("....#.#...#..#.#...."));
        assert_eq!(next.zero_pos, 5);
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
            ("....#...#....#.....#..#..#..#....", 4),
            ("....##..##...##....#..#..#..##....", 4),
            ("....#.#...#..#.#....#..#..#...#....", 5),
            (".....#.#..#...#.#...#..#..##..##....", 5),
            ("......#...##...#.#..#..#...#...#....", 5),
            ("......##.#.#....#...#..##..##..##....", 5),
            (".....#..###.#...##..#...#...#...#....", 5),
            (".....#....##.#.#.#..##..##..##..##....", 5),
            (".....##..#..#####....#...#...#...#....", 5),
            ("....#.#..#...#.##....##..##..##..##....", 5),
            (".....#...##...#.#...#.#...#...#...#....", 5),
            (".....##.#.#....#.#...#.#..##..##..##....", 5),
            ("....#..###.#....#.#...#....#...#...#....", 5),
            ("....#....##.#....#.#..##...##..##..##....", 5),
            ("....##..#..#.#....#....#..#.#...#...#....", 5),
            ("....#.#..#...#.#...##...#...#.#..##..##....", 6),
            (".....#...##...#.#.#.#...##...#....#...#....", 6),
            (".....##.#.#....#####.#.#.#...##...##..##....", 6),
            ("....#..###.#..#.#.#######.#.#.#..#.#...#....", 6),
            ("....#....##....#####...#######....#.#..##....", 6),
        ];

        let patterns = parse_patterns(input.lines());
        let pots = parse_pots("#..#.#..##......###...###");
        let pots = ensure_empty_pots(Pots::new(&pots));

        tests.iter().fold(pots, |pots, test| {
            let next = next_state(&patterns, pots);
            assert_eq!(next.zero_pos, test.1);
            assert_eq!(next.list, parse_pots(test.0));
            next
        });
    }

    #[test]
    fn test_sum_pots_position() {
        let pots = parse_pots("#.#.#");
        let pots = ensure_empty_pots(Pots::new(&pots));
        assert_eq!(sum_pots_position(&pots, 0), 6);
        let pots = parse_pots(".#.#.#");
        let pots = ensure_empty_pots(Pots::new(&pots));
        assert_eq!(sum_pots_position(&pots, 0), 9);
    }

    #[test]
    fn test_sum_pots_position_example() {
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
        let patterns = parse_patterns(input.lines());
        let pots = parse_pots("#..#.#..##......###...###");
        let pots = ensure_empty_pots(Pots::new(&pots));

        let pots = (0..20).fold(pots, |pots, _| next_state(&patterns, pots));

        assert_eq!(sum_pots_position(&pots, 0), 325);
    }

    #[test]
    fn test_play_game() {
        let input = "initial state: #..#.#..##......###...###\n\
                     \n\
                     ...## => #\n\
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

        let test_pots = parse_pots("#..#.#..##......###...###");
        let test_pots = ensure_empty_pots(Pots::new(&test_pots));

        let pattern_input = "...## => #\n\
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
        let test_patterns = parse_patterns(pattern_input.lines());

        let (pots, patterns) = parse_input(&input);
        assert_eq!(pots.list, test_pots.list);
        assert_eq!(patterns, test_patterns);

        let pots = play_game(pots, &patterns, 20);
        assert_eq!(sum_pots_position(&pots, 0), 325);
    }

}
