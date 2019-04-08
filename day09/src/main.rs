use std::collections::VecDeque;
use std::io::{self, Read};

#[inline(always)]
fn modulus(a: i64, b: usize) -> usize {
    let b = b as i64;
    (((a % b) + b) % b) as usize
}

struct GameState {
    marbles: VecDeque<u32>,
    scores: Vec<u32>,
    marble_idx: usize,
    marble_value: u32,
}

impl GameState {
    fn new(players: usize, marbles: usize) -> GameState {
        let mut marbles = VecDeque::with_capacity(marbles - (marbles / 23 * 2) + 1);
        marbles.push_back(0);

        let scores: Vec<u32> = vec![0; players];
        GameState {
            marbles,
            scores,
            marble_idx: 0,
            marble_value: 0,
        }
    }
}

// advance the game until some player score
fn next_state(mut gs: GameState) -> GameState {
    for _ in 0..22 {
        let mut idx = gs.marble_idx;
        let marbles_len = gs.marbles.len();
        idx = (idx + 2) % marbles_len;
        if idx == 0 {
            idx = marbles_len;
        }
        gs.marble_value += 1;
        gs.marble_idx = idx;
        gs.marbles.insert(idx, gs.marble_value);
    }

    gs.marble_value += 1;
    let player = gs.marble_value as usize % gs.scores.len();
    gs.scores[player] += gs.marble_value;

    let to_remove = modulus(gs.marble_idx as i64 - 7, gs.marbles.len());
    gs.scores[player] += gs.marbles.remove(to_remove).unwrap();
    gs.marble_idx = to_remove;

    gs
}

fn play_game(marbles: usize, players: usize) -> GameState {
    let turns = marbles / 23;
    (0..turns).fold(GameState::new(players, marbles), |gs, _| next_state(gs))
}

fn parse_game(input: &str) -> (usize, usize) {
    let cols: Vec<_> = input.split_whitespace().collect();
    let players = cols
        .get(0)
        .and_then(|c| c.parse::<usize>().ok())
        .expect("Number of players");
    let marbles = cols
        .get(6)
        .and_then(|c| c.parse::<usize>().ok())
        .expect("Number of marbles");
    (players, marbles)
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let input = input;

    let (players, marbles) = parse_game(&input);

    let gs = play_game(marbles, players);
    let max_score = *gs.scores.iter().max().expect("Maximum score");
    println!("{}", max_score);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{parse_game, play_game};

    #[test]
    fn test_parse_game_examples() {
        let input = "10 players; last marble is worth 1618 points";
        assert_eq!(parse_game(input), (10, 1618));
    }

    #[test]
    fn test_play_game_examples() {
        let gs = play_game(25, 9);
        assert_eq!(*gs.scores.iter().max().unwrap(), 32);

        let gs = play_game(1618, 10);
        assert_eq!(*gs.scores.iter().max().unwrap(), 8317);

        let gs = play_game(7999, 13);
        assert_eq!(*gs.scores.iter().max().unwrap(), 146_373);

        let gs = play_game(1104, 17);
        assert_eq!(*gs.scores.iter().max().unwrap(), 2764);

        let gs = play_game(6111, 21);
        assert_eq!(*gs.scores.iter().max().unwrap(), 54718);

        let gs = play_game(5807, 30);
        assert_eq!(*gs.scores.iter().max().unwrap(), 37305);
    }
}
