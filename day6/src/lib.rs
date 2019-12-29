use std::collections::HashMap;
type Graph = HashMap<String, Vec<String>>;

fn add_edge(graph: &mut Graph, tail: &str, head: &str) {
    let head = String::from(head);
    let tail = String::from(tail);
    if let Some(heads) = graph.get_mut(&tail) {
        heads.push(head);
    } else {
        graph.insert(tail, vec![head]);
    }
}

mod pt1 {
    use super::*;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    fn process_line(graph: &mut Graph, line: &str) {
        let mut iter = line.split(')');
        let tail = iter.next().unwrap();
        let head = iter.next().unwrap();
        add_edge(graph, tail, head);
    }

    fn parse_input(path_to_input: &str) -> Graph {
        let file = File::open(path_to_input).unwrap();
        let reader = BufReader::new(file);
        let mut graph = HashMap::new();
        reader
            .lines()
            .for_each(|line| process_line(&mut graph, &line.unwrap()));
        graph
    }

    fn walk(graph: &Graph, distance_from_center: i32, node: &str) -> i32 {
        distance_from_center
            + if let Some(connections) = graph.get(node) {
                connections
                    .iter()
                    .map(|node| walk(graph, distance_from_center + 1, node))
                    .sum()
            } else {
                0
            }
    }

    pub fn pt1(path_to_input: &str) -> i32 {
        let graph = parse_input(path_to_input);
        walk(&graph, 0, "COM")
    }
}

mod pt2 {
    use super::*;
    use std::collections::HashSet;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    fn process_line(graph: &mut Graph, line: &str) {
        let mut iter = line.split(')');
        let tail = iter.next().unwrap();
        let head = iter.next().unwrap();
        add_edge(graph, tail, head);
        add_edge(graph, head, tail);
    }

    fn parse_input(path_to_input: &str) -> Graph {
        let file = File::open(path_to_input).unwrap();
        let reader = BufReader::new(file);
        let mut graph = HashMap::new();
        reader
            .lines()
            .for_each(|line| process_line(&mut graph, &line.unwrap()));
        graph
    }

    fn walk(
        graph: &Graph,
        visited: HashSet<String>,
        path_length: i32,
        node: &str,
        destination: &str,
    ) -> i32 {
        if node == destination {
            path_length - 2
        } else if let Some(connections) = graph.get(node) {
            let min_path_opt = connections
                .iter()
                .filter(|&node| !visited.contains(node))
                .map(|node| {
                    let mut visited_clone = visited.clone();
                    visited_clone.insert(String::from(node));
                    walk(graph, visited_clone, path_length + 1, node, destination)
                })
                .filter(|&x| x >= 0)
                .min();
            if let Some(length) = min_path_opt {
                length
            } else {
                -1
            }
        } else {
            -1
        }
    }

    pub fn pt2(path_to_input: &str) -> i32 {
        let graph = parse_input(path_to_input);
        walk(&graph, HashSet::new(), 0, "YOU", "SAN")
    }
}

pub use pt1::pt1;
pub use pt2::pt2;

#[cfg(test)]
mod tests {
    use super::pt1::*;
    #[test]
    fn pt1_test_works() {
        assert_eq!(pt1("test_input_pt1"), 42);
    }

    #[test]
    fn pt1_works() {
        assert_eq!(pt1("input"), 301_100);
    }

    use super::pt2::*;
    #[test]
    fn pt2_test_works() {
        assert_eq!(pt2("test_input_pt2"), 4);
    }

    #[test]
    fn pt2_works() {
        assert_eq!(pt2("input"), 547);
    }
}
