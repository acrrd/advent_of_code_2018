type Edge = (u8, u8);

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
    use super::{parse_edge, parse_edges, Edge};

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
}
