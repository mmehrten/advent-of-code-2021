use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
/// Parse the file path from command line arguments.
///
/// # Arguments
///
/// * `args` - the command line arguments
///
/// # Returns
///
/// A single command line argument - panics if zero or more than one argument are passed.
fn parse_file_path(args: &[String]) -> &str {
    if args.len() != 2 {
        panic!(
            "Expected one file path and an optional window size to run against, got: {} arguments",
            args.len() - 1
        );
    }
    let input_path = &args[1];
    input_path.as_str()
}

#[cfg(test)]
mod test_parse_file_path {
    use crate::parse_file_path;

    #[test]
    fn one_arg_ok() {
        assert_eq!(
            parse_file_path(&vec!["script_path".to_string(), "arg_text".to_string()][..]),
            "arg_text"
        );
    }

    #[test]
    #[should_panic]
    fn no_arg_fail() {
        parse_file_path(&Vec::new());
    }

    #[test]
    #[should_panic]
    fn many_arg_fail() {
        parse_file_path(
            &vec![
                "script_path".to_string(),
                "arg_text".to_string(),
                "extra_arg".to_string(),
            ][..],
        );
    }
}

/// Open an input path and return a buffered reader over the contents.
fn get_buf_reader(input_path: &str) -> BufReader<File> {
    // Create a buffer to read the file line by line
    let contents =
        File::open(input_path).expect(format!("Error reading file: {}", input_path).as_str());
    let reader = BufReader::new(contents);
    reader
}

#[cfg(test)]
mod test_get_buf_reader {
    use crate::get_buf_reader;

    #[test]
    #[should_panic]
    fn error_file_handled() {
        get_buf_reader("inputs/noexist.txt");
    }

    #[test]
    fn example_file_handled() {
        get_buf_reader("inputs/example.txt");
    }
}

struct Node {
    id: usize,
    name: String,
    is_start: bool,
    is_end: bool,
    is_large: bool,
}

struct Graph {
    nodes: Vec<Node>,
    adjascency: HashMap<usize, Vec<usize>>,
    starting_node_idx: usize,
    ending_node_idx: usize,
}

impl Graph {
    /// Create a Graph using an iterator of lines containing graph connections.
    ///
    /// Connections can be of the form {source_node}-{target_node}, where
    /// `start` is the starting node and `end` is the final node.
    /// All edges are bidirectional. Lowercase nodes are taken to be "small" - e.g. can only be visited
    /// once in a traversal. Otherwise, nodes are taken to be "large" and can be visited multiple times
    /// in a traversal.
    fn from_lines<'a, I>(lines: I) -> Graph
    where
        I: Iterator<Item = Result<String, std::io::Error>>,
    {
        // Parse out all the node names and their mappings
        let node_names = lines
            .map(|line| line.expect("Failed to read line from file."))
            .map(|s| s.split("-").map(|r| r.to_string()).collect::<Vec<String>>())
            .flatten()
            .collect::<Vec<String>>();

        // Assign each node name a unique ID
        let mut current_node_id = 0;
        let mut name_to_id = HashMap::new();
        for node_name in &node_names {
            if !name_to_id.contains_key(node_name) {
                name_to_id.insert(node_name, current_node_id);
                current_node_id += 1;
            }
        }

        // Create a vector of unique nodes
        let mut nodes = Vec::new();
        // Fill nodes with garbage to insert real nodes after
        for _ in 0..name_to_id.len() {
            nodes.push(Node {
                id: 0,
                name: "".to_string(),
                is_start: true,
                is_end: true,
                is_large: true,
            });
        }
        let mut starting_node_idx = 0;
        let mut ending_node_idx = 0;
        for (node_name, node_id) in &name_to_id {
            let node_name = *node_name;
            let node = Node {
                id: *node_id,
                name: node_name.clone(),
                is_start: node_name == "start",
                is_end: node_name == "end",
                is_large: node_name == &node_name.to_uppercase(),
            };
            println!(
                "id={}, name={}, start={}, end={}, large={}",
                node.id, node.name, node.is_start, node.is_end, node.is_large
            );
            if node.is_start {
                starting_node_idx = node.id;
            }
            if node.is_end {
                ending_node_idx = node.id;
            }
            nodes[*node_id] = node;
        }
        let mut adj = HashMap::new();
        for idx in (0..node_names.len()).step_by(2) {
            let id0 = name_to_id.get(&node_names[idx]).unwrap();
            let id1 = name_to_id.get(&node_names[idx + 1]).unwrap();
            let _ = adj
                .entry(*id0)
                .and_modify(|v: &mut Vec<usize>| v.push(*id1))
                .or_insert(vec![*id1]);
            // links are bidirectional for all nodes but start->node and node->end
            if *id0 == starting_node_idx || *id1 == ending_node_idx {
                continue;
            }
            let _ = adj
                .entry(*id1)
                .and_modify(|v: &mut Vec<usize>| v.push(*id0))
                .or_insert(vec![*id0]);
        }
        Graph {
            nodes: nodes,
            adjascency: adj,
            starting_node_idx: starting_node_idx,
            ending_node_idx: ending_node_idx,
        }
    }

