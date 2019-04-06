use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::io::{self, Read};

type Edge = (u8, u8);
type Graph = HashMap<u8, Vec<u8>>;

fn from_edges(edges: impl Iterator<Item = Edge>) -> Graph {
    edges.fold(Graph::new(), |mut graph, (from, to)| {
        graph.entry(to).or_default();
        graph.entry(from).or_default().push(to);
        graph
    })
}

fn parse_edge(line: &str) -> Edge {
    let bs = line.as_bytes();
    (bs[5], bs[36])
}

fn parse_edges(input: &str) -> impl Iterator<Item = Edge> + '_ {
    input.lines().map(parse_edge)
}

fn get_in_edges_count(graph: &Graph) -> HashMap<&u8, u8> {
    graph
        .iter()
        .fold(HashMap::new(), |mut in_edges, (k, edges)| {
            in_edges.entry(k).or_default();
            edges.iter().fold(in_edges, |mut in_edges, edge| {
                *in_edges.entry(edge).or_default() += 1;
                in_edges
            })
        })
}

fn init_queue<'a>(in_edges: &HashMap<&'a u8, u8>) -> BinaryHeap<Reverse<&'a u8>> {
    in_edges
        .iter()
        .filter(|(_, &v)| v == 0)
        .map(|(k, _)| Reverse(*k))
        .collect()
}

fn topological_order(graph: &Graph) -> Vec<u8> {
    let mut in_edges = get_in_edges_count(&graph);

    let mut queue = init_queue(&in_edges);

    let mut order: Vec<u8> = Vec::with_capacity(graph.len());

    while !queue.is_empty() {
        let Reverse(k) = queue.pop().unwrap();
        order.push(*k);

        graph.get(k).unwrap().iter().for_each(|edge| {
            in_edges.entry(edge).and_modify(|c| {
                *c -= 1;
                if *c == 0 {
                    queue.push(Reverse(edge));
                }
            });
        });
    }

    order
}

fn schedule_tasks(graph: Graph, get_time: impl Fn(u8) -> u32, worker_n: usize) -> Vec<(u8, u32)> {
    let mut in_edges = get_in_edges_count(&graph);

    let mut queue = init_queue(&in_edges);

    let mut order: Vec<(u8, u32)> = Vec::with_capacity(graph.len());

    let mut workers: Vec<(u32, u8)> = Vec::with_capacity(worker_n);

    let mut time: u32 = 0;
    while !queue.is_empty() || !workers.is_empty() {
        // fill workers
        while workers.len() < worker_n && !queue.is_empty() {
            let Reverse(k) = queue.pop().expect("Queue is empty");
            workers.push((time + get_time(*k), *k));
        }

        // do one step
        let (idx, (task_time, task)) = workers
            .iter()
            .cloned()
            .enumerate()
            .min_by_key(|(_, data)| *data)
            .expect("No elements in workers");
        workers.remove(idx);

        time = task_time;
        order.push((task, task_time));

        graph.get(&task).unwrap().iter().for_each(|edge| {
            in_edges.entry(edge).and_modify(|c| {
                *c -= 1;
                if *c == 0 {
                    queue.push(Reverse(edge));
                }
            });
        });
    }

    order
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let input = input;

    let graph = from_edges(parse_edges(&input));
    let order = topological_order(&graph);
    let (_, time) = schedule_tasks(graph, |t| (t - b'A') as u32 + 61, 5)
        .last()
        .expect("Schedule result")
        .clone();
    println!("{}", String::from_utf8_lossy(&order));
    println!("{}", time);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{from_edges, parse_edge, parse_edges, schedule_tasks, topological_order, Edge};

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

    #[test]
    fn test_topological_order() {
        let edges = vec![(b'C', b'A'), (b'C', b'F'), (b'A', b'B')];
        let graph = from_edges(edges.into_iter());
        assert_eq!(topological_order(&graph), vec![b'C', b'A', b'B', b'F']);
    }

    #[test]
    fn test_topological_order_example() {
        let input = "Step C must be finished before step A can begin.\n\
                     Step C must be finished before step F can begin.\n\
                     Step A must be finished before step B can begin.\n\
                     Step A must be finished before step D can begin.\n\
                     Step B must be finished before step E can begin.\n\
                     Step D must be finished before step E can begin.\n\
                     Step F must be finished before step E can begin.";
        assert_eq!(
            topological_order(&from_edges(parse_edges(input))),
            vec![b'C', b'A', b'B', b'D', b'F', b'E']
        );
    }

    #[test]
    fn test_schedule_tasks() {
        let input = "Step C must be finished before step A can begin.\n\
                     Step C must be finished before step F can begin.\n\
                     Step A must be finished before step B can begin.\n\
                     Step A must be finished before step D can begin.\n\
                     Step B must be finished before step E can begin.\n\
                     Step D must be finished before step E can begin.\n\
                     Step F must be finished before step E can begin.";
        assert_eq!(
            schedule_tasks(from_edges(parse_edges(input)), |t| (t - b'A') as u32 + 1, 2),
            vec![
                (b'C', 3),
                (b'A', 4),
                (b'B', 6),
                (b'F', 9),
                (b'D', 10),
                (b'E', 15)
            ]
        );
    }
}
