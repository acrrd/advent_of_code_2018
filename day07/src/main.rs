use itertools::Itertools;
use std::collections::HashMap;

type Edge = (u8, u8);
type Graph = HashMap<u8, Vec<u8>>;

fn from_edges(edges: impl Iterator<Item = Edge>) -> Graph {
    edges.into_group_map()
}

fn parse_edge(line: &str) -> Edge {
    let bs = line.as_bytes();
    (bs[5], bs[36])
}

fn parse_edges(input: &str) -> impl Iterator<Item = Edge> + '_ {
    input.lines().map(parse_edge)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::{from_edges, parse_edge, parse_edges, Edge};

    #[test]
    fn test_parse_edge() {
        let input = "Step C must be finished before step A can begin.";
        assert_eq!(parse_edge(input), (b'C', b'A'));
    }

    #[test]
    fn test_parse_edges() {
        let input = "Step C must be finished before step A can begin.\n\
                     Step C must be finished before step F can begin.\n\
                     Step A must be finished before step B can begin.";
        let edges = vec![(b'C', b'A'), (b'C', b'F'), (b'A', b'B')];
        assert_eq!(parse_edges(input).collect::<Vec<Edge>>(), edges);
    }

    #[test]
    fn test_from_edges() {
        let edges = vec![(b'C', b'A'), (b'C', b'F'), (b'A', b'B')];
        let graph = from_edges(edges.into_iter());
        assert!(graph.contains_key(&b'C'));
        assert_eq!(*graph.get(&b'C').unwrap(), vec![b'A', b'F']);
        assert!(graph.contains_key(&b'A'));
        assert_eq!(*graph.get(&b'A').unwrap(), vec![b'B']);
        assert!(graph.contains_key(&b'F'));
        assert_eq!(*graph.get(&b'F').unwrap(), vec![]);
        assert!(graph.contains_key(&b'B'));
        assert_eq!(*graph.get(&b'B').unwrap(), vec![]);
    }
}