    /// Find all adjascent nodes to the given node index in the graph.
    ///
    /// Uses a pre-calculated adjascency list, so lookups are O(1).
    fn neighbors(&self, idx: usize) -> Vec<&Node> {
        let mut neighbor_nodes = Vec::new();
        match self.adjascency.get(&idx) {
            Some(node_ids) => {
                for id in node_ids {
                    neighbor_nodes.push(self.get(*id));
                }
            }
            None => (),
        }
        neighbor_nodes
    }

    /// Get a node at a given index.
    fn get(&self, idx: usize) -> &Node {
        &self.nodes[idx]
    }

    /// Count the number of valid traversals from the starting node to the ending node.
    ///
    /// Uses DFS to traverse all paths in the graph.
    fn get_paths_to_end_dfs(&self) -> usize {
        let mut nodes_to_search = VecDeque::new();
        let mut paths_to_end = 0;
        nodes_to_search.push_back((self.get(self.starting_node_idx), Vec::new()));

        while nodes_to_search.len() > 0 {
            let (this_node, mut path) = nodes_to_search.pop_front().unwrap();
            path.push(this_node.id);
            for neighbor in self.neighbors(this_node.id) {
                if neighbor.is_end {
                    paths_to_end += 1;
                    continue;
                }
                if !neighbor.is_large && path.contains(&neighbor.id) {
                    continue;
                }
                nodes_to_search.push_front((neighbor, path.clone()));
            }
        }
        paths_to_end
    }
}

/// Count the number of viable paths from the starting node to the ending node in a graph.
///
/// There are two types of graph nodes:
///
/// * Large nodes - can be visited any number of times in a traversal, denoted by an uppercase node name
/// * Small nodes - can be visited only once in a traversal, denoted by a lowercase node name
///
/// # Arguments
///
/// * `input_path` - The input file path containing the graph to traverse.
///
/// # Returns
///
/// The number of distinct paths from start to end.
///
/// # Examples
///
/// ## Basic
///
/// The following example has 10 paths:
///
/// ```
/// start-A
/// start-b
/// A-c
/// A-b
/// b-d
/// A-end
/// b-end
/// ```
///
/// ```
///    start
//     /   \
// c--A-----b--d
//     \   /
//      end
// ```
fn solution(input_path: &str) -> usize {
    let reader = get_buf_reader(input_path);
    Graph::from_lines(reader.lines()).get_paths_to_end_dfs()
}

/// Print the number of valid traversals from the starting node to an ending node in a graph,
/// where connections between nodes are defined in the provided input file.
///
/// Usage:
///
/// ```
/// $ aoc inputs/example.txt
/// Valid paths: 10
/// ```
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_path = parse_file_path(&args);
    let sol = solution(input_path);
    println!("Valid paths: {:?}", sol);
}

#[cfg(test)]
mod test_solution {
    use crate::solution;

    #[test]
    fn example_correct() {
        assert_eq!(solution("inputs/example.txt"), 10);
    }

    #[test]
    fn example_large_correct() {
        assert_eq!(solution("inputs/example_large.txt"), 226);
    }

    #[test]
    fn question_correct() {
        assert_eq!(solution("inputs/challenge.txt"), 3779);
    }
}
