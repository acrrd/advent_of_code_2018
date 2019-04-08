use std::io::{self, Read};

// double linked list like structure
// instead of having pointers to prev and next nodes it
// keep their indexes in the vector
#[derive(Debug)]
struct MarbleNode {
    value: u32,
    prev: usize,
    next: usize,
}

type MarbleIndex = usize;

#[derive(Debug)]
struct MarbleList {
    nodes: Vec<MarbleNode>,
}

impl MarbleList {
    fn new(marbles: usize) -> MarbleList {
        let mut nodes = Vec::with_capacity(marbles);
        nodes.push(MarbleNode {
            value: 0,
            prev: 0,
            next: 0,
        });
        MarbleList { nodes: nodes }
    }

    fn get(self: &Self, cur: MarbleIndex) -> u32 {
        self.nodes.get(cur).expect("Cannot find MarbleNode").value
    }

    fn add(self: &mut Self, cur: MarbleIndex, value: u32) -> MarbleIndex {
        let new = self.nodes.len();
        let cur_node = self.nodes.get(cur).expect("Cannot find MarbleNode");

        let prev = cur_node.next;
        let prev_node = self.nodes.get_mut(prev).expect("Cannot find MarbleNode");
        let next = prev_node.next;
        prev_node.next = new;

        let next_node = self.nodes.get_mut(next).expect("Cannot find MarbleNode");
        next_node.prev = new;

        self.nodes.push(MarbleNode {
            value,
            prev: prev,
            next: next,
        });

        new
    }

    fn back(self: &Self, cur: MarbleIndex) -> MarbleIndex {
        let mut node = self.nodes.get(cur).expect("Cannot find MarbleNode");
        for _ in 0..6 {
            node = self.nodes.get(node.prev).expect("Cannot find MarbleNode");
        }
        node.prev
    }

    // we are leaving hole in the vector :(
    fn remove(self: &mut Self, cur: MarbleIndex) -> MarbleIndex {
        let node = self.nodes.get(cur).expect("Cannot find MarbleNode");
        let prev = node.prev;
        let next = node.next;
        let prev_node = self.nodes.get_mut(prev).expect("Cannot find MarbleNode");
        prev_node.next = next;

        let next_node = self.nodes.get_mut(next).expect("Cannot find MarbleNode");
        next_node.prev = prev;

        next
    }
}

struct GameState {
    marbles: MarbleList,
    marble_cur: MarbleIndex,
    scores: Vec<u32>,
    marble_value: u32,
}

impl GameState {
    fn new(players_n: usize, marbles_n: usize) -> GameState {
        let scores: Vec<u32> = vec![0; players_n];
        GameState {
            marbles: MarbleList::new(marbles_n),
            marble_cur: 0,
            scores,
            marble_value: 0,
        }
    }
}

// advance the game until some player score
fn next_state(mut gs: GameState) -> GameState {
    for _ in 0..22 {
        gs.marble_value += 1;
        gs.marble_cur = gs.marbles.add(gs.marble_cur, gs.marble_value);
    }

    gs.marble_value += 1;
    let player = gs.marble_value as usize % gs.scores.len();
    gs.scores[player] += gs.marble_value;

    let to_remove = gs.marbles.back(gs.marble_cur);
    gs.scores[player] += gs.marbles.get(to_remove);

    gs.marble_cur = gs.marbles.remove(to_remove);

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

    let gs = play_game(marbles * 100, players);
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
